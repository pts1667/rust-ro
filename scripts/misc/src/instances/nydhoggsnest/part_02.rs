use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn laphine_prisoner_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Laphine Prisoner]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ins_nyd").get()? == 5 {
            ctx.mes("You... are not of the Sapha tribe... Are you... an outsider?")?;
            ctx.next()?;
            ctx.lines_as(
                "Laphine Prisoner",
                args!["Have you ever come in contact with the Laphine tribe? Have you ever been to Splendide?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Laphine Prisoner",
                args!["Please, talk to my people in Splendide. They will come to help me."],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "What happened to you?:What's in the cave to the north?:Tell me about your tribe.:I will leave you alone.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["During out last battle with the Sapha tribe... they caught me and took me prisoner."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["The Sapha tribe attacked first. They are destroying the World Tree."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["That's why the war started. But I don't want to fight with the Sapha tribe anymore..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Laphine Prisoner", args!["If it wasn't for that... I wouldn't survive here..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["Please. Bring this news to all of Splendide. Send someone to save me... or they might kill me."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Laphine Prisoner", args!["There is..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["No... I can't tell you... It's the secret of our tribe..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["I just want to say this...if the Sapha tribe intrude that place, we will never forgive them!"],
                    )?;
                    ctx.var("ins_nyd").set(Val::from(52))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["The Laphines have protected the Yggdrasil World Tree for generations."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["We live in a... different time, different land... we came for an expedition."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Laphine Prisoner", args!["After we received reports from our spy about the World Tree's strange symptoms, we recgnized the Manuk's existence."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["They are mining metal, and in so doing, destroying the World Tree's roots..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laphine Prisoner",
                        args!["So we dispatched an expedition here. Soonafter, the war with the Sapha began..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("ins_nyd").get()? == 92 {
                ctx.mes("The sapha tribe went as far as cooperating with an unknown race?")?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["What is your purpose? Why do you want to know what that place is?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["lright...it's as you have assumed, a place connecting to the World Tree. It's also the Guardian's Nest."],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Guardian's Nest?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Yes, that's the nest of Nidhoggur, the Guardian of the World Tree..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Only a marked guardian's servant from the Laphine tribe is allowed to enter."],
                )?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["That place was initially closed off from the public, but then the giants of the Sapha tribe turned things around..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["To obtain more minerals, they started to dig with madness."],
                )?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["In the end, they have harmed a part of the World Tree's root that lies very close to their mine, and thus, the World Tree became very ill."])?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["After we have arrived here knowing the facts, the World Tree was already in a very bad state due to the many factories built by the Sapha tribe."])?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["Even though, for a long time, we have been attacking the Sapha tribe, trying to force them to leave, they are not so easily defeated, leading to this stalemate."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["During this time, your people have travelled through the space-time gap to this land."],
                )?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["Because of those many unexpected reasons, the Guardian's nest, which was meant to stay hidden deep underground, has been discovered by your race..."])?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["Now you understand how serious this situation is. Because of the Sapha tribe, we are no longer able to approach the Guardian, and seek his teachings of wisdom."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Hmph...even if I don't cooperate, I know you will find other means to get in..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["Looking at the current situation of the Laphine tribe, we do not have enough power to surround the Manuk fields, and chase the giants away."])?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["Okay, if it has already become like this, I will help you. There are no dead-ends. If you keep going, you will find a solution."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Even if I don't say anything, my situation will not change."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["I have already told you everything you wanted to hear. If you have any other requests, please let me know."],
                )?;
                ctx.var("ins_nyd").set(Val::from(102))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ins_nyd").get()? == 102 {
                ctx.mes("Ah, you said that there is a strange power blocking the entrance to the Guardian's nest, right?")?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["That's because only the ones chosen by the Guardian may enter."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["We have been the Guardian's servants for generations, and have been protecting the World Tree ever since."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args![
                        "Before we were captured and brought here, there were 3 servants including me. Our task was to heal the World Tree."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["One was killed in our last war with the Sapha tribe...and I have been taken captive."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["I will give you my proof. Don't worry, I'm just temporarily marking you as a guardian's servant."],
                )?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Also, remember this spell, it's needed to open the gate of the Guardian."],
                )?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["AnomarDu Ha OdesUdenVer Ie "])?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["remuAlaAsh Mu ModtasAn Yu Dur"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["TalsehrDur So CyaReMush Di DielAlaWos Ie RuffserIman Ie "],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Go find the fairy guarding the gate, and say this spell."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["he proof and the spell will confirm that you are one of the Guardian's servants."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Please meet the Guardian, and come back with an answer to everything. And tell the answer to..."],
                )?;
                ctx.var("ins_nyd").set(Val::from(112))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ((((((ctx.var("ins_nyd").get()? == 71 || ctx.var("ins_nyd").get()? == 81) || ctx.var("ins_nyd").get()? == 91)
                || ctx.var("ins_nyd").get()? == 101)
                || ctx.var("ins_nyd").get()? == 111)
                || ctx.var("ins_nyd").get()? == 121)
                || ctx.var("ins_nyd").get()? == 131)
            {
                ctx.mes("... ...")?;
                ctx.next()?;
                ctx.lines_as("Manuk Guard", args!["Hey, outsider! Step away from the prisoner!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ins_nyd").get()? == 112 {
                ctx.mes("Please meet the Guardian, and come back with an answer to everything. And tell the answer to...")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ins_nyd").get()? == 132 {
                ctx.mes("Yes...I heard your conversation with the Sapha tribe...")?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["If our tribe were to really trust in the Sapha tribe...I don't know."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["Our hatred towards them has already reached an abnormal level..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Laphine Prisoner", args!["Those giants...can they really be trusted?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine Prisoner",
                    args!["It's...it's better if you don't believe the one called Etorr...the minds of the Sapha tribe is unpredictable."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.mes("... ...")?;
                ctx.next()?;
                ctx.lines_as("Manuk Guard", args!["Hey, outsider! Step away from the prisoner!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.mes("AmanVilShar Ie DorLuShar Mu Re")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn laphine_prisoner_edq(ctx: &Ctx) -> Script {
    laphine_prisoner_edq_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapEdq2Step {
    Start,
    OnTouch,
}

fn trap_edq2_run(ctx: &Ctx, mut step: TrapEdq2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapEdq2Step::Start => {
                step = TrapEdq2Step::OnTouch;
                continue 'machine;
            }
            TrapEdq2Step::OnTouch => {
                if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ins_nyd").get()? == 52) {
                    ctx.lines_as(
                        "Manuk Field Elite Soldier",
                        args!["This is Neat Etorr's order. Please come with us to see him."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manuk Field Elite Soldier",
                        args!["Bring him to Neat Etorr. All soldiers be prepared."],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("man_in01"), Val::from(311), Val::from(54)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_edq2(ctx: &Ctx) -> Script {
    trap_edq2_run(ctx, TrapEdq2Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_edq2_ontouch(ctx: &Ctx) -> Script {
    trap_edq2_run(ctx, TrapEdq2Step::OnTouch, Vec::new()).map(|_| ())
}

fn murdered_yggdrasilid_1f_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_exit = Val::from(0);
    if ctx.var("'ins_nyd2").get()? == 0 {
        ctx.mes("When a faint light enters your heart, a voice sounds in your head.")?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree World Tree Yggdrasil",
            args!["It's all over... servants of the Guardian... Hurry up and leave this place."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(ctx, &[Val::from("Who are you?:What do you mean?")])? {
                    1 => {
                        ctx.lines_as("World Tree World Tree Yggdrasil", args!["I... I am the World Tree Yggdrasil, servant of the Guardian of Nidhoggur, as well as the High Priest leading the Laphine Tribe."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "World Tree World Tree Yggdrasil",
                            args![
                                "So you're not priestess of the Laphine Tribe... How did you get in? No, there's no time to answer that."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "World Tree World Tree Yggdrasil",
                            args!["Hurry... and leave... leave this place before it's too late."],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "World Tree World Tree Yggdrasil",
                            args!["The guardian... something's wrong with the guardian. I don't know what made him like this."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "World Tree World Tree Yggdrasil",
                            args!["This... this is no longer the nest of the Guardian of the World Tree Yggdrasil."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("World Tree Yggdrasil", args!["Darkness took over the Guardian and destroyed all living things... now this place has become the cursed home of monsters."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "World Tree Yggdrasil",
                            args!["Now the vile Nidhoggur's Shadow is wreaking havoc here..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("World Tree Yggdrasil", args!["Now's not too late, hurry and get out... tell the Laphine Tribe about this... tell the commanders of Alfheim..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "World Tree Yggdrasil",
                            args!["My soul... it has been trapped here. You're the only ones I can trust now."],
                        )?;
                        ctx.next()?;
                        l_exit = Val::from(1);
                    }
                    _ => {}
                }
                if l_exit.clone().is_true() {
                    break 'l1;
                }
            }
        }
        let choice = runtime::select_values(ctx, &[Val::from("Nidhoggur's Shadow?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("World Tree Yggdrasil", args!["The Guardian Nidhoggur... he's not in the nest."])?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["He...for some reason abandoned his own shadow, and left."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["All that's left, is the ugly Shadow of the Guardian of Nidhoggur...the Shadow that is going mad."],
        )?;
        ctx.next()?;
        ctx.lines_as("World Tree Yggdrasil", args!["The Shadow sucked all the nutrients from the World Tree Yggdrasil, and has gone mad when there is nothing more left to obtain. Now, he wants this land."])?;
        ctx.next()?;
        ctx.lines_as("World Tree Yggdrasil", args!["Once the ugly Shadow leaves here to steal power from the other World Yggdrasil Trees, there will be great destruction. This world will become hell."])?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["You must... tell the commanders of Alfheim about this, and come up with a plan. Only them..."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Is there nothing else we can do?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["With our current powers...it is impossible to defeat the Guardian."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["But the one going mad is not the Guardian, but his dark Shadow...maybe we can trap him here..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["Are you willing to accept this mission? Even if it means to pay with your life?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Leave it to us.:We can't do it.")])?) == 2 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("World Tree Yggdrasil", args!["I thank you deeply for your decision. I will use what is left of my powers to open up the path towards the Guardian's Nest."])?;
        ctx.next()?;
        ctx.mes("[World Tree Yggdrasil]")?;
        if ctx
            .call(
                Function::IsPartyLeader,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
            )?
            .loosely_equals(&Val::from(1))
        {
            ctx.mes("The path to the Guardian's Nest is just past the waterfall by the large World Tree Yggdrasil to the North. The defensive mechanisms of the Sanctuary will start immediately.")?;
            ctx.next()?;
            ctx.lines_as(
                "World Tree Yggdrasil",
                args![
                    "Defeat all of Nidhoggur's guardians and go through the waterfall into the nest... and stop Nidhoggur's Shadow there."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("World Tree Yggdrasil", args!["The gate will open soon. Go defeat all of the guardians... you must kill them all in 30 minutes before the gate opens..."])?;
            ctx.next()?;
            ctx.lines_as(
                "World Tree Yggdrasil",
                args!["30 minutes... that's the limit of my powers. Please hurry."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "World Tree Yggdrasil",
                args!["And... Be careful... Be careful of the shadow's power."],
            )?;
            ctx.var("ins_nyd2").set(Val::from(1))?;
            ctx.var("'ins_nyd2").set(ctx.var("ins_nyd2").get()?)?;
            ctx.call(
                Function::DoNpcEvent,
                vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd_1f_timer")])? + Val::from("::OnEnable"))],
            )?;
            ctx.call(
                Function::DoNpcEvent,
                vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnEnable"))],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("The path to the Guardian's Nest is just past the waterfall by the large World Tree Yggdrasil to the North. The defensive mechanisms will start immediately.")?;
            ctx.next()?;
            ctx.lines_as(
                "World Tree Yggdrasil",
                args![
                    "Defeat all of Nidhoggur's guardians and go through the waterfall into the nest... and stop Nidhoggur's Shadow there."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("World Tree Yggdrasil", args!["The gate will open soon. Go defeat all of the guardians... you must kill them all in 30 minutes before the gate opens..."])?;
            ctx.next()?;
            ctx.lines_as(
                "World Tree Yggdrasil",
                args!["30 minutes... that's the limit of my powers. Please hurry."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "World Tree Yggdrasil",
                args!["And... Be careful... Be careful of the shadow's power."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("'ins_nyd2").get()? == 1 {
        ctx.lines_as("World Tree Yggdrasil", args!["The path to the Guardian's Nest is just past the waterfall by the large World Tree Yggdrasil to the North. The defensive mechanisms will start immediately."])?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["Defeat all of Nidhoggur's guardians and go through the waterfall into the nest... and stop Nidhoggur's Shadow there."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["The gate will open soon. Go defeat all of the guardians."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("'ins_nyd2").get()? == 2 {
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["The path to the Guardian's Nest is just past the waterfall by the large World Tree Yggdrasil to the North."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["Use your powers... and destroy the vile Shadow..."],
        )?;
        ctx.next()?;
        ctx.lines_as("World Tree Yggdrasil", args!["This is... all I can do for you..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "World Tree Yggdrasil",
            args!["Those who want to taint the sacred Sanctuary of the Guardian... Get out."],
        )?;
        ctx.call(Function::Warp, vec![Val::from("mid_camp"), Val::from(100), Val::from(100)])?;
    }
    return Err(Stop::End);
}

pub fn murdered_yggdrasilid_1f(ctx: &Ctx) -> Script {
    murdered_yggdrasilid_1f_body(ctx, Vec::new()).map(|_| ())
}

fn murdered_yggdrasilid_1f_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("'ins_nyd2").get()? == 0 {
        ctx.mes("What's a woman from the Laphine Tribe doing here...")?;
        ctx.next()?;
        ctx.mes("What's happening? Let's go check it out.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn murdered_yggdrasilid_1f_ontouch(ctx: &Ctx) -> Script {
    murdered_yggdrasilid_1f_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn murdered_yggdrasilid_1f_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("'ins_nyd2").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn murdered_yggdrasilid_1f_oninstanceinit(ctx: &Ctx) -> Script {
    murdered_yggdrasilid_1f_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NydCallMon1Step {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
}

fn nyd_call_mon_1_run(ctx: &Ctx, mut step: NydCallMon1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_mob_dead_num = Val::from(0);
    'machine: loop {
        match step {
            NydCallMon1Step::Start => {
                step = NydCallMon1Step::OnInstanceInit;
                continue 'machine;
            }
            NydCallMon1Step::OnInstanceInit => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])?],
                )?;
                return Err(Stop::End);
            }
            NydCallMon1Step::OnEnable => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(220),
                        Val::from(250),
                        Val::from("Nidhoggur's Guardian#1"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(220),
                        Val::from(252),
                        Val::from("Nidhoggur's Guardian#2"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(240),
                        Val::from(270),
                        Val::from("Nidhoggur's Guardian#3"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(240),
                        Val::from(272),
                        Val::from("Nidhoggur's Guardian#4"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(200),
                        Val::from(200),
                        Val::from("Nidhoggur's Guardian#5"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(210),
                        Val::from(210),
                        Val::from("Nidhoggur's Guardian#6"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(225),
                        Val::from(265),
                        Val::from("Nidhoggur's Guardian#7"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(225),
                        Val::from(270),
                        Val::from("Nidhoggur's Guardian#8"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(245),
                        Val::from(235),
                        Val::from("Nidhoggur's Guardian#9"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(255),
                        Val::from(255),
                        Val::from("Nidhoggur's Guardian#10"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(225),
                        Val::from(245),
                        Val::from("Nidhoggur's Guardian#11"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(230),
                        Val::from(280),
                        Val::from("Nidhoggur's Guardian#12"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        l_map_s.clone(),
                        Val::from("Nidhoggur's Guardian : Protect the Guardian's Sanctuary. Get rid of the intruders."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff99"),
                    ],
                )?;
                return Err(Stop::End);
            }
            NydCallMon1Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])?],
                )?;
                return Err(Stop::End);
            }
            NydCallMon1Step::OnMyMobDead => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?;
                l_mob_dead_num = ctx.call(
                    Function::MobCount,
                    vec![
                        l_map_s.clone(),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                if l_mob_dead_num.clone().number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("All of Nidhoggur's Guardians have been defeated!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff99"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd_1f_timer")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_to2f_warp")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.var("ins_nyd2").set(Val::from(2))?;
                    ctx.var("'ins_nyd2").set(ctx.var("ins_nyd2").get()?)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_call_mon_1(ctx: &Ctx) -> Script {
    nyd_call_mon_1_run(ctx, NydCallMon1Step::Start, Vec::new()).map(|_| ())
}

pub fn nyd_call_mon_1_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_call_mon_1_run(ctx, NydCallMon1Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_call_mon_1_onenable(ctx: &Ctx) -> Script {
    nyd_call_mon_1_run(ctx, NydCallMon1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_call_mon_1_ondisable(ctx: &Ctx) -> Script {
    nyd_call_mon_1_run(ctx, NydCallMon1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_call_mon_1_onmymobdead(ctx: &Ctx) -> Script {
    nyd_call_mon_1_run(ctx, NydCallMon1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_oninstanceinit(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_onenable(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ondisable(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ontimer900000(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnTimer900000, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ontimer1200000(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnTimer1200000, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ontimer1500000(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnTimer1500000, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ontimer1800000(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnTimer1800000, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ontimer1830000(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnTimer1830000, Vec::new()).map(|_| ())
}

pub fn ins_nyd_1f_timer_ontimer1850000(ctx: &Ctx) -> Script {
    ins_nyd_1f_timer_run(ctx, InsNyd1fTimerStep::OnTimer1850000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NydTo2fWarpStep {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

fn nyd_to2f_warp_run(ctx: &Ctx, mut step: NydTo2fWarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            NydTo2fWarpStep::Start => {
                step = NydTo2fWarpStep::OnInstanceInit;
                continue 'machine;
            }
            NydTo2fWarpStep::OnInstanceInit => {
                step = NydTo2fWarpStep::OnDisable;
                continue 'machine;
            }
            NydTo2fWarpStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_to2f_warp")])?],
                )?;
                return Err(Stop::End);
            }
            NydTo2fWarpStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_to2f_warp")])?],
                )?;
                return Err(Stop::End);
            }
            NydTo2fWarpStep::OnTouch => {
                ctx.var("ins_nyd2").set(Val::from(3))?;
                ctx.var("'ins_nyd2").set(ctx.var("ins_nyd2").get()?)?;
                ctx.call(
                    Function::Warp,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from(200),
                        Val::from(10),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_to2f_warp(ctx: &Ctx) -> Script {
    nyd_to2f_warp_run(ctx, NydTo2fWarpStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_to2f_warp_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_to2f_warp_run(ctx, NydTo2fWarpStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_to2f_warp_ondisable(ctx: &Ctx) -> Script {
    nyd_to2f_warp_run(ctx, NydTo2fWarpStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_to2f_warp_onenable(ctx: &Ctx) -> Script {
    nyd_to2f_warp_run(ctx, NydTo2fWarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_to2f_warp_ontouch(ctx: &Ctx) -> Script {
    nyd_to2f_warp_run(ctx, NydTo2fWarpStep::OnTouch, Vec::new()).map(|_| ())
}

fn ins_nyd1_spawn_mobs_run(ctx: &Ctx, mut step: InsNyd1SpawnMobsStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            InsNyd1SpawnMobsStep::Start => {
                step = InsNyd1SpawnMobsStep::OnInstanceInit;
                continue 'machine;
            }
            InsNyd1SpawnMobsStep::OnInstanceInit => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Ancient Tree"),
                        Val::from(2019),
                        Val::from(40),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyTreeDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Rhyncho"),
                        Val::from(2020),
                        Val::from(30),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyRhynDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Phylla"),
                        Val::from(2021),
                        Val::from(30),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyPhyDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Aqua Elemental"),
                        Val::from(2016),
                        Val::from(30),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyAquaDead")),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Dark Pinguicula"),
                        Val::from(2015),
                        Val::from(30),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyPingDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1SpawnMobsStep::OnMyTreeDead => {
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from(0),
                        Val::from(0),
                        Val::from("Ancient Tree"),
                        Val::from(2019),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyTreeDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1SpawnMobsStep::OnMyRhynDead => {
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from(0),
                        Val::from(0),
                        Val::from("Rhyncho"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyRhynDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1SpawnMobsStep::OnMyPhyDead => {
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from(0),
                        Val::from(0),
                        Val::from("Phylla"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyPhyDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1SpawnMobsStep::OnMyAquaDead => {
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from(0),
                        Val::from(0),
                        Val::from("Aqua Elemental"),
                        Val::from(2016),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyAquaDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1SpawnMobsStep::OnMyPingDead => {
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from(0),
                        Val::from(0),
                        Val::from("Dark Pinguicula"),
                        Val::from(2015),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd1_spawn_mobs")])? + Val::from("::OnMyPingDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_nyd1_spawn_mobs(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_nyd1_spawn_mobs_oninstanceinit(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_nyd1_spawn_mobs_onmytreedead(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::OnMyTreeDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd1_spawn_mobs_onmyrhyndead(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::OnMyRhynDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd1_spawn_mobs_onmyphydead(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::OnMyPhyDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd1_spawn_mobs_onmyaquadead(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::OnMyAquaDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd1_spawn_mobs_onmypingdead(ctx: &Ctx) -> Script {
    ins_nyd1_spawn_mobs_run(ctx, InsNyd1SpawnMobsStep::OnMyPingDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nyd2fEnterStep {
    Start,
    OnTouch,
}

fn nyd_2f_enter_run(ctx: &Ctx, mut step: Nyd2fEnterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fEnterStep::Start => {
                step = Nyd2fEnterStep::OnTouch;
                continue 'machine;
            }
            Nyd2fEnterStep::OnTouch => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_enter_broad")])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_enter")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_enter(ctx: &Ctx) -> Script {
    nyd_2f_enter_run(ctx, Nyd2fEnterStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_ontouch(ctx: &Ctx) -> Script {
    nyd_2f_enter_run(ctx, Nyd2fEnterStep::OnTouch, Vec::new()).map(|_| ())
}

fn nyd_2f_enter_broad_run(ctx: &Ctx, mut step: Nyd2fEnterBroadStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fEnterBroadStep::Start => {
                step = Nyd2fEnterBroadStep::OnInstanceInit;
                continue 'machine;
            }
            Nyd2fEnterBroadStep::OnInstanceInit => {
                step = Nyd2fEnterBroadStep::OnDisable;
                continue 'machine;
            }
            Nyd2fEnterBroadStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_enter_broad")])?],
                )?;
                return Err(Stop::End);
            }
            Nyd2fEnterBroadStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_enter_broad")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fEnterBroadStep::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from("Nidhoggur's Shadow : No more... I can't stand this anymore..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Nyd2fEnterBroadStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from("Nidhoggur's Shadow : I need... I need the World Tree Yggdrasil's powers..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Nyd2fEnterBroadStep::OnTimer18000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from("Nidhoggur's Shadow : Destroy... everything..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_enter_broad")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_enter_broad(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_broad_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_broad_ondisable(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_broad_onenable(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_broad_ontimer12000(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_broad_ontimer15000(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn nyd_2f_enter_broad_ontimer18000(ctx: &Ctx) -> Script {
    nyd_2f_enter_broad_run(ctx, Nyd2fEnterBroadStep::OnTimer18000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nyd2fDdrControlStep {
    Start,
    OnTouch,
}

fn nyd_2f_ddr_control_run(ctx: &Ctx, mut step: Nyd2fDdrControlStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fDdrControlStep::Start => {
                step = Nyd2fDdrControlStep::OnTouch;
                continue 'machine;
            }
            Nyd2fDdrControlStep::OnTouch => {
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                ctx.mes("From below the gorgeous stones, a strange breeze is forming.")?;
                ctx.next()?;
                ctx.mes("The strange power slowly surrounds your body, the dimension is starting to shift.")?;
                ctx.close_window()?;
                ctx.call(
                    Function::Warp,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from(199),
                        Val::from(255),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_ddr_control(ctx: &Ctx) -> Script {
    nyd_2f_ddr_control_run(ctx, Nyd2fDdrControlStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_ddr_control_ontouch(ctx: &Ctx) -> Script {
    nyd_2f_ddr_control_run(ctx, Nyd2fDdrControlStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nyd2fBossEnterStep {
    Start,
    OnTouch,
}

fn nyd_2f_boss_enter_run(ctx: &Ctx, mut step: Nyd2fBossEnterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fBossEnterStep::Start => {
                step = Nyd2fBossEnterStep::OnTouch;
                continue 'machine;
            }
            Nyd2fBossEnterStep::OnTouch => {
                if ctx
                    .call(
                        Function::IsPartyLeader,
                        vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                    )?
                    .loosely_equals(&Val::from(1))
                {
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter")])?],
                    )?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_boss_enter(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_run(ctx, Nyd2fBossEnterStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_ontouch(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_run(ctx, Nyd2fBossEnterStep::OnTouch, Vec::new()).map(|_| ())
}

fn nyd_2f_boss_enter_call_run(ctx: &Ctx, mut step: Nyd2fBossEnterCallStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            Nyd2fBossEnterCallStep::Start => {
                step = Nyd2fBossEnterCallStep::OnInstanceInit;
                continue 'machine;
            }
            Nyd2fBossEnterCallStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])?],
                )?;
                return Err(Stop::End);
            }
            Nyd2fBossEnterCallStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])?],
                )?;
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(199),
                        Val::from(327),
                        Val::from("Nidhoggur's Shadow"),
                        Val::from(2022),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        l_map_s.clone(),
                        Val::from("Nidhoggur's Shadow : I will devour all of you... you and the World Tree Yggdrasil."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff99"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fBossEnterCallStep::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])?],
                )?;
                return Err(Stop::End);
            }
            Nyd2fBossEnterCallStep::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = Nyd2fBossEnterCallStep::OnWarpColor;
                continue 'machine;
            }
            Nyd2fBossEnterCallStep::OnWarpColor => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Nidhoggur's Shadow : In this chaos... your blood is just what I need."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFFFF00"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_white")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_yellow")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_green")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_red")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::InstanceWarpAll,
                        vec![
                            l_map_s.clone(),
                            Val::from(115),
                            Val::from(278),
                            ctx.call(Function::InstanceId, vec![])?,
                        ],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Nidhoggur's Shadow : I will freeze every last drop of your blood."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFFFF00"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_red")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_yellow")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_green")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_white")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::InstanceWarpAll,
                        vec![
                            l_map_s.clone(),
                            Val::from(115),
                            Val::from(373),
                            ctx.call(Function::InstanceId, vec![])?,
                        ],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Nidhoggur's Shadow : Sleep for eternity in an empty illusion."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFFFF00"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_red")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_white")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_green")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_yellow")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::InstanceWarpAll,
                        vec![
                            l_map_s.clone(),
                            Val::from(284),
                            Val::from(278),
                            ctx.call(Function::InstanceId, vec![])?,
                        ],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 4 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Nidhoggur's Shadow : I'll let you enjoy the pain of dying slowly."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFFFF00"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_red")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_white")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_yellow")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_green")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::InstanceWarpAll,
                        vec![
                            l_map_s.clone(),
                            Val::from(284),
                            Val::from(374),
                            ctx.call(Function::InstanceId, vec![])?,
                        ],
                    )?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            Nyd2fBossEnterCallStep::OnMyMobDead => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?;
                if ctx
                    .call(
                        Function::MobCount,
                        vec![
                            l_map_s.clone(),
                            (ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnMyMobDead")),
                        ],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Nidhoggur's Shadow : World Tree Yggdrasil's guardian... his powers are disappearing..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff99"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("World Tree Yggdrasil#2F")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_logic")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_red_c")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_white_c")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_yellow_c")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_green_c")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.var("ins_nyd2").set(Val::from(4))?;
                    ctx.var("'ins_nyd2").set(ctx.var("ins_nyd2").get()?)?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_boss_enter_call(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_call_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_call_onenable(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_call_ondisable(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_call_ontimer180000(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_call_onwarpcolor(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::OnWarpColor, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_call_onmymobdead(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_call_run(ctx, Nyd2fBossEnterCallStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nyd2fBossEnterLogicStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTimer180000,
}

fn nyd_2f_boss_enter_logic_run(ctx: &Ctx, mut step: Nyd2fBossEnterLogicStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fBossEnterLogicStep::Start => {
                step = Nyd2fBossEnterLogicStep::OnInstanceInit;
                continue 'machine;
            }
            Nyd2fBossEnterLogicStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_logic")])?],
                )?;
                return Err(Stop::End);
            }
            Nyd2fBossEnterLogicStep::OnEnable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fBossEnterLogicStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_logic")])?],
                )?;
                return Err(Stop::End);
            }
            Nyd2fBossEnterLogicStep::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnWarpColor"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_boss_enter_logic(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_logic_run(ctx, Nyd2fBossEnterLogicStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_logic_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_logic_run(ctx, Nyd2fBossEnterLogicStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_logic_onenable(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_logic_run(ctx, Nyd2fBossEnterLogicStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_logic_ondisable(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_logic_run(ctx, Nyd2fBossEnterLogicStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_boss_enter_logic_ontimer180000(ctx: &Ctx) -> Script {
    nyd_2f_boss_enter_logic_run(ctx, Nyd2fBossEnterLogicStep::OnTimer180000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WorldTreeYggdrasil2fStep {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
}

fn world_tree_yggdrasil_2f_run(ctx: &Ctx, mut step: WorldTreeYggdrasil2fStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WorldTreeYggdrasil2fStep::Start => {
                ctx.lines_as(
                    "World Tree Yggdrasil",
                    args!["Thank you. You're the saviour of humans and the Laphine Tribe."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "World Tree Yggdrasil",
                    args!["You've defeated Nidhoggur's Shadow. But...It's not gone for good. Its powers are merely put to sleep."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "World Tree Yggdrasil",
                    args!["But you have won us plenty of time. This is great."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "World Tree Yggdrasil",
                    args!["Please go and report this to those in charge of Splendide and Manuk."],
                )?;
                ctx.next()?;
                ctx.lines_as("World Tree Yggdrasil", args!["As you have seen, neither the Sapha tribe nor the Laphine tribe is at fault for the weakening of the World Tree. It's all because of the Guardian's Shadow..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "World Tree Yggdrasil",
                    args!["The Guardian's insanity...is caused by powers unknown to us... You must tell this to everyone."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "World Tree Yggdrasil",
                    args!["Now... Allow me to escort you out of the cursed nest."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Please let me out.:I want to look around for a while.")])? {
                    1 => {
                        ctx.var("'ins_nyd2").set(Val::from(0))?;
                        ctx.var("ins_nyd").set(Val::from(203))?;
                        ctx.call(Function::Warp, vec![Val::from("nyd_dun02"), Val::from(98), Val::from(196)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "World Tree Yggdrasil",
                            args!["Is that so... I'll be around if you want to leave."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = WorldTreeYggdrasil2fStep::OnInstanceInit;
                continue 'machine;
            }
            WorldTreeYggdrasil2fStep::OnInstanceInit => {
                step = WorldTreeYggdrasil2fStep::OnDisable;
                continue 'machine;
            }
            WorldTreeYggdrasil2fStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("World Tree Yggdrasil#2F")])?],
                )?;
                return Err(Stop::End);
            }
            WorldTreeYggdrasil2fStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("World Tree Yggdrasil#2F")])?],
                )?;
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?, Val::from("World Tree Yggdrasil : You did good. Have everyone go to the Magic Circle in the middle, and get ready for the destruction of the nest."), ctx.constant("BC_MAP")?, Val::from("0x00ff99")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn world_tree_yggdrasil_2f(ctx: &Ctx) -> Script {
    world_tree_yggdrasil_2f_run(ctx, WorldTreeYggdrasil2fStep::Start, Vec::new()).map(|_| ())
}

pub fn world_tree_yggdrasil_2f_oninstanceinit(ctx: &Ctx) -> Script {
    world_tree_yggdrasil_2f_run(ctx, WorldTreeYggdrasil2fStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn world_tree_yggdrasil_2f_ondisable(ctx: &Ctx) -> Script {
    world_tree_yggdrasil_2f_run(ctx, WorldTreeYggdrasil2fStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn world_tree_yggdrasil_2f_onenable(ctx: &Ctx) -> Script {
    world_tree_yggdrasil_2f_run(ctx, WorldTreeYggdrasil2fStep::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nyd2fRedStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTouch,
    OnTimer10000,
}

fn nyd_2f_red_run(ctx: &Ctx, mut step: Nyd2fRedStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fRedStep::Start => {
                step = Nyd2fRedStep::OnInstanceInit;
                continue 'machine;
            }
            Nyd2fRedStep::OnInstanceInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_warp1"))],
                        )? + Val::from("::OnEnable")),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_warp2"))],
                        )? + Val::from("::OnEnable")),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_warp3"))],
                        )? + Val::from("::OnEnable")),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_c"))],
                        )? + Val::from("::OnEnable")),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedStep::OnDisable => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_warp1"))],
                        )? + Val::from("::OnDisable")),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_warp2"))],
                        )? + Val::from("::OnDisable")),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_warp3"))],
                        )? + Val::from("::OnDisable")),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedStep::OnTouch => {
                if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("red")).is_true() {
                    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_BLEEDING")?, Val::from(60000), Val::from(0)],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("white")).is_true() {
                    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_FREEZE")?, Val::from(20000), Val::from(0)],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("yellow")).is_true() {
                    ctx.call(Function::PercentHeal, vec![Val::from(0), Val::from(-50)])?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_SLEEP")?, Val::from(20000), Val::from(0)],
                    )?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_CONFUSION")?, Val::from(60000), Val::from(0)],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("green")).is_true() {
                    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(-50)])?;
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_POISON")?, Val::from(60000), Val::from(0)],
                    )?;
                }
                return Err(Stop::End);
            }
            Nyd2fRedStep::OnTimer10000 => {
                ctx.call(Function::DisableNpc, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_red(ctx: &Ctx) -> Script {
    nyd_2f_red_run(ctx, Nyd2fRedStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_2f_red_run(ctx, Nyd2fRedStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_onenable(ctx: &Ctx) -> Script {
    nyd_2f_red_run(ctx, Nyd2fRedStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_ondisable(ctx: &Ctx) -> Script {
    nyd_2f_red_run(ctx, Nyd2fRedStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_ontouch(ctx: &Ctx) -> Script {
    nyd_2f_red_run(ctx, Nyd2fRedStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_ontimer10000(ctx: &Ctx) -> Script {
    nyd_2f_red_run(ctx, Nyd2fRedStep::OnTimer10000, Vec::new()).map(|_| ())
}
