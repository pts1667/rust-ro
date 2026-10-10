use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn s_102fshadowdust1_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_mob_dead_num = Val::from(0);
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("5@tower")])?;
    l_mob_dead_num = ctx.call(
        Function::MobCount,
        vec![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#102FShadowDust1")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    if l_mob_dead_num.clone().number()? < 1 {
        ctx.call(
            Function::MapAnnounce,
            vec![
                l_map_s.clone(),
                Val::from("Mysterious Voice: Who are you to dare intrude upon my sanctuary?!"),
                ctx.constant("BC_MAP")?,
                Val::from("0xffff00"),
            ],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#102FShadowDust")])? + Val::from("::OnDisable"))],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("Lucid Crystal#102")])? + Val::from("::OnEnable"))],
        )?;
    }
    return Err(Stop::End);
}

pub fn s_102fshadowdust1_onmymobdead(ctx: &Ctx) -> Script {
    s_102fshadowdust1_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn s_102fshadowdust_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn s_102fshadowdust(ctx: &Ctx) -> Script {
    s_102fshadowdust_body(ctx, Vec::new()).map(|_| ())
}

fn s_102fshadowdust_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DisableNpc,
        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("#102FShadowDust")])?],
    )?;
    return Err(Stop::End);
}

pub fn s_102fshadowdust_oninstanceinit(ctx: &Ctx) -> Script {
    s_102fshadowdust_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn s_102fshadowdust_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![ctx.call(Function::InstanceMapName, vec![Val::from("5@tower")])?, Val::from("All")],
    )?;
    return Err(Stop::End);
}

pub fn s_102fshadowdust_ondisable(ctx: &Ctx) -> Script {
    s_102fshadowdust_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn s_102fshadowdust_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("5@tower")])?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(72),
            Val::from(93),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(70),
            Val::from(87),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(68),
            Val::from(83),
            Val::from("Thorn of Pureness"),
            Val::from(1961),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(70),
            Val::from(80),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(74),
            Val::from(81),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(78),
            Val::from(72),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(81),
            Val::from(70),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(78),
            Val::from(84),
            Val::from("Thorn of Recovery"),
            Val::from(1959),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(85),
            Val::from(72),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(91),
            Val::from(74),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(79),
            Val::from(77),
            Val::from("Thorn of Pureness"),
            Val::from(1961),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(82),
            Val::from(80),
            Val::from("Thorn of Recovery"),
            Val::from(1959),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(87),
            Val::from(83),
            Val::from("Thorn of Recovery"),
            Val::from(1959),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(80),
            Val::from(92),
            Val::from("Thorn of Pureness"),
            Val::from(1961),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(81),
            Val::from(89),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(85),
            Val::from(93),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(86),
            Val::from(90),
            Val::from("Thorn of Recovery"),
            Val::from(1959),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(88),
            Val::from(88),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(91),
            Val::from(87),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(95),
            Val::from(94),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(91),
            Val::from(96),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(90),
            Val::from(82),
            Val::from("Thorn of Pureness"),
            Val::from(1961),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(96),
            Val::from(98),
            Val::from("Thorn of Magic"),
            Val::from(1960),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(83),
            Val::from(76),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(74),
            Val::from(85),
            Val::from("Thorny Skeleton"),
            Val::from(1958),
            Val::from(1),
        ],
    )?;
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
    if subject1 == 1 {
        ctx.call(
            Function::Monster,
            vec![
                l_map_s.clone(),
                Val::from(82),
                Val::from(85),
                Val::from("Thorny Skeleton"),
                Val::from(1958),
                Val::from(1),
            ],
        )?;
    } else if subject1 == 2 {
        ctx.call(
            Function::Monster,
            vec![
                l_map_s.clone(),
                Val::from(82),
                Val::from(85),
                Val::from("Thorn of Magic"),
                Val::from(1960),
                Val::from(1),
            ],
        )?;
    } else if subject1 == 3 {
        ctx.call(
            Function::Monster,
            vec![
                l_map_s.clone(),
                Val::from(82),
                Val::from(85),
                Val::from("Thorn of Pureness"),
                Val::from(1961),
                Val::from(1),
            ],
        )?;
    }
    let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
    if subject2 == 1 {
        ctx.call(
            Function::Monster,
            vec![
                l_map_s.clone(),
                Val::from(84),
                Val::from(85),
                Val::from("Thorny Skeleton"),
                Val::from(1958),
                Val::from(1),
            ],
        )?;
    } else if subject2 == 2 {
        ctx.call(
            Function::Monster,
            vec![
                l_map_s.clone(),
                Val::from(84),
                Val::from(85),
                Val::from("Thorn of Magic"),
                Val::from(1960),
                Val::from(1),
            ],
        )?;
    } else if subject2 == 3 {
        ctx.call(
            Function::Monster,
            vec![
                l_map_s.clone(),
                Val::from(84),
                Val::from(85),
                Val::from("Thorn of Pureness"),
                Val::from(1961),
                Val::from(1),
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn s_102fshadowdust_onenable(ctx: &Ctx) -> Script {
    s_102fshadowdust_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn manager_mode5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.mes("This NPC manages the crystal on the 100th Level. Please enter the password.")?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from("dmc2008"), Val::from(1)])?;
    ctx.next()?;
    if l_i.clone() == 1 {
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("Lucid Crystal#102")])? + Val::from("::OnEnable"))],
        )?;
        ctx.mes("The 100th Level's crystal has been activated.")?;
    } else {
        ctx.mes("Please enter the correct password.")?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn manager_mode5(ctx: &Ctx) -> Script {
    manager_mode5_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("^0066ffYou took a sip of the spring's clear water, and you feel invigorated.^000000")?;
    ctx.call(
        Function::Heal,
        vec![
            (ctx.var("MaxHp").get()?.try_sub(ctx.var("Hp").get()?)?),
            (ctx.var("MaxSp").get()?.try_sub(ctx.var("Sp").get()?)?),
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn life_spring_1(ctx: &Ctx) -> Script {
    life_spring_1_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_1_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("Life Spring#2")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn life_spring_1_oninstanceinit(ctx: &Ctx) -> Script {
    life_spring_1_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn life_spring_2(ctx: &Ctx) -> Script {
    life_spring_2_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn life_spring_2_onenable(ctx: &Ctx) -> Script {
    life_spring_2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_2_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("Life Spring#3")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn life_spring_2_ontimer2000(ctx: &Ctx) -> Script {
    life_spring_2_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn life_spring_3(ctx: &Ctx) -> Script {
    life_spring_3_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_3_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn life_spring_3_onenable(ctx: &Ctx) -> Script {
    life_spring_3_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn life_spring_3_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("Life Spring#2")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn life_spring_3_ontimer2000(ctx: &Ctx) -> Script {
    life_spring_3_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn tyrant_s_throne_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn tyrant_s_throne(ctx: &Ctx) -> Script {
    tyrant_s_throne_body(ctx, Vec::new()).map(|_| ())
}

fn tyrant_s_throne_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::AreaMonster,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from(154),
            Val::from(73),
            Val::from(156),
            Val::from(75),
            Val::from("Watcher's Son"),
            Val::from(1627),
            Val::from(10),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("Tyrant's Throne#")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn tyrant_s_throne_oninstanceinit(ctx: &Ctx) -> Script {
    tyrant_s_throne_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn tyrant_s_throne_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_mob_dead_num = Val::from(0);
    l_mob_dead_num = ctx.call(
        Function::MobCount,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            (ctx.call(Function::InstanceNpcName, vec![Val::from("Tyrant's Throne#")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    if l_mob_dead_num.clone().number()? < 1 {
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#1st Beeper")])? + Val::from("::OnEnable"))],
        )?;
        ctx.call(
            Function::DisableNpc,
            vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Tyrant's Throne#")])?],
        )?;
    }
    return Err(Stop::End);
}

pub fn tyrant_s_throne_onmymobdead(ctx: &Ctx) -> Script {
    tyrant_s_throne_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn s_1st_beeper(ctx: &Ctx) -> Script {
    s_1st_beeper_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn s_1st_beeper_onenable(ctx: &Ctx) -> Script {
    s_1st_beeper_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_ontimer500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?, Val::from("Guests, huh? I hope you've come here knowing that you'll be buried in this place. If you didn't know, well... it's too late!"), ctx.constant("BC_MAP")?, Val::from("0x00ffcc")])?;
    return Err(Stop::End);
}

pub fn s_1st_beeper_ontimer500(ctx: &Ctx) -> Script {
    s_1st_beeper_ontimer500_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_ontimer5500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("This is why you adventurers always end up dead."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_1st_beeper_ontimer5500(ctx: &Ctx) -> Script {
    s_1st_beeper_ontimer5500_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_ontimer10500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("I may applaud you for your courage... Of course, I intend to play with you a little bit first."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_1st_beeper_ontimer10500(ctx: &Ctx) -> Script {
    s_1st_beeper_ontimer10500_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_ontimer15500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("You know, I like watching humans running around in fear."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_1st_beeper_ontimer15500(ctx: &Ctx) -> Script {
    s_1st_beeper_ontimer15500_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_ontimer20500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            l_map_s.clone(),
            Val::from("Let's see who runs fastest. Are you ready?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::AreaMonster,
        vec![
            l_map_s.clone(),
            Val::from(151),
            Val::from(66),
            Val::from(153),
            Val::from(106),
            Val::from("Bone Guardian"),
            Val::from(1152),
            Val::from(50),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#1st Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::AreaMonster,
        vec![
            l_map_s.clone(),
            Val::from(158),
            Val::from(66),
            Val::from(160),
            Val::from(106),
            Val::from("Bone Guardian"),
            Val::from(1152),
            Val::from(50),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#1st Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_1st_beeper_ontimer20500(ctx: &Ctx) -> Script {
    s_1st_beeper_ontimer20500_body(ctx, Vec::new()).map(|_| ())
}

fn s_1st_beeper_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_mob_dead_num = Val::from(0);
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    l_mob_dead_num = ctx.call(
        Function::MobCount,
        vec![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#1st Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    if l_mob_dead_num.clone().number()? < 1 {
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#2nd Beeper")])? + Val::from("::OnEnable"))],
        )?;
    } else {
        ctx.call(
            Function::MapAnnounce,
            vec![
                l_map_s.clone(),
                ((Val::from("Remaining Targets ") + l_mob_dead_num.clone()) + Val::from("ea")),
                ctx.constant("BC_MAP")?,
                Val::from("0x00ff99"),
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn s_1st_beeper_onmymobdead(ctx: &Ctx) -> Script {
    s_1st_beeper_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_beeper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn s_2nd_beeper(ctx: &Ctx) -> Script {
    s_2nd_beeper_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_beeper_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn s_2nd_beeper_onenable(ctx: &Ctx) -> Script {
    s_2nd_beeper_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_beeper_ontimer500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Well, I guess they aren't too challenging for you."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_2nd_beeper_ontimer500(ctx: &Ctx) -> Script {
    s_2nd_beeper_ontimer500_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_beeper_ontimer5500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Let's speed up a little bit, shall we?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_2nd_beeper_ontimer5500(ctx: &Ctx) -> Script {
    s_2nd_beeper_ontimer5500_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_beeper_ontimer10500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            l_map_s.clone(),
            Val::from("I demand an encore!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::AreaMonster,
        vec![
            l_map_s.clone(),
            Val::from(151),
            Val::from(66),
            Val::from(153),
            Val::from(106),
            Val::from("Wind Guardian"),
            Val::from(1263),
            Val::from(30),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#2nd Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::AreaMonster,
        vec![
            l_map_s.clone(),
            Val::from(158),
            Val::from(66),
            Val::from(160),
            Val::from(106),
            Val::from("Wind Guardian"),
            Val::from(1263),
            Val::from(30),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#2nd Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_2nd_beeper_ontimer10500(ctx: &Ctx) -> Script {
    s_2nd_beeper_ontimer10500_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_beeper_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_mob_dead_num = Val::from(0);
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    l_mob_dead_num = ctx.call(
        Function::MobCount,
        vec![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#2nd Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    if l_mob_dead_num.clone().number()? < 1 {
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#3rd Beeper")])? + Val::from("::OnEnable"))],
        )?;
    } else {
        ctx.call(
            Function::MapAnnounce,
            vec![
                l_map_s.clone(),
                ((Val::from("Remaining Targets ") + l_mob_dead_num.clone()) + Val::from("ea")),
                ctx.constant("BC_MAP")?,
                Val::from("0x00ff99"),
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn s_2nd_beeper_onmymobdead(ctx: &Ctx) -> Script {
    s_2nd_beeper_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_beeper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn s_3rd_beeper(ctx: &Ctx) -> Script {
    s_3rd_beeper_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_beeper_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn s_3rd_beeper_onenable(ctx: &Ctx) -> Script {
    s_3rd_beeper_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_beeper_ontimer500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Yes, this is getting exciting!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_3rd_beeper_ontimer500(ctx: &Ctx) -> Script {
    s_3rd_beeper_ontimer500_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_beeper_ontimer5500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("I'll remember you as one of a few that have managed to entertain me."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_3rd_beeper_ontimer5500(ctx: &Ctx) -> Script {
    s_3rd_beeper_ontimer5500_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_beeper_ontimer10500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            l_map_s.clone(),
            Val::from("How would you like to play one more round?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::AreaMonster,
        vec![
            l_map_s.clone(),
            Val::from(151),
            Val::from(66),
            Val::from(153),
            Val::from(106),
            Val::from("Sword Edge Guardian"),
            Val::from(1132),
            Val::from(20),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#3rd Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::AreaMonster,
        vec![
            l_map_s.clone(),
            Val::from(158),
            Val::from(66),
            Val::from(160),
            Val::from(106),
            Val::from("Sword Edge Guardian"),
            Val::from(1132),
            Val::from(20),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#3rd Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_3rd_beeper_ontimer10500(ctx: &Ctx) -> Script {
    s_3rd_beeper_ontimer10500_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_beeper_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_mob_dead_num = Val::from(0);
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    l_mob_dead_num = ctx.call(
        Function::MobCount,
        vec![
            l_map_s.clone(),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#3rd Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    if l_mob_dead_num.clone().number()? < 1 {
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#4th Beeper")])? + Val::from("::OnEnable"))],
        )?;
    } else {
        ctx.call(
            Function::MapAnnounce,
            vec![
                l_map_s.clone(),
                ((Val::from("Remaining Targets ") + l_mob_dead_num.clone()) + Val::from("ea")),
                ctx.constant("BC_MAP")?,
                Val::from("0x00ff99"),
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn s_3rd_beeper_onmymobdead(ctx: &Ctx) -> Script {
    s_3rd_beeper_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn s_4th_beeper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn s_4th_beeper(ctx: &Ctx) -> Script {
    s_4th_beeper_body(ctx, Vec::new()).map(|_| ())
}

fn s_4th_beeper_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn s_4th_beeper_onenable(ctx: &Ctx) -> Script {
    s_4th_beeper_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn s_4th_beeper_ontimer500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Okay, the time has come to make my appearance!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_4th_beeper_ontimer500(ctx: &Ctx) -> Script {
    s_4th_beeper_ontimer500_body(ctx, Vec::new()).map(|_| ())
}

fn s_4th_beeper_ontimer5500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Do you want to know who I am?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_4th_beeper_ontimer5500(ctx: &Ctx) -> Script {
    s_4th_beeper_ontimer5500_body(ctx, Vec::new()).map(|_| ())
}

fn s_4th_beeper_ontimer10500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            l_map_s.clone(),
            Val::from("You'll soon know. Mine is the face of death!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::Monster,
        vec![
            l_map_s.clone(),
            Val::from(156),
            Val::from(147),
            Val::from("Naght Sieger"),
            Val::from(1956),
            Val::from(1),
            (ctx.call(Function::InstanceNpcName, vec![Val::from("#4th Beeper")])? + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn s_4th_beeper_ontimer10500(ctx: &Ctx) -> Script {
    s_4th_beeper_ontimer10500_body(ctx, Vec::new()).map(|_| ())
}

fn s_4th_beeper_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("Lost Soul#102")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn s_4th_beeper_onmymobdead(ctx: &Ctx) -> Script {
    s_4th_beeper_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn lost_soul_102_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.mes("You're carrying too much stuff. Why don't you put some of it away, and then come back?")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Lost Souls",
        args!["It's you that have liberated us from the evil Naght Sieger."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lost Souls",
        args!["Thank you so much. Now we can escape from this cold and dark place... to heaven."],
    )?;
    ctx.next()?;
    if (ctx.call(Function::CountItem, vec![Val::from(13412)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(13413)])?.number()? > 0)
    {
        ctx.lines_as("Lost Souls", args!["Hey, you have the remnants of Naght Sieger with you."])?;
        ctx.next()?;
        ctx.lines_as("Lost Souls", args!["They may appear to be one-handed swords, but I can put them together to make a two-handed one if you want. That's the only way I can repay you for freeing me."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Make a Two-Handed Sword.:No, thanks.")])? {
            1 => {
                ctx.lines_as(
                    "Lost Souls",
                    args!["If it is already upgraded or has a card inside, those effects will be disappear. Is this ok with you?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("It's fine with me. Please make one.:No way!")])? {
                    1 => {
                        ctx.lines_as(
                            "Lost Souls",
                            args!["Good, then I'll combine these to create a two-handed sword."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(13412), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(13413), Val::from(1)])?;
                        ctx.call(Function::GetItem, vec![Val::from(1185), Val::from(1)])?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Lost Souls",
                            args!["I see. I guess you aren't as greedy or ambitious as those other adventurers."],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as(
                    "Lost Souls",
                    args!["I see. I guess you aren't as greedy or ambitious as those other adventurers."],
                )?;
                ctx.next()?;
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Lost Souls",
        args!["I'd like to talk to you more, but I've... I've got to go now."],
    )?;
    ctx.next()?;
    ctx.lines_as("Lost Souls", args!["Farewell, young adventurer. I wish you good luck."])?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(223), Val::from(36)])?;
    return Err(Stop::End);
}

pub fn lost_soul_102(ctx: &Ctx) -> Script {
    lost_soul_102_body(ctx, Vec::new()).map(|_| ())
}

fn lost_soul_102_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DisableNpc,
        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Lost Soul#102")])?],
    )?;
    return Err(Stop::End);
}

pub fn lost_soul_102_oninstanceinit(ctx: &Ctx) -> Script {
    lost_soul_102_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn lost_soul_102_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(
        Function::EnableNpc,
        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Lost Soul#102")])?],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#Effect30")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn lost_soul_102_onenable(ctx: &Ctx) -> Script {
    lost_soul_102_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn lost_soul_102_ontimer500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("This... This can't be happening! I can't be defeated!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xffff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn lost_soul_102_ontimer500(ctx: &Ctx) -> Script {
    lost_soul_102_ontimer500_body(ctx, Vec::new()).map(|_| ())
}

fn lost_soul_102_ontimer5500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Nooo! My soul... My shell...! Nooo~!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xffff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn lost_soul_102_ontimer5500(ctx: &Ctx) -> Script {
    lost_soul_102_ontimer5500_body(ctx, Vec::new()).map(|_| ())
}

fn lost_soul_102_ontimer10500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
            Val::from("Naght Sieger's body has turned into dark ashes that scattered in the wind."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ffcc"),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn lost_soul_102_ontimer10500(ctx: &Ctx) -> Script {
    lost_soul_102_ontimer10500_body(ctx, Vec::new()).map(|_| ())
}

fn effect30_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn effect30(ctx: &Ctx) -> Script {
    effect30_body(ctx, Vec::new()).map(|_| ())
}

fn effect30_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DisableNpc,
        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("#Effect30")])?],
    )?;
    return Err(Stop::End);
}

pub fn effect30_oninstanceinit(ctx: &Ctx) -> Script {
    effect30_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn effect30_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CONE")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn effect30_onenable(ctx: &Ctx) -> Script {
    effect30_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn effect30_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#Effect31")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn effect30_ontimer1000(ctx: &Ctx) -> Script {
    effect30_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn effect31_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn effect31(ctx: &Ctx) -> Script {
    effect31_body(ctx, Vec::new()).map(|_| ())
}

fn effect31_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::DisableNpc,
        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("#Effect31")])?],
    )?;
    return Err(Stop::End);
}

pub fn effect31_oninstanceinit(ctx: &Ctx) -> Script {
    effect31_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn effect31_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CONE")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn effect31_onenable(ctx: &Ctx) -> Script {
    effect31_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn effect31_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#Effect30")])? + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn effect31_ontimer1000(ctx: &Ctx) -> Script {
    effect31_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}
