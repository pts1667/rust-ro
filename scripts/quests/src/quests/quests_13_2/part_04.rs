use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn wanted_sign_ep13_2ect01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_mdrama").get()?.number()? > 5 {
        ctx.lines_as(
            "Wanted",
            args![
                " ",
                "Wanted: Dandelion",
                "Requester: Rin",
                "Reward : treasure box.",
                " ",
                "Notice : We thought that all of the Dandelion gang were terminated.",
                "But they weren't.",
                "I've heard that there was a Dandelion's black clothing found in Ash-Vacuum.",
                "There were some problems...",
                "I need someone to find the Dandelion and terminate them.",
                "I can provide a treasure box that I cherish.",
                "I've heard a rumor that he's in a cave somewhere..!",
                " ",
                "PS> They have a bad reputation...",
                "So you may not want to go alone...",
                " ",
                "PSS> I can't reward you directly...",
                "'Pinedel' will reward you once the deed is done..!",
                " "
            ],
        )?;
        ctx.next()?;
        if ctx.var("ep13_2_wanted").get()? == 0 {
            ctx.mes("Do you accept Rin's request?")?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I accept that request.:No, I can't.")],
            )?) == 1
            {
                ctx.mes("I accepted Rin's request.")?;
                ctx.var("ep13_2_wanted").set(Val::from(1))?;
                ctx.call(Function::SetQuest, vec![Val::from(7076)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("No, I won't get involved in it.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("I've already decided to take the request.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("There must be someone who remove the wanted notice paper.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wanted_sign_ep13_2ect01(ctx: &Ctx) -> Script {
    wanted_sign_ep13_2ect01_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_0_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.mes("- You're carrying too many items. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_dayegg").get()? == 1 && ctx.call(Function::CountItem, vec![Val::from(6093)])?.number()? < 10) {
        ctx.mes("We just got fresh eggs from the dragon nest.")?;
        ctx.call(Function::GetItem, vec![Val::from(6093), Val::from(1)])?;
        ctx.call(Function::DisableNpc, vec![])?;
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_0(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_0_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_0_ontimer420000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_0_ontimer420000(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_0_ontimer420000_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_0_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_0_onenable(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_0_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_0_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_0_ondisable(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_0_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.mes("- You're carrying too many items. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_dayegg").get()? == 1 && ctx.call(Function::CountItem, vec![Val::from(6093)])?.number()? < 10) {
        ctx.mes("We just got fresh eggs from the dragon nest.")?;
        ctx.call(Function::GetItem, vec![Val::from(6093), Val::from(1)])?;
        ctx.call(Function::DisableNpc, vec![])?;
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_1(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_1_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_1_ontimer240000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_1_ontimer240000(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_1_ontimer240000_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_1_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_1_onenable(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_1_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_1_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_1_ondisable(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_1_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.mes("- You're carrying too many items. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_dayegg").get()? == 1 && ctx.call(Function::CountItem, vec![Val::from(6093)])?.number()? < 10) {
        ctx.mes("We just got fresh eggs from the dragon nest.")?;
        ctx.call(Function::GetItem, vec![Val::from(6093), Val::from(1)])?;
        ctx.call(Function::DisableNpc, vec![])?;
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_2(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_2_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_2_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_2_ontimer120000(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_2_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_2_onenable(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn dragon_egg_ep13_degg_2_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn dragon_egg_ep13_degg_2_ondisable(ctx: &Ctx) -> Script {
    dragon_egg_ep13_degg_2_ondisable_body(ctx, Vec::new()).map(|_| ())
}

pub fn egg_keeper_draco_13_1(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_1_run(ctx, EggKeeperDraco131Step::Start, Vec::new()).map(|_| ())
}

pub fn egg_keeper_draco_13_1_onenable(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_1_run(ctx, EggKeeperDraco131Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn egg_keeper_draco_13_1_ondisable(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_1_run(ctx, EggKeeperDraco131Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn egg_keeper_draco_13_1_ontouch(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_1_run(ctx, EggKeeperDraco131Step::OnTouch, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_2(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_2_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Egg Keeper Draco#13_2")])?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_2_onenable(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_2_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Egg Keeper Draco#13_2")])?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_2_ondisable(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_2_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Egg Keeper Draco#13_2")])?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("nyd_dun02"),
            Val::from(206),
            Val::from(157),
            Val::from("Egg Keeper Draco"),
            Val::from(2013),
            Val::from(1),
            Val::from("Egg Keeper Draco#13_4::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_2_ontouch(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_3(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_3_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_3_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_3_onmymobdead(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_3_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_3_ontimer180000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Egg Keeper Draco#13_1")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_3_ontimer180000(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_3_ontimer180000_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_4(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_4_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_4_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_4_onmymobdead(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_4_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn egg_keeper_draco_13_4_ontimer180000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Egg Keeper Draco#13_2")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn egg_keeper_draco_13_4_ontimer180000(ctx: &Ctx) -> Script {
    egg_keeper_draco_13_4_ontimer180000_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan01(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan01_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan01_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Stranger#ep13_2_dan01")])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan01_onenable(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan01_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan01_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Stranger#ep13_2_dan01")])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan01_ondisable(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan01_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan01_oncall_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Monster,
        vec![
            Val::from("nyd_dun02"),
            Val::from(144),
            Val::from(103),
            Val::from("Runway Dandelion"),
            Val::from(2026),
            Val::from(1),
            Val::from("Stranger#ep13_2_dan03::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan01_oncall(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan01_oncall_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan01_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("nyd_dun02"), Val::from("Stranger#ep13_2_dan03::OnMyMobDead")],
    )?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan01_onreset(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan01_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan01_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_SURPRISE")?,
            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Stranger#ep13_2_dan01")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Stranger#ep13_2_dan02::OnCall")])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan01_ontouch(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan01_ontouch_body(ctx, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::Start, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02_oninit(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02_ondisable(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02_onenable(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02_oncall(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::OnCall, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02_onreset(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::OnReset, Vec::new()).map(|_| ())
}

pub fn stranger_ep13_2_dan02_ontouch(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan02_run(ctx, StrangerEp132Dan02Step::OnTouch, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan03(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan03_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan03_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan03_onmymobdead(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan03_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan03_ontimer300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
        ctx.call(Function::EnableNpc, vec![Val::from("Stranger#ep13_2_dan01")])?;
    } else {
        ctx.call(Function::DoNpcEvent, vec![Val::from("Stranger#ep13_2_dan02::OnEnable")])?;
    }
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan03_ontimer300000(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan03_ontimer300000_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan04_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan04(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan04_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan04_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan04_onmymobdead(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan04_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_ep13_2_dan04_ontimer300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
        ctx.call(Function::EnableNpc, vec![Val::from("Stranger#ep13_2_dan02")])?;
    } else {
        ctx.call(Function::DoNpcEvent, vec![Val::from("Stranger#ep13_2_dan01::OnEnable")])?;
    }
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn stranger_ep13_2_dan04_ontimer300000(ctx: &Ctx) -> Script {
    stranger_ep13_2_dan04_ontimer300000_body(ctx, Vec::new()).map(|_| ())
}

fn schwarzwald_mechanic_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_trs_time01 = Val::from(0);
    if !(ctx.call(Function::CheckWeight, vec![Val::from(2782), Val::from(1)])?.is_true()) {
        ctx.lines_as(
            "Mechanic Engineer Dorance",
            args![
                "It looks like you're carrying too many things.",
                "Why not put some of your items in storage and come back?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_1_edq").get()?.number()? > 2 || ctx.var("ep13_start").get()? == 100) {
        if ctx.var("ep13_2_rhea").get()?.number()? < 1 {
            ctx.lines_as(
                "Mechanic Engineer Dorance",
                args!["Do you know about the Ring of the Ancient Wise King? If you wear that Ring, you can talk to animals."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mechanic Engineer Dorance",
                args!["I don't know if it is a legend based on truth or not... but something like that would come in pretty handy."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mechanic Engineer Dorance",
                args!["Outside of the Expedition Camp, There's a race of animals which uses a mysterious language not used in Midgard."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mechanic Engineer Dorance",
                args!["We don't know if they are friend or foe, but it will be much easier for our research here if we could find out."],
            )?;
            ctx.next()?;
            ctx.lines_as("Mechanic Engineer Dorance", args!["So, the Triple Alliance Research Group is working together to find out. I would like to know how far the ^0000ff Arunafeltz Linguist^000000's decoding process is. Would you go and find out for me?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mechanic Engineer Dorance",
                args!["The ^0000ff Arunafeltz Linguist^000000 should be on the second floor of the Alliance Headquarters."],
            )?;
            ctx.call(Function::SetQuest, vec![Val::from(8243)])?;
            ctx.var("ep13_2_rhea").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("ep13_2_rhea").get()?.number()? > 0 && ctx.var("ep13_2_rhea").get()?.number()? < 7) {
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["Outside of the Expedition Camp, There's a race who uses a mysterious language which is not used in Midgard"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["We don't know if they are friend or foe, one thing for sure is it will be more easier for our research here."],
                )?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["So, the Triple Alliance Research Group is working together to find out. I would like to know how far the ^0000ff Arunafeltz Linguist^000000's decoding process is. Would you go and find out for me?"])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Go to a big building in center of the Expedition Camp and inside that building, meet ^0000ff Arunafeltz Linguist^000000 and ask him how far his progress is."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["The ^0000ff Arunafeltz Linguist^000000 should be on the second floor of the Alliance Headquarters."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ep13_2_rhea").get()? == 7 {
                ctx.mes("- He's making a grim face at a flooding pile of research documents. -")?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Ah, You are here."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["t seems like you've done much for us. I thank you for that.", "But...Sigh..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args![
                        "Can you see this? The amount of these research documents?",
                        "It is a good thing that we finally decoded the other world's language... it's just too much."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["We can't always accompany a Linguist for an interpretion... We need to communicate with those other world races in order to explore this world..."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["To store this amount of information will need a super computer the size of this headquarter building and its processing speed will be very very slow."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Sigh... Anyways, If there's any way to store an enormous amount of data, I should be able to make a very-small portable translator."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["I heard that by using a Magical Spell of Rune-Midgarts, you can transpose data to a magical power and lock it into a gem... I'd like some suggestions from the Rune-Midgarts Magician."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Would you be kind to ask them for help?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["While you are gone, I be here organizing these documents."],
                )?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8248)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8249)])?;
                ctx.var("ep13_2_rhea").set(Val::from(8))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("ep13_2_rhea").get()?.number()? > 7 && ctx.var("ep13_2_rhea").get()?.number()? < 11) {
                ctx.lines_as("Mechanic Engineer Dorance", args!["To store this amount of information will need a super computer with a size of this headquarter building. and its processing speed will be very very slow..."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Sigh... Anyways, If there's any way to store an enormous amount of data, I could be able to make a very-small portable translator."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["I heard that by using the Magical Spell of Rune-Midgarts, you could transpose a data to a magical power and lock it into a gem....I'd like to get some suggestion from the Magician of Rune-Midgarts."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Would you be kind to ask them for help?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["While you are gone, I be here organizing these documents."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ep13_2_rhea").get()? == 11 {
                ctx.lines_as("Mechanic Engineer Dorance", args!["How'd it go?"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Give him the Gem.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Ah, Looks like you've succeeded.", "Inputting data into a gem with a magical power... It's something you can't do with a technology... It's a bit resentful to admit, but it's magnificent."])?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["Well then, I should be working on making a machine to read the magical data in this Gem so we could use them right away."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["Hmm...", "Fortunately, It won't be as hard as we thought."],
                )?;
                ctx.next()?;
                ctx.lines_as("Mechanic Engineer Dorance", args!["If we transform a magical power wave to an eletric signal and connect it to a human body, then transmit a data to a brain directly. That will allow us to receive and understand a data without any loading process."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args![
                        "I don't have time to explain everything. I have to start the work right away.",
                        "With all preperation done, allow me about one hour to finish this thing."
                    ],
                )?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8252)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8253)])?;
                ctx.var("ep13_2_rhea").set(Val::from(12))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ep13_2_rhea").get()? == 12 {
                l_trs_time01 = ctx.call(Function::CheckQuest, vec![Val::from(8253), ctx.constant("PLAYTIME")?])?;
                if l_trs_time01.clone() == 2 {
                    ctx.lines_as("Mechanic Engineer Dorance", args!["Sigh, What should I do now?!"])?;
                    ctx.next()?;
                    ctx.lines(args!["- Dorance is walking here and there -", "- in a dither. -"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Is something wrong?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args!["What should I do?! Ain't I REALLY a genius?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["I've made it!!!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args![
                            "Convenient portable size!",
                            "Fashionable and practical design!",
                            "Features the Other World's Words / Idiom / Grammer / Phrases!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args![
                            "Have you experienced the loading speed?",
                            "Don't you even mention it if you haven't."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args![
                            "That's not all!",
                            "For the Well-Being life, it will buff your strength and intelligence!!!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args!["With all these marvelous features, it's just 39,900 zeny....!!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["...Just what are you trying to imitate..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["Important thing is, I've made it!!"])?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["Well, Seeing is believing.", "Take a look."])?;
                    ctx.next()?;
                    ctx.mes("- He handed over the small ring. -")?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["The name is 'the Ring of Wise King'. I designed it after the Ring used by ancient king to communicate with animals."])?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["Do not just look at it like any other rings. You see this small gem here? This socket which holds a gem and a ring part are precisely crafted, it will transform a magical power wave to electric signal and trasmit it directly to our brain."])?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["Since this electric signal is same as the signal used by our body's nerve system, it will enable us to understand and use the language fluently."])?;
                    ctx.next()?;
                    ctx.lines_as("Mechanic Engineer Dorance", args!["Once this translator gets popularized, it'd be just a matter of time for us to advance into the other world. Well, the problem is...we don't have enough quantity yet..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args![
                            "Hmm...",
                            "Anyways, now we've got the idea, I will give you this first piece of work."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mechanic Engineer Dorance",
                        args![
                            "Thank you for helping us out.",
                            "I hope this ring would be a big help for your journey through the other world."
                        ],
                    )?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(8253)])?;
                    ctx.var("ep13_2_rhea").set(Val::from(100))?;
                    ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(300000)])?;
                    ctx.call(Function::GetItem, vec![Val::from(2782), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "- He has scattered mechanical parts -",
                        "- all over the room and so wrapped up -",
                        "- in his work -"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args!["- I might get bitten if I bother him -", "- Let's leave him alone -"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as("Mechanic Engineer Dorance", args!["Thank you for your help."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mechanic Engineer Dorance",
                    args!["I hope this ring would be a big help for your journey through the other world."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as(
            "Mechanic Engineer Dorance",
            args!["Do you know about the Ring of Ancient Wise King? If you wear that Ring, you could talk to an animal."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mechanic Engineer Dorance",
            args!["I don't know if it is a legend based on a truth or not... something like that will come in handy."],
        )?;
        ctx.next()?;
        ctx.lines_as("Mechanic Engineer Dorance", args!["By the way, Who are you? This building is under Commander's control. This place is not to be entered freely... Are you acquainted with our Commander?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mechanic Engineer Dorance",
            args![
                "Are you looking for something to do? Perhaps I could use some help... Well, I don't know if you are trustworthy just yet."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mechanic Engineer Dorance", args!["If you are good enough to solve some problems within the expedition camp, and carry out a mission given by Commander... then I think I could trust you."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn schwarzwald_mechanic_1(ctx: &Ctx) -> Script {
    schwarzwald_mechanic_1_body(ctx, Vec::new()).map(|_| ())
}

fn arunafeltz_linguist_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_check_manque00 = Val::from(0);
    let mut l_check_splque00 = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_man_score00 = Val::from(0);
    let mut l_man_score01 = Val::from(0);
    let mut l_man_score02 = Val::from(0);
    let mut l_man_score03 = Val::from(0);
    let mut l_spl_score00 = Val::from(0);
    let mut l_spl_score01 = Val::from(0);
    let mut l_spl_score02 = Val::from(0);
    let mut l_spl_score03 = Val::from(0);
    if ctx.var("ep13_2_rhea").get()?.number()? < 1 {
        ctx.lines_as(
            "Linguist Dictionary",
            args!["Language distinguishes humans from animals. Every single intellectual life form has their own language."],
        )?;
        ctx.next()?;
        ctx.lines_as("Linguist Dictionary", args!["Language is a method / system which delievers one's feeling or thinking by voice or letters. Anyway, its purpose is to communicate."])?;
        ctx.next()?;
        ctx.lines_as("Linguist Dictionary", args!["Difference between a language and a sound is that language is a socially implied promise by making common system of communication to make it possible to understand eachother."])?;
        ctx.next()?;
        ctx.lines_as("Linguist Dictionary", args!["That Socially implied promise, if we could grasp the meaning of a system and structure, we will be able to understand other intellectual life form's language...", "...................................And just who might you be?"])?;
        ctx.next()?;
        ctx.lines_as("Linguist Dictionary", args!["This is a restricted place under control of our Commander. No one could enter here without proper approval... Are you acquainted with our Commander?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ep13_2_rhea").get()? == 1 {
            ctx.lines_as("Linguist Dictionary", args!["Language is a distinctive feature of human race compared to animals. Every single intellectual life form has their own language."])?;
            ctx.next()?;
            ctx.lines_as("Linguist Dictionary", args!["Language is a method / system which delievers one's feeling or thinking by voice or letters. Anyway, its purpose is to communicate."])?;
            ctx.next()?;
            ctx.lines_as("Linguist Dictionary", args!["Difference between a language and a sound is that language is a socially implied promise by making common system of communication to make it possible to understand eachother."])?;
            ctx.next()?;
            ctx.lines_as("Linguist Dictionary", args!["That Socially implied promise, if we could grasp the meaning of a system and structure, we will be able to understand other intellectual life form's language...", "...................................And just who might you be?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I am Me!:I don't have a name to tell you!")])? {
                1 => {}
                2 => {
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["What the heck?", "If you are so bored, why don't you go fishing?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.lines_as("Linguist Dictionary", args!["Ah~ I've heard from our Commander about you. You've done much for the Midgard Alliance Camp. Anyways, what brings you here?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I thought I had confused you a bit...:What were you doing?")])? {
                1 => {
                    ctx.lines_as("Linguist Dictionary", args!["Confusion is a feeling that comes from distraction of one's mind when something that couldn't be predicted happens. I'm not a person who easily gets confused."])?;
                    ctx.next()?;
                }
                2 => {}
                _ => {}
            }
            ctx.lines_as("Linguist Dictionary", args!["I'm a linguist from Arunafeltz. I'm a scholor who studies a system and a structure of every kinds of language in the world.", "And...don't you think the reason man like me is here is that there's a predictable cause?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Linguist Dictionary",
                args![
                    "From what I heard, you've done much for the Midgard Alliance Research Group.",
                    "And while you were helping us, I believe you've seen and visited many places here in the other world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Linguist Dictionary",
                args!["Have you met the other world's people by any chance?"],
            )?;
            ctx.next()?;
            l_check_splque00 = ctx.call(Function::CheckQuest, vec![Val::from(2158)])?;
            l_check_manque00 = ctx.call(Function::CheckQuest, vec![Val::from(2159)])?;
            if (l_check_splque00.clone() == 0 && l_check_manque00.clone() == 0) {
                ctx.lines_as("Linguist Dictionary", args!["Looks like you haven't encountered them yet."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["Rumor here says that there's a fairy-looking and a giant wood people living out in the fields next to our camp here."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["Apparently these people are speaking in a strange languages that we humans can't understand...but no one seems to be able to remember how their languages sound like."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["A NEW language!! This is an exciting discovery for a linguist like myself, but as you know, there's dangerous monsters outside... It's not possible for a mere scholor to go out wandering around."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Linguist Dictionary",
                    args!["I've said this much, I think you'd be able to guess what I'm going to ask you to do."],
                )?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["The Alliance Research Group has great interest in other intelligent life. Each country's secret agents are making every effort to communicate with those two races, there's just not enough information."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["Do me a favor and go outside of the camp, find the fairies and wooden giants, and remember everything that they are saying, word for word so that I can hear what their language sounds like."])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8243)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8244)])?;
                ctx.var("ep13_2_rhea").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Linguist Dictionary",
                    args!["From the look on your face, I think you have something crossed in your mind."],
                )?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["Rumor here says that there's a fairy-looking and a giant wood people living out in the fields next to our camp here."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["Apparently these people are speaking in a strange languages that we humans can't understand...but no one seems to be able to remember how their languages sound like."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["A NEW language!! This is an exciting discovery for a linguist like myself, but as you know, there's dangerous monsters outside... It's not possible for a mere scholor to go out wandering around."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Linguist Dictionary",
                    args!["I've said this much, I think you'd be able to guess what I'm going to ask you to do."],
                )?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["The Alliance Research Group has great interest in other intelligent life. Each country's secret agents are making every effort to communicate with those two races, there's just not enough information."])?;
                ctx.next()?;
                ctx.lines_as("Linguist Dictionary", args!["Do me a favor and go outside of the camp, find the fairies and wooden giants, and remember everything that they are saying, word for word so that I can hear what their language sounds like."])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8243)])?;
                ctx.call(Function::SetQuest, vec![Val::from(8244)])?;
                ctx.var("ep13_2_rhea").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("ep13_2_rhea").get()? == 2 {
                l_check_splque00 = ctx.call(Function::CheckQuest, vec![Val::from(2158)])?;
                l_check_manque00 = ctx.call(Function::CheckQuest, vec![Val::from(2159)])?;
                if (l_check_splque00.clone().number()? > 0 && l_check_manque00.clone().number()? > 0) {
                    ctx.lines_as("Linguist Dictionary", args!["You are back!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args![
                            "So, What were they saying?",
                            "Of course you remember them exactly as what they've said, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["Now, let's start with the Fairies! What is the fairy's language like? Please write them down here sentence by sentence!"])?;
                    ctx.next()?;
                    l_spl_score00 = Val::from(0);
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "RLGHLRXLA TKANTLFDMS" {
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "And?"],
                        )?;
                        l_spl_score01 = (l_spl_score00.clone() + Val::from(1));
                        ctx.next()?;
                    } else {
                        ctx.lines(args![
                            ((Val::from("") + l_input_s.clone()) + Val::from("....")),
                            "Mmm... Hmm...?"
                        ])?;
                        l_spl_score01 = l_spl_score00.clone();
                        ctx.next()?;
                    }
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "WJACK TNAHRDNJSDMFH" {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "And?"])?;
                        l_spl_score02 = (l_spl_score01.clone() + Val::from(1));
                        ctx.next()?;
                    } else {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "I see...?"])?;
                        l_spl_score02 = l_spl_score01.clone();
                        ctx.next()?;
                    }
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if (l_input_s.clone() == "WLSGHKWND !!" || l_input_s.clone() == "WLSGHKWND") {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "Is that all?"])?;
                        l_spl_score03 = (l_spl_score02.clone() + Val::from(1));
                        ctx.next()?;
                    } else {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "Is that all?"])?;
                        l_spl_score03 = l_spl_score02.clone();
                        ctx.next()?;
                    }
                    if l_spl_score03.clone().number()? > 2 {
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args![
                                "So...If we put'em together,",
                                "RLGHLRXLA TKANTLFDMS",
                                "WJACK TNAHRDNJSDMFH",
                                "WLSGHKWND !!",
                                "It'd be like this."
                            ],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as("Linguist Dictionary", args!["Hmm...?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args![
                                "You sure this is exactly what they've said?",
                                "I can't understand anything from this..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args!["Would you please go back again and check if it is exactly what they were saying?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Linguist Dictionary", args!["Good. then next is the Wooden Giants! What's their language like? Write them down sentence by sentence just as you did with the Fairy's."])?;
                    ctx.next()?;
                    l_man_score00 = Val::from(0);
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "TJDTMFJDNS CJFDI" {
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "And?"],
                        )?;
                        l_man_score01 = (l_man_score00.clone() + Val::from(1));
                        ctx.next()?;
                    } else {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "And?"])?;
                        l_man_score01 = l_man_score00.clone();
                        ctx.next()?;
                    }
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "TKADLFDMF QKATOS" {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "And?"])?;
                        l_man_score02 = (l_man_score01.clone() + Val::from(1));
                        ctx.next()?;
                    } else {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "And?"])?;
                        l_man_score02 = l_man_score01.clone();
                        ctx.next()?;
                    }
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if (l_input_s.clone() == "EKDTLSDML DLFMADMS.." || l_input_s.clone() == "EKDTLSDML DLFMADMS") {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "This is it?"])?;
                        l_man_score03 = (l_man_score02.clone() + Val::from(1));
                        ctx.next()?;
                    } else {
                        ctx.lines(args![((Val::from("") + l_input_s.clone()) + Val::from("....")), "Is that all?"])?;
                        l_man_score03 = l_man_score02.clone();
                        ctx.next()?;
                    }
                    if l_man_score03.clone().number()? > 2 {
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args![
                                "So, If we put'em together,",
                                "TJDTMFJDNS CJFDI",
                                "TKADLFDMF QKATOS",
                                "EKDTLSDML DLFMADMS..",
                                "It will be like this."
                            ],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as("Linguist Dictionary", args!["Hmm...?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args!["You sure this is exactly what they've said?", "Hmm...I don't get the idea..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Linguist Dictionary",
                            args!["Would you please go back again and check if it is exactly what they were saying?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Linguist Dictionary", args![".Hmm....Hm..I see..."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["....Ummmm..."])?;
                    ctx.next()?;
                    ctx.mes("- Dictionary stared at the paper intently. -")?;
                    ctx.var("ep13_2_rhea").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Linguist Dictionary", args!["According to the Expedition Camp's investigation, there is a different ethnic race that is similar to fairy's, along with a different ethnic race that is similar to giants."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["It is said that they are using a language that cannot be understood by people from Midgard... ...Although we all say this, nobody is able to remember their conversation / words."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["A New language!!! As a linguist, I cannot sit here. But outside, there are many different monsters, as a scholar, it is too dangerous to go outside."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["and as our conversation has reached this point, you should have an idea of my request, right?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["The Tripple Alliance Research Group is very interested in other intelligent life. In order to try and talk to them, national officials have carried out secret effort in every possible way. However, because of lacking information, they are facing difficulties."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["If you've encountered those races around the Expedition Camp, you must memorize their conversation! Then report back to me!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("ep13_2_rhea").get()? == 3 {
                    ctx.lines_as("Linguist Dictionary", args!["............................"])?;
                    ctx.next()?;
                    ctx.mes("- Dictionary stared at the paper intently. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("He looks serious, Let's leave him alone:Talk to him")])? {
                        1 => {
                            ctx.lines_as("Linguist Dictionary", args!["............................"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {}
                        _ => {}
                    }
                    ctx.lines_as("Linguist Dictionary", args!["Ahhhhhhh!!!!!!!!"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Have you figured something out?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args![
                            "I....CAN'T! UNDERSTAND IT!!!",
                            "Whaaaat~~iiis~~ thiiis~~~ Just how do you read thisssssss~~~ I can't understand a single rule in this~~~~ Huh?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args![".................................."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["Hmph!", "Yes?"])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("He looks serious. Let's leave him alone.:Ask him what's going on.")],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Linguist Dictionary",
                                args!["............................", "...................Hmph..."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {}
                        _ => {}
                    }
                    ctx.lines_as("Linguist Dictionary", args!["Ah! Well, It's not as easy as I thought. These languages can't be heard anywhere on the Midgard Continent. No known phonetic rules seem to apply here."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["It's hard to grasp their rule or system, and before everything else, I don't even know if this pronounciation is right. I've never heard of this kind of pronounciation."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["Ah, right, I heard that a ^0000ffRune-Midgarts Magician^000000 knows a way to conduct research based on a sample of language, would you go and ask him to help me?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["The ^0000ffRune-Midgarts Magician^000000 is wandering inside of the Expedition Camp."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["It's difficult to continue on a research without some more samples of language. Please do my favor."],
                    )?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(8244)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(8245)])?;
                    ctx.var("ep13_2_rhea").set(Val::from(4))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ep13_2_rhea").get()?.number()? > 3 && ctx.var("ep13_2_rhea").get()?.number()? < 6) {
                    ctx.lines_as("Linguist Dictionary", args!["I heard that ^0000ffRune-Midgarts Magician^000000 knows a way to conduct a research from a sample of language, would you go see him and ask him to help me?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["The ^0000ff Rune-Midgarts Magician^000000 is wandering inside of the Expedition Camp."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["It's difficult to continue this research without some more samples of language. Please do my favor."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_2_rhea").get()? == 6 {
                    ctx.mes("- The Linguist is writing on a paper as if he's obsessed by something -")?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["...Oh, Hello there."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["I heard that you brought us a Gem with the other world's language recorded within it. Thanks to your help, decoding process is going well.", "Actually, it's going TOO well..."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["It's a wonder that at the moment when I heard that language first-hand I could decode it flawlessly in spite of myself. It must be the blessing of Goddess Freya."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["But the problem is...", "How are we going to put together this tremendous data and make it into a portable translator... I just can't figure out how."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["I wonder what the Mechanic Engineer from the Schwarzwald Republic would think... Would you be kind to go ask him?"])?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(8247)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(8248)])?;
                    ctx.var("ep13_2_rhea").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ep13_2_rhea").get()?.number()? > 6 && ctx.var("ep13_2_rhea").get()?.number()? < 13) {
                    ctx.lines_as("Linguist Dictionary", args!["Just how are we going to put together these tremendous data and make it into a portable translator...I just can't figure out how."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["I wonder what the Mechanic Engineer from the Republic of Schwarzwald would think...Would you be kind to go ask him?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_2_rhea").get()?.number()? > 12 {
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args![
                            "So, Finally we were able to make a translator!",
                            "How is it working? Isn't there any problem with it?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["I hope this translator would help you with your journey."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["However, Although using a translator is important, the basics of language and communication is that you should try to understand others."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["You should not just rely everything on a mere machine...don't forget to be understanding. Enjoy your journey in the otherworld with that translator."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args![
                            "So, Finally we were able to make a translator!",
                            "How is it working? Isn't there any problem with it?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Linguist Dictionary",
                        args!["I hope this translator would help you with your journey."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["However, Although using a translator is important, the basics of language and communication is that you should try to understand others."])?;
                    ctx.next()?;
                    ctx.lines_as("Linguist Dictionary", args!["You should not just rely everything on a mere machine...don't forget to be understanding. Enjoy your journey in the otherworld with that translator."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn arunafeltz_linguist_1(ctx: &Ctx) -> Script {
    arunafeltz_linguist_1_body(ctx, Vec::new()).map(|_| ())
}

fn rune_midgarts_magician_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_check_manjew00 = Val::from(0);
    let mut l_check_spljew00 = Val::from(0);
    if ctx.var("ep13_2_rhea").get()?.number()? < 4 {
        ctx.lines_as(
            "Magician Whisper",
            args!["To store a tremendous amount of information, well, it's impossible with existing technology."],
        )?;
        ctx.next()?;
        ctx.lines_as("Magician Whisper", args!["Ancient Rune-Midgarts Kingdom used a some kind of magic onto a magical gem to store large information...Using a gem as a storage..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Magician Whisper",
            args!["Oh, pardon me, you have such a comfortable atmosphere and I involuntarily spoke to you. Who might you be?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ep13_2_rhea").get()? == 4 {
            ctx.lines_as(
                "Magician Whisper",
                args!["To store a tremendous amount of information, well, it's impossible with existing technology."],
            )?;
            ctx.next()?;
            ctx.lines_as("Magician Whisper", args!["The Ancient Rune-Midgarts Kingdom used some kind of magic on a magical gem to store large amounts of information... Using a gem as storage..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["Oh, pardon me, you have such a comfortable atmosphere and I involuntarily spoke to you. Who might you be?"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Uh, Mr. Dictionary sent...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Magician Whisper",
                args!["Oh? You mean that Linguist from the Arunafeltz. And that means we need a Recorded Language Sample."],
            )?;
            ctx.next()?;
            ctx.lines_as("Magician Whisper", args!["Hmmm... Actually, I've buried magical gems which records sound waves with its magical power, near their territory...to gather information about their language."])?;
            ctx.next()?;
            ctx.lines_as("Magician Whisper", args!["But, Assassins who were in charge of burying those gems... They are working on some other mission, so they can't go and retrieve those gems."])?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["Without someone do us a favor and retrieve those gems, there's no way I could give you the sample."],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Then should I come back later?:What if I go and retrieve them?")],
            )? {
                1 => {
                    ctx.lines_as("Magician Whisper", args![".............Sure, you could wait for 100 years..."])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What?"])?;
                    ctx.next()?;
                    ctx.lines_as("Magician Whisper", args!["Ah, No, No, It's nothing."])?;
                    ctx.next()?;
                }
                2 => {}
                _ => {}
            }
            ctx.lines_as(
                "Magician Whisper",
                args![
                    "If you could do that, it will save us whole lot of time and that'd be good for us! Wouldn't it? Don't you think so?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args![
                    "I belive you've met those two alien races out there?",
                    "Assassins buried Magical Gems near their villages without them noticing."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["First Gem is buried under the Mushroom-like structure near Fairy's village."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["And the Second Gem is buried under the tree near Giant's village."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["Bring back those two gems and that would help me greatly."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["Well, I can't force you to do it if you are too busy with other matters..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args!["There's an old saying, The one who's thirsty will dig for a well..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Magician Whisper",
                args![
                    "I almost forgot! You should be careful when you dig for the Gem.",
                    "Assassins installed a trap to protect Gems as they bury them."
                ],
            )?;
            ctx.call(Function::CompleteQuest, vec![Val::from(8245)])?;
            ctx.call(Function::SetQuest, vec![Val::from(8246)])?;
            ctx.var("ep13_2_rhea").set(Val::from(5))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_2_rhea").get()? == 5 {
                l_check_spljew00 = ctx.call(Function::CheckQuest, vec![Val::from(8241)])?;
                l_check_manjew00 = ctx.call(Function::CheckQuest, vec![Val::from(8242)])?;
                if (l_check_spljew00.clone().number()? > 0 && l_check_manjew00.clone().number()? > 0) {
                    if (ctx.call(Function::CountItem, vec![Val::from(7575)])?.number()? > 0
                        && ctx.call(Function::CountItem, vec![Val::from(7576)])?.number()? > 0)
                    {
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["Wow, You are back already!", "Well then, let me see them!"],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["- As you handed over the Gem, -", "- he examined it throughly-"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args![
                                "Hmm...",
                                "There's lots of noises caught in, but I think I could extract a voice in no time."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["I will extract a voice recorded in this gem and send them to the Linguist from Arunafeltz."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Magician Whisper", args!["It looks like there's many kind of convenient machines. Voices I've extracted can be sent directly to the Linguist from Arunafeltz by using this recording machine."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["So I will be ok by on my own. Thank you for your trouble."],
                        )?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(8241)])?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(8242)])?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(8246)])?;
                        ctx.call(Function::SetQuest, vec![Val::from(8247)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7575), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7576), Val::from(1)])?;
                        ctx.var("ep13_2_rhea").set(Val::from(6))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Magician Whisper", args!["Ah, that was quick."])?;
                        ctx.next()?;
                        ctx.lines_as("Magician Whisper", args!["Please give meh the gem. I will extract the voice record inside the gem, then give it to the Linguist from Arunafeltz."])?;
                        ctx.next()?;
                        ctx.lines_as("Magician Whisper", args!["...... But.... where's the gem?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["First Gem is buried under the Mushroom-like structure near Fairy's village."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["And the Second Gem is buried under the tree near Giant's village."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["Bring back those two gems and that would help me greatly."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["Well, I can't force you to do it if you are too busy with other matters..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["There's an old saying, The one who's thirsty will dig for a well..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args![
                            "I almost forgot! You should be careful when you dig for the Gem.",
                            "Assassins installed a trap to protect Gems as they bury them."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if (ctx.var("ep13_2_rhea").get()?.number()? > 5 && ctx.var("ep13_2_rhea").get()?.number()? < 8) {
                    ctx.lines_as("Magician Whisper", args!["Well, if you are not busy at the time, please go check how's the Linguist from Arunafeltz's research is coming up."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["I should be working on extracting voice data and send them to the Linguist from Arunafeltz."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_2_rhea").get()? == 8 {
                    ctx.lines_as("Magician Whisper", args!["Hmm...There's more data than we thought..."])?;
                    ctx.next()?;
                    ctx.lines_as("Magician Whisper", args!["Of course it is a piece of cake to turn data into magical power and put'em into a gem...Most important thing is, I don't know if there's a jewl that has enough storage space to store that much of data..."])?;
                    ctx.next()?;
                    ctx.lines_as("Magician Whisper", args!["I heard that cold side of a field outside of the Expedition Camp, ^0000ff Mysterious Ore that can't be found in the Midgard."])?;
                    ctx.next()?;
                    ctx.lines_as("Magician Whisper", args!["Only if we could find that ore, it would make this work much easier... What do you think about that? Do you think you could go and find one for me?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Magician Whisper",
                        args!["While you are gone, I will organize the data to put into the Gem."],
                    )?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(8249)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(8250)])?;
                    ctx.var("ep13_2_rhea").set(Val::from(9))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_2_rhea").get()? == 9 {
                    if ctx.call(Function::CountItem, vec![Val::from(6048)])?.number()? > 0 {
                        ctx.lines_as("Magician Whisper", args!["Oh! Is this that ore?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["This sure is interesting. It has not been refined yet, but I can sense mysterious power from this ore."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["It's something like... the special wave with a life force."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["Let's test and see see how much magical power it could store."],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- He closed his eyes and chanted a spell -",
                            "- with the ore on his hand. -"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["Lumos Nox Densaugeo Dissendium Diffindo Engorgio Mobiliarbus Expecto patronum!!!!"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                        ctx.lines_as("Magician Whisper", args!["Wow!!! Th, This is unbelievable!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["An ore with this much storage space, only a size of fingernail could store both of two languages!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args![
                                "Could you wait for a minute?",
                                "Take some rest while I craft this ore into a gem and put the magical data into it. You look exhausted..."
                            ],
                        )?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(8250)])?;
                        ctx.call(Function::SetQuest, vec![Val::from(8251)])?;
                        ctx.call(Function::DelItem, vec![Val::from(6048), Val::from(1)])?;
                        ctx.var("ep13_2_rhea").set(Val::from(10))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Magician Whisper", args!["I heard that the cold side of the field outside of the Expedition Camp has a Unidentified Mineral that can't be found in Midgard."])?;
                        ctx.next()?;
                        ctx.lines_as("Magician Whisper", args!["Only if we could find that ore, it would make this work much easier... What do you think about that? Do you think you could go and find one for me?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Magician Whisper",
                            args!["While you are gone, I will organize the data to put into the Gem."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_2_rhea").get()? == 10 {
                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 3 {
                            ctx.lines(args!["- His face turned a bit pale, -", "- He looked pleased as he saw you -"])?;
                            ctx.next()?;
                            ctx.lines_as("Magician Whisper", args!["Ah! Just in time!"])?;
                            ctx.next()?;
                            ctx.lines_as("Magician Whisper", args!["I've just finished infusing the magical power.", "From now on, it's up to the Mechanic Engineer from Schwarzwald to combined this gem with a machine to make it work as a real-time translator."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magician Whisper",
                                args![
                                    "I will ask you one last favor.",
                                    "Please deliver this crafted gem to the Mechanic Engineer from Schwarzwald."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Magician Whisper", args!["I'm really sorry to ask you so many favors..., Ah...I've used up too much magical power...I should rest for a while."])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "- ^0000ff Received the Crafted Gem ^000000 -",
                                "- ^0000ff from Whisper !!^000000 -"
                            ])?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(8251)])?;
                            ctx.call(Function::SetQuest, vec![Val::from(8252)])?;
                            ctx.var("ep13_2_rhea").set(Val::from(11))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as("Magician Whisper", args!["Could you wait for a minute?", "Take some rest while I craft this ore into a gem and put the magical data into it. You look exhausted..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("ep13_2_rhea").get()? == 11 {
                            ctx.lines_as(
                                "Magician Whisper",
                                args!["Please deliver this crafted gem to the Mechanic Engineer from Schwarzwald."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Magician Whisper", args!["I'm really sorry to ask you so many favors..., Ah...I've used up too much magical power...I should rest for a while."])?;
                            ctx.next()?;
                            ctx.lines(args!["- He looks so sleepy and said -", "- as he rubbing his closing eyes. -"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ep13_2_rhea").get()? == 12 {
                            ctx.lines_as("Magician Whisper", args!["How's the translator making process going?"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ep13_2_rhea").get()?.number()? > 12 {
                            ctx.lines_as(
                                "Magician Whisper",
                                args!["I've just received a message that the translator is finished!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magician Whisper",
                                args!["We are short on the amount just yet, still it will make our advance to the other world easier!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Magician Whisper",
                                args!["I've just received a message that the translator is finished!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Magician Whisper",
                                args!["We are short on the amount just yet, still it will make our advance to the other world easier!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
}

pub fn rune_midgarts_magician_1(ctx: &Ctx) -> Script {
    rune_midgarts_magician_1_body(ctx, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_1(ctx: &Ctx) -> Script {
    half_buried_gem_1_run(ctx, HalfBuriedGem1Step::Start, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_1_oninit(ctx: &Ctx) -> Script {
    half_buried_gem_1_run(ctx, HalfBuriedGem1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_1_onenable(ctx: &Ctx) -> Script {
    half_buried_gem_1_run(ctx, HalfBuriedGem1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_1_ondisable(ctx: &Ctx) -> Script {
    half_buried_gem_1_run(ctx, HalfBuriedGem1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_1_onmymobdead(ctx: &Ctx) -> Script {
    half_buried_gem_1_run(ctx, HalfBuriedGem1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_1_ontimer300000(ctx: &Ctx) -> Script {
    half_buried_gem_1_run(ctx, HalfBuriedGem1Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_2(ctx: &Ctx) -> Script {
    half_buried_gem_2_run(ctx, HalfBuriedGem2Step::Start, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_2_oninit(ctx: &Ctx) -> Script {
    half_buried_gem_2_run(ctx, HalfBuriedGem2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_2_onenable(ctx: &Ctx) -> Script {
    half_buried_gem_2_run(ctx, HalfBuriedGem2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_2_ondisable(ctx: &Ctx) -> Script {
    half_buried_gem_2_run(ctx, HalfBuriedGem2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_2_onmymobdead(ctx: &Ctx) -> Script {
    half_buried_gem_2_run(ctx, HalfBuriedGem2Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn half_buried_gem_2_ontimer300000(ctx: &Ctx) -> Script {
    half_buried_gem_2_run(ctx, HalfBuriedGem2Step::OnTimer300000, Vec::new()).map(|_| ())
}

fn translator_preparation_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "EP13 Translator Quest Preparation",
        args!["First, you must confirm that you are my master."],
    )?;
    ctx.next()?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])?;
    if l_i.clone() == -1 {
        ctx.lines_as("EP 13 Translator Quest Preparation", args!["Cancelled"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == 0 {
        ctx.lines_as("EP 13 Translator Quest Preparation", args!["Try again."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "EP 13 Translator Quest Preparation",
            args!["I'm the NPC who gives the Episode 13.2 Translator Quest's Linked-Quest Items."],
        )?;
        ctx.next()?;
        ctx.mes("What would you like to do?")?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Receive EP13.1 Quest:Receive Quest Window - Meeting with Fairy and Giant",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "EP 13 Translator Quest Preparation",
                    args!["Episode 13.1 - The Report Quest Reward Item Received."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "EP 13 Translator Quest Preparation",
                    args![
                        ((Val::from("Currently the Report Quest's Set Item is ") + ctx.var("ep13_1_edq").get()?) + Val::from(".")),
                        "Do you want to proceed?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No:Yes")])?) == 1 {
                    ctx.lines_as("EP 13 Translator Quest Preparation", args!["Cancelled"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "EP 13 Translator Quest Preparation",
                    args!["Episode 13.1 - The Report Quest Reward Item Received."],
                )?;
                ctx.var("ep13_1_edq").set(Val::from(14))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "EP 13 Translator Quest Preparation",
                    args!["Episode 13.1 - Receiving the Quest window - Meeting with Fairy and Giant."],
                )?;
                ctx.next()?;
                ctx.lines_as("EP 13 Translator Quest Preparation", args!["Do you want to proceed?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No:Yes")])?) == 1 {
                    ctx.lines_as("EP 13 Translator Quest Preparation", args!["Cancelled"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "EP 13 Translator Quest Preparation",
                    args!["Episode 13.1 - The Report Quest Reward Item Received."],
                )?;
                ctx.call(Function::SetQuest, vec![Val::from(2158)])?;
                ctx.call(Function::SetQuest, vec![Val::from(2159)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn translator_preparation_1(ctx: &Ctx) -> Script {
    translator_preparation_1_body(ctx, Vec::new()).map(|_| ())
}
