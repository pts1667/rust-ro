use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn monster_controler_aru_gd_run(ctx: &Ctx, mut step: MonsterControlerAruGdStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterControlerAruGdStep::Start => {
                step = MonsterControlerAruGdStep::OnInit;
                continue 'machine;
            }
            MonsterControlerAruGdStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Controler1#aru::OnKill")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MonsterControlerAruGdStep::OnTimer3600000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Controler1#aru::OnEnable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_dun01"),
                        Val::from("Kublin: Aargh!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_dun01"),
                        Val::from("Morestone: Stop righ there! You thief!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_controler_aru_gd(ctx: &Ctx) -> Script {
    monster_controler_aru_gd_run(ctx, MonsterControlerAruGdStep::Start, Vec::new()).map(|_| ())
}

pub fn monster_controler_aru_gd_oninit(ctx: &Ctx) -> Script {
    monster_controler_aru_gd_run(ctx, MonsterControlerAruGdStep::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_controler_aru_gd_ontimer3600000(ctx: &Ctx) -> Script {
    monster_controler_aru_gd_run(ctx, MonsterControlerAruGdStep::OnTimer3600000, Vec::new()).map(|_| ())
}

pub fn monster_controler1_aru(ctx: &Ctx) -> Script {
    monster_controler1_aru_run(ctx, MonsterControler1AruStep::Start, Vec::new()).map(|_| ())
}

pub fn monster_controler1_aru_onenable(ctx: &Ctx) -> Script {
    monster_controler1_aru_run(ctx, MonsterControler1AruStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster_controler1_aru_onkill(ctx: &Ctx) -> Script {
    monster_controler1_aru_run(ctx, MonsterControler1AruStep::OnKill, Vec::new()).map(|_| ())
}

pub fn monster_controler1_aru_onmymobdead(ctx: &Ctx) -> Script {
    monster_controler1_aru_run(ctx, MonsterControler1AruStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn dwarf_aru_gd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_chk_urquest = Val::from(0);
    let mut l_chk_urquest1 = Val::from(0);
    let mut l_chk_yourgdname_s = Val::from("");
    if ctx.call(Function::GetCharacterId, vec![Val::from(2)])? == 0 {
        ctx.lines_as(
            "Dwarf",
            args!["Hey did you see an ugly Goblin come by? He stole something from me!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_chk_urquest = ctx.call(Function::CheckQuest, vec![Val::from(2143), ctx.constant("PLAYTIME")?])?;
    l_chk_yourgdname_s = ctx.call(
        Function::GetGuildInfo,
        vec![ctx.call(Function::GetCharacterId, vec![Val::from(2)])?, Val::from(0)],
    )?;
    if ctx.var("$@gdeventv_a1").get()? == 0 {
        if ctx.var("$@gdevents_a$").get()? == "" {
            ctx.var("$@gdeventv_a1").set(Val::from(1))?;
            ctx.var("$@gdevents_a$").set(l_chk_yourgdname_s.clone())?;
            ctx.lines_as("Dwarf", args!["Help me!", "Please, help me!"])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("What happened?:Nevermind.")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as("Dwarf", args!["I am Morestone and I collect rare gems."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morestone",
                        args![
                            "In my travels, I was told that there were a lot of gems in Valfreyja and Nidhoggur, so I came down here.",
                            "But here, the soil is very hard to dig into. Fortunately, my ^3131FFPickaxe^000000 never lets me down!"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Pickaxe!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Morestone",
                        args!["Yes, my beloved pickaxe!", "I always carry it with me, you know?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morestone",
                        args!["We started working here together.", "After a few days, we finally found something!"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Something strange??")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Morestone",
                        args![
                            "No, but it was worth quite a lot.",
                            "But we did not have much time to celebrate. Suddenly, a monster appeared that stole my pickaxe.",
                            "His name was^3131FFKublin^000000!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Morestone", args!["He stole my Pickaxe!", "I can't live without it..."])?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(6010)])?.number()? > 0 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Is this the pickaxe that you've been looking for?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Morestone", args!["You found my Pickaxe?", "Show me, please!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "Oh, my! You've returned it to me!",
                                "My precious pickaxe, I thought I lost you forever."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args!["You are great! What guild are you from?", "Could it be Gravity or Mercury?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![((Val::from("I am a member of the ^3131FF") + l_chk_yourgdname_s.clone()) + Val::from("^."))],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Morestone", args!["Oh... That guild will receive my greatest respect."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "Oh! My friend, I am very grateful for your help.",
                                "As a reward, I will tell you about a mysterious area I have discovered recently."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("A mysterious area?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Morestone", args!["That's right. I found it when I was digging around here.", "It looks like it was made for some special purpose, but since there are no gems around there, I have no interest in it."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args!["Instead of going there alone, I think it would be more fun to go with your friends..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "If you want, I can take ^3131FFyou and your guild members^000000 to explore that area.",
                                "Do you want to go there now?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Wait! I'm not ready yet.:Let's go!")])? {
                            1 => {
                                ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                                ctx.var("$@gdevents_a$").set(Val::from(""))?;
                                ctx.lines_as("Morestone", args!["Take your time, and find a place to gather your friends."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Morestone",
                                    args![
                                        "Alright! Let's go.",
                                        "If your friends visit me again later, I will guide them to that area again.",
                                        "Don't forget, dwarves are grateful beings! Hahaha!"
                                    ],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(6010), Val::from(1)])?;
                                ctx.var("$@gdeventv_a1").set(Val::from(1))?;
                                ctx.var("$@gdevents_a$").set(l_chk_yourgdname_s.clone())?;
                                ctx.close_window()?;
                                ctx.call(Function::SetQuest, vec![Val::from(2144)])?;
                                ctx.call(Function::Warp, vec![Val::from("arug_que01"), Val::from(103), Val::from(133)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                        ctx.var("$@gdevents_a$").set(Val::from(""))?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "I will tell you how to find him.",
                                "Kublin wears a ridiculous golden hat, It should be easy to recognise him by that."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                    ctx.var("$@gdevents_a$").set(Val::from(""))?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.lines_as("Dwarf", args!["Ahhh..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else if ctx.var("$@gdevents_a$").get()?.loosely_equals(&l_chk_yourgdname_s.clone()) {
            l_chk_urquest1 = ctx.call(Function::CheckQuest, vec![Val::from(2144)])?;
            if (l_chk_urquest1.clone() == 0 || l_chk_urquest1.clone() == 1) {
                ctx.lines_as(
                    "Morestone",
                    args![
                        ((Val::from("I, Morestone, have made an alliance with the ") + ctx.var("$@gdevents_a$").get()?)
                            + Val::from(" guild.")),
                        "Oh, you are a member.",
                        "Would you like to go to the mysterious area?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Let's go.:No, thanks.")])? {
                    1 => {
                        ctx.lines_as("Morestone", args!["I hope you enjoy yourself, my friend."])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("arug_que01"), Val::from(103), Val::from(133)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Morestone", args!["If you need my assistance, just ask.", "Ah! Dont forget, I hate monsters! So I don't want to see them. It will be better if you ask for another favour."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.var("$@gdeventv_a1").set(Val::from(1))?;
                ctx.var("$@gdevents_a$").set(l_chk_yourgdname_s.clone())?;
                ctx.lines_as("Dwarf", args!["Help me!", "Please, help me!"])?;
                ctx.next()?;
                'b4: {
                    let subject4 = Val::from(runtime::select_values(ctx, &[Val::from("What happened?:Nevermind.")])?);
                    let mut matched4 = false;
                    let no_case4 = !subject4.loosely_equals(&Val::from(1)) && !subject4.loosely_equals(&Val::from(2));
                    if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                        matched4 = true;
                    }
                    if matched4 {
                        ctx.lines_as("Dwarf", args!["I am Morestone and I collect rare gems."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "In my travels, I was told that there were a lot of gems in Valfreyja and Nidhoggur, so I came down here.",
                                "But here, the soil is very hard to dig into. Fortunately, my ^3131FFPickaxe^000000 never lets me down!"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Pickaxe!")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Morestone",
                            args!["Yes, my beloved pickaxe!", "I always carry it with me, you know?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args!["We started working here together.", "After a few days, we finally found something!"],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Something strange??")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "No, but it was worth quite a lot.",
                                "But we did not have much time to celebrate. Suddenly, a monster appeared that stole my pickaxe.",
                                "His name was^3131FFKublin^000000!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Morestone", args!["He stole my Pickaxe!", "I can't live without it..."])?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, vec![Val::from(6010)])?.number()? > 0 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Is this the pickaxe that you've been looking for?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Morestone", args!["You found my Pickaxe?", "Show me, please!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "Oh, my! You've returned it to me!",
                                    "My precious pickaxe, I thought I lost you forever."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args!["You are great! What guild are you from?", "Could it be Gravity or Mercury?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![((Val::from("I am a member of the ^3131FF") + l_chk_yourgdname_s.clone()) + Val::from("^."))],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Morestone", args!["Oh... That guild will receive my greatest respect."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "Oh! My friend, I am very grateful for your help.",
                                    "As a reward, I will tell you about a mysterious area I have discovered recently."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("A mysterious area?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as("Morestone", args!["That's right. I found it when I was digging around here.", "It looks like it was made for some special purpose, but since there are no gems around there, I have no interest in it."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args!["Instead of going there alone, I think it would be more fun to go with your friends..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "If you want, I can take ^3131FFyou and your guild members^000000 to explore that area.",
                                    "Do you want to go there now?"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Wait! I'm not ready yet.:Let's go!")])? {
                                1 => {
                                    ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                                    ctx.var("$@gdevents_a$").set(Val::from(""))?;
                                    ctx.lines_as("Morestone", args!["Take your time, and find a place to gather your friends."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Morestone",
                                        args![
                                            "Alright! Let's go.",
                                            "If your friends visit me again later, I will guide them to that area again.",
                                            "Don't forget, dwarves are grateful beings! Hahaha!"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(6010), Val::from(1)])?;
                                    ctx.var("$@gdeventv_a1").set(Val::from(1))?;
                                    ctx.var("$@gdevents_a$").set(l_chk_yourgdname_s.clone())?;
                                    ctx.close_window()?;
                                    ctx.call(Function::SetQuest, vec![Val::from(2144)])?;
                                    ctx.call(Function::Warp, vec![Val::from("arug_que01"), Val::from(103), Val::from(133)])?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                            ctx.var("$@gdevents_a$").set(Val::from(""))?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "I will tell you how to find him.",
                                    "Kublin wears a ridiculous golden hat, It should be easy to recognise him by that."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                        matched4 = true;
                    }
                    if matched4 {
                        ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                        ctx.var("$@gdevents_a$").set(Val::from(""))?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.lines_as("Dwarf", args!["Ah...."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        } else {
            ctx.lines_as(
                "Morestone",
                args![
                    ((Val::from("I, Morestone, have made an alliance with the ") + ctx.var("$@gdevents_a$").get()?) + Val::from(" guild.")),
                    "Hm, you're not a member.",
                    "Could you please give them my greetings?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("$@gdevents_a$").get()?.loosely_equals(&l_chk_yourgdname_s.clone()) {
            l_chk_urquest1 = ctx.call(Function::CheckQuest, vec![Val::from(2144)])?;
            if (l_chk_urquest1.clone() == 0 || l_chk_urquest1.clone() == 1) {
                ctx.lines_as(
                    "Morestone",
                    args![
                        "[Morestone]",
                        ((Val::from("I, Morestone, have made an alliance with the ") + ctx.var("$@gdevents_a$").get()?)
                            + Val::from(" guild.")),
                        "Oh, you are a member.",
                        "Would you like to go to the mysterious area?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Let's go.:No, thanks.")])? {
                    1 => {
                        ctx.lines_as("Morestone", args!["I hope you enjoy yourself, my friend."])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("arug_que01"), Val::from(103), Val::from(133)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Morestone", args!["If you need my assistance, just ask.", "Ah! Dont forget, I hate monsters! So I don't want to see them. It will be better if you ask for another favour."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.var("$@gdeventv_a1").set(Val::from(1))?;
                ctx.var("$@gdevents_a$").set(l_chk_yourgdname_s.clone())?;
                ctx.lines_as("Dwarf", args!["Help me!", "Please, help me!"])?;
                ctx.next()?;
                'b7: {
                    let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("What happened?:Nevermind.")])?);
                    let mut matched7 = false;
                    let no_case7 = !subject7.loosely_equals(&Val::from(1)) && !subject7.loosely_equals(&Val::from(2));
                    if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                        matched7 = true;
                    }
                    if matched7 {
                        ctx.lines_as("Dwarf", args!["I am Morestone and I collect rare gems."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "In my travels, I was told that there were a lot of gems in Valfreyja and Nidhoggur, so I came down here.",
                                "But here, the soil is very hard to dig into. Fortunately, my ^3131FFPickaxe^000000 never lets me down!"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Pickaxe!")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Morestone",
                            args!["Yes, my beloved pickaxe!", "I always carry it with me, you know?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morestone",
                            args!["We started working here together.", "After a few days, we finally found something!"],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Something strange??")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Morestone",
                            args![
                                "No, but it was worth quite a lot.",
                                "But we did not have much time to celebrate. Suddenly, a monster appeared that stole my pickaxe.",
                                "His name was^3131FFKublin^000000!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Morestone", args!["He stole my Pickaxe!", "I can't live without it..."])?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, vec![Val::from(6010)])?.number()? > 0 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Is this the pickaxe that you've been looking for?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Morestone", args!["You found my Pickaxe?", "Show me, please!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "Oh, my! You've returned it to me!",
                                    "My precious pickaxe, I thought I lost you forever."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args!["You are great! What guild are you from?", "Could it be Gravity or Mercury?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![((Val::from("I am a member of the ^3131FF") + l_chk_yourgdname_s.clone()) + Val::from("^."))],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Morestone", args!["Oh... That guild will receive my greatest respect."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "Oh! My friend, I am very grateful for your help.",
                                    "As a reward, I will tell you about a mysterious area I have discovered recently."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("A mysterious area?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as("Morestone", args!["That's right. I found it when I was digging around here.", "It looks like it was made for some special purpose, but since there are no gems around there, I have no interest in it."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args!["Instead of going there alone, I think it would be more fun to go with your friends..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "If you want, I can take ^3131FFyou and your guild members^000000 to explore that area.",
                                    "Do you want to go there now?"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Wait! I'm not ready yet.:Let's go!")])? {
                                1 => {
                                    ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                                    ctx.var("$@gdevents_a$").set(Val::from(""))?;
                                    ctx.lines_as("Morestone", args!["Take your time, and find a place to gather you friends."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Morestone",
                                        args![
                                            "Alright! Let's go.",
                                            "If your friends visit me again later, I will guide them to that area again.",
                                            "Don't forget, dwarves are grateful beings! Hahaha!"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(6010), Val::from(1)])?;
                                    ctx.var("$@gdeventv_a1").set(Val::from(1))?;
                                    ctx.var("$@gdevents_a$").set(l_chk_yourgdname_s.clone())?;
                                    ctx.close_window()?;
                                    ctx.call(Function::SetQuest, vec![Val::from(2144)])?;
                                    ctx.call(Function::Warp, vec![Val::from("arug_que01"), Val::from(103), Val::from(133)])?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                            ctx.var("$@gdevents_a$").set(Val::from(""))?;
                            ctx.lines_as(
                                "Morestone",
                                args![
                                    "I will tell you how to find him.",
                                    "Kublin wears a ridiculous golden hat, It should be easy to recognise him by that."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                        matched7 = true;
                    }
                    if matched7 {
                        ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                        ctx.var("$@gdevents_a$").set(Val::from(""))?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.lines_as("Dwarf", args!["Ah...."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        } else {
            ctx.lines_as(
                "Morestone",
                args![
                    ((Val::from("I, Morestone, have made an alliance with the ") + ctx.var("$@gdevents_a$").get()?) + Val::from(" guild.")),
                    "Hm, you're not a member.",
                    "Could you please give them my greetings?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn dwarf_aru_gd(ctx: &Ctx) -> Script {
    dwarf_aru_gd_body(ctx, Vec::new()).map(|_| ())
}

fn dwarf_aru_gd_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Dwarf#aru_gd")])?;
    ctx.var("$@gdeventv_a1").set(Val::from(0))?;
    ctx.var("$@gdevents_a$").set(Val::from(""))?;
    return Err(Stop::End);
}

pub fn dwarf_aru_gd_oninit(ctx: &Ctx) -> Script {
    dwarf_aru_gd_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn dwarf_aru_gd_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Dwarf#aru_gd")])?;
    return Err(Stop::End);
}

pub fn dwarf_aru_gd_onenable(ctx: &Ctx) -> Script {
    dwarf_aru_gd_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn pierrot_pier_aru_gd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_que_2143 = Val::from(0);
    let mut l_sprchg_gd = Val::from(0);
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
    l_sprchg_gd = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
    if l_sprchg_gd.clone() == 1 {
        ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(950)])?;
    } else if l_sprchg_gd.clone() == 2 {
        ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(715)])?;
    } else if l_sprchg_gd.clone() == 3 {
        ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(714)])?;
    } else if l_sprchg_gd.clone() == 4 {
        ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(785)])?;
    } else {
        ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(876)])?;
    }
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait!! -",
            "- You're carrying too many items, -",
            "- you can't receive the materials. -",
            "- Please use the Kafra Services, -",
            "- and come back later. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx
        .call(
            Function::GetGuildInfo,
            vec![ctx.call(Function::GetCharacterId, vec![Val::from(2)])?, Val::from(2)],
        )?
        .loosely_equals(&Val::from(1))
    {
        if ctx.var("$@gdeventv_a2").get()? == 0 {
            l_que_2143 = ctx.call(Function::CheckQuest, vec![Val::from(2143), ctx.constant("PLAYTIME")?])?;
            if l_que_2143.clone() == -1 {
                ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(715)])?;
                ctx.mes("A lonely clown is juggling.")?;
                ctx.next()?;
                ctx.mes("When looked at closely, the clown is just a puppet that looks like a human.")?;
                ctx.next()?;
                ctx.mes("The clown stops, then starts moving in accordance to your movements, noises start to emit from it's mouth.")?;
                ctx.next()?;
                ctx.lines_as("Pierrot Pier", args!["Beep beep beep.", "Hello, my friends!"])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: Beep beep beep! Hello, my friends!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pierrot Pier",
                    args![
                        "I am the loyal servant of Gergath, and I have finally received my orders.",
                        "I am happy to hear all the laughter, but without my master's permission, I can't do anything."
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from(
                            "Pierrot Pier: I am happy to hear all the laughter, but without my master's permission, I can't do anything.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Pierrot Pier", args!["Did you get permission from Gergath?"])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: Did you get permission from Gergath?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I need to check that.:No.")])? {
                    1 => {
                        ctx.lines_as("Pierrot Pier", args!["Please give me the palm of your hand."])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Please give me the palm of your hand."),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Pierrot Pier", args!["Let me see..."])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Let me see..."),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as("Pierrot Pier", args!["Hm..."])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Hm..."),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as("Pierrot Pier", args!["Okay, I see..."])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Okay, I see..."),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as("Pierrot Pier", args!["Indeed..."])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Indeed..."),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_STARE")?])?;
                        ctx.lines_as("Pierrot Pier", args!["Verification completed!"])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Verification completed!"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Pierrot Pier",
                            args!["Hm? that's right.", "When is that person coming? I am very bored~!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["^3131FF<You're someone who can only think about nonsense everyday, you have no focus at all.>^000000"],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: <You're someone who can only think about nonsense everyday, you have no focus at all.>"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                ctx.lines_as("Pierrot Pier", args!["Hm? You don't think so?"])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: Hm? You don't think so?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["Haha, I'm just kidding. Beep beep.", "Ah, you are the one my master speaks of."],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: Haha, I'm just kidding. Beep beep. You are the one my master speaks of."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pierrot Pier",
                    args![
                        "Come, the Gergath has left a message for you.",
                        "It's a bit old, but it should still be legible."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Pierrot Pier", args!["Alright, let's begin!"])?;
                ctx.call(Function::SetQuest, vec![Val::from(2143)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Gergath#aru_gd::OnEnable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_que_2143.clone() == 0 || l_que_2143.clone() == 1) {
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["Let's talk after I finished reading my master's message. Beep beep."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["My master Gergath sincerely wishes you joy for you and your family everyday."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["Alright, would you like to play the game Gergath has prepared for you?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Game instructions.:Skip instructions.:Refuse game.")])? {
                    1 => {
                        ctx.lines_as(
                            "Pierrot Pier",
                            args!["The game prepared by my master is very unique, yet simple and fun!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: The game prepared by my lord is very unique, yet simple and fun!"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Pierrot Pier", args!["It's called \"Find the Treasure Map\"!!"])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: It's called \"Find the Treasure Map\"!!"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Pierrot Pier",
                            args![
                                "Do you see this large and green field? Beep, beep?",
                                "I will show you the most incredible magic here.",
                                "I will turn this place very white. Veeery white!"
                            ],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: I will turn this place very white. Veeery white!"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Pierrot Pier",
                            args![
                                "The game instruction is just to find the treasure map within the time limit.",
                                "Sounds easy, right?"
                            ],
                        )?;
                        ctx.call(Function::MapAnnounce, vec![Val::from("arug_que01"), Val::from("Pierrot Pier: The game instruction is just to find the treasure map within the time limit. ounds easy, right?"), ctx.constant("BC_MAP")?, Val::from("0x99CC00")])?;
                        ctx.next()?;
                    }
                    2 => {}
                    3 => {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.lines_as("Pierrot Pier", args!["Oh, you don't want to play?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["Okay, I'm ready to begin.", "Shall we start? Beep, beep?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Start.")])? {
                    1 => {
                        ctx.lines_as("Pierrot Pier", args!["Let me know when you are ready."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Pierrot Pier", args!["Alright! Let us begin!"])?;
                        ctx.next()?;
                        ctx.lines_as("Pierrot Pier", args!["Ladies, and gentlemen."])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Ladies, and gentlemen."),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Pierrot Pier", args!["Who will find the treasure map in this white world?"])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Who will find the treasure map in this white world?"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Pierrot Pier", args!["Amongst all of you, who shall be the lucky one?"])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Amongst all of you, who shall be the lucky one?"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Pierrot Pier", args!["Let the game.. Begin!"])?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("arug_que01"),
                                Val::from("Pierrot Pier: Let the game.. Begin!"),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x99CC00"),
                            ],
                        )?;
                        ctx.var("$@gdeventv_a2").set(Val::from(1))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Controller#gdevent_a::Ongame_start")])?;
                        ctx.call(Function::EraseQuest, vec![Val::from(2143)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        } else {
            if ctx.var("$@gdeventv_a2").get()? == 1 {
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["Did you find the treasure map?", "Show me what you have in your hands! Beep, beep!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Pierrot Pier", args!["Let me see..."])?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(6031)])?.number()? > 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Controller#gdevent_a::OnStop")])?;
                    ctx.var("$@gdeventv_a2").set(Val::from(3))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("eff_mvp#aru_gd::Onmvp")])?;
                    ctx.lines_as("Pierrot Pier", args!["Wow~~!!", "Success~!!", "What a success~!!"])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Wow~~!! Success~!! What a success~!!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(6031), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.call(Function::CountItem, vec![Val::from(6030)])?.number()? > 0 {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "Ahh, what a shame, it seems like you haven't found the treasure map yet.",
                            "Quickly! Your time is running out! Hurry up!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "I don't see anything. Have you even started yet? Beep?",
                            "Hehe, while you're talking to me, the time is slowly ticking away~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("$@gdeventv_a2").get()? == 2 {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "Wah, why is it like this~!!",
                            "Not enough? But this makes the game fun, no? Hahaha!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["What did you think?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("It was pretty hard.:I should've been successful...")])? {
                        1 => {
                            ctx.lines_as(
                                "Pierrot Pier",
                                args![
                                    "It's like trying to find a needle in a haystack!",
                                    "It's hard, but if you find it, it's worth ten times the effort. Beep, beep."
                                ],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as("Pierrot Pier", args!["Aaah~! Time is gold.", "Precious time goes by so fast."])?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    ctx.lines_as("Pierrot Pier", args!["I, Pierrot Piere, am not a heartless clown! Beep beep."])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: I, Pierrot Piere, am not a heartless clown! Beep beep."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pierrot Pier",
                        args!["Your success is my happiness!", "I'll give you one more chance. How's that? Beep?"],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: I'll give you one more chance. How's that? Beep?"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pierrot Pier",
                        args!["Okay, I'm ready to begin.", "Shall we start? Beep, beep?"],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Okay, I'm ready to begin. Shall we start? Beep, beep?"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("No.:Start.")])? {
                        1 => {
                            ctx.lines_as("Pierrot Pier", args!["Let me know when you are ready."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Pierrot Pier", args!["Alright! Let us begin!"])?;
                            ctx.next()?;
                            ctx.lines_as("Pierrot Pier", args!["Ladies, and gentlemen."])?;
                            ctx.call(
                                Function::MapAnnounce,
                                vec![
                                    Val::from("arug_que01"),
                                    Val::from("Pierrot Pier: Ladies, and gentlemen."),
                                    ctx.constant("BC_MAP")?,
                                    Val::from("0x99CC00"),
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Pierrot Pier", args!["Who will find the treasure map in this white world?"])?;
                            ctx.call(
                                Function::MapAnnounce,
                                vec![
                                    Val::from("arug_que01"),
                                    Val::from("Pierrot Pier: Who will find the treasure map in this white world?"),
                                    ctx.constant("BC_MAP")?,
                                    Val::from("0x99CC00"),
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Pierrot Pier", args!["Amongst all of you, who shall be the lucky one?"])?;
                            ctx.call(
                                Function::MapAnnounce,
                                vec![
                                    Val::from("arug_que01"),
                                    Val::from("Pierrot Pier: Amongst all of you, who shall be the lucky one?"),
                                    ctx.constant("BC_MAP")?,
                                    Val::from("0x99CC00"),
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Pierrot Pier", args!["Let the game.. Begin!"])?;
                            ctx.call(
                                Function::MapAnnounce,
                                vec![
                                    Val::from("arug_que01"),
                                    Val::from("Pierrot Pier: Let the game.. Begin!"),
                                    ctx.constant("BC_MAP")?,
                                    Val::from("0x99CC00"),
                                ],
                            )?;
                            ctx.var("$@gdeventv_a2").set(Val::from(10))?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Controller#gdevent_a::Ongame_start")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("$@gdeventv_a2").get()? == 3 {
                    l_que_2143 = ctx.call(Function::CheckQuest, vec![Val::from(2143)])?;
                    if l_que_2143.clone() == 3 {
                        ctx.call(Function::EraseQuest, vec![Val::from(2143)])?;
                    }
                    ctx.lines_as("Pierrot Pier", args!["How did you do it?"])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: How did you do it?"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["You managed to find a needle in a haystack!", "Amazing!"])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: You managed to find a needle in a haystack! Amazing!!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pierrot Pier",
                        args!["You've completeled an unbelievable task, I will give you the wonderful gift master has prepared!! Ha!"],
                    )?;
                    ctx.call(Function::MapAnnounce, vec![Val::from("arug_que01"), Val::from("Pierrot Pier: You've completeled an unbelievable task, I will give you the wonderful gift my master has prepared!! Ha!"), ctx.constant("BC_MAP")?, Val::from("0x99CC00")])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Here, take Pierre's Treasure Boxes."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("eff_mvp#aru_gd::Onmvp")])?;
                    ctx.var("$@gdeventv_a2").set(Val::from(5))?;
                    ctx.call(Function::GetItem, vec![Val::from(14596), Val::from(10)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("$@gdeventv_a2").get()? == 4 {
                    ctx.lines_as("Pierrot Pier", args!["Incredible! Unbelievable! Beep beep!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("$@gdeventv_a2").get()? == 5 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Seeing your smiles, makes Pierrot feel very happy~ See you next time!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.lines_as("Pierrot Pier", args!["Did you have fun?"])?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["Seeing your smiles, makes Pierrot feel very happy~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "I hope to see you again very soon, I must go back to being a doll now.",
                            "See you next time!"
                        ],
                    )?;
                    ctx.call(Function::EraseQuest, vec![Val::from(2144)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args!["Did you find the treasure map?", "Show me what you have in your hands! Beep, beep!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["Let me see."])?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(6031)])?.number()? > 0 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Controller#gdevent_a::OnStop")])?;
                        ctx.var("$@gdeventv_a2").set(Val::from(3))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("eff_mvp#aru_gd::Onmvp")])?;
                        ctx.lines_as("Pierrot Pier", args!["Wow~~!!", "Success~!!", "What a success~!!"])?;
                        ctx.call(Function::DelItem, vec![Val::from(6031), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.call(Function::CountItem, vec![Val::from(6030)])?.number()? > 0 {
                        ctx.lines_as(
                            "Pierrot Pier",
                            args![
                                "Ahh, what a shame, it seems like you haven't found the treasure map yet.",
                                "Quickly! Your time is running out! Hurry up!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Pierrot Pier",
                            args![
                                "I don't see anything. Have you even started yet? Beep?",
                                "Hehe, while you're talking to me, the time is slowly ticking away~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    } else {
        if ctx.var("$@gdeventv_a2").get()? == 0 {
            ctx.call(Function::SetNpcDisplay, vec![Val::from("Pierrot Pier#aru_gd"), Val::from(715)])?;
            ctx.mes("A lonely clown is juggling.")?;
            ctx.next()?;
            ctx.mes("When looked at closely, the clown is just a puppet that looks like a human.")?;
            ctx.next()?;
            ctx.mes("The clown stops, then starts moving in accordance to your movements, noises start to emit from it's mouth.")?;
            ctx.next()?;
            ctx.lines_as("Pierrot Pier", args!["Beep beep beep.", "Hello, my friends!"])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    Val::from("arug_que01"),
                    Val::from("Pierrot Pier: Beep beep beep! Hello, my friends!"),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x99CC00"),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pierrot Pier",
                args![
                    "I am the loyal servant of Gergath, and I have finally received my orders.",
                    "I am happy to hear all the laughter, but without my master's permission, I can't do anything."
                ],
            )?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    Val::from("arug_que01"),
                    Val::from(
                        "Pierrot Pier: I am happy to hear all the laughter, but without my master's permission, I can't do anything.",
                    ),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x99CC00"),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Pierrot Pier", args!["Did you get permission from the Gergath?"])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    Val::from("arug_que01"),
                    Val::from("Pierrot Pier: Did you get permission from Gergath?"),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x99CC00"),
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I need to check that.:No.")])? {
                1 => {
                    ctx.lines_as("Pierrot Pier", args!["Please give me the palm of your hand."])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Please give me the palm of your hand."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["Let me see..."])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Let me see..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines_as("Pierrot Pier", args!["Hm..."])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Hm..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines_as("Pierrot Pier", args!["Okay, I see..."])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Okay, I see..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines_as("Pierrot Pier", args!["Indeed..."])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Indeed..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_STARE")?])?;
                    ctx.lines_as("Pierrot Pier", args!["Verification completed!"])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Verification completed!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args!["Hm? that's right.", "When is that person coming? I am very bored~!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.lines_as(
                "Pierrot Pier",
                args!["^3131FF<Although you're a weirdo, you seem to be very dedicated>^000000. Beep beep."],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.lines_as("Pierrot Pier", args!["Hm? You don't think so?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Pierrot Pier",
                args!["The one my lord appointed is ^3131FF< someone who leads many people >^000000. Beep."],
            )?;
            ctx.next()?;
            ctx.lines_as("Pierrot Pier", args!["Pierrot wants to be someone like that, too. Beep."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("$@gdeventv_a2").get()? == 1 {
                ctx.lines_as(
                    "Pierrot Pier",
                    args!["Did you find the treasure map?", "Show me what you have in your hands! Beep, beep!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Pierrot Pier", args!["Let me see..."])?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(6031)])?.number()? > 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Controller#gdevent_a::OnStop")])?;
                    ctx.var("$@gdeventv_a2").set(Val::from(3))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("eff_mvp#aru_gd::Onmvp")])?;
                    ctx.lines_as("Pierrot Pier", args!["Wow~~!!", "Success~!!", "What a success~!!"])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_que01"),
                            Val::from("Pierrot Pier: Wow~~!! Success~!! What a success~!!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(6031), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.call(Function::CountItem, vec![Val::from(6030)])?.number()? > 0 {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "Ahh, what a shame, it seems like you haven't found the treasure map yet.",
                            "Quickly! Your time is running out! Hurry up!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "I don't see anything. Have you even started yet? Beep?",
                            "Hehe, while you're talking to me, the time is slowly ticking away~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("$@gdeventv_a2").get()? == 2 {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "Wah, why is it like this~!!",
                            "Not enough? But this makes the game fun, no? Hahaha!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("$@gdeventv_a2").get()? == 3 {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "Congratulations, you have succeeded!",
                            "I will talk to your leader about other details."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("$@gdeventv_a2").get()? == 4 {
                    ctx.lines_as("Pierrot Pier", args!["Incredible! Unbelievable! Beep beep!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("$@gdeventv_a2").get()? == 5 {
                    ctx.lines_as("Pierrot Pier", args!["Did you have fun?"])?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["Seeing your smiles, makes Pierrot feel very happy~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pierrot Pier",
                        args![
                            "I hope to see you again very soon, I must go back to being a doll now.",
                            "See you next time!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Pierrot Pier",
                        args!["Did you find the treasure?", "Show me that thing you are holding, now!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Pierrot Pier", args!["Let me see..."])?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(6031)])?.number()? > 0 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Controller#gdevent_a::OnStop")])?;
                        ctx.var("$@gdeventv_a2").set(Val::from(3))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("eff_mvp#aru_gd::Onmvp")])?;
                        ctx.lines_as("Pierrot Pier", args!["Wow~~!!", "Success~!!", "What a success~!!"])?;
                        ctx.call(Function::DelItem, vec![Val::from(6031), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.call(Function::CountItem, vec![Val::from(6030)])?.number()? > 0 {
                        ctx.lines_as(
                            "Pierrot Pier",
                            args![
                                "Ahh, what a shame, it seems like you haven't found the treasure map yet.",
                                "Quickly! Your time is running out! Hurry up!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Pierrot Pier",
                            args![
                                "I don't see anything. Have you even started yet? Beep?",
                                "Hehe, while you're talking to me, the time is slowly ticking away~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn pierrot_pier_aru_gd(ctx: &Ctx) -> Script {
    pierrot_pier_aru_gd_body(ctx, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::Start, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_oninit(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::OnInit, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_onwin(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::Onwin, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_ongame_start(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::OngameStart, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_onstop(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::OnStop, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_ontimer40000(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::OnTimer40000, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_ontimer60000(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn controller_gdevent_a_ontimer63000(ctx: &Ctx) -> Script {
    controller_gdevent_a_run(ctx, ControllerGdeventAStep::OnTimer63000, Vec::new()).map(|_| ())
}

fn paper_sp_1_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_1_a(ctx: &Ctx) -> Script {
    paper_sp_1_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_1_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while1 = Val::from(0);
    let mut l_paper_x1 = Val::from(0);
    let mut l_paper_y1 = Val::from(0);
    l_paper_while1 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while1.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while1 = (l_paper_while1.clone() + Val::from(1));
                l_paper_x1 = ctx.call(Function::Rand, vec![Val::from(81), Val::from(95)])?;
                l_paper_y1 = ctx.call(Function::Rand, vec![Val::from(87), Val::from(100)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x1.clone(),
                        l_paper_y1.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_1_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_1_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_1_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x1 = Val::from(0);
    let mut l_paper_y1 = Val::from(0);
    l_paper_x1 = ctx.call(Function::Rand, vec![Val::from(81), Val::from(95)])?;
    l_paper_y1 = ctx.call(Function::Rand, vec![Val::from(87), Val::from(100)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x1.clone(),
            l_paper_y1.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_1_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_1_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_2_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_2_a(ctx: &Ctx) -> Script {
    paper_sp_2_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_2_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while2 = Val::from(0);
    let mut l_paper_x2 = Val::from(0);
    let mut l_paper_y2 = Val::from(0);
    l_paper_while2 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while2.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while2 = (l_paper_while2.clone() + Val::from(1));
                l_paper_x2 = ctx.call(Function::Rand, vec![Val::from(96), Val::from(110)])?;
                l_paper_y2 = ctx.call(Function::Rand, vec![Val::from(87), Val::from(100)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x2.clone(),
                        l_paper_y2.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_2_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_2_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_2_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x2 = Val::from(0);
    let mut l_paper_y2 = Val::from(0);
    l_paper_x2 = ctx.call(Function::Rand, vec![Val::from(96), Val::from(110)])?;
    l_paper_y2 = ctx.call(Function::Rand, vec![Val::from(87), Val::from(100)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x2.clone(),
            l_paper_y2.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_2_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_2_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_3_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_3_a(ctx: &Ctx) -> Script {
    paper_sp_3_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_3_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while3 = Val::from(0);
    let mut l_paper_x3 = Val::from(0);
    let mut l_paper_y3 = Val::from(0);
    l_paper_while3 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while3.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while3 = (l_paper_while3.clone() + Val::from(1));
                l_paper_x3 = ctx.call(Function::Rand, vec![Val::from(111), Val::from(124)])?;
                l_paper_y3 = ctx.call(Function::Rand, vec![Val::from(87), Val::from(100)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x3.clone(),
                        l_paper_y3.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_3_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_3_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_3_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x3 = Val::from(0);
    let mut l_paper_y3 = Val::from(0);
    l_paper_x3 = ctx.call(Function::Rand, vec![Val::from(111), Val::from(124)])?;
    l_paper_y3 = ctx.call(Function::Rand, vec![Val::from(87), Val::from(100)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x3.clone(),
            l_paper_y3.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_3_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_3_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_4_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_4_a(ctx: &Ctx) -> Script {
    paper_sp_4_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_4_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while4 = Val::from(0);
    let mut l_paper_x4 = Val::from(0);
    let mut l_paper_y4 = Val::from(0);
    l_paper_while4 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while4.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while4 = (l_paper_while4.clone() + Val::from(1));
                l_paper_x4 = ctx.call(Function::Rand, vec![Val::from(81), Val::from(95)])?;
                l_paper_y4 = ctx.call(Function::Rand, vec![Val::from(73), Val::from(86)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x4.clone(),
                        l_paper_y4.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_4_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_4_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_4_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x4 = Val::from(0);
    let mut l_paper_y4 = Val::from(0);
    l_paper_x4 = ctx.call(Function::Rand, vec![Val::from(81), Val::from(95)])?;
    l_paper_y4 = ctx.call(Function::Rand, vec![Val::from(73), Val::from(86)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x4.clone(),
            l_paper_y4.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_4_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_4_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_5_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_5_a(ctx: &Ctx) -> Script {
    paper_sp_5_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_5_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while5 = Val::from(0);
    let mut l_paper_x5 = Val::from(0);
    let mut l_paper_y5 = Val::from(0);
    l_paper_while5 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while5.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while5 = (l_paper_while5.clone() + Val::from(1));
                l_paper_x5 = ctx.call(Function::Rand, vec![Val::from(96), Val::from(110)])?;
                l_paper_y5 = ctx.call(Function::Rand, vec![Val::from(73), Val::from(86)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x5.clone(),
                        l_paper_y5.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_5_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_5_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_5_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x5 = Val::from(0);
    let mut l_paper_y5 = Val::from(0);
    l_paper_x5 = ctx.call(Function::Rand, vec![Val::from(96), Val::from(110)])?;
    l_paper_y5 = ctx.call(Function::Rand, vec![Val::from(73), Val::from(86)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x5.clone(),
            l_paper_y5.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_5_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_5_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_6_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_6_a(ctx: &Ctx) -> Script {
    paper_sp_6_a_body(ctx, Vec::new()).map(|_| ())
}
