use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum SlaveLeftStep {
    Start,
    OnTouch,
}

fn slave_left_run(ctx: &Ctx, mut step: SlaveLeftStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            SlaveLeftStep::Start => {
                step = SlaveLeftStep::OnTouch;
                continue 'machine;
            }
            SlaveLeftStep::OnTouch => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        l_map_s.clone(),
                        Val::from("Apostle of Baphomet : Kill the humans! Don't let them interrupt the revival of our Master!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(55),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(51),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(58),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(53),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(54),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(55),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(56),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(58),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(56),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(60),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(59),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(54),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(55),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(56),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(50),
                        Val::from(65),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1867),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(61),
                        Val::from(65),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_left")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn slave_left(ctx: &Ctx) -> Script {
    slave_left_run(ctx, SlaveLeftStep::Start, Vec::new()).map(|_| ())
}

pub fn slave_left_ontouch(ctx: &Ctx) -> Script {
    slave_left_run(ctx, SlaveLeftStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SlaveRightStep {
    Start,
    OnTouch,
}

fn slave_right_run(ctx: &Ctx, mut step: SlaveRightStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            SlaveRightStep::Start => {
                step = SlaveRightStep::OnTouch;
                continue 'machine;
            }
            SlaveRightStep::OnTouch => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        l_map_s.clone(),
                        Val::from("Apostle of Baphomet : Kill the humans! Don't let them interrupt the revival of our Master!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(105),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(104),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(107),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(106),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(102),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(103),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(103),
                        Val::from(67),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(109),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(108),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(101),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(106),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(102),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(104),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(103),
                        Val::from(66),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(109),
                        Val::from(65),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1867),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(108),
                        Val::from(65),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_right")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn slave_right(ctx: &Ctx) -> Script {
    slave_right_run(ctx, SlaveRightStep::Start, Vec::new()).map(|_| ())
}

pub fn slave_right_ontouch(ctx: &Ctx) -> Script {
    slave_right_run(ctx, SlaveRightStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SlaveDownStep {
    Start,
    OnTouch,
}

fn slave_down_run(ctx: &Ctx, mut step: SlaveDownStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            SlaveDownStep::Start => {
                step = SlaveDownStep::OnTouch;
                continue 'machine;
            }
            SlaveDownStep::OnTouch => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        l_map_s.clone(),
                        Val::from("Apostle of Baphomet : Kill the humans! Don't let them interrupt the revival of our Master!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(78),
                        Val::from(41),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(79),
                        Val::from(42),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(78),
                        Val::from(46),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(81),
                        Val::from(41),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(81),
                        Val::from(42),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1869),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(79),
                        Val::from(43),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1291),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(77),
                        Val::from(40),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(79),
                        Val::from(41),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(79),
                        Val::from(42),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(79),
                        Val::from(43),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(79),
                        Val::from(48),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1117),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(78),
                        Val::from(49),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1132),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(78),
                        Val::from(41),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(74),
                        Val::from(42),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(72),
                        Val::from(48),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1867),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(72),
                        Val::from(38),
                        Val::from("Apostle of Baphomet"),
                        Val::from(1292),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_down")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn slave_down(ctx: &Ctx) -> Script {
    slave_down_run(ctx, SlaveDownStep::Start, Vec::new()).map(|_| ())
}

pub fn slave_down_ontouch(ctx: &Ctx) -> Script {
    slave_down_run(ctx, SlaveDownStep::OnTouch, Vec::new()).map(|_| ())
}

fn magical_seal_ss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_seal_check = Val::from(0);
    l_seal_check = ctx.call(Function::CheckQuest, vec![Val::from(3041), ctx.constant("PLAYTIME")?])?;
    if (l_seal_check.clone() == 0 || l_seal_check.clone() == 1) {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SILENCEATTACK")?])?;
        ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
        ctx.call(
            Function::StartStatus,
            vec![ctx.constant("SC_STONE")?, Val::from(30000), Val::from(0)],
        )?;
        ctx.mes("Your SP has not recovered yet. You lost your SP on the altar, but it seems the power of the seal has returned.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if l_seal_check.clone() == 2 {
        ctx.call(Function::EraseQuest, vec![Val::from(3041)])?;
    }
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEXDIVINA")?])?;
    ctx.call(Function::DisableNpc, vec![])?;
    l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?;
    if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "0" {
        ctx.call(
            Function::AreaMobUseSkill,
            vec![
                l_map_s.clone(),
                Val::from(79),
                Val::from(81),
                Val::from(10),
                Val::from(1929),
                Val::from("NPC_INVINCIBLEOFF"),
                Val::from(1),
                Val::from(0),
                Val::from(0),
                ctx.constant("ET_HELP")?,
                Val::from(0),
            ],
        )?;
    } else if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "2" {
        ctx.call(
            Function::AreaMobUseSkill,
            vec![
                l_map_s.clone(),
                Val::from(123),
                Val::from(109),
                Val::from(10),
                Val::from(1929),
                Val::from("NPC_INVINCIBLEOFF"),
                Val::from(1),
                Val::from(0),
                Val::from(0),
                ctx.constant("ET_HELP")?,
                Val::from(0),
            ],
        )?;
    } else if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "4" {
        ctx.call(
            Function::AreaMobUseSkill,
            vec![
                l_map_s.clone(),
                Val::from(123),
                Val::from(22),
                Val::from(10),
                Val::from(1929),
                Val::from("NPC_INVINCIBLEOFF"),
                Val::from(1),
                Val::from(0),
                Val::from(0),
                ctx.constant("ET_HELP")?,
                Val::from(0),
            ],
        )?;
    } else if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "8" {
        ctx.call(
            Function::AreaMobUseSkill,
            vec![
                l_map_s.clone(),
                Val::from(35),
                Val::from(21),
                Val::from(10),
                Val::from(1929),
                Val::from("NPC_INVINCIBLEOFF"),
                Val::from(1),
                Val::from(0),
                Val::from(0),
                ctx.constant("ET_HELP")?,
                Val::from(0),
            ],
        )?;
    } else if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "10" {
        ctx.call(
            Function::AreaMobUseSkill,
            vec![
                l_map_s.clone(),
                Val::from(35),
                Val::from(109),
                Val::from(10),
                Val::from(1929),
                Val::from("NPC_INVINCIBLEOFF"),
                Val::from(1),
                Val::from(0),
                Val::from(0),
                ctx.constant("ET_HELP")?,
                Val::from(0),
            ],
        )?;
    }
    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
    ctx.call(
        Function::StartStatus,
        vec![ctx.constant("SC_STONE")?, Val::from(20000), Val::from(0)],
    )?;
    ctx.call(Function::SetQuest, vec![Val::from(3041)])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            l_map_s.clone(),
            Val::from("The seal activated by putting magical power into the altar."),
            ctx.constant("BC_MAP")?,
            Val::from("0x87ceeb"),
        ],
    )?;
    ctx.mes("I can feel the power of the altar came back by adding magical power.")?;
    ctx.next()?;
    ctx.mes("But you can't use your magic for 3 minutes because you used your SP on the altar.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn magical_seal_ss(ctx: &Ctx) -> Script {
    magical_seal_ss_body(ctx, Vec::new()).map(|_| ())
}

fn magical_seal_ss_oninstanceinit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn magical_seal_ss_oninstanceinit(ctx: &Ctx) -> Script {
    magical_seal_ss_oninstanceinit_body(ctx, Vec::new()).map(|_| ())
}

fn the_main_altar_ss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("'ins_baphomet").get()? == 5
        && ctx
            .call(
                Function::IsPartyLeader,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
            )?
            .loosely_equals(&Val::from(1)))
    {
        ctx.mes("An evil power, too terrible to describe, lies under the great altar radiating a violet color.")?;
        ctx.next()?;
        ctx.mes("Complicated Magical Rune letters blink rapidly, attempting to suppress the dreadful power within.")?;
        ctx.next()?;
        ctx.mes("The bottom of the Main Altar trembles furiously.")?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_METEORSTORM")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_METEORSTORM")?])?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Watch out! Something... Something is coming."],
        )?;
        ctx.var("'ins_baphomet").set(Val::from(6))?;
        ctx.call(
            Function::DoNpcEvent,
            vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad")])? + Val::from("::OnEnable"))],
        )?;
        ctx.call(
            Function::DisableNpc,
            vec![ctx.call(Function::InstanceNpcName, vec![Val::from("The Main Altar#ss")])?],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("An evil power, too terrible to describe, lies under the great altar radiating a violet color.")?;
        ctx.next()?;
        ctx.mes("Complicated Magical Rune letters blink rapidly, attempting to suppress the dreadful power within.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn the_main_altar_ss(ctx: &Ctx) -> Script {
    the_main_altar_ss_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AncientHeroSSoul2fStep {
    Start,
    OnInstanceInit,
}

fn ancient_hero_s_soul_2f_run(ctx: &Ctx, mut step: AncientHeroSSoul2fStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AncientHeroSSoul2fStep::Start => {
                ctx.call(Function::Cutin, vec![Val::from("ins_cata_champ_s"), Val::from(2)])?;
                if ctx.call(Function::CheckQuest, vec![Val::from(3041)])?.number()? >= 0 {
                    ctx.call(Function::EraseQuest, vec![Val::from(3041)])?;
                }
                ctx.lines_as(
                    "Ancient Hero's Soul",
                    args![
                        "Good job, my descendants... You've finished the long-cherished task that me and my bretheren could not complete."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ancient Hero's Soul",
                    args!["I really appreciate your help.", "Our souls can finally rest in peace..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Ancient Hero's Soul", args!["The struggle for peace on this world will never end. But... my role here is finally over because there are brave heroes like you."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Let me leave the shrine.:Stop talking.")])? {
                    1 => {
                        ctx.lines_as(
                            "Ancient Hero's Soul",
                            args!["Okay. I'll let you and your group leave here safely."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ancient Hero's Soul",
                            args!["If you leave, please say hello to Patrick for me."],
                        )?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.var("'ins_baphomet").set(Val::from(0))?;
                        ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(310), Val::from(150)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Ancient Hero's Soul",
                            args!["Do you still have something to do here? If you're done I'll let you leave safely..."],
                        )?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = AncientHeroSSoul2fStep::OnInstanceInit;
                continue 'machine;
            }
            AncientHeroSSoul2fStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Ancient Hero's Soul#2F")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ancient_hero_s_soul_2f(ctx: &Ctx) -> Script {
    ancient_hero_s_soul_2f_run(ctx, AncientHeroSSoul2fStep::Start, Vec::new()).map(|_| ())
}

pub fn ancient_hero_s_soul_2f_oninstanceinit(ctx: &Ctx) -> Script {
    ancient_hero_s_soul_2f_run(ctx, AncientHeroSSoul2fStep::OnInstanceInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ins2fHeroBroadStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer3000,
    OnTimer6000,
    OnTimer9000,
    OnTimer12000,
    OnTimer15000,
    OnTimer17000,
}

fn ins_2f_hero_broad_run(ctx: &Ctx, mut step: Ins2fHeroBroadStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ins2fHeroBroadStep::Start => {
                step = Ins2fHeroBroadStep::OnEnable;
                continue 'machine;
            }
            Ins2fHeroBroadStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad")])?],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Ancient Hero's Soul : My God! The seal of the Main Altar is weakening!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Ancient Hero's Soul : My descendants... Listen carefully to what I'm going to say."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnTimer9000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : The altars that control the Main Altar's power are located in the Northeast, Southeast, Southwest and Northwest corners of this room."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Ancient Hero's Soul : Find these altars and activate their seals before Baphomet revives."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Baphomet : It's too late, weaklings... Now, you'll feel the despair of death!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroadStep::OnTimer17000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Baphomet : No one can harm me here. You will be my first sacrifice."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad2")])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_2f_hero_broad(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_onenable(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ondisable(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ontimer3000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ontimer6000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ontimer9000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ontimer12000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ontimer15000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad_ontimer17000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad_run(ctx, Ins2fHeroBroadStep::OnTimer17000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ControlBaphometStep {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
    OnMyMobDead,
}

fn control_baphomet_run(ctx: &Ctx, mut step: ControlBaphometStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            ControlBaphometStep::Start => {
                step = ControlBaphometStep::OnInstanceInit;
                continue 'machine;
            }
            ControlBaphometStep::OnInstanceInit => {
                step = ControlBaphometStep::OnDisable;
                continue 'machine;
            }
            ControlBaphometStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])?],
                )?;
                return Err(Stop::End);
            }
            ControlBaphometStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])?],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from(79),
                        Val::from(64),
                        Val::from("Baphomet#"),
                        Val::from(1929),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            ControlBaphometStep::OnMyMobDead => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?;
                if ctx
                    .call(
                        Function::MobCount,
                        vec![
                            l_map_s.clone(),
                            (ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])? + Val::from("::OnMyMobDead")),
                        ],
                    )?
                    .number()?
                    < 1
                {
                    ctx.var("'ins_baphomet").set(Val::from(7))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(3041)])?.is_true() {
                        ctx.call(Function::EraseQuest, vec![Val::from(3041)])?;
                    }
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Baphomet : No! Nonono! How dare these weaklings defeat me!... No!!..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xdb7093"),
                        ],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Ancient Hero's Soul#2F")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_down")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_left")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_right")])?],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad2")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_enter_broad")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("control_baphomet")])?],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn control_baphomet(ctx: &Ctx) -> Script {
    control_baphomet_run(ctx, ControlBaphometStep::Start, Vec::new()).map(|_| ())
}

pub fn control_baphomet_oninstanceinit(ctx: &Ctx) -> Script {
    control_baphomet_run(ctx, ControlBaphometStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn control_baphomet_ondisable(ctx: &Ctx) -> Script {
    control_baphomet_run(ctx, ControlBaphometStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn control_baphomet_onenable(ctx: &Ctx) -> Script {
    control_baphomet_run(ctx, ControlBaphometStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn control_baphomet_onmymobdead(ctx: &Ctx) -> Script {
    control_baphomet_run(ctx, ControlBaphometStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn ins_2f_hero_broad2_run(ctx: &Ctx, mut step: Ins2fHeroBroad2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ins2fHeroBroad2Step::Start => {
                step = Ins2fHeroBroad2Step::OnInstanceInit;
                continue 'machine;
            }
            Ins2fHeroBroad2Step::OnInstanceInit => {
                step = Ins2fHeroBroad2Step::OnDisable;
                continue 'machine;
            }
            Ins2fHeroBroad2Step::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad2")])?],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad2")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Ancient Hero's Soul : Don't be discouraged, Baphomet can still be defeated!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Ancient Hero's Soul : Go to the altars and activate their seals."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer13000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Ancient Hero's Soul : Once the seals recover their power, Baphomet will be vulnerable."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer16000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : You should lure Baphomet to the unsealed Altars. Otherwise, your efforts will be futile."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer19000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : We have only 1 hour to stop Baphomet. If time runs out, the power of the seals will be useless."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer22000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Baphomet : It's useless. Make more seals. I'll crush them all. None of you will survive!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroBroad2Step::OnTimer26000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : The magical power of the central seal is running out. Go to the central seal and put the magical power."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])? + Val::from("::OnEnable"))],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_broad2")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_2f_hero_broad2(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::Start, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_oninstanceinit(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ondisable(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_onenable(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer8000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer11000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer13000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer16000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer19000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer19000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer22000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer22000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_broad2_ontimer26000(ctx: &Ctx) -> Script {
    ins_2f_hero_broad2_run(ctx, Ins2fHeroBroad2Step::OnTimer26000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S2fCallmonPatternCStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    Ongo,
    OnTimer3600000,
}

fn s_2f_callmon_pattern_c_run(ctx: &Ctx, mut step: S2fCallmonPatternCStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S2fCallmonPatternCStep::Start => {
                step = S2fCallmonPatternCStep::OnInstanceInit;
                continue 'machine;
            }
            S2fCallmonPatternCStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])?],
                )?;
                return Err(Stop::End);
            }
            S2fCallmonPatternCStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])? + Val::from("::Ongo"))],
                )?;
                return Err(Stop::End);
            }
            S2fCallmonPatternCStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])?],
                )?;
                return Err(Stop::End);
            }
            S2fCallmonPatternCStep::Ongo => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern")])? + Val::from("::OnEnable"))],
                )?;
                return Err(Stop::End);
            }
            S2fCallmonPatternCStep::OnTimer3600000 => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])? + Val::from("::OnDisable"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_2f_callmon_pattern_c(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_c_run(ctx, S2fCallmonPatternCStep::Start, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_c_oninstanceinit(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_c_run(ctx, S2fCallmonPatternCStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_c_onenable(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_c_run(ctx, S2fCallmonPatternCStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_c_ondisable(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_c_run(ctx, S2fCallmonPatternCStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_c_ongo(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_c_run(ctx, S2fCallmonPatternCStep::Ongo, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_c_ontimer3600000(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_c_run(ctx, S2fCallmonPatternCStep::OnTimer3600000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S2fCallmonPatternStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTimer300000,
}

fn s_2f_callmon_pattern_run(ctx: &Ctx, mut step: S2fCallmonPatternStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S2fCallmonPatternStep::Start => {
                step = S2fCallmonPatternStep::OnInstanceInit;
                continue 'machine;
            }
            S2fCallmonPatternStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern")])?],
                )?;
                return Err(Stop::End);
            }
            S2fCallmonPatternStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern")])?],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S2fCallmonPatternStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern")])?],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S2fCallmonPatternStep::OnTimer300000 => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_down")])?],
                )?;
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_left")])?],
                )?;
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("slave_right")])?],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("2f_callmon_pattern_c")])? + Val::from("::Ongo"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_2f_callmon_pattern(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_run(ctx, S2fCallmonPatternStep::Start, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_oninstanceinit(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_run(ctx, S2fCallmonPatternStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_onenable(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_run(ctx, S2fCallmonPatternStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_ondisable(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_run(ctx, S2fCallmonPatternStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn s_2f_callmon_pattern_ontimer300000(ctx: &Ctx) -> Script {
    s_2f_callmon_pattern_run(ctx, S2fCallmonPatternStep::OnTimer300000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ins2fHeroPatternCStep {
    Start,
    OnInstanceInit,
    OnEnable,
    Ongo,
    OnDisable,
    OnTimer3600000,
    OnTimer3605000,
}

fn ins_2f_hero_pattern_c_run(ctx: &Ctx, mut step: Ins2fHeroPatternCStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ins2fHeroPatternCStep::Start => {
                step = Ins2fHeroPatternCStep::OnInstanceInit;
                continue 'machine;
            }
            Ins2fHeroPatternCStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])?],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternCStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])? + Val::from("::Ongo"))],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternCStep::Ongo => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern")])? + Val::from("::OnEnable"))],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternCStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern")])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])?],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternCStep::OnTimer3600000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                        Val::from("Baphomet : krrrr... Now you can't stop me with the seals. All you can do is wait for death!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternCStep::OnTimer3605000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : We can't stop Baphomet with the magical power of the seals anymore. Now everything depends on God..."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])? + Val::from("::OnDisable"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_2f_hero_pattern_c(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_c_oninstanceinit(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_c_onenable(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_c_ongo(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::Ongo, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_c_ondisable(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_c_ontimer3600000(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::OnTimer3600000, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_c_ontimer3605000(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_c_run(ctx, Ins2fHeroPatternCStep::OnTimer3605000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ins2fHeroPatternStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTimer70000,
}

fn ins_2f_hero_pattern_run(ctx: &Ctx, mut step: Ins2fHeroPatternStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ins2fHeroPatternStep::Start => {
                step = Ins2fHeroPatternStep::OnInstanceInit;
                continue 'machine;
            }
            Ins2fHeroPatternStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern")])?],
                )?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern")])?],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ins2fHeroPatternStep::OnTimer70000 => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?,
                            Val::from("Ancient Hero's Soul : The seal of the Main Altar is running out. Strengthen the Main Altar's seal!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFFFF00"),
                        ],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                    )?;
                } else if subject1 == 2 {
                    ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : The magical power of the seal at 2 o'clock is running out. Go to 2 o'clock and put the magical power in the seal."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                    )?;
                } else if subject1 == 3 {
                    ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : The magical power of the seal at 4 o'clock is running out. Go to 4 o'clock and put the magical power in the seal."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                    )?;
                } else if subject1 == 4 {
                    ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : The magical power of the seal at 8 o'clock is running out. Go to 8 o'clock and put the magical power in the seal."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                    )?;
                } else if subject1 == 5 {
                    ctx.call(Function::MapAnnounce, vec![ctx.call(Function::InstanceMapName, vec![Val::from("2@cata")])?, Val::from("Ancient Hero's Soul : The magical power of the seal at 10 o'clock is running out. Go to 10 o'clock and put the magical power in the seal."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#0")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#2")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#4")])?],
                    )?;
                    ctx.call(
                        Function::DisableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#8")])?],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Magical Seal#10")])?],
                    )?;
                }
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_2f_hero_pattern_c")])? + Val::from("::Ongo"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_2f_hero_pattern(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_run(ctx, Ins2fHeroPatternStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_oninstanceinit(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_run(ctx, Ins2fHeroPatternStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_onenable(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_run(ctx, Ins2fHeroPatternStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_ondisable(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_run(ctx, Ins2fHeroPatternStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ins_2f_hero_pattern_ontimer70000(ctx: &Ctx) -> Script {
    ins_2f_hero_pattern_run(ctx, Ins2fHeroPatternStep::OnTimer70000, Vec::new()).map(|_| ())
}
