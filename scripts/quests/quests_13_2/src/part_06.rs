use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn middle_ranked_laphine_la_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
            ctx.lines_as(
                "Flowery",
                args![
                    "It looks like you're carrying too many things.",
                    "Why not put some of your items in storage and come back?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("ep13_2_tre1").get()? == 1 {
            ctx.lines_as(
                "Flowery",
                args!["Huh? Seed of the flower of Alfheim?", "Do I have that one? Hmm~"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Flowery",
                args![
                    "As you see, there're so many stuffs",
                    "in my room, so I don't know",
                    "exactly what I have",
                    "or not in my room."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Flowery",
                args![
                    "But I'll try to find cuz you came",
                    "here to help Grenouille",
                    "He's always pretend to tenacious,",
                    "but sometimes he help others well! Huhu!"
                ],
            )?;
            ctx.next()?;
            ctx.mes("- rummaging -")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.next()?;
            ctx.lines(args!["- rummaging -", "- rummaging -"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.next()?;
            ctx.lines(args!["- rummaging -", "- rummaging -", "- rummaging -"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
            ctx.next()?;
            ctx.lines_as("Flowery", args!["Wow! Come here!", "I found a seed!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Flowery",
                args![
                    "Hmm... But this one is",
                    "one of the most common and trifle",
                    "kind in Alfheim, so you can find",
                    "this wild flower at anywhere...",
                    "This one would be ok?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Flowery",
                args![
                    "Umm.. But I don't have",
                    "any other seed of flower..",
                    "So, just bring this, here you are~"
                ],
            )?;
            ctx.var("ep13_2_tre1").set(Val::from(2))?;
            ctx.call(Function::GetItem, vec![Val::from(7193), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_2_tre1").get()? == 2 {
            ctx.lines_as(
                "Flowery",
                args![
                    "I gave you the seed, so it's up to you that what would you do with the seed.",
                    "I feel dizzy when I look at my room."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Flowery", args!["Yuuuum~", "Can you believe that right in front of us is a battle field? Though it's very silent and peaceful at the moment. So do I~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Middle-Ranked Laphine",
            args![
                "NeiKoFulo An DielOdesMush Or KoDanaOsa Ir FarBurnes Ir ",
                "NeOsaVa Mu BurVeTi Ra OsaDeh"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn middle_ranked_laphine_la(ctx: &Ctx) -> Script {
    middle_ranked_laphine_la_body(ctx, Vec::new()).map(|_| ())
}

fn purifier_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_2_tre1").get()? == 2 {
        if ctx.call(Function::CountItem, vec![Val::from(7193)])?.number()? > 0 {
            ctx.lines(args![
                "- You put the seed in the",
                "device and watched it",
                "as Grenouille told you. -"
            ])?;
            ctx.next()?;
            ctx.mes("- Booowoong -")?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEALSP")?])?;
            ctx.next()?;
            ctx.lines(args![
                "- When the lights gone,",
                "a big flower of Alfheim was lied",
                "in your hand, instead of the seed. -"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7193), Val::from(1)])?;
            ctx.var("ep13_2_tre1").set(Val::from(3))?;
            ctx.call(Function::GetItem, vec![Val::from(6079), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Where are the seeds?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn purifier(ctx: &Ctx) -> Script {
    purifier_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["Oh my hometown..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["Umm... This is...", "I can feel the scent of Alfheim.", "My hometown Alfheim!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Exhausted Soldier", args!["Ahh! Feeling much better!"])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_1::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#1")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["Oh my hometown..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_1(ctx: &Ctx) -> Script {
    exhausted_soldier_1_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#1")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_1_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_1_run(ctx: &Ctx, mut step: Tukare1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare1Step::Start => {
                step = Tukare1Step::OnEnable;
                continue 'machine;
            }
            Tukare1Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare1Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_1(ctx: &Ctx) -> Script {
    tukare_1_run(ctx, Tukare1Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_1_onenable(ctx: &Ctx) -> Script {
    tukare_1_run(ctx, Tukare1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_1_ontimer60000(ctx: &Ctx) -> Script {
    tukare_1_run(ctx, Tukare1Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn exhausted_soldier_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["Want to take a rest..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["What happen?", "Feel better suddenly! I'm so happy now!", "Let's go to work!"],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_2::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#2")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["Want to take a rest..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_2(ctx: &Ctx) -> Script {
    exhausted_soldier_2_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#2")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_2_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_2_run(ctx: &Ctx, mut step: Tukare2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare2Step::Start => {
                step = Tukare2Step::OnEnable;
                continue 'machine;
            }
            Tukare2Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare2Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_2(ctx: &Ctx) -> Script {
    tukare_2_run(ctx, Tukare2Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_2_onenable(ctx: &Ctx) -> Script {
    tukare_2_run(ctx, Tukare2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_2_ontimer60000(ctx: &Ctx) -> Script {
    tukare_2_run(ctx, Tukare2Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn exhausted_soldier_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["My hometown Alfheim..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["Umm... This is...", "I can feel the scent of Alfheim.", "My hometown Alfheim!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Exhausted Soldier", args!["Let's beat Sapha and go back home!"])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_3::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#3")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["My hometown Alfheim..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_3(ctx: &Ctx) -> Script {
    exhausted_soldier_3_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#3")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_3_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_3_run(ctx: &Ctx, mut step: Tukare3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare3Step::Start => {
                step = Tukare3Step::OnEnable;
                continue 'machine;
            }
            Tukare3Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare3Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_3(ctx: &Ctx) -> Script {
    tukare_3_run(ctx, Tukare3Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_3_onenable(ctx: &Ctx) -> Script {
    tukare_3_run(ctx, Tukare3Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_3_ontimer60000(ctx: &Ctx) -> Script {
    tukare_3_run(ctx, Tukare3Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn exhausted_soldier_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["Huu..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["Umm... This is...", "I can feel the scent of Alfheim.", "My hometown Alfheim!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Exhausted Soldier", args!["Ahh! Feels like at home!!"])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_4::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#4")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["Huu..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_4(ctx: &Ctx) -> Script {
    exhausted_soldier_4_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_4_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#4")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_4_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_4_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_4_run(ctx: &Ctx, mut step: Tukare4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare4Step::Start => {
                step = Tukare4Step::OnEnable;
                continue 'machine;
            }
            Tukare4Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare4Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#4")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_4(ctx: &Ctx) -> Script {
    tukare_4_run(ctx, Tukare4Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_4_onenable(ctx: &Ctx) -> Script {
    tukare_4_run(ctx, Tukare4Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_4_ontimer60000(ctx: &Ctx) -> Script {
    tukare_4_run(ctx, Tukare4Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn exhausted_soldier_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["Oh my hometown..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["Umm... This is...", "I can feel the scent of Alfheim.", "My hometown Alfheim!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Exhausted Soldier", args!["Ahh! Feeling much better!"])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_5::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#5")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["Oh my hometown..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_5(ctx: &Ctx) -> Script {
    exhausted_soldier_5_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_5_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#5")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_5_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_5_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_5_run(ctx: &Ctx, mut step: Tukare5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare5Step::Start => {
                step = Tukare5Step::OnEnable;
                continue 'machine;
            }
            Tukare5Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare5Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#5")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_5(ctx: &Ctx) -> Script {
    tukare_5_run(ctx, Tukare5Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_5_onenable(ctx: &Ctx) -> Script {
    tukare_5_run(ctx, Tukare5Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_5_ontimer60000(ctx: &Ctx) -> Script {
    tukare_5_run(ctx, Tukare5Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn exhausted_soldier_6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["Huu..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["I can feel the scent of Alfheim.", "My hometown Alfheim!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["Yes, Splendide also a part of Alfheim!", "Go Laphines!"],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_6::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#6")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["Huu..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_6(ctx: &Ctx) -> Script {
    exhausted_soldier_6_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_6_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#6")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_6_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_6_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_6_run(ctx: &Ctx, mut step: Tukare6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare6Step::Start => {
                step = Tukare6Step::OnEnable;
                continue 'machine;
            }
            Tukare6Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare6Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#6")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_6(ctx: &Ctx) -> Script {
    tukare_6_run(ctx, Tukare6Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_6_onenable(ctx: &Ctx) -> Script {
    tukare_6_run(ctx, Tukare6Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_6_ontimer60000(ctx: &Ctx) -> Script {
    tukare_6_run(ctx, Tukare6Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn exhausted_soldier_7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.var("ep13_2_tre1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? > 0 {
                ctx.lines_as("Exhausted Soldier", args!["Oh my hometown..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Use perfume.:Don't use perfume.")])? {
                    1 => {
                        ctx.mes("(Spray-)")?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Exhausted Soldier",
                            args!["Umm... This is...", "I can feel the scent of Alfheim.", "My hometown Alfheim!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Exhausted Soldier", args!["Ahh! Feeling much better!"])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.call(Function::DelItem, vec![Val::from(6082), Val::from(1)])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#tukare_7::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("Exhausted Soldier#7")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("- You decide to do nothing. -")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            return Err(Stop::End);
        } else {
            ctx.lines_as("Exhausted Soldier", args!["Oh my hometown..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Exhausted Soldier", args!["CyaResehr"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn exhausted_soldier_7(ctx: &Ctx) -> Script {
    exhausted_soldier_7_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_soldier_7_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#7")])?;
    return Err(Stop::End);
}

pub fn exhausted_soldier_7_oninit(ctx: &Ctx) -> Script {
    exhausted_soldier_7_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn tukare_7_run(ctx: &Ctx, mut step: Tukare7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tukare7Step::Start => {
                step = Tukare7Step::OnEnable;
                continue 'machine;
            }
            Tukare7Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tukare7Step::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Exhausted Soldier#7")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tukare_7(ctx: &Ctx) -> Script {
    tukare_7_run(ctx, Tukare7Step::Start, Vec::new()).map(|_| ())
}

pub fn tukare_7_onenable(ctx: &Ctx) -> Script {
    tukare_7_run(ctx, Tukare7Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn tukare_7_ontimer60000(ctx: &Ctx) -> Script {
    tukare_7_run(ctx, Tukare7Step::OnTimer60000, Vec::new()).map(|_| ())
}

fn bazett_teablack_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            " - Hold on!! -",
            " - You are carrying too many different items - ",
            " - You cannot receive the reward - ",
            " - Please use the Kafra service - ",
            " - And try again. - "
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_rhea").get()?.number()? < 100 {
        ctx.lines_as(
            "Industrious Man",
            args![
                "Hu~~",
                "That's pretty interesting...",
                "I will take note of it in my research papers."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
        ctx.next()?;
        ctx.lines_as("Industrious Man", args!["write...write...", "crunch...crunch..."])?;
        ctx.next()?;
        ctx.mes(" - He doesn't seem to recognize that I am standing next to him as he continues writing something. -")?;
        ctx.next()?;
        ctx.lines_as("Industrious Man", args!["write...write...", "crunch...crunch...", "...."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_busut").get()?.number()? < 1 {
        if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
            ctx.lines_as(
                "Industrious Man",
                args![
                    "Hu~~",
                    "That's pretty interesting...",
                    "I will take note of it in my research papers."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
            ctx.next()?;
            ctx.lines_as("Industrious Man", args!["write...write...", "crunch...crunch..."])?;
            ctx.next()?;
            ctx.mes(" - He doesn't seem to recognize that I am standing next to him as he continues writing something. -")?;
            ctx.next()?;
            ctx.lines_as("Industrious Man", args!["write..write...", "crunch..crunch...", "...."])?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["Auch!!!", "Who...who are you?!", "How long have you been standing there?"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Uh, I'm just passing by.", "You look like you're busy with something."],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["See you~!!"])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_PROFUSELY_SWEAT")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Industrious Man", args!["Uh!! Wait!!", "Your finger... Is that..?!"])?;
            ctx.next()?;
            ctx.mes(" - He grabs your hand with a surprised look -")?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args![
                    "This is the ^0000FFRing of the Ancient Wise King^000000!!!!!",
                    "This has to be fate that I've met you!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["My research is not going so well. The god of fate must have sent you to me. I'm sure of it!!"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["......", "......What?!"],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_HUK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Industrious Man", args!["Would you like to help me with my search?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("What kind of search?:Sure.:Borrrring.")])? {
                1 => {
                    ctx.lines_as(
                        "Industrious Man",
                        args!["To put it simply, I'm searching for fairies and giants that live around here."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Industrious Man",
                        args![
                            "Though I believe they all live in the same place, they are sure to have different cultures.",
                            "...first and foremost is proof of their existence."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Industrious Man", args!["What about it?", "Are you interested?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Sure.:Not really.")])? {
                        1 => {
                            ctx.lines_as("Industrious Man", args!["Haha... I just know that I'm right!", "Hahahaha!!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Industrious Man",
                                args![
                                    "So let's work together from this point on. Let me introduce myself.",
                                    "As you can see from my name tag, my name is ^0000FFBazett Teablack^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![((Val::from("I am ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("OK, so what can I do for you?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Bazett",
                                args![
                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(" let's see..!!")),
                                    "Search Manuk and Splendide fields everyday for signs of the fairies or giants."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bazett",
                                args![
                                    "It's not difficult.",
                                    "So please share any information that you can gather as you travel through those areas."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bazett",
                                args!["Hopefully I can find what I am looking for with the information you can find."],
                            )?;
                            ctx.var("ep13_2_busut").set(Val::from(2))?;
                            ctx.call(Function::SetQuest, vec![Val::from(11101)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Industrious Man",
                                args![
                                    ".............",
                                    ".............",
                                    ".............",
                                    ".............",
                                    "...........Why!!!!!"
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Industrious Man",
                                args!["Well if you decide to change your mind, come back to me."],
                            )?;
                            ctx.var("ep13_2_busut").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as("Industrious Man", args!["Haha... I just know that I'm right!", "Hahahaha!!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Industrious Man",
                        args![
                            "So let's work together from this point on. Let me introduce myself.",
                            "As you can see from my name tag, my name is ^0000FFBazett Teablack^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![((Val::from("I am ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("OK, so what can I do for you?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Bazett",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(" let's see..!!")),
                            "Search Manuk and Splendide fields everyday for signs of the fairies or giants."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazett",
                        args![
                            "It's not difficult.",
                            "So please share any information that you can gather as you travel through those areas."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazett",
                        args!["Hopefully I can find what I am looking for with the information you can find."],
                    )?;
                    ctx.var("ep13_2_busut").set(Val::from(2))?;
                    ctx.call(Function::SetQuest, vec![Val::from(11101)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Industrious Man", args!["............."])?;
                    ctx.next()?;
                    ctx.lines_as("Industrious Man", args![".............", "............."])?;
                    ctx.next()?;
                    ctx.lines_as("Industrious Man", args![".............", ".............", "............."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Industrious Man",
                        args![".............", ".............", ".............", "............."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Industrious Man",
                        args![
                            ".............",
                            ".............",
                            ".............",
                            ".............",
                            "...........Why!!!!!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Industrious Man",
                        args!["Well if you decide to change your mind, come back to me."],
                    )?;
                    ctx.var("ep13_2_busut").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as("Industrious Man", args!["Gthgh sdsWryi", "Apeu hjsu opuer "])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["?????", "What'd you say?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["Oh! Sorry, I think I said it wrong...", "I was just infatuated with my research..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["You should find a way to understand this strange language that I've discovered here in the Ash Vacuum."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["Without the ability to communicate it would be really difficult to get around here, don't you think?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Industrious Man", args!["TalDathMush Di nahDeh", "ReAnduDu So sehr"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["?????", "Huh?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["Oh! I must've said it wrong...", "My research hasn't been going too well lately."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["There should be a way for you to understand this strange language here."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Industrious Man",
                args!["Without the ability to communicate it would be really difficult to get around here, don't you think?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_2_busut").get()? == 1 {
        ctx.lines_as(
            "Industrious Man",
            args!["Ah! You've returned!", "Now are you interested in my research?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
            1 => {
                ctx.lines_as("Industrious Man", args!["Haha... I just know that I'm right!", "Hahahaha!!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Industrious Man",
                    args![
                        "So let's work together from this point on. Let me introduce myself.",
                        "As you can see from my name tag, my name is ^0000FFBazett Teablack^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![((Val::from("I am ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("OK, so what can I do for you?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Bazett",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(" let's see..!!")),
                        "Search Manuk and Splendide fields everyday for signs of the fairies or giants."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bazett",
                    args![
                        "It's not difficult.",
                        "So please share any information that you can gather as you travel through those areas."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bazett",
                    args!["Hopefully I can find what I am looking for with the information you can find."],
                )?;
                ctx.var("ep13_2_busut").set(Val::from(2))?;
                ctx.call(Function::SetQuest, vec![Val::from(11101)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Industrious Man",
                    args![".............", "Well if you decide to change your mind, come back to me."],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_2_busut").get()? == 2 {
        ctx.lines_as(
            "Bazett",
            args![
                "Um... for today can you search for giants in the Manuk Field?",
                "That place is pretty cold so, you might need a coat."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bazett",
            args![
                "I used this note so it should still be useful. Take it...",
                "Ah... and don't worry about the title of the note."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Bazett", args!["After you've finished searching bring that note back to me."])?;
        ctx.next()?;
        ctx.lines_as("Bazett", args!["Got it? Ok take care and see you soon."])?;
        ctx.var("ep13_2_busut").set(Val::from(3))?;
        ctx.var("ep13_2_bs1").set(Val::from(1))?;
        ctx.var("ep13_2_bs2").set(Val::from(1))?;
        ctx.var("ep13_2_bs3").set(Val::from(1))?;
        ctx.var("ep13_2_bs4").set(Val::from(1))?;
        ctx.call(Function::GetItem, vec![Val::from(6074), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11101), Val::from(11102)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_busut").get()? == 3 {
        ctx.lines_as("Bazett", args!["How's the search going?"])?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(6074)])?.number()? < 1 {
            ctx.lines_as("Bazett", args!["!!!!!!!", "You lost the notes!!?", "Sigh..."])?;
            ctx.next()?;
            ctx.lines_as("Bazett", args!["What's done has been done.", "I'll give you a new one."])?;
            ctx.call(Function::GetItem, vec![Val::from(6074), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Bazett", args!["Take it easy..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_2_busut").get()? == 4 {
        ctx.lines_as("Bazett", args!["Oh! You're done with the investigation?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:Not yet...")])? {
            1 => {
                if ctx.call(Function::CountItem, vec![Val::from(6074)])?.number()? < 1 {
                    ctx.lines_as("Bazett", args!["!!!!!!!", "You lost the notes!!?", "Sigh..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazett",
                        args![
                            "What's done has been done.",
                            "You must be tired, go take a rest.",
                            "I'll go prepare new notes.",
                            "You can just go investigate again for me."
                        ],
                    )?;
                    ctx.var("ep13_2_busut").set(Val::from(7))?;
                    ctx.call(Function::EraseQuest, vec![Val::from(11102)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(11104)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Bazett", args!["May I take a look at the notes first?", "Oh!!"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        " - Bazett is reading the detailed contents - ",
                        " - He seems to be captivated. - ",
                        " - It's better if I leave him alone. - ",
                        " - So he can finish. - "
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Bazett", args!["Mm...there are actually such things?!"])?;
                    ctx.next()?;
                    ctx.lines_as("Bazett", args!["Mm...I see, I see."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazett",
                        args![
                            "Even though the content is simple, it's well organized and interesting.",
                            ((Val::from("As expected of ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!!"))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazett",
                        args![
                            "You've done well.",
                            "You must be exhausted. Go take a rest. We'll continue tomorrow."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bazett",
                        args![
                            "Ah... this isn't much, but it's a coin that the giants use.",
                            "Maybe you can buy something from them with this."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(6074), Val::from(1)])?;
                    ctx.var("ep13_2_busut").set(Val::from(7))?;
                    if ctx.var("ep13_2_bs1").get()? == 3 {
                        ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(1)])?;
                    }
                    if ctx.var("ep13_2_bs2").get()? == 3 {
                        ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(1)])?;
                    }
                    if ctx.var("ep13_2_bs3").get()? == 3 {
                        ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(1)])?;
                    }
                    if ctx.var("ep13_2_bs4").get()? == 3 {
                        ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(1)])?;
                    }
                    ctx.call(Function::EraseQuest, vec![Val::from(11102)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(11104)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as("Bazett", args!["Take it easy..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Bazett",
            args![
                "You've done well.",
                "You must be exhausted. Go take a rest, we'll continue tomorrow."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn bazett_teablack_ep13bs(ctx: &Ctx) -> Script {
    bazett_teablack_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn worker_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_2_busut").get()? == 3 {
            if ctx.var("ep13_2_bs1").get()? == 1 {
                ctx.lines_as(
                    "Worker",
                    args!["Oops, it's dangerous, almost broken.", "I should hurry to change it...otherwise."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What's the matter?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Worker", args!["Ah....here.....um...", "......", ".........", "Nothing!!"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What's up?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Worker",
                    args!["You are an outsider.", "......", "I was suprised by your voice."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Yes, sorry to startle you."],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SMILE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Worker",
                    args![
                        "Haha, it's ok.",
                        "Recently I heard about people like you but it's the first time I've actually met one."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Worker", args!["You speak our language pretty good."])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Ah...anyway..you look like you're having some trouble. What happened?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Worker", args!["Nothing!!"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Are you sure? Fate has to have brought us together for a reason.", "Tell me~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Worker", args!["......", "Frankly..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Worker",
                    args!["The screw is too old to use to fix the tent, so I should change it before it breaks."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Worker",
                    args!["I can't leave so I'm just waiting for my friend to pass by to help me."],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Can I help you?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Worker",
                    args!["No~that's alright. We just met so I can't ask you for a favor."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["That's ok! We can help each other!"],
                )?;
                ctx.next()?;
                ctx.mes("- Suddenly there is a screeching sound as the screw breaks -")?;
                ctx.next()?;
                ctx.lines_as("Worker", args!["Ugh!!!", "**Sigh**"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_CRY")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Worker", args!["Well that's that!", "Anyway my name is ^0000FFGill^000000."])?;
                ctx.next()?;
                ctx.lines_as("Gill", args!["I need something to fix the tent with, if you can get^0000FF 30 Horn of Hillslion^000000, I can make them into sturdy enough screws."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gill",
                    args!["Sorry, again for making you do this but I will be waiting here for you."],
                )?;
                ctx.var("ep13_2_bs1").set(Val::from(2))?;
                ctx.call(Function::SetQuest, vec![Val::from(11105)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ep13_2_bs1").get()? == 2 {
                if ctx.call(Function::CountItem, vec![Val::from(6032)])?.number()? > 29 {
                    ctx.lines_as(
                        "Gill",
                        args!["You helped me collect all of the Horns of Hillslion?", "Thank you so much."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Gill", args!["It's all because of you, we are able to prevent anything disastrous.", ((Val::from("^0000FFEven though you're also an alien race, but compared to the vile fairies^000000, I'm glad to have met someone like ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
                    ctx.call(Function::DelItem, vec![Val::from(6032), Val::from(30)])?;
                    ctx.var("ep13_2_bs1").set(Val::from(3))?;
                    ctx.var("ep13_2_busut").set(Val::from(4))?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(11105)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Gill", args!["Sorry, again for making you do this but I will be waiting here for you to bring back the^0000FF 30 Horn of Hillslions.^000000."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("ep13_2_bs1").get()? == 3 {
                    ctx.lines_as("Gill", args!["Thank you for helping me.", ((Val::from("^0000FFEven though you're also an alien race, but compared to the vile fairies^000000, I'm glad to have met someone like ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Worker",
                        args!["This is too dangerous, it's become too loose...", "It must be replaced soon..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("ep13_2_busut").get()? == 4 {
                if ctx.var("ep13_2_bs1").get()? == 1 {
                    ctx.lines_as(
                        "Worker",
                        args!["This is too dangerous, it's become too loose...", "It must be replaced soon..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Is something wrong?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args!["Ah... see here...", "......", ".........", "Nevermind. Nothing."],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You don't have to be like this."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args![
                            "You are one of the strange race, aren't you?",
                            "......",
                            "To suddenly hear you speak our language gave me a bit of a shock."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Since I'm travelling here, so I thought I'd need it..."],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SMILE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args![
                            "Haha, is that so?",
                            "I've heard of the rumour that your race exists, but I've never seen one before."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Worker", args!["And to think you can speak our language. Interesting."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Ah...but just a moment ago, you were having a hard time with something. Is something wrong?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Worker", args!["It's nothing."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Ah~ It must be fate that we met~", "You can just tell me."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Worker", args!["......", "Well..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args!["The screw that is used to keep our tent in place is rusting away... we must get a new one."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args![
                            "But I can't leave this place, so I was waiting for someone to pass by.",
                            "And you just happened to talk to me. I thought you were someone from our tribe."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I can help you."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args!["No, it's alright. We just met, so I don't want to trouble you."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Don't worry about it! Life is all about helping eachother, don't you think so?"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        " - While the disagreement is taking place, - ",
                        " - The screw that has held its ground til now - ",
                        " - Finally gives in and breaks in half. - "
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Worker", args!["Ah!!!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_CRY")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Worker",
                        args![
                            "I can't just care about pride now.",
                            "Let me introduce myself, I am ^0000FFGill^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gill",
                        args![
                            "I need a screw to stablize our tent, and it can be found from the ^0000FFHillslion^000000 monster.",
                            "Please help me collect ^0000FF30 Horns of Hillslion^000000, and that should be enough for now."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Gill", args!["I'm really sorry, so please hurry."])?;
                    ctx.var("ep13_2_bs1").set(Val::from(2))?;
                    ctx.call(Function::SetQuest, vec![Val::from(11105)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_2_bs1").get()? == 2 {
                    if ctx.call(Function::CountItem, vec![Val::from(6032)])?.number()? > 29 {
                        ctx.lines_as(
                            "Gill",
                            args!["You helped me collect all of the Horns of Hillslion?", "Thank you so much."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Gill", args!["It's all because of you, we are able to prevent anything disastrous.", ((Val::from("^0000FFEven though you're also an alien race, but compared to the vile fairies^000000, I'm glad to have met someone like ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
                        ctx.call(Function::DelItem, vec![Val::from(6032), Val::from(30)])?;
                        ctx.var("ep13_2_bs1").set(Val::from(3))?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(11105)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Gill", args!["Making you do something like this, I feel ashamed."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_2_bs1").get()? == 3 {
                        ctx.lines_as("Gill", args!["Thank you for helping me.", ((Val::from("^0000FFEven though you're also an alien race, but compared to the vile Fairies^000000, I'm glad to have met someone like ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Worker",
                            args!["This is too dangerous, it's become too loose...", "It must be replaced soon..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else {
                ctx.lines_as(
                    "Gill",
                    args!["You're here again?", "Talking with someone occasionally is quite relaxing."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as("Worker", args!["Ehahdie O Ehai", "Ohek Hekdh I dkek", "Ohehp Qe Tehdhah"])?;
        ctx.next()?;
        ctx.lines_as(
            "Worker",
            args!["Ehaodke Thdieqak Khehdi", "PHhdkel", "Thhdqdcczk U dheagelokd dok"],
        )?;
        ctx.next()?;
        ctx.mes("- You can't understand what he's saying. - ")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn worker_ep13bs(ctx: &Ctx) -> Script {
    worker_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StrangeDeviceEp13OutStep {
    Start,
    OnTouch,
}

fn strange_device_ep13_out_run(ctx: &Ctx, mut step: StrangeDeviceEp13OutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StrangeDeviceEp13OutStep::Start => {
                ctx.lines(args![
                    "There's something strange here.",
                    "Maybe that device can be controlled from here."
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Try to control.:Don't try to control it.")])? {
                    1 => {
                        ctx.mes("You press the device buttons and it suddenly becomes dark.")?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("nyd_dun02"), Val::from(139), Val::from(268)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("You decide to leave it alone.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = StrangeDeviceEp13OutStep::OnTouch;
                continue 'machine;
            }
            StrangeDeviceEp13OutStep::OnTouch => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn strange_device_ep13_out(ctx: &Ctx) -> Script {
    strange_device_ep13_out_run(ctx, StrangeDeviceEp13OutStep::Start, Vec::new()).map(|_| ())
}

pub fn strange_device_ep13_out_ontouch(ctx: &Ctx) -> Script {
    strange_device_ep13_out_run(ctx, StrangeDeviceEp13OutStep::OnTouch, Vec::new()).map(|_| ())
}

fn strange_device_ep13_in_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_way_loot = Val::from(0);
    if ctx.var("$@08_ep13nydun02_in").get()? == 1 {
        ctx.lines(args![
            "The device has already been activated.",
            "You must wait for the controls to reset."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "There's a controllable device here.",
            "You might be able to control that device from here."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Control.:Quit.")])? {
            1 => {
                if ctx.var("$@08_ep13nydun02_in").get()? == 1 {
                    ctx.mes("Seems to have been started.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.var("$@08_ep13nydun02_in").set(Val::from(1))?;
                    l_way_loot = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                    if l_way_loot.clone() == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_s1::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_11::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_21::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_22::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_22_2::OnEnable")])?;
                    } else {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_s3::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_13::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_14::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_24::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_24_2::OnEnable")])?;
                    }
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    ctx.lines(args!["The device is now on.", "Panels have appeared across the ledge."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.mes("You decide not to control the device.")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    return Err(Stop::End);
}

pub fn strange_device_ep13_in(ctx: &Ctx) -> Script {
    strange_device_ep13_in_body(ctx, Vec::new()).map(|_| ())
}

fn strange_device_ep13_in_ontimer70000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_nd2f_mng::OnReset")])?;
    ctx.var("$@08_ep13nydun02_in").set(Val::from(0))?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn strange_device_ep13_in_ontimer70000(ctx: &Ctx) -> Script {
    strange_device_ep13_in_ontimer70000_body(ctx, Vec::new()).map(|_| ())
}

fn strange_device_ep13_in_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99_4")?])?;
    return Err(Stop::End);
}

pub fn strange_device_ep13_in_ontouch(ctx: &Ctx) -> Script {
    strange_device_ep13_in_ontouch_body(ctx, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_onenable(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_ontimer2000(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_ontimer4000(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_ontimer6000(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_ontimer8000(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_ontimer10000(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_0_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_s_0_run(ctx, Ep13WarpS0Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_onenable(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ontimer2000(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ontimer4000(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ontimer6000(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ontimer8000(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ontimer10000(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ontimer12000(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_1_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_s_1_run(ctx, Ep13WarpS1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer2000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer4000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer6000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer8000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer10000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer12000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ontimer15000(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn ep13_warp_s_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_s_2_run(ctx, Ep13WarpS2Step::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ep13WarpW0Step {
    Start,
    OnTouch,
}

fn ep13_warp_w_0_run(ctx: &Ctx, mut step: Ep13WarpW0Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13WarpW0Step::Start => {
                step = Ep13WarpW0Step::OnTouch;
                continue 'machine;
            }
            Ep13WarpW0Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("nyd_dun01"), Val::from(214), Val::from(68)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ep13_warp_w_0(ctx: &Ctx) -> Script {
    ep13_warp_w_0_run(ctx, Ep13WarpW0Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_w_0_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_w_0_run(ctx, Ep13WarpW0Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_22_2(ctx: &Ctx) -> Script {
    ep13_warp_22_2_run(ctx, Ep13Warp222Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_22_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_22_2_run(ctx, Ep13Warp222Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_22_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_22_2_run(ctx, Ep13Warp222Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_22_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_22_2_run(ctx, Ep13Warp222Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_22_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_22_2_run(ctx, Ep13Warp222Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_24_2(ctx: &Ctx) -> Script {
    ep13_warp_24_2_run(ctx, Ep13Warp242Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_24_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_24_2_run(ctx, Ep13Warp242Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_24_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_24_2_run(ctx, Ep13Warp242Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_24_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_24_2_run(ctx, Ep13Warp242Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_24_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_24_2_run(ctx, Ep13Warp242Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_43_2(ctx: &Ctx) -> Script {
    ep13_warp_43_2_run(ctx, Ep13Warp432Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_43_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_43_2_run(ctx, Ep13Warp432Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_43_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_43_2_run(ctx, Ep13Warp432Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_43_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_43_2_run(ctx, Ep13Warp432Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_43_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_43_2_run(ctx, Ep13Warp432Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_45_2(ctx: &Ctx) -> Script {
    ep13_warp_45_2_run(ctx, Ep13Warp452Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_45_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_45_2_run(ctx, Ep13Warp452Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_45_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_45_2_run(ctx, Ep13Warp452Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_45_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_45_2_run(ctx, Ep13Warp452Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_45_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_45_2_run(ctx, Ep13Warp452Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_61_2(ctx: &Ctx) -> Script {
    ep13_warp_61_2_run(ctx, Ep13Warp612Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_61_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_61_2_run(ctx, Ep13Warp612Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_61_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_61_2_run(ctx, Ep13Warp612Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_61_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_61_2_run(ctx, Ep13Warp612Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_61_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_61_2_run(ctx, Ep13Warp612Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_65_2(ctx: &Ctx) -> Script {
    ep13_warp_65_2_run(ctx, Ep13Warp652Step::Start, Vec::new()).map(|_| ())
}
