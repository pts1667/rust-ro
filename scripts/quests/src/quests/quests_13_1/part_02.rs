use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn officer_b_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 20 {
        ctx.lines_as(
            "Officer",
            args!["Why don't you go back there?", "Are you afraid to go back?", "Khkhkh!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer",
            args![
                "Just go back to the meeting room.",
                "Your allies are there together.",
                "Khkhkh!",
                "Nevermind.",
                "Bye."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 19 {
        ctx.lines_as("Officer A", args!["So why do you come to me?", "What's wrong?"])?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["Got any business?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args![
                "I think not.",
                "You come to tell me that you're leaving for the mission.",
                "Am I right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["You were sent by Sikaiz...", "Right?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args!["Ok, I got your report.", "Just go back there.", "Good luck."],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["Just do your best", "...or whatever you do..."])?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10075), Val::from(10076)])?;
        ctx.var("ep13_ryu").set(Val::from(20))?;
        ctx.lines_as("Officer A", args!["Just return there. Good job."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 18 {
        ctx.lines_as("Officer A", args!["Sikaiz isn't understandable...", "I can't help it..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args![
                "His voice for peace is",
                "only his own way.",
                "Three kingdoms for one mission...",
                "No peace...",
                "We even get no advantage."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args!["We can't leave them as they are now. Though I am the leader among them, I'm just a figurehead."],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer A", args!["Is there any way to change their ways?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args!["If they changed,", "we wouldn't get in trouble.", "But he is not compromising."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args!["So what??", "We wouldn't get our interests,", "and just do nothing for ourselves?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["There's always Munkenro."])?;
        ctx.next()?;
        ctx.lines_as("Officer A", args!["So what...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args!["As honest as Sikaiz...", "As flexible as Sikaiz...", "But easy to go over him..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args!["We can't cut him out,", "so just take the lead from Munkenro. It's easier!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args![
                "Oh!!",
                "That's a good idea.",
                "Anyway...",
                "Some visitors are coming.",
                "Let's talk about it later."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10074), Val::from(10075)])?;
        ctx.var("ep13_ryu").set(Val::from(19))?;
        ctx.lines_as("Officer B", args!["Um, what's your business?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Officer B",
        args!["The most important things are money and power. If we don't get at least one, we're just like the slummy poor."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn officer_b(ctx: &Ctx) -> Script {
    officer_b_body(ctx, Vec::new()).map(|_| ())
}

fn rift_guard_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.lines_as("Guard", args!["Ah, you are a member of the alliance that discovered the dimensional rift? Welcome. Everything is prepared. Just enter and find the camp when you pass through the rift."])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["The new head of the alliance, Munkenro, will direct all of us."])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Then you will pass.", "Are you ready?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, sure.:Please wait.")])? {
            1 => {
                ctx.lines_as("Guard", args!["Good luck."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild22b"), Val::from(38), Val::from(195)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Guard",
                    args!["Just tell me once you are ready. I will let you pass right away."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Guard",
        args![
            "You're not allowed to enter here.",
            "If you have any business, just ask the guard behind me, or just leave."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rift_guard_1(ctx: &Ctx) -> Script {
    rift_guard_1_body(ctx, Vec::new()).map(|_| ())
}

fn rift_guard_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guard",
        args![
            "Are you the new adventurer?",
            "If you're going to the other side of the rift, the head of the alliance will guide you there."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Guard",
        args!["He should be near the center of the Gorge. He is stronger than the former one, so be careful with your words."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Guard",
        args!["If you want to run away from here, just let me know, I will guide you."],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Escape.:Nah.")])? {
        1 => {
            ctx.lines_as("Guard", args!["Ok.", "Follow me, I will let you out of here."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("moc_fild20"), Val::from(342), Val::from(179)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Guard", args!["Ok, just go ahead for yourself."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn rift_guard_2(ctx: &Ctx) -> Script {
    rift_guard_2_body(ctx, Vec::new()).map(|_| ())
}

fn rift_guard_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guard",
        args![
            "Are you new here?",
            "You look like you need to find the head of the alliance. You can find him near the center of the dimensional rift."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Guard", args!["Just right there.", "There's a pillar of stone.", "Very close!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rift_guard_3(ctx: &Ctx) -> Script {
    rift_guard_3_body(ctx, Vec::new()).map(|_| ())
}

fn munkenro_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 100 {
        ctx.lines_as("Munkenro", args!["Are you ready to discover Ash-Vacuum?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, of course!:Please, wait...")])? {
            1 => {
                ctx.lines_as(
                    "Munkenro",
                    args![
                        "I wish you good luck!",
                        "I hope you are successful in your adventures in Ash-Vacuum!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Munkenro",
                    args![
                        "(...I just follow your lead!)",
                        "(I remember you're strong-willed.)",
                        "(Though I am guilty,)",
                        "(I will do my best.)"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("mid_camp"), Val::from(210), Val::from(291)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Munkenro", args!["Go prepare."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_ryu").get()? == 20 {
        ctx.lines_as(
            "Munkenro",
            args!["I am the head of the Three Kingdoms Alliance. You've come here for the discovery of the dimensional rift. Am I right?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args![
                "Um. You're the one that",
                "did some favors before...",
                ((Val::from("Your name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                "I remember you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["Sikaiz retired from his position urgently. I am his replacement."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["You don't need to know in detail all about it now. That's not important to you. Is it?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["You are here for the discovery of the dimensional rift. I can't send you there right away. I know that you've passed your tests, but that's the past. You have not been tested by me. ~"])?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["So you need to be tested again. I don't have too much time to test you now. So let's just do a short test."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["Now the soldiers here will start to attack you... If you survive, I will approve of you. Just do it in good time."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["This can prove that you're strong enough. That's the minimum test I can perform."],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10076), Val::from(10077)])?;
        ctx.var("ep13_ryu").set(Val::from(21))?;
        ctx.lines_as("Munkenro", args!["Once you get ready,", "just let me know."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 21 {
        ctx.lines_as("Munkenro", args!["Are you ready??", "Let's start!"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ready!!:Please, wait...")])? {
            1 => {
                ctx.lines_as("Munkenro", args!["Ok, just have a good adventure."])?;
                ctx.next()?;
                ctx.var("ep13_ryu").set(Val::from(22))?;
                if ctx.call(Function::IsBeginQuest, vec![Val::from(10077)])? == 1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(10077), Val::from(10078)])?;
                }
                ctx.call(Function::DoNpcEvent, vec![Val::from("Head of the Alliance#moo::OnEnable")])?;
                ctx.lines_as(
                    "Munkenro",
                    args!["If you are too late,", "it will be considered as a failure, so come back soon."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Munkenro",
                    args![
                        "If you hesitate,",
                        "it means you fail.",
                        "I will give you one more chance. Go ahead."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("$@ep13_test").get()? == 2 {
        ctx.lines_as(
            "Munkenro",
            args![
                "You're quite shrewd...",
                "How come you passed the test",
                "before? I can't believe this."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["I will give you one more chance.", "Don't make any mistakes again."],
        )?;
        ctx.next()?;
        ctx.var("ep13_ryu").set(Val::from(21))?;
        ctx.var("$@ep13_test").set(Val::from(0))?;
        ctx.lines_as("Munkenro", args!["Once you get ready,", "just let me know,", "to start!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("$@ep13_test").get()? == 1 {
        ctx.lines_as(
            "Munkenro",
            args!["We are testing the other adventurers, so unless you have any business with me, visit me later."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("$@ep13_test").get()? == 0 && ctx.var("ep13_ryu").get()? == 22) {
        ctx.lines_as(
            "Munkenro",
            args![
                "Good! You did a good job!",
                "Though we didn't get enough time to test, I think it's almost perfect."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["You can feel I am too strict... I'm just doing what I need to do as head of the Alliance."],
        )?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["If I can't manage the alliance well, I will feel guilty, that I am not doing Sikaiz's job any justice. Anyway, I will do my best."])?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["Huh? I'm just whining.", "You don't need to hear this.", "Just forget it."],
        )?;
        ctx.next()?;
        ctx.call(Function::CompleteQuest, vec![Val::from(10078)])?;
        ctx.var("ep13_ryu").set(Val::from(100))?;
        ctx.call(Function::GetExperience, vec![Val::from(660000), Val::from(210000)])?;
        ctx.lines_as("Munkenro", args!["I will let you go there.", "Let me know once you are ready."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn munkenro_2(ctx: &Ctx) -> Script {
    munkenro_2_body(ctx, Vec::new()).map(|_| ())
}

fn head_of_the_alliance_moo_run(ctx: &Ctx, mut step: HeadOfTheAllianceMooStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HeadOfTheAllianceMooStep::Start => {
                step = HeadOfTheAllianceMooStep::OnInit;
                continue 'machine;
            }
            HeadOfTheAllianceMooStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Head of the Alliance#moo")])?;
                return Err(Stop::End);
            }
            HeadOfTheAllianceMooStep::OnEnable => {
                ctx.call(Function::MapAnnounce, vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance: I command that all soldiers attack the adventurers around here for 15 minutes. This is an order!"), ctx.constant("BC_MAP")?])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Head of the Alliance#moo")])?;
                ctx.var("$@ep13_test").set(Val::from(1))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("moc_fild22b"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Allied Soldier"),
                        Val::from(1851),
                        Val::from(80),
                        Val::from("Head of the Alliance#moo::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeadOfTheAllianceMooStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance#moo::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            HeadOfTheAllianceMooStep::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance#moo::OnMyMobDead")],
                )?;
                ctx.call(Function::MapAnnounce, vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance: All alliance members. All of you should stop your attacks and return to work. That's all."), ctx.constant("BC_MAP")?])?;
                ctx.var("$@ep13_test").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Head of the Alliance#moo")])?;
                return Err(Stop::End);
            }
            HeadOfTheAllianceMooStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance#moo::OnMyMobDead")],
                    )?
                    .number()?
                    < 41
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Head of the Alliance#moo::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            HeadOfTheAllianceMooStep::OnTimer900000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance#moo::OnMyMobDead")],
                )?;
                ctx.call(Function::MapAnnounce, vec![Val::from("moc_fild22b"), Val::from("Head of the Alliance: All alliance members. All of you should stop your attacks and return to work. That's all."), ctx.constant("BC_MAP")?])?;
                ctx.var("$@ep13_test").set(Val::from(2))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Head of the Alliance#moo")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn head_of_the_alliance_moo(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::Start, Vec::new()).map(|_| ())
}

pub fn head_of_the_alliance_moo_oninit(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::OnInit, Vec::new()).map(|_| ())
}

pub fn head_of_the_alliance_moo_onenable(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn head_of_the_alliance_moo_onreset(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::OnReset, Vec::new()).map(|_| ())
}

pub fn head_of_the_alliance_moo_ondisable(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn head_of_the_alliance_moo_onmymobdead(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn head_of_the_alliance_moo_ontimer900000(ctx: &Ctx) -> Script {
    head_of_the_alliance_moo_run(ctx, HeadOfTheAllianceMooStep::OnTimer900000, Vec::new()).map(|_| ())
}

fn rift_guard_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guard",
        args![
            "Are you the new adventurer?",
            "If you're going to the other side of the rift, the head of the alliance will guide you there."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Guard",
        args!["He should be near the center of the Gorge. He is stronger than the former one, so be careful with your words."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Guard",
        args!["If you want to run away from here, just let me know, I will guide you."],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Escape.:Nah.")])? {
        1 => {
            ctx.lines_as("Guard", args!["Ok.", "Follow me, I will let you out of here."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("moc_fild20"), Val::from(342), Val::from(179)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Guard", args!["Ok, just go ahead for yourself."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn rift_guard_4(ctx: &Ctx) -> Script {
    rift_guard_4_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_guard_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 100 {
        ctx.lines_as("Guard", args!["- Trembling in fear. -"])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["- Trembling in fear. -"])?;
        ctx.next()?;
        ctx.lines_as(
            "Guard",
            args![
                "Honestly, I should keep",
                "to the dimensional rift,",
                "but it's quite scary.",
                "I ran away from it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard",
            args![
                "Ah?",
                "You're quite brave!",
                "And not scared about things around you. I envy you..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard",
            args![
                "I am a coward but,",
                "I can send you through the dimensional rift. That's the only thing I can do for you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard",
            args!["I send you there easily,", "but don't tell others about me. Ok??"],
        )?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Then, do you want to go through the Time-Space Gap now??"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No, thanks.")])? {
            1 => {
                ctx.lines_as(
                    "Guard",
                    args!["Ok, then I will send you there. As I already said, don't tell anybody about me. Anyway... good luck!!"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild22b"), Val::from(49), Val::from(195)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Guard", args!["It's ok.", "Just don't tell anybody about me. Please."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as("Guard", args!["- Trembling in fear. -"])?;
    ctx.next()?;
    ctx.lines_as(
        "Guard",
        args![
            "onestly, I should keep",
            "to the dimensional rift,",
            "but it's quite scary.",
            "I ran away from it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Guard",
        args![
            "- Trembling. -",
            "Please keep the secret",
            "about me being here.",
            "I am so scared!!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn time_space_gap_guard(ctx: &Ctx) -> Script {
    time_space_gap_guard_body(ctx, Vec::new()).map(|_| ())
}

fn allied_manager_gm_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Manager", args!["Please enter the password."])?;
    ctx.next()?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(8028), Val::from(0), Val::from(0), Val::from(9000)])?;
    if l_i.clone() == -2 {
        ctx.lines_as("Manager", args!["Incorrect password."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == -1 {
        ctx.lines_as("Manager", args!["Please enter a password other then 0."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == 0 {
        ctx.lines_as("Manager", args!["Nevermind then."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Manager", args!["What would you like to do?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Reset the Allied Attacks.:Nothing.")])? {
            1 => {
                ctx.lines_as("Manager", args!["Resetting the allied attacks."])?;
                ctx.var("$@ep13_test").set(Val::from(2))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Head of the Alliance#moo")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Manager", args!["Nevermind then."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn allied_manager_gm(ctx: &Ctx) -> Script {
    allied_manager_gm_body(ctx, Vec::new()).map(|_| ())
}

fn marian_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    if (ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0
        || (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 1000)
    {
        ctx.lines_as("Marian", args!["You have too many items~", "Drop some and come back to me."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()?.number()? < 1 {
        if (ctx.var("ep13_ryu").get()?.number()? > 99 || ctx.var("ep13_start").get()?.number()? > 99) {
            ctx.lines_as(
                "Marian",
                args!["You must be a stranger here.", "Is this your first visit here?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                1 => {
                    ctx.lines_as(
                        "Marian",
                        args!["As expected~", "I'm not lying, but usually", "I never forget a face."],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marian",
                        args![
                            "Well, anyway...",
                            "If this is your first time here,",
                            "you'd better see ^0000FFInstructor Lugen^000000.",
                            "He can help you adjust",
                            "to these surroundings."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Marian", args!["Go to the right to find", "^0000FFInstructor Lugen^000000."])?;
                    ctx.var("ep13_newbs").set(Val::from(1))?;
                    ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(11084)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Marian", args!["Well~ It is weird~"])?;
                    ctx.next()?;
                    ctx.lines(args![" - ransacking - ", " - flap - "])?;
                    ctx.next()?;
                    ctx.lines_as("Marian", args!["Hmm, you're not", "from around here."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marian",
                        args![
                            "Go to the right along this way",
                            "and you'll find ^0000FFInstructor Lugen^000000.",
                            "Talk to him, first."
                        ],
                    )?;
                    ctx.var("ep13_newbs").set(Val::from(1))?;
                    ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(11084)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Marian", args!["......", "I'm sorry but,", "I'm a bit busy now..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 1 {
        ctx.lines_as("Marian", args!["Go to the right to see ^0000FFInstructor Lugen^000000."])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_newbs").get()?.number()? > 1 && ctx.var("ep13_newbs").get()?.number()? < 13) {
        ctx.lines_as(
            "Marian",
            args!["Wow, good to see you again.", "Do you feel comfortable around here?"],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 13 {
        ctx.lines_as("Marian", args!["Hey, how are you?", "What brings you here?"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["The instructor sent me.", "He told me to get supplies from the motherland."],
        )?;
        ctx.next()?;
        ctx.lines_as("Marian", args!["Ah, you must be talking about~?", "Let me see~~"])?;
        ctx.next()?;
        ctx.lines_as("Marian", args!["This and that~~"])?;
        ctx.next()?;
        ctx.lines_as("Marian", args!["Hmm, not this one."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marian",
            args!["Ah, I got it!", "Here they are!", "And this is for you.", "Go, go~~"],
        )?;
        ctx.var("ep13_newbs").set(Val::from(14))?;
        ctx.call(Function::GetItem, vec![Val::from(6045), Val::from(3)])?;
        ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11091), Val::from(11092)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_newbs").get()?.number()? > 13 && ctx.var("ep13_newbs").get()?.number()? < 20) {
        if ctx.var("ep13_newbs").get()?.number()? < 16 {
            l_i = Val::from(3);
        } else if ctx.var("ep13_newbs").get()?.number()? < 18 {
            l_i = Val::from(2);
        } else {
            l_i = Val::from(1);
        }
        if runtime::op(&ctx.call(Function::CountItem, vec![Val::from(6045)])?, "<", &l_i.clone())?.is_true() {
            ctx.lines_as("Marian", args!["Ha! You misplaced the Supply Box?!", "What the~~?"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Marian",
                args!["Ok, I have spares.", "Let me give one to you.", "Don't misplace this one, ok?!"],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(6045), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Marian",
                args!["Do your job, man~", "And please say hello", "to the instructor."],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Marian",
            args!["Hey~", "How have you been recently?", "I am busy all the time..."],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Marian",
            args![
                "The supplies from the motherland are...",
                "Ah...",
                "Nothing, do not care...",
                "Hohoho..."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn marian_ep13bs(ctx: &Ctx) -> Script {
    marian_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn instructor_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines_as(
            "Instructor Lugen",
            args!["You are carrying too much weight.", "Please try again after losing some weight."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()?.number()? < 1 {
        ctx.lines_as(
            "Instructor Lugen",
            args!["Huuu, no time to rest.", "When re-enforcements come."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 1 {
        ctx.lines_as(
            "Instructor Lugen",
            args!["Huuu, no time to rest.", "When re-enforcements come."],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me."])?;
        ctx.next()?;
        ctx.lines_as("Instructor Lugen", args!["Hi!", "What brings you here?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Marian sent me to here."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "Ah!",
                "Are you from the motherland?",
                "I am Lugen Jednic from.",
                "the federal survey team,",
                "in charge of educating.",
                "new employees."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Instructor Lugen", args!["To join the team,", "you simply need to register."])?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "Once you register, you",
                "will be provided with",
                "ccommodations and food.",
                "There will be plenty of",
                "missions for you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Instructor Lugen", args!["Will you join the survey team?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
            1 => {
                ctx.lines_as(
                    "Instructor Lugen",
                    args![
                        "Registration is simple.",
                        "Go to the big building.",
                        "in the center and talk",
                        "to the receptionist."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Instructor Lugen",
                    args![
                        "When you finish the",
                        "registration, come back",
                        "to me and I will assign",
                        "your accomodations."
                    ],
                )?;
                ctx.var("ep13_newbs").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11084), Val::from(11085)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Instructor Lugen",
                    args!["You don't want to join?", "If you change your mind, come back later."],
                )?;
                ctx.var("ep13_newbs").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_newbs").get()? == 2 {
        ctx.lines_as("Instructor Lugen", args!["Ah, you came back.", "I have been waiting for you."])?;
        ctx.next()?;
        ctx.lines_as("Instructor Lugen", args!["Will ou join the survey team?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I will register..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Instructor Lugen",
                    args![
                        "[Instructor Lugen]",
                        "Registration is simple.",
                        "Go to the big building.",
                        "in the center and talk",
                        "to the receptionist."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Instructor Lugen",
                    args![
                        "When you finish the",
                        "registration, come back",
                        "to me and I will assign",
                        "your accomodations."
                    ],
                )?;
                ctx.var("ep13_newbs").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11084), Val::from(11085)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Instructor Lugen",
                    args!["You don't want to join?", "If you change your mind, come back later."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_newbs").get()? == 3 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "To register, Go to the",
                "big building in the center",
                "and talk to the receptionist."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "When you finish the",
                "registration, come back",
                "to me and I will assign",
                "your accomodations."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 4 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "I got a call from the receptionist.",
                "Now take your nameplate and go",
                "to your accomodations down",
                "in the barracks."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args!["So take some rest and I will", "see you tomorrow morning."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "You need to share the barracks",
                "with others, so make sure to",
                "talk with them while you're there."
            ],
        )?;
        ctx.var("ep13_newbs").set(Val::from(5))?;
        ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
        ctx.call(Function::CompleteQuest, vec![Val::from(11086)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_newbs").get()?.number()? > 4 && ctx.var("ep13_newbs").get()?.number()? < 12) {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "You need to share the barracks",
                "with others, so make sure to",
                "talk with them while you're there."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 12 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "How was your stay here?",
                "It is cold. Warm yourself",
                "and try not to catch a cold."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![((Val::from("And I have a request of you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "It is to distribute supplies",
                "from the motherland,",
                "It's not very difficult.",
                "Just think of it as a way",
                "to meet people here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "Anyway, Go to Marian,",
                "whom you met when",
                "you first came here and",
                "get the supplies from her."
            ],
        )?;
        ctx.var("ep13_newbs").set(Val::from(13))?;
        ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
        ctx.call(Function::SetQuest, vec![Val::from(11091)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 13 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "Anyway, Go to Marian,",
                "whom you met when",
                "you first came here and",
                "get the supplies from her."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 14 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 2 {
            ctx.lines_as(
                "Instructor Lugen",
                args!["Did you receive the supplies?", "Can you show them to me, please?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["Hmm..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Instructor Lugen",
                args!["Ok, then, please deliver", "them to their receivers."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "The first box should be sent to",
                    "Jan, northwest of the camp.",
                    "She may be around the fence."
                ],
            )?;
            ctx.var("ep13_newbs").set(Val::from(15))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11092), Val::from(11093)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "It seems that they",
                    "are not enough.",
                    "Can you check them",
                    "with Marian, please?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 15 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 2 {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "The first box should be sent to",
                    "Jan, northwest of the camp.",
                    "She may be around the fence."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "It seems that they",
                    "are not enough.",
                    "Can you check them",
                    "with Marian, please?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 16 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "So, did you get the",
                "supplies to Jan?",
                "She was waiting",
                "for it a month ago.",
                "Thanks so much."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 1 {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "Next, this box is for Gerard.",
                    "He is investigating across the",
                    "west bridge outside the camp."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["Freya be with you."])?;
            ctx.var("ep13_newbs").set(Val::from(17))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11094), Val::from(11095)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "It seems that they",
                    "are not enough.",
                    "Can you check them",
                    "with Marian, please?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 17 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 1 {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "Next, this box is for Gerard.",
                    "He is investigating across the",
                    "west bridge outside the camp."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "It seems that they",
                    "are not enough.",
                    "Can you check them",
                    "with Marian, please?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 18 {
        ctx.lines_as("Instructor Lugen", args!["Well done.", "Is Gerard well?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "I've been worrying",
                "about him since he",
                "disappeared after",
                "going out for the",
                "investigation."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 0 {
            ctx.lines_as(
                "Instructor Lugen",
                args!["And the last box is for", "Alberto at the entrance", "of the eastern field."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "Perhaps, he ordered",
                    "the heavy coat.",
                    "It's freezing there so",
                    "you'd better hurry."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["Please deliver it to him."])?;
            ctx.var("ep13_newbs").set(Val::from(19))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11096), Val::from(11097)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "It seems that they",
                    "are not enough.",
                    "Can you check them",
                    "with Marian, please?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 19 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 0 {
            ctx.lines_as(
                "Instructor Lugen",
                args!["And the last box is for", "Alberto at the entrance", "of the eastern field."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "It seems that they",
                    "are not enough.",
                    "Can you check them",
                    "with Marian, please?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 20 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "It was really good to request",
                ((Val::from("you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "Your work has been",
                "a great help to us.",
                "I really appreciate it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "You must be tired now,",
                "please take care of yourself.",
                "I have more things",
                "to do now, ha..."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.var("ep13_newbs").set(Val::from(21))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(11098)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 21 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
            ctx.lines_as("Instructor Lugen", args!["There must be no", "time to rest for me.", "Huu...~"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["What is wrong with you?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["Ah, nothing...", "I'm just talking to myself."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I've never seen you sigh.", "If you have something", "to say please tell me."],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["It's just a personal thing..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Tell me.", "I think I can help you."],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["I-I'm, sorry to tell you but..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "I came here together",
                    "with my friend but we",
                    "aren't allowed to leave,",
                    "so I don't know how he is."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Instructor Lugen",
                args!["It was rumored that", "he's gotten worse so", "I'm worried about him."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Instructor Lugen",
                args!["But, I'm so busy working", "and it makes me...", "*sigh*"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Help.:Just listen.")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I will help with your problem!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Instructor Lugen", args!["No, it is ok."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Don't say no!",
                            "It's not a problem at all!",
                            "So, I'll go to him for you",
                            "and see if he is ok, alright?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Instructor Lugen", args!["......"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Hey, I really want to help~"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Instructor Lugen",
                        args![
                            "......",
                            "I really appreciate that.",
                            "If so, can you please",
                            "deliver this letter to him?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You can count on me!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Instructor Lugen",
                        args![
                            "My friend, Otto, is working",
                            "on the barrier in the west.",
                            "Please give it to him."
                        ],
                    )?;
                    ctx.var("ep13_newbs").set(Val::from(23))?;
                    ctx.call(Function::GetItem, vec![Val::from(6043), Val::from(1)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(11099)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Ah, ya work is tough."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Instructor Lugen",
                        args!["Yes.. I really appreciate your listening to me.", "It makes me feel better."],
                    )?;
                    ctx.var("ep13_newbs").set(Val::from(22))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Instructor Lugen", args!["There must be no", "time to rest for me.", "Huu...~"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 22 {
        ctx.lines_as("Instructor Lugen", args!["There must be no", "time to rest for me.", "Huu...~"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Help him.:Just pass.")])? {
            1 => {
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["May I help you?"])?;
                ctx.next()?;
                ctx.lines_as("Instructor Lugen", args!["No, it's ok."])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Don't say no!",
                        "It is not difficult at all!",
                        "So, I'm supposed to go to him and see if he's ok, right?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Instructor Lugen", args!["......"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hey, I really want to help~"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Instructor Lugen",
                    args![
                        "......",
                        "I really appreciate that.",
                        "If so, can you please deliver this letter to him?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["You believe me!!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Instructor Lugen",
                    args!["My friend, Otto, is working on the barrier in the west.", "Please give it to him."],
                )?;
                ctx.var("ep13_newbs").set(Val::from(23))?;
                ctx.call(Function::GetItem, vec![Val::from(6043), Val::from(1)])?;
                ctx.call(Function::SetQuest, vec![Val::from(11099)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hmm..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_newbs").get()? == 23 {
        if ctx.call(Function::CountItem, vec![Val::from(6043)])?.number()? < 1 {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "Ah, did you lose the letter?",
                    "That's ok. It's no big deal.",
                    "I'll write it again."
                ],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(6043), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "My friend, Otto, is working",
                    "on the barrier in the west.",
                    "Please give it to him."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 24 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Instructor! I met Otto!", "He is doing very well~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "Phew, it's really good to hear.",
                "I was so worried about him",
                "being sick and far away",
                "from his homeland."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I have a letter from him, too."],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6044)])?.number()? < 1 {
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh, ehh..?!?!"])?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["......"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Instructor Lugen",
                args![
                    "I am far away from my home",
                    "the feeling that family and",
                    "friends give to me are really important."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["I don't know how to thank you."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hehe, don't mention it.", "And Otto said that he", "would come to see you."],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_SHY")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["Did he?", "It's been so long", "since we last met."])?;
            ctx.next()?;
            ctx.lines_as("Instructor Lugen", args!["And this is for you."])?;
            ctx.call(Function::DelItem, vec![Val::from(6044), Val::from(1)])?;
            ctx.var("ep13_newbs").set(Val::from(100))?;
            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(5)])?;
            ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(100000)])?;
            ctx.call(Function::CompleteQuest, vec![Val::from(11100)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 100 {
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "Thanks for doing that.",
                "My friend wrote me that he",
                "would have some spare time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "It is good to see you,",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                "enjoy yourself here.",
                "If you have any troubles,",
                "come to see me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Lugen",
            args![
                "I so happy that I've",
                "met a person like",
                ((Val::from("you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Instructor Lugen", args!["It seems that I nave no time to rest this week."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn instructor_ep13bs(ctx: &Ctx) -> Script {
    instructor_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn otto_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 23 {
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Mr. Otto?!"])?;
        ctx.next()?;
        ctx.lines_as("Otto", args!["Hmm, how can I help you?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Instructor Lugen sent me!", "He is worrying about you getting worse."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Otto",
            args![
                "Oh, is he...",
                "It was just a cold.",
                "It was nothing.",
                "He's still overprotective.."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That's good to hear.", "Ah, and he wrote this letter for you."],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6043)])?.number()? < 1 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Eh, where is it..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Otto", args!["......"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Otto",
                args!["We are not allowed to leave our posts,", "so I am wondering about him, as well."],
            )?;
            ctx.next()?;
            ctx.lines_as("Otto", args!["A letter, I never expected this, huhu."])?;
            ctx.next()?;
            ctx.lines(args!["- Otto is reading the letter -", "- with the smile on his face. -"])?;
            ctx.next()?;
            ctx.lines_as(
                "Otto",
                args![
                    "I am sorry but, can you please",
                    "tell him that I am ok now by",
                    "giving him this letter."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Otto",
                args![
                    "And I will have a vacation",
                    "in the near future and will",
                    "use that time to visit him."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(6043), Val::from(1)])?;
            ctx.var("ep13_newbs").set(Val::from(24))?;
            ctx.call(Function::GetItem, vec![Val::from(6044), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11099), Val::from(11100)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 24 {
        if ctx.call(Function::CountItem, vec![Val::from(6044)])?.number()? < 1 {
            ctx.lines_as("Otto", args!["You lost the letter?", "I'll write it again."])?;
            ctx.call(Function::GetItem, vec![Val::from(6044), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Otto",
                args![
                    "I am sorry but, can you please",
                    "tell him that I am ok now by",
                    "giving him this letter."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Otto",
                args![
                    "And I will have a vacation",
                    "in the near future and will",
                    "use that time to visit him."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()?.number()? > 99 {
        ctx.lines_as(
            "Otto",
            args![
                "Thanks to you,",
                "I was able to meet",
                "him some days ago.",
                "He, wasn't feeling good, either."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Otto",
            args![
                "It seems that I will",
                "have more free time,",
                "so I will take that time",
                "to visit him more often."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Otto", args!["This place has a totally different environment on both sides."])?;
        ctx.next()?;
        ctx.lines_as("Otto", args!["Is it natural,", "or artificial?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn otto_ep13bs(ctx: &Ctx) -> Script {
    otto_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn receptionist_brink_ep13b_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()?.number()? < 3 {
        ctx.lines_as("Brink", args!["Hmm... Hey...", "...I mean...", "Hmm...", "...I would..."])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 3 {
        ctx.lines_as("Brink", args!["Hmm... Hey...", "...What makes...", "Hmm...", "...I would..."])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.next()?;
        ctx.lines_as("Brink", args!["Hmmm... Hey...", "...register...", "I mean..."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I am here to register.:Just stand.")])? {
            1 => {
                ctx.lines_as("Brink", args!["Ah... I mean...", "Hmm...", "...Hey...", "What...your name..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![((Val::from("My name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Brink",
                    args![
                        "...Ah...",
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                        "You...",
                        "...registered...Hmm...",
                        "..."
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.next()?;
                ctx.lines_as("Brink", args!["Here...", "Hmm...", "sign..."])?;
                ctx.next()?;
                ctx.lines_as("Brink", args!["Hmmm...", "and...", "I mean..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Is it finished?"])?;
                ctx.next()?;
                ctx.lines_as("Brink", args!["Hm...", "...Yes...", "Then..."])?;
                ctx.next()?;
                ctx.lines_as("Brink", args!["And...", "......This thing...", "...Ehh..."])?;
                ctx.next()?;
                ctx.lines(args!["- I guess I'm registered -", "- I'd better go back to Lugen -"])?;
                ctx.var("ep13_newbs").set(Val::from(4))?;
                ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11085), Val::from(11086)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Brink", args!["And...", "......This thing...", "...Ehh..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Brink", args!["...Hey...", "I mean...", "...Well...nice...", "Then..."])?;
        ctx.next()?;
        ctx.lines_as("Brink", args!["...Life...", "...Hmmm..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn receptionist_brink_ep13b(ctx: &Ctx) -> Script {
    receptionist_brink_ep13b_body(ctx, Vec::new()).map(|_| ())
}
