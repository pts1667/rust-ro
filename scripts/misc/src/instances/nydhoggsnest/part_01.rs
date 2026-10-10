use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum YggdrasilGatekeeperStep {
    Start,
    LEnter,
    OnTouch,
}

fn yggdrasil_gatekeeper_run(ctx: &Ctx, mut step: YggdrasilGatekeeperStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_ins_nyd_check = Val::from(0);
    let mut l_ins_nyd_check2 = Val::from(0);
    let mut l_md_name_s = Val::from("");
    let mut l_party_id = Val::from(0);
    'machine: loop {
        match step {
            YggdrasilGatekeeperStep::Start => {
                if ctx.var("ins_nyd").get()? == 0 {
                    ctx.mes(
                        "A great stone gate stands before you. The sculpture of a terrible dragon spreads its powerful looking wings.",
                    )?;
                    ctx.next()?;
                    ctx.mes("Near the bottom of the gate, Laphine tribeswomen have been turned to stone and now look like they are part of the great door.")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Move closer to look more carefully.:Step back.")])? {
                        1 => {
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                            ctx.call(Function::PushPc, vec![Val::from(3), Val::from(3)])?;
                            if ctx.var("ep13_1_edq").get()? == 14 {
                                ctx.var("ep13_1_edq").set(Val::from(15))?;
                            }
                            ctx.mes("A mysterious power prevents you from getting too close. It looks like there is something strong beyond the door...")?;
                            ctx.next()?;
                            ctx.mes("Perhaps there's a great hidden secret beyond the gate, beyond expectation.")?;
                            ctx.next()?;
                            ctx.mes("It would be better to go back to camp and inform the others and ask for help.")?;
                            if ctx.var("ep13_1_edq").get()? != 15 {
                                ctx.next()?;
                                ctx.mes("You'll have to obtain the others trust in the expendition camp by working hard.")?;
                            }
                            ctx.var("ins_nyd").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if ctx.var("ins_nyd").get()? == 1 {
                        ctx.mes("A mysterious power prevents you from getting too close. It looks like there is something strong beyond the door...")?;
                        ctx.next()?;
                        ctx.mes("Perhaps there's a great hidden secret beyond the gate, beyond expectation.")?;
                        ctx.next()?;
                        ctx.mes("It would be better to go back to camp and inform the others and ask for help.")?;
                        if (ctx.var("ep13_1_edq").get()? == 14 || ctx.var("ep13_1_edq").get()? == 15) {
                            ctx.var("ep13_1_edq").set(Val::from(15))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.next()?;
                        ctx.mes("You'll have to obtain the others trust in the expendition camp by working hard.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ins_nyd").get()? == 111 || ctx.var("ins_nyd").get()? == 112) {
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CHANGECOLD")?])?;
                            ctx.mes("The strange sensation surrounding your body has disappeared")?;
                            ctx.next()?;
                            ctx.mes("When you touch the stone gate, you hear a commanding voice.")?;
                            ctx.next()?;
                            ctx.lines_as("??????", args!["Wingless one... Our promised words..."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("'Guardian's spell'!:Take a step back.")])? {
                                1 => {
                                    ctx.lines_as(
                                        "??????",
                                        args!["Promised words... Guardian's spell... proof of their existence."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Yggdrasil Gatekeeper",
                                        args!["In the name of Yggdrasiliad, I will accept you as a servant of the Guardian."],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CHANGECOLD")?])?;
                                    ctx.var("ins_nyd").set(Val::from(200))?;
                                    ctx.lines_as("Yggdrasil Gatekeeper", args!["I accept your entrance through the Guardian's gate. You are now considered a faithful servant of the Guardian Nidhoggur."])?;
                                    ctx.next()?;
                                    ctx.mes("The voice has disappeared, and the dark power is calming down from behind the stone gate.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else if ((ctx.var("ins_nyd").get()? == 131 || ctx.var("ins_nyd").get()? == 132)
                            || ctx.var("ins_nyd").get()?.number()? > 199)
                        {
                            l_party_id = ctx.call(Function::GetCharacterId, vec![Val::from(1)])?;
                            l_md_name_s = Val::from("Nidhoggur's Nest");
                            l_ins_nyd_check = ctx.call(Function::CheckQuest, vec![Val::from(3135), ctx.constant("PLAYTIME")?])?;
                            l_ins_nyd_check2 = ctx.call(Function::CheckQuest, vec![Val::from(3136), ctx.constant("PLAYTIME")?])?;
                            ctx.mes("As I put my hands on the stone gate, a voice sounded from the depth of my heart.")?;
                            ctx.next()?;
                            if (l_ins_nyd_check.clone() == -1 && l_ins_nyd_check2.clone() == -1) {
                                if !(ctx
                                    .call(
                                        Function::InstanceCheckParty,
                                        vec![l_party_id.clone(), Val::from(2), Val::from(70)],
                                    )?
                                    .is_true())
                                {
                                    ctx.lines_as("Yggdrasil Gatekeeper", args!["Where are the other servants, so you can work together? Each servant cannot be admitted here individually..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Yggdrasil Gatekeeper", args!["Come with at least 1 more servant... Only party leaders can accept admission to Nidhoggur's Nest."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Yggdrasil Gatekeeper",
                                        args!["And only 1 representative of you needs to talk to me, so don't annoy me..."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx
                                    .call(Function::IsPartyLeader, vec![l_party_id.clone()])?
                                    .loosely_equals(&Val::from(1))
                                {
                                    ctx.lines_as(
                                        "Yggdrasil Gatekeeper",
                                        args!["The loyal servants of the Guardian... what can I do for you?"],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("Please allow me to enter.:I want to go in.:I want to leave.")],
                                    )? {
                                        1 => {
                                            if ctx.call(Function::InstanceCreate, vec![l_md_name_s.clone()])?.number()? < 0 {
                                                ctx.lines_as("Yggdrasil Gatekeeper", args!["The Guardian seems to wish to be alone. I will go in and check, please wait out here."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Yggdrasil Gatekeeper",
                                                args!["I've recorded your request, are you ready to go inside?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Yggdrasil Gatekeeper", args!["If you are ready, I will allow you to enter."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            step = YggdrasilGatekeeperStep::LEnter;
                                            continue 'machine;
                                        }
                                        3 => {
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                ctx.lines_as(
                                    "Yggdrasil Gatekeeper",
                                    args!["If you have the dungeon generated already, you can enter it."],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("I want to go in.:I want to leave.")])?) == 2 {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                step = YggdrasilGatekeeperStep::LEnter;
                                continue 'machine;
                            } else if (l_ins_nyd_check.clone() == 0 || l_ins_nyd_check.clone() == 1) {
                                if (ctx.var("ins_nyd2").get()? == 3 || ctx.var("ins_nyd2").get()? == 4) {
                                    ctx.lines_as("Yggdrasil Gatekeeper", args!["With the defeat of Nidhoggur's Shadow, the roots of the World Tree Yggdrasil are also affected."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Yggdrasil Gatekeeper",
                                        args!["After Nidhoggur's Shadow disappears, at least 3 days is needed for stabilizing."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Yggdrasil Gatekeeper",
                                    args!["If you have the dungeon generated already, you can enter it."],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("I want to go in.:I want to leave.")])?) == 2 {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                step = YggdrasilGatekeeperStep::LEnter;
                                continue 'machine;
                            } else if l_ins_nyd_check.clone() == 2 {
                                if (l_ins_nyd_check2.clone() == 0 || l_ins_nyd_check2.clone() == 1) {
                                    ctx.lines_as("Yggdrasil Gatekeeper", args!["The time limit to enter the dungeon has expired. You must wait for the World Tree to stabilize its power before trying to re-enter."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if l_ins_nyd_check2.clone() == 2 {
                                    ctx.lines_as(
                                        "Yggdrasil Gatekeeper",
                                        args!["The World Tree Yggdrasil has stabilized. Would you like to enter Nidhoggur's Nest again?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Yggdrasil Gatekeeper",
                                        args!["If you would like to enter again, please register with me."],
                                    )?;
                                    ctx.call(Function::EraseQuest, vec![Val::from(3135)])?;
                                    ctx.call(Function::EraseQuest, vec![Val::from(3136)])?;
                                    ctx.var("ins_nyd2").set(Val::from(0))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.mes("A great stone gate stands before you. The sculpture of a terrible dragon spreads its powerful looking wings.")?;
                            ctx.next()?;
                            ctx.mes("Near the bottom of the gate, Laphine tribeswomen have been turned to stone and now look like they are part of the great door.")?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Move closer to look more carefully.:Step back.")])? {
                                1 => {
                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                                    ctx.call(Function::PushPc, vec![Val::from(3), Val::from(3)])?;
                                    ctx.mes("A mysterious power prevents you from getting too close. It looks like there is something strong beyond the door...")?;
                                    ctx.next()?;
                                    ctx.mes("Perhaps there's a great hidden secret beyond the gate, beyond expectation.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            YggdrasilGatekeeperStep::LEnter => {
                'b5: {
                    let subject5 = ctx.call(Function::InstanceEnter, vec![Val::from("Nidhoggur's Nest")])?;
                    let mut matched5 = false;
                    let no_case5 = !subject5.loosely_equals(&ctx.constant("IE_OTHER")?)
                        && !subject5.loosely_equals(&ctx.constant("IE_NOINSTANCE")?)
                        && !subject5.loosely_equals(&ctx.constant("IE_NOMEMBER")?)
                        && !subject5.loosely_equals(&ctx.constant("IE_OK")?);
                    if !matched5 && subject5.loosely_equals(&ctx.constant("IE_OTHER")?) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as("Yggdrasil Gatekeeper", args!["An unknown error has occurred."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5.loosely_equals(&ctx.constant("IE_NOINSTANCE")?) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as(
                            "Yggdrasil Gatekeeper",
                            args!["You didn't ask to be admitted... You should accept admission first before entering."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5.loosely_equals(&ctx.constant("IE_NOMEMBER")?) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as("Yggdrasil Gatekeeper", args!["Where are the other servants, so you can work together? Each servant cannot be admitted here individually..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5.loosely_equals(&ctx.constant("IE_OK")?) {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("nyd_dun02"),
                                (((ctx.call(
                                    Function::GetPartyName,
                                    vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                                )? + Val::from("'s party member "))
                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from(" has entered Nidhoggur's Nest.")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x00ff99"),
                            ],
                        )?;
                        if ctx.call(Function::CheckQuest, vec![Val::from(3135)])? == -1 {
                            ctx.call(Function::SetQuest, vec![Val::from(3135)])?;
                        }
                        if ctx.call(Function::CheckQuest, vec![Val::from(3136)])? == -1 {
                            ctx.call(Function::SetQuest, vec![Val::from(3136)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = YggdrasilGatekeeperStep::OnTouch;
                continue 'machine;
            }
            YggdrasilGatekeeperStep::OnTouch => {
                if ctx.var("ins_nyd").get()? == 0 {
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CHANGECOLD")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CHANGECOLD")?])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn yggdrasil_gatekeeper(ctx: &Ctx) -> Script {
    yggdrasil_gatekeeper_run(ctx, YggdrasilGatekeeperStep::Start, Vec::new()).map(|_| ())
}

pub fn yggdrasil_gatekeeper_ontouch(ctx: &Ctx) -> Script {
    yggdrasil_gatekeeper_run(ctx, YggdrasilGatekeeperStep::OnTouch, Vec::new()).map(|_| ())
}

fn historian_magnifier_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Historian Magniffer]")?;
    if ctx.var("ins_nyd").get()? == 1 {
        ctx.mes("Sure, the mainland also has lots of interesting adventures... Hello, I am Magnifier, a historian dispatched from the Prontera royal court.")?;
        ctx.next()?;
        ctx.lines_as(
            "Historian Magniffer",
            args!["Finding another line of work might make for a really worthy job, but only a historian gets to know the world over.."],
        )?;
        ctx.next()?;
        ctx.lines_as("Historian Magniffer", args!["How this world is organized... and the way of the future! With our studies of the past and present we can predict what is to come."])?;
        ctx.next()?;
        ctx.lines_as(
            "Historian Magniffer",
            args!["We are expecting a lot from you, expert adventurer. So, if you find anything... just tell me."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ins_nyd").get()? == 2 {
            ctx.mes("Does Commander Agip want to talk to me? Let's listen to his story.")?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magniffer",
                args![
                    "Did you find the cave that the fairy tribes treat as a holy place? You are a really tough cookie. What did you find?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Historian Magnifier", args!["..."])?;
            ctx.next()?;
            ctx.mes("... ...")?;
            ctx.next()?;
            ctx.lines_as("Historian Magnifier", args!["Wait a second... I have a brilliant idea."])?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["Let's see... This book... No... this one...? Hmm... Maybe this..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Historian Magnifier", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Historian Magnifier", args!["... ..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["Oh, here it is! World Tree Yggdrasil and God's tribes... This is their book!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["Maybe you found the central line to enter into the World Tree Yggdrasil!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["If that's true, you've found the greatest discovery since the harnessing of mana. Isn't this exciting?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["But we need a lot more information... Are they refusing you admission?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["I will send a message to my assistant who is in the Prontera Library. So, help her find more information."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian Magnifier",
                args!["I will definitely help you find a way to enter the World Tree directly so, just believe in me! Okay~!!"],
            )?;
            ctx.var("ins_nyd").set(Val::from(3))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ins_nyd").get()? == 3 {
                ctx.mes("Why are you standing there? Go to my assistant in the Prontera Library!")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ins_nyd").get()? == 4 {
                    ctx.mes(
                        "You've come back... Good, how's Naomi? Actually, I don't need to worry about her. She is always cheerful. Haha.",
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["You look like you have a lot on your mind... Your face is full of curiosity and questions."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["So, did you read the whole story that I have prepared?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Not yet.:I read all the stories.")])? {
                        1 => {
                            ctx.lines_as("Historian Magnifier", args!["Sheesh~ I prepared these stories for you carefully, but you didn't bother to check anything out did you?"])?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["It would be better if you returned after reading all of them. That's very basic data of what we should do for the future."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Historian Magnifier",
                                args!["Hm, good job. Maybe I don't need to check anything else, right?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["I sent you to figure out which basic materials will be needed for the jobs ahead of us. You should bring research reports..."])?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["You might complain about why I didn't bring any myself... That's because I trust your abilities, don't ever take anything for granted."])?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["But a while ago, while you were tranferring reports from Commander Agip to the mainland, you lost those reports... remember?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Historian Magnifier",
                                args!["So far, nothing's come up... Was it that somebody attacked you?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["Somehow, the truth will come out, but we should be careful of shocking the natives if we go there unannounced and they're not prepared for our arrival."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Historian Magnifier",
                                args!["I've talked too much... Anyway, as you know through my report, you've found a great thing!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["Firstly, we should find out more about the place. I expect we can, but... we can't do much without help from others."])?;
                            ctx.next()?;
                            ctx.lines_as("Historian Magnifier", args!["I have heard about recent expeditions of adventurers that have tried to contact the tribes... Have you heard anything about this?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Historian Magnifier",
                                args!["Anyway, let's try to contact them first, to be clear about any caves or treasures."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Historian Magnifier",
                                args![
                                    "For now, you try to contact the Sapha and Laphine tribes, and try to extract information from them."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Historian Magnifier",
                                args!["I'll also keep searching here. If you find anything, come back and let me know."],
                            )?;
                            ctx.var("ins_nyd").set(Val::from(5))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ((ctx.var("ins_nyd").get()? == 5 || ctx.var("ins_nyd").get()? == 51) || ctx.var("ins_nyd").get()? == 52) {
                    ctx.mes("Okay, let's try to contact them first, to be clear about any caves or treasures.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["For now, you try to contact the Sapha and Laphine tribes, and try to extract more information."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["I'll also keep searching here. If you find anything, come back and let me know."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ins_nyd").get()? == 61 || ctx.var("ins_nyd").get()? == 62) {
                    ctx.mes("Ah, you've come at a proper time. I found a curious thing while looking for reports from Arunafeltz.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["Right now, we are standing on part of one of the roots of the World Tree Yggdrasil."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Historian Magnifier", args!["This spot is connected to World Tree by the root. I think we can expect confrontations between the Sapha and Laphine here eventually, don't you think?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["As I expected, the cave is the entrance to go to one of Yggdrasil's roots..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["Did you find anything about the Sapha and Laphine?"],
                    )?;
                    ctx.next()?;
                    ctx.mes("...")?;
                    ctx.next()?;
                    ctx.mes("... ...")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["Both sides act ambiguously, so... I'm getting worried..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["The two tribes have some trouble amongst their top leaders. It's not anything official, but..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["Let's report to Commander Agip about the situation so far. Then, we wait on his decision."],
                    )?;
                    ctx.var("ins_nyd").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ins_nyd").get()? == 7 || ctx.var("ins_nyd").get()? == 8) {
                    ctx.mes("Report to Commander Hibba Agip about what we have discovered, since time is dependent on his decision.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ((((ctx.var("ins_nyd").get()? == 121 || ctx.var("ins_nyd").get()? == 122) || ctx.var("ins_nyd").get()? == 131)
                    || ctx.var("ins_nyd").get()? == 132)
                    || ctx.var("ins_nyd").get()? == 14)
                {
                    ctx.mes("So that's how it is... we were right about some parts of it... it's called the Guardian's Nest.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["We have gained a large amount of knowledge today, but..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["What we have figured out... how is it going to influence mankind? It's so unpredictable..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["This is only the beginning...we will be quite busy from now on."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args![
                            "First, report to Commander Agip, then act according to the situation. Let me organize my research findings..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("Sure, the mainland also has lots of interesting adventures... Hello, I am Magnifier, a historian dispatched from the Prontera royal court.")?;
                    ctx.next()?;
                    ctx.lines_as("Historian Magnifier", args!["Finding another line of work might make for a really worthy job, but only a historian gets to know the world over.."])?;
                    ctx.next()?;
                    ctx.lines_as("Historian Magnifier", args!["How this world is organized... and the way of the future! With our studies of the past and present we can predict what is to come."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Magnifier",
                        args!["We are expecting a lot from you, expert adventurer. So, if you find anything... just tell me."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn historian_magnifier_edq(ctx: &Ctx) -> Script {
    historian_magnifier_edq_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_naomi_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_name_s = Val::from("");
    ctx.mes("[Assistant Naomi]")?;
    l_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
    if ctx.var("ins_nyd").get()? == 3 {
        ctx.mes("The doctor never ever tries to come back, and there're too many things to do... How can I do it all...")?;
        ctx.next()?;
        ctx.lines_as(
            "Assistant Naomi",
            args!["Hey, you. Please move these books. Put them into shelf 3 row B."],
        )?;
        ctx.next()?;
        ctx.lines_as(l_name_s.clone(), args!["Ah...um..I...am..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Assistant Naomi",
            args!["Don't you see I am too busy? Don't hesitate. Just do it."],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Look busy, and take a step back.:Help her just this once.")])? {
            1 => {
                ctx.lines_as("Assistant Naomi", args!["Gosh! Where is-? Where did-? Ugh! It's so difficult!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Ah... if you're done moving those, then these should go in shelf 3 row B."],
                )?;
                ctx.next()?;
                ctx.lines_as(l_name_s.clone(), args!["Ah...I...see..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Now, I am almost done... Who are you? Are you a new assistant to Dr. Magnifier?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args!["He asked me to bring some reports. Didn't he say anything?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Hmm... I haven't seen him in over a year! What's he doing now?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args!["He said that he would send a message to you... didn't you get it?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Message? ...Let's see... I never expected him to write a message..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["I will check the mailbox, wait a minute. If you get bored read those books."],
                )?;
                ctx.next()?;
                ctx.mes("...")?;
                ctx.next()?;
                ctx.mes("... ...")?;
                ctx.next()?;
                ctx.mes("It's too messy due to lots of stacked books and files. Dr. Magnifier looks like he has a ton of reports.")?;
                ctx.next()?;
                ctx.lines_as(l_name_s.clone(), args!["'Birth of the World', 'The Fiction of Odin's Myth', 'God's Battle Then After', 'Dreams of the Tribes'. There are a variety of books..."])?;
                ctx.next()?;
                ctx.mes("...")?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Oh, sorry I took so long. There was too much mail, so it took me a while to find stuff."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["He has sent me mail over 20 times. I did not know that..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Ah, here's the message about you. He's said to share the information on research and reports."],
                )?;
                ctx.next()?;
                ctx.lines_as(l_name_s.clone(), args!["What is the Doctor's area of expertise?"])?;
                ctx.next()?;
                ctx.lines_as("Assistant Naomi", args!["Ever since 5 years ago, he has been curious about how the world started, and so he began his search for the God of creation."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["He researches combat between Odin and the Gods, and about the Gods' origins and life."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args!["So, did he already know that the Rebirth of Satan Morocc has occurred before?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["I can't be sure, but he thought someday it would occur. It broke out earlier than he expected though."],
                )?;
                ctx.next()?;
                ctx.lines_as(l_name_s.clone(), args!["But Rune-Midgarts approved this research?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Our academics are not a religion. And they too have curiosity about this world's history."],
                )?;
                ctx.next()?;
                ctx.lines_as("Assistant Naomi", args!["The combat of Odin vs. the Gods, and the God's sons and their purpose... The Doctor has researched this his whole life."])?;
                ctx.next()?;
                ctx.lines_as(l_name_s.clone(), args!["But those reports haven't come out yet. Have they?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["That's why he sent you here. By the way, this isn't the first time I've heard this."],
                )?;
                ctx.next()?;
                ctx.lines_as("Assistant Naomi", args!["After the establishment of the Rune-Midgarts Kingdom and Arunafeltz, the rumors have spread in secret about their tribes, myths, etc."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Anyway, I should make sure that you read all these books, and I'll just keep doing my work."],
                )?;
                ctx.next()?;
                ctx.lines_as(l_name_s.clone(), args!["Shouldn't I have filed the books?"])?;
                ctx.next()?;
                ctx.lines_as("Assistant Naomi", args!["The Doctor said he will pick up the books through another person. Besides, don't you need the basic information on what to do?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Naomi",
                    args!["Before you go back to the Doctor, you had better read these books. So, I will go back to work."],
                )?;
                ctx.var("ins_nyd").set(Val::from(4))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("ins_nyd").get()? == 4 {
        ctx.mes("Browse around, to take a look at the books.")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Discovery of Heterogeneity:Report of Indigenous Tribes")])? {
            1 => {
                ctx.mes("Satan Morocc has known that he didn't resurrect normally or by himself.")?;
                ctx.next()?;
                ctx.mes("Continuously, adventurers from Rune-Midgarts have attacked him and he is slowly losing his power. He would need more time to resurrect completely.")?;
                ctx.next()?;
                ctx.mes("Satan Morocc stopped to destroy the city of Morocc, turning it into a ruined desert, then started to rip the world apart.")?;
                ctx.next()?;
                ctx.mes(
                    "Satan Morocc was worried about those who would give chase, so he created Morocc clones to keep watch behind him.",
                )?;
                ctx.next()?;
                ctx.mes("Modeled after Morocc, their appearance made it difficult to go around the time-space gap.")?;
                ctx.next()?;
                ctx.mes("Still the adventurers gave chase. They came from all over the world, trying to approach the Dimensional Gorge.")?;
                ctx.next()?;
                ctx.mes("The reports of these adventurers have been sent to representatives of all kingdoms, and an expedition team has been created to find out more information.")?;
                ctx.next()?;
                ctx.mes(
                    "The scientists of Schwarzwald created a combination metal, using fragments of metals found in the dimensional gorge.",
                )?;
                ctx.next()?;
                ctx.mes("The Schwarzwald Republic requested approval to find the source of the new metal, and since Rune-Midgarts couldn't complete the test themselves, they finally accepted.")?;
                ctx.next()?;
                ctx.mes("They associated together to gather volunteers. The Assassin Guild was the first to volunteer.")?;
                ctx.next()?;
                ctx.mes("The Assassins have a terrible past with Satan Morocc, so they gathered 18 members to chase him down.")?;
                ctx.next()?;
                ctx.mes("About 3 hours later, all 18 members returned without any problem, and each man and woman shared the information that they had collected.")?;
                ctx.next()?;
                ctx.mes("They had discovered another world with a definitively different nature and environment. And indeed, people could also live there.")?;
                ctx.next()?;
                ctx.mes("The most surprising thing is the flow of time. The 18 assassins had stayed for about 2 weeks in there, yet they returned within 3 hours after departing.")?;
                ctx.next()?;
                ctx.mes("The last thing to be tested... was to send adventurers who volunteered to explore the new world.")?;
                ctx.next()?;
                ctx.mes("There was a flood of adventurer applications. Lots of volunteers disappeared over the dimensional gorge, and they brought back new data.")?;
                ctx.next()?;
                ctx.mes(
                    "The new world could support 3 completely different eco-systems dependant upon the race of people that lived there.",
                )?;
                ctx.next()?;
                ctx.mes("The heterogenous phenomenon needed to be studied thoroughly and carefully in order to under the relationship between thair world and ours.")?;
                ctx.next()?;
                ctx.mes("Just when it was expected to be impossible to travel into a different world, the first page of a new chapter was opened.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes(
                    "Long ago, there wasn't a sun, moon, or stars; just empty earth... and Ymir was born. Then, by making sons, Ymir grew.",
                )?;
                ctx.next()?;
                ctx.mes("But, his sons grew as well and he was killed by Odin, Vili, and Ve; 3 brothers, Gods, that attacked from different sides.")?;
                ctx.next()?;
                ctx.mes("At that time of Ymir's fall his blood flooded the world... killing all in it's path.")?;
                ctx.next()?;
                ctx.mes("Only Hvergelmir of the Sapha tribe escaped from this flooding of blood. And he swore vengeance in Jotunheim, which is covered with foggy snow.")?;
                ctx.next()?;
                ctx.mes("Currently, one of the Sapha tribe has been discovered from beyond the Dimensional Gorge.")?;
                ctx.next()?;
                ctx.mes("Other than the Sapha tribe, there was another tribe beyond the time-space gap, known as the Laphine.")?;
                ctx.next()?;
                ctx.mes("The Laphine tribes gathered as well for an expedition to explore the time-space gap and figure out the World Tree's strange symptoms and perharps a cure method.")?;
                ctx.next()?;
                ctx.mes("The Laphine tribe was charged with the management of Yggdrasil, to establish their lands close to Asgard, and to protect the balance of Yggdrasil's magic power.")?;
                ctx.next()?;
                ctx.mes("The Laphine tribe has never contacted anyone outside of Asgard. But since they found that Yggdrasil's power if weakening...")?;
                ctx.next()?;
                ctx.mes("They have declared they will attend to the high courts for the first time in 1000 human years, since the end of the battles of Gods vs. Magicians.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.mes("The doctor never ever tries to come back, and there're too many things to do... How can I do it all...")?;
        ctx.next()?;
        ctx.lines_as(
            "Assistant Naomi",
            args!["Don't you see that I'm too busy? Don't dawdle, just go!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn assistant_naomi_edq(ctx: &Ctx) -> Script {
    assistant_naomi_edq_body(ctx, Vec::new()).map(|_| ())
}

fn grumbling_soldier_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Grumbling Soldier]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        ctx.mes("Nowadays, the world has turned unstable. I can't even fly comfortably anymore.")?;
        ctx.next()?;
        if ctx.var("ins_nyd").get()? == 5 {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "What's with the cave up north?:Who are the Sapha tribesmen?:Hmm. We can talk later.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args!["What? If you wander around there... you might return with injuries."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Grumbling Soldier", args!["I don't know exactly how to explain it, but it's like it has a bad mood. There are lots of terrible monsters there."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args!["They whisper to each other, so... something is there... But I don't care..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args![
                            "Might be... those Sapha tribesmen have dug in the cave before... They do have a special talent for digging."
                        ],
                    )?;
                    ctx.var("ins_nyd").set(Val::from(51))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args![
                            "Oh! It is because of them that we have been living here, a lowdown and dirty city, for over one hundred years."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args!["They don't care if there's trouble with the Yggdrasil."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args!["They are surely full of bad ideas, so... they have destroyed the Yggdrasil's root."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grumbling Soldier",
                        args!["We are here to make sure that the Sapha don't make things worse."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Grumbling Soldier",
                args!["Nowadays, the world has turned unstable. I can't even fly comfortably anymore."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grumbling Soldier",
                args!["What can you expect when Manuk giants start to dig into the world to destroy the Yggdrasil..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grumbling Soldier",
                args!["And the worst thing is... that strange things are strutting along the streets of the towns..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grumbling Soldier",
                args!["Yes, you... What do you think about the way the government is handling this...?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grumbling Soldier",
                args!["Although they ignore your track record, still, one should be careful..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.mes("SeLarsmar Di marThusVil U SeMushVohl")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn grumbling_soldier_edq(ctx: &Ctx) -> Script {
    grumbling_soldier_edq_body(ctx, Vec::new()).map(|_| ())
}

fn sighing_soldier_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Sighing Soldier]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        ctx.mes("When will we be finished with this combat with the Sapha? Ugghhhh...")?;
        ctx.next()?;
        if ctx.var("ins_nyd").get()? == 5 {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "What's with the cave up north?:Who are the Sapha tribesmen?:Hmm. We can talk later.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Sighing Soldier",
                        args!["Well... I'm not sure, but we have avoided going to that area."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Sighing Soldier", args!["Sometimes, the dispatched researchers hang around here... I feel bad that there's nothing to see... without any reason..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sighing Soldier",
                        args!["But the command officers make sure that there's something hidden in there."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Sighing Soldier", args!["We don't know if the monsters there are strong, so we never checked it out. But on a personal level, nobody wants to go there..."])?;
                    ctx.var("ins_nyd").set(Val::from(51))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Sighing Soldier", args!["I don't know what others think about it... but we've had some trouble with our attitude against the Sapha tribes."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sighing Soldier",
                        args!["We didn't try to solve the problems with talk. We attacked them first."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sighing Soldier",
                        args!["Maybe... our command officers don't want to accept other species different than us..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Sighing Soldier", args!["Don't misunderstand... Recently, we have talked about your particular species and our commanders feel grateful to you."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sighing Soldier",
                        args!["Anyway... I'm just exhausted during this useless and nerve-wracking situation... Sigh."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Sighing Soldier",
                args!["When will we be finished with this combat with the Sapha? Ugghhhh..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sighing Soldier",
                args!["Frankly, I don't think of you or any Sapha is our enemy. Sigh..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sighing Soldier",
                args!["Is there no way to resolve this by communicating? There's no meaning in useless combat! Geez."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.mes("VohlLarsmar Ha DielCyatas")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn sighing_soldier_edq(ctx: &Ctx) -> Script {
    sighing_soldier_edq_body(ctx, Vec::new()).map(|_| ())
}

fn commander_lebiordirr_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Commander Lebiordirr]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ins_nyd").get()? == 51 {
            ctx.mes("Are you...? Are you the one collecting information from my soldiers...?")?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["You should be cautious. Our tribe has respected the existence of you humans but..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["I've taken a great risk in allowing you in here. So be wary."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["Don't attract too much attention because that would make your people look bad."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["Now, we don't need to talk much about this, so just go back where you came from."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["Unless you have any messages for me...? Your face says you do..."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("No. Nothing. We can talk later.:I intend to stay. For good reasons.")],
            )?) == 1
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Splendide Guard", args!["Sir, I can drag this pest out right now."])?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["No. Let's hear an explanation. Good? Now, if you want..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["I have known that, recently, time has broken... so, you're here exploring for a solution, no?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["Our tribe has respected you, so just forget about the searching and exploring around here."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["I heard that you are to search for any treasure in the closed cave to the north."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["That place is banned by order of the Laphine tribe. So, people can't just go there without permission."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["If you can't follow this rule, I will stop associating with you and ban all of your people from here."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["We still have an unstable relationship with the Sapha, we usually don't worry about outsiders."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Commander Lebiordirr",
                args!["Now, if you understand this, inform your friends."],
            )?;
            ctx.var("ins_nyd").set(Val::from(61))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ins_nyd").get()? == 61 {
                ctx.mes("If you can't follow this rule, I will stop associating with you and ban all of your people from here.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Commander Lebiordirr",
                    args!["We still have an unstable relationship with the Sapha, we usually don't worry about outsiders."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Commander Lebiordirr",
                    args!["Now, if you understand this, inform your friends."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ins_nyd").get()? == 81 {
                    ctx.mes("Why have you come back, outsider?")?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("To ask the Laphines about exploring...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Commander Lebiordirr", args!["Exploring what? Choose your words wisely?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Do we not understand each other? What do you want?"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("I must explore the cave.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["That means... that you are ignoring my warning? Is this your decision or are you just following orders?"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("I'm just following orders")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Are you trying to insult me on purpose, outsider?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Why do you want to explore? It better be a very good reason."],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("It's about Dr. Magnifier's report...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["So... What do I care about an outsider's report?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["To protect the Yggdrasil! That's the Laphine's fate. Can you say that about yourself?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Do you have any proof of your birth with Odin and Yggdrasil's blessings?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["I shouldn't say anymore. Please don't take this as being rude but, please go now!"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Then I'll ask the Sapha for help.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Commander Lebiordirr", args!["What are you talking about, outsider?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Splendide Guard",
                        args!["Sir, I can get rid of this rude outsider if you wish?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["No, wait... The Sapha tribesmen know the meaning of that place? Did you ask to associate with them?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Commander Lebiordirr", args!["Ah... How tricky... Are you testing us?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Splendide Guard",
                        args!["Calm down, sir. I will throw this outsider into prison."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["No... No, wait. Ok, I will accept it. I can accept your admission into the Holy Sekos."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Commander Lebiordirr", args!["Okay. If you are a servant of the Yggdrasil as you claim to be, I will allow you admission. But I am not responsible for your actions."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["And make sure that if you find anything out of the ordinary in there, that you share it with us!"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("But of course!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["So... since I have agreed to allow your exploration... Arioss, help them, and take the results."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aide Arioss",
                        args!["Sir... are you sure? This is an invasion of the Holy Sekos..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Commander Lebiordirr", args!["Invasion is a harsh word. I am allowing them entrance. Better them than a dirty giant. It might just save our lives..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Here, outsider. Arioss will explain the situation with the giants. Talk with him..."],
                    )?;
                    ctx.var("ins_nyd").set(Val::from(91))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (((((ctx.var("ins_nyd").get()? == 91 || ctx.var("ins_nyd").get()? == 101) || ctx.var("ins_nyd").get()? == 111)
                    || ctx.var("ins_nyd").get()? == 200)
                    || ctx.var("ins_nyd").get()? == 201)
                    || ctx.var("ins_nyd").get()? == 202)
                {
                    ctx.mes("Outsider. Arioss here will explain the situation with the giants, talk with him...")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ((((((ctx.var("ins_nyd").get()? == 72 || ctx.var("ins_nyd").get()? == 82) || ctx.var("ins_nyd").get()? == 92)
                    || ctx.var("ins_nyd").get()? == 102)
                    || ctx.var("ins_nyd").get()? == 112)
                    || ctx.var("ins_nyd").get()? == 122)
                    || ctx.var("ins_nyd").get()? == 132)
                {
                    ctx.mes("Welcome to the Laphine camp in Splendide, outsider... I am Lebiordirr. I am in charge here.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Be cautious of your actions. We already have lots of problems with the Sapha tribe as it is."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["If you are cautious with your actions, I won't place any harm on you. Fare well."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ins_nyd").get()? == 203 {
                    ctx.mes("I was waiting for you. You came back safe, that's good news. Did you find anything?")?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Explain about the guardian Nidhoggur's leave.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Commander Lebiordirr", args!["What? The Guardian is not in his nest...?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args![
                            "And because of his disppearance, the Guardian's Shadow is currently destorying the roots of the World Tree...?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Commander Lebiordirr", args!["That's unbelievable. You must be insulting the Guardian's and our pride. I did not provide you with our help for that."])?;
                    ctx.next()?;
                    ctx.lines_as("Aide Arioss", args!["It's not like that, Commander, they speak the truth."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["What are you saying? Arioss, do not forget your place as the Guardian's priest."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Aide Arioss", args!["Even though I have not seen it with my own eyes, this does explain why we lost communication with the great World Tree Yggdrasil."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Do not speak of His Highness, the World Tree Yggdrasil's name so lightly."],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Pass along World Tree Yggdrasil's words.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["His Highness, the World Tree Yggdrasil, said that?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Commander Lebiordirr", args!["The reason behind all of this... is not because of the Sapha tribe, but because of the sudden leave of the Guardian? And the Guardian has given up on his identity?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["This must be reported... reported to the High Priest of Alfheim... Unbelievable."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aide Arioss",
                        args!["Commander... do we need to alert the rest of the tribe...?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Commander Lebiordirr", args!["You don't need to worry about this, Arioss. As commander, I will handle it. You just pretend nothing happened..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Strange one, thank you for your cooperation in such situations... Please forget what has happened today..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Arioss, please compensate this strange one for the help. I need to go rest..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Aide Arioss", args!["Commander..."])?;
                    ctx.var("ins_nyd").set(Val::from(121))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ins_nyd").get()? == 121 || ctx.var("ins_nyd").get()? == 131) {
                    ctx.mes("Strange one, thank you for your cooperation in such situations... Please forget what has happened today...")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("Welcome to the Laphine camp in Splendide, outsider... I am Lebiordirr. I am in charge here.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["Be cautious of your actions. We already have lots of problems with the Sapha tribe as it is."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Commander Lebiordirr",
                        args!["If you are cautious with your actions, I won't place any harm on you. Fare well."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        ctx.mes("ThusDurnah Ra SharVeldIyaz U UorAmanDur Yee neaOsaAdor Yee...")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn commander_lebiordirr_edq(ctx: &Ctx) -> Script {
    commander_lebiordirr_edq_body(ctx, Vec::new()).map(|_| ())
}

fn aide_arioss_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Aide Arioss]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ins_nyd").get()? == 91 {
            ctx.mes("Sigh, since it's Commander Lebiordirr's wish, I will cooperate with you...")?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["You want to go there to find out exactly what that place is?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["Alright...it's as you have assumed, a place connecting to the World Tree. It's also the Guardian's Nest."],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Guardian's Nest?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Aide Arioss",
                args!["Yes, that's the nest of Nidhoggur, the Guardian of the World Tree..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["Only a marked guardian's servant from the Laphine tribe is allowed to enter."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args![
                    "That place was initially closed off from the public, but then the giants of the Sapha tribe turned things around..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["To obtain more minerals, they started to dig with madness."],
            )?;
            ctx.next()?;
            ctx.lines_as("Aide Arioss", args!["In the end, they have harmed a part of the World Tree's root that lies very close to their mine, and thus, the World Tree became very ill."])?;
            ctx.next()?;
            ctx.lines_as("Aide Arioss", args!["After we have arrived here knowing the facts, the World Tree was already in a very bad state due to the many factories built by the Sapha tribe."])?;
            ctx.next()?;
            ctx.lines_as("Aide Arioss", args!["Even though, for a long time, we have been attacking the Sapha tribe, trying to force them to leave, they are not so easily defeated, leading to this stalemate."])?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["During this time, your people have travelled through the space-time gap to this land."],
            )?;
            ctx.next()?;
            ctx.lines_as("Aide Arioss", args!["Because of those many unexpected reasons, the Guardian's nest, which was meant to stay hidden deep underground, has been discovered by your race..."])?;
            ctx.next()?;
            ctx.lines_as("Aide Arioss", args!["Now you understand how serious this situation is. Because of the Sapha tribe, we are no longer able to approach the Guardian, and seek his teachings of wisdom."])?;
            ctx.next()?;
            ctx.lines_as("Aide Arioss", args!["It's very insulting to our pride... but if you can help us enter that place again, and speak to the Guardian, maybe we can find a solution..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["Then we'll leave it to you. As for your request, I will do my best to cooperate."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aide Arioss",
                args!["I have already told you what you wanted to know... If you need anything else, just let me know."],
            )?;
            ctx.var("ins_nyd").set(Val::from(101))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ins_nyd").get()? == 101 {
                ctx.mes("Ah, you said that there is a strange power blocking the entrance to the Guardian's nest, right?")?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["That's because only the ones chosen by the Guardian may enter."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["We have been the Guardian's servants for generations, and have been protecting the World Tree ever since."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args![
                        "Before we were captured and brought here, there were 3 servants including me. Our task was to heal the World Tree."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["One was killed in our last war with the Sapha tribe... and the other has been taken captive."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["I will give you my proof. Don't worry, I'm just temporarily marking you as a guardian's servant."],
                )?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["Also, remember this spell, it's needed to open the gate of the Guardian."],
                )?;
                ctx.next()?;
                ctx.lines_as("Aide Arioss", args!["AnomarDu Ha OdesUdenVer Ie "])?;
                ctx.next()?;
                ctx.lines_as("Aide Arioss", args!["remuAlaAsh Mu ModtasAn Yu Dur"])?;
                ctx.next()?;
                ctx.lines_as("Aide Arioss", args!["TalsehrDur So CyaReMush Di DielAlaWos Ie RuffserIman Ie "])?;
                ctx.next()?;
                ctx.lines_as("Aide Arioss", args!["Go find the fairy guarding the gate, and say this spell."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["The proof and the spell will confirm that you are one of the Guardian's servants."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["Please meet the Guardian, and come back with an answer to everything. I believe in you."],
                )?;
                ctx.var("ins_nyd").set(Val::from(111))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ((((ctx.var("ins_nyd").get()? == 101 || ctx.var("ins_nyd").get()? == 111) || ctx.var("ins_nyd").get()? == 200)
                || ctx.var("ins_nyd").get()? == 201)
                || ctx.var("ins_nyd").get()? == 202)
            {
                ctx.mes("Please meet with the Guardian and take a wise answer from him. I will trust you.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ((((ctx.var("ins_nyd").get()? == 72 || ctx.var("ins_nyd").get()? == 82) || ctx.var("ins_nyd").get()? == 92)
                || ctx.var("ins_nyd").get()? == 102)
                || ctx.var("ins_nyd").get()? == 112)
            {
                ctx.mes("... ...")?;
                ctx.next()?;
                ctx.mes("Not even caring about this a single bit? What a stupid woman...")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ins_nyd").get()? == 121 {
                ctx.mes("Things have actually become like this... as priests of the Guardian, it's our responsibility...")?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args![
                        "I represent the entire Laphine tribe, and show you our gratitude. This must all be very hard for our commander..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["It can't compare with the effort you have put in for us... but please accept our token of friendship."],
                )?;
                ctx.call(Function::GetExperience, vec![Val::from(150000), Val::from(35000)])?;
                ctx.call(Function::GetItem, vec![Val::from(6081), Val::from(10)])?;
                ctx.var("ins_nyd").set(Val::from(131))?;
                ctx.lines_as(
                    "Aide Arioss",
                    args!["If we can help you with anything in the future, we will do all we can to assist you. Once again, thank you."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ins_nyd").get()? == 131 {
                ctx.mes("If we can help you with anything in the future, we will do all we can to assist you. Once again, thank you.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.mes("... ...")?;
                ctx.next()?;
                ctx.mes("Never give attention to... um... a blunt woman...")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.mes("AmanVilShar Ie DorLuShar Mu Re")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn aide_arioss_edq(ctx: &Ctx) -> Script {
    aide_arioss_edq_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapSEdqStep {
    Start,
    OnTouch,
}

fn trap_s_edq_run(ctx: &Ctx, mut step: TrapSEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapSEdqStep::Start => {
                step = TrapSEdqStep::OnTouch;
                continue 'machine;
            }
            TrapSEdqStep::OnTouch => {
                if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ins_nyd").get()? == 51) {
                    ctx.lines_as("Splendide Guard", args!["That man is currently under arrest."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Splendide Guard",
                        args!["You'd better behave, Aide Arioss says that different races shall be treated the same way."],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("spl_in01"), Val::from(109), Val::from(58)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_s_edq(ctx: &Ctx) -> Script {
    trap_s_edq_run(ctx, TrapSEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn trap_s_edq_ontouch(ctx: &Ctx) -> Script {
    trap_s_edq_run(ctx, TrapSEdqStep::OnTouch, Vec::new()).map(|_| ())
}

fn splendide_guard_1_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Splendide Guard]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        ctx.mes("This is the Splendide office. Don't act impolitely.")?;
    } else {
        ctx.mes("ThusDurnah Ra SharVeldIyaz U UorAmanDur Yee neaOsaAdor Yee ")?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn splendide_guard_1_edq(ctx: &Ctx) -> Script {
    splendide_guard_1_edq_body(ctx, Vec::new()).map(|_| ())
}

fn splendide_guard_2_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Splendide Guard]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        ctx.mes("This is the Splendide office. Don't act impolitely.")?;
    } else {
        ctx.mes("ThusDurnah Ra SharVeldIyaz U UorAmanDur Yee neaOsaAdor Yee ")?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn splendide_guard_2_edq(ctx: &Ctx) -> Script {
    splendide_guard_2_edq_body(ctx, Vec::new()).map(|_| ())
}

fn neat_etorr_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Neat Etorr]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ins_nyd").get()? == 52 {
            ctx.mes("Guest from the other world, please excuse our rudeness...")?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["As the leader of the Sapha tribe, I have something very important to tell you. That is why I asked for you..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["Your race has come to this land not long ago through the space-time gap."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["You should have already seen what is going on. We have been at war with the Laphine tribe for a very long time."],
            )?;
            ctx.next()?;
            ctx.lines_as("Neat Etorr", args!["Recently, because you do not understand the current situations, you have done some things that I, as a leader, can't ignore any longer."])?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["I hope that your people will becareful of your actions on this land, and terminate any unnecessary interventions."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Leave quietly.:Don't know what he's talking about, and ask for details.")],
            )?) == 1
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Manuk Field Elite Soldier",
                args!["Are we really just letting these people go after they have stirred up problems on our land?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["After what happened, I don't think they know what else they can do. Let me explain."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["For now, we have acknowledged your race, and have been tolerant towards your activities."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["But we have recently started to suspect you of being the Laphine tribe's eyes and ears."],
            )?;
            ctx.next()?;
            ctx.lines_as("Neat Etorr", args!["Not long ago, We have heard that you have found remains in an abandoned cave in the north, and have been conducting investigations and researches regarding it."])?;
            ctx.next()?;
            ctx.lines_as("Neat Etorr", args!["We have captured a Laphine tribe priest during the last war, and we have heard that you have had contact with said captive."])?;
            ctx.next()?;
            ctx.lines_as("Neat Etorr", args!["That captive is the only reference for our tribe, so we have taken good care of her. She is related to your current investigation."])?;
            ctx.next()?;
            ctx.lines_as("Neat Etorr", args!["For your people, who have no direct connections, it is not a place satisfy your curiosity. I hope that you stop your investigations, and mind your own business."])?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["If you don't accept our request, then do not expect any cooperations between our races in the future."],
            )?;
            ctx.next()?;
            ctx.lines_as("Neat Etorr", args!["Because of the war against the Laphine tribe, everything is a mess. But even then, we cannot allow a foreign race to interfere."])?;
            ctx.next()?;
            ctx.lines_as(
                "Neat Etorr",
                args!["You have heard it all, now please report back to your race."],
            )?;
            ctx.var("ins_nyd").set(Val::from(62))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ins_nyd").get()? == 62 {
                ctx.mes("That captive is the only reference for our tribe, so we have taken good care of her. She is related to your current investigation.")?;
                ctx.next()?;
                ctx.lines_as("Neat Etorr", args!["For your people, who have no direct connections, it is not a place to satisfy your curiosity. I hope that you stop your investigations, and mind your own business."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Neat Etorr",
                    args!["If you don't accept our request, then do not expect any cooperations between our races in the future."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ins_nyd").get()? == 72 {
                    ctx.mes("Recently, because you do not understand the current situations, you have done some things that I, as a leader, can't ignore any longer.")?;
                    ctx.next()?;
                    ctx.lines_as("Neat Etorr", args!["I hope that your people will becareful of your actions on this land, and terminate any unnecessary interventions."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ins_nyd").get()? == 82 {
                        ctx.mes("Do you need me for something, strange one?")?;
                        ctx.next()?;
                        let choice = runtime::select_values(
                            ctx,
                            &[Val::from("Received invitation from the Sapha tribe to cooperate and investigate.")],
                        )?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Neat Etorr", args!["Cooperate and investigate? What do you mean?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["It seems like you did not understand what I said. What are you thinking?"],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Please allow me to investigate the cave.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Neat Etorr", args!["From what I can tell, you are disrespecting my request. Is this your intention, or your people's intention?"])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("It's our intention.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Neat Etorr", args!["Ah, it is not an easy decision. You frighten me."])?;
                        ctx.next()?;
                        ctx.lines_as("Neat Etorr", args!["But may I ask why you want to do this?"])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Explain Professor Magnifier's theory.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["It's surprising that you are able to obtain such results. We also have a similar theory."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["At least we also think that we must use the Laphine prisoner to get information."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Neat Etorr", args!["This problem has been the root of our conflicts with the Laphine tribe. Of course, it may also be an opportunity to resolve them."])?;
                        ctx.next()?;
                        ctx.lines_as("Neat Etorr", args!["Therefore, it's more of a reason to depend on our own powers. There is no room for you to interfere. Please give up."])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Give up request, and ask Laphine tribe for help instead.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Manuk Field Elite Soldier",
                            args!["You finally showed your true face. I knew you were a spy from the Laphine tribe!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["Ah...you'd go as far as saying that. What good will it do for you, helping the Laphine tribe?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["Those obnoxious dwarves offended our right of living, and are trying to get rid of us."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["The reason we don't allow anyone to enter that site is because it's full of suspicions."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Neat Etorr", args!["Alright, I'll approve of your request, and let you investigate the secret the Laphine tribe has hidden inside the cave."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["If you promise to share all of your research results and findings, we will accept your request."],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Of course.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Neat Etorr", args!["Alright, from now on, we will cooperate with you, and allow you to directly communicate with the Laphine prisoner."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Manuk Field Elite Soldier",
                            args!["Commander...are you sure of this? We don't even know if they're friend or foe..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Neat Etorr", args!["With just our powers, it's impossible to figure out what the Laphine is up to. I think the past has proven that."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["What they're doing right now could potentially lead us to a solution."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args![
                                "Strange one, we hope our cooperation will be a good one. Now please go interrogate the Laphine prisoner."
                            ],
                        )?;
                        ctx.var("ins_nyd").set(Val::from(92))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (((((ctx.var("ins_nyd").get()? == 92 || ctx.var("ins_nyd").get()? == 102)
                        || ctx.var("ins_nyd").get()? == 112)
                        || ctx.var("ins_nyd").get()? == 200)
                        || ctx.var("ins_nyd").get()? == 201)
                        || ctx.var("ins_nyd").get()? == 202)
                    {
                        ctx.mes(
                            "Strange one, we hope our cooperation will be a good one. Now please go interrogate the Laphine prisoner.",
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ((((((ctx.var("ins_nyd").get()? == 71 || ctx.var("ins_nyd").get()? == 81)
                        || ctx.var("ins_nyd").get()? == 91)
                        || ctx.var("ins_nyd").get()? == 101)
                        || ctx.var("ins_nyd").get()? == 111)
                        || ctx.var("ins_nyd").get()? == 121)
                        || ctx.var("ins_nyd").get()? == 131)
                    {
                        ctx.mes("Outsider. Welcome to Manuk, the village of the Sapha. I am its representative, Neat Etorr.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["We are just a small village, nothing special... but rest comfortably."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("ins_nyd").get()? == 203 {
                        ctx.mes("I was waiting for you. It's good that you're safe. Did you find anything?")?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Explain the sudden leave of the Guardian.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["So... that's the nest of the Guardian of the World Tree, the sacred grounds for the Laphine tribe."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["But because of the angry leave of the Guardian, his shadow is wreaking havoc on the World Tree?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Neat Etorr", args!["If what you're saying is true, then there is no more reason for us to continue fighting the Laphine tribe..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["What a huge discovery... So Nidhoggur is no longer the Guardian of the World Tree...?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args![
                                "Nidhoggur's Shadow came to exist in this world, and harmed the roots of the World Tree... what a disaster."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Pass along the World Tree Yggdrasil's message.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Neat Etorr", args!["Is that what the priest of the Guardian said?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["From now on, we need to talk about this with the Laphine tribe."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["But of course...we don't know if they're reasonable enough...hehe."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["This is all we needed from you...What is left is business between us and the Laphine tribe."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["Thank you for helping us with such a huge problem. You may forget about it now."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["It's not a lot, but please this as a token of our appreciation."],
                        )?;
                        ctx.call(Function::GetExperience, vec![Val::from(150000), Val::from(35000)])?;
                        ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(10)])?;
                        ctx.var("ins_nyd").set(Val::from(132))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("ins_nyd").get()? == 132 {
                        ctx.mes("Strange one, thank you for helping us in the time of need. I will never forget your kindness.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("Outsider. Welcome to the Manuk village of Sapha. I am its representative, Neat Etorr.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Neat Etorr",
                            args!["We are just a small village, nothing special... but rest comfortably."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    } else {
        ctx.mes("Tkeh likek Ohek QekhlHkl PkedlioH.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn neat_etorr_edq(ctx: &Ctx) -> Script {
    neat_etorr_edq_body(ctx, Vec::new()).map(|_| ())
}

fn manuk_guard_1_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Manuk Guard]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        ctx.mes("I'm guarding this Laphine prisoner. Leave me alone.")?;
    } else {
        ctx.mes("Klekod Oi Thekd Pheid Okei.")?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn manuk_guard_1_edq(ctx: &Ctx) -> Script {
    manuk_guard_1_edq_body(ctx, Vec::new()).map(|_| ())
}

fn manuk_guard_2_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Manuk Guard]")?;
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        ctx.mes("I'm guarding this Laphine prisoner. Leave me alone.")?;
    } else {
        ctx.mes("Liek QUekdk Ohei Vue.")?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn manuk_guard_2_edq(ctx: &Ctx) -> Script {
    manuk_guard_2_edq_body(ctx, Vec::new()).map(|_| ())
}
