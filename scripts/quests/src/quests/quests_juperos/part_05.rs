use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Monster221Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer2000,
    OnTimer4000,
    OnTimer7000,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster2_2_1_run(ctx: &Ctx, mut step: Monster221Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster221Step::Start => {
                step = Monster221Step::OnInit;
                continue 'machine;
            }
            Monster221Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-1")])?;
                return Err(Stop::End);
            }
            Monster221Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-1")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster2#2-1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster221Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#2-1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Monster221Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("How about now?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area2")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area2")],
                )?;
                return Err(Stop::End);
            }
            Monster221Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Let me see..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster221Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Just how strong you are!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area2")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area2")],
                )?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(126),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(127),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(128),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(129),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(130),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(131),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(132),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(133),
                        Val::from(236),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-1::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster221Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Bwahaha! You're only good at running away!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster221Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-1::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster221Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area2"),
                            Val::from("Zzzzt. Zzzzt..... "),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-1")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-1")])?;
                    ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_2_1(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_oninit(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_ondisable(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_onenable(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_ontimer2000(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_ontimer4000(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_ontimer7000(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_ontimer300000(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_ontimer300002(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_2_1_onmymobdead(ctx: &Ctx) -> Script {
    monster2_2_1_run(ctx, Monster221Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn hole_2_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("2"), Val::from(2)])?;
    if ctx.var("$@juprearea2inuse").get()? == 1 {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("2"), Val::from(255)])?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7359)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7357)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7358)])?.number()? > 0)
    {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Insert a Crest Piece.:Cancel.")])? {
            1 => {
                if ctx.call(Function::CountItem, vec![Val::from(7357)])?.number()? > 0 {
                    ctx.lines(args![
                        "^3355FFYou take out your",
                        "Crest Piece and place",
                        "it into the slot where it",
                        "happens to fit perfectly.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TOPRANK")?])?;
                    ctx.call(Function::Cutin, vec![Val::from("2-1"), Val::from(2)])?;
                    ctx.next()?;
                    if ctx.var("$@juprearea2inuse").get()? == 1 {
                        ctx.lines(args![
                            "^3355FFNothing happens.",
                            "Perhaps an alarm or",
                            "some other safety measure",
                            "was activated to keep the",
                            "Crest Piece from activating",
                            "this transportation device.",
                            "You retrieve the Crest Piece.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("2-1"), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "^3355FFThe slot rotates and",
                            "the Crest Piece moves as",
                            "if it were turning a key. You",
                            "feel a weak tremor as a Warp",
                            "Portal to the other side is",
                            "activated. You then retrieve",
                            "your Crest Piece.^000000"
                        ])?;
                        ctx.call(Function::InitNpcTimer, vec![])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#2-2::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#2-2")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-2")])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("2-1"), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines(args![
                        "^3355FFUnfortunately, you're",
                        "not carrying anything",
                        "that might be able to fit",
                        "into the slot and activate",
                        "this mechanical device.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("2"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hmmm...", "Do I have anything", "that might make this", "weird machine work?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("2"), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("2"), Val::from(255)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn hole_2_2(ctx: &Ctx) -> Script {
    hole_2_2_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_2_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_2_2_onstop_timer(ctx: &Ctx) -> Script {
    hole_2_2_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_2_ontimer22500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-2")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-2")])?;
    return Err(Stop::End);
}

pub fn hole_2_2_ontimer22500(ctx: &Ctx) -> Script {
    hole_2_2_ontimer22500_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp22Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer22500,
}

fn warp_2_2_run(ctx: &Ctx, mut step: Warp22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp22Step::Start => {
                step = Warp22Step::OnInit;
                continue 'machine;
            }
            Warp22Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-2")])?;
                return Err(Stop::End);
            }
            Warp22Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#2-2")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp22Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area2"), Val::from(142), Val::from(191)])?;
                return Err(Stop::End);
            }
            Warp22Step::OnTimer22500 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_2_2(ctx: &Ctx) -> Script {
    warp_2_2_run(ctx, Warp22Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_2_2_oninit(ctx: &Ctx) -> Script {
    warp_2_2_run(ctx, Warp22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_2_2_onenable(ctx: &Ctx) -> Script {
    warp_2_2_run(ctx, Warp22Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_2_2_ontouch(ctx: &Ctx) -> Script {
    warp_2_2_run(ctx, Warp22Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_2_2_ontimer22500(ctx: &Ctx) -> Script {
    warp_2_2_run(ctx, Warp22Step::OnTimer22500, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm22Step {
    Start,
    OnInit,
    OnTouch,
}

fn red_alarm_2_2_run(ctx: &Ctx, mut step: RedAlarm22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm22Step::Start => {
                step = RedAlarm22Step::OnInit;
                continue 'machine;
            }
            RedAlarm22Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-2")])?;
                return Err(Stop::End);
            }
            RedAlarm22Step::OnTouch => {
                ctx.var("$@juprearea2inuse").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#2-2::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_2_2(ctx: &Ctx) -> Script {
    red_alarm_2_2_run(ctx, RedAlarm22Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_2_oninit(ctx: &Ctx) -> Script {
    red_alarm_2_2_run(ctx, RedAlarm22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_2_ontouch(ctx: &Ctx) -> Script {
    red_alarm_2_2_run(ctx, RedAlarm22Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn22Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_2_2_run(ctx: &Ctx, mut step: RedAlarmOn22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn22Step::Start => {
                step = RedAlarmOn22Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn22Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-2")])?;
                return Err(Stop::End);
            }
            RedAlarmOn22Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#2-2")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn22Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Come on, come on!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn22Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Yes. Run... Right into my hands!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn22Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Do you want to know who I am?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-2::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn22Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("You will know, once you defeat all of my minions!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn22Step::OnTimer8000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_2_2(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_2_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_2_2_run(ctx, RedAlarmOn22Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster122Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_2_2_run(ctx: &Ctx, mut step: Monster122Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster122Step::Start => {
                step = Monster122Step::OnInit;
                continue 'machine;
            }
            Monster122Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-2")])?;
                return Err(Stop::End);
            }
            Monster122Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-2")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster1#2-2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster122Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#2-2")])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(126),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(127),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(128),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(129),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(130),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(131),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(132),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(133),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-2::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster122Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I can't believe how cowardly you really are..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster122Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-2::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster122Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-2::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-2")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_2_2(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_2_2_oninit(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_2_2_ondisable(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_2_2_onenable(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_2_2_ontimer300000(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_2_2_ontimer300002(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_2_2_onmymobdead(ctx: &Ctx) -> Script {
    monster1_2_2_run(ctx, Monster122Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster222Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer2000,
    OnTimer4000,
    OnTimer6000,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster2_2_2_run(ctx: &Ctx, mut step: Monster222Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster222Step::Start => {
                step = Monster222Step::OnInit;
                continue 'machine;
            }
            Monster222Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-2")])?;
                return Err(Stop::End);
            }
            Monster222Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-2")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster2#2-2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster222Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#2-2")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Monster222Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I was the head of this underground laboratory."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster222Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("But that was a long time ago, back when I was merely a human."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster222Step::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I was called Vesper Newton. Hahah, they called me a mad man back then."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var(".mymobs").set(Val::from(13))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(126),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(127),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(128),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(129),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(130),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(131),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(132),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(133),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(133),
                        Val::from(156),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(127),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(129),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(130),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(132),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-2::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster222Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("...Not yet."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster222Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-2::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster222Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area2"),
                            Val::from("Not yet!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-2")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-2")])?;
                    ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_2_2(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_oninit(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_ondisable(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_onenable(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_ontimer2000(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_ontimer4000(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_ontimer6000(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_ontimer300000(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_ontimer300002(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_2_2_onmymobdead(ctx: &Ctx) -> Script {
    monster2_2_2_run(ctx, Monster222Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn hole_2_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("3"), Val::from(2)])?;
    if ctx.var("$@juprearea2inuse").get()? == 1 {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("3"), Val::from(255)])?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7359)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7357)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7358)])?.number()? > 0)
    {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Insert a Crest Piece.:Cancel.")])? {
            1 => {
                if ctx.call(Function::CountItem, vec![Val::from(7358)])?.number()? > 0 {
                    ctx.lines(args![
                        "^3355FFYou take out your",
                        "Crest Piece and place",
                        "it into the slot where it",
                        "happens to fit perfectly.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TOPRANK")?])?;
                    ctx.call(Function::Cutin, vec![Val::from("3-1"), Val::from(2)])?;
                    ctx.next()?;
                    if ctx.var("$@juprearea2inuse").get()? == 1 {
                        ctx.lines(args![
                            "^3355FFNothing happens.",
                            "Perhaps an alarm or",
                            "some other safety measure",
                            "was activated to keep the",
                            "Crest Piece from activating",
                            "this transportation device.",
                            "You retrieve the Crest Piece.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("3-1"), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "^3355FFThe slot rotates and",
                            "the Crest Piece moves as",
                            "if it were turning a key. You",
                            "feel a weak tremor as a Warp",
                            "Portal to the other side is",
                            "activated. You then retrieve",
                            "your Crest Piece.^000000"
                        ])?;
                        ctx.call(Function::InitNpcTimer, vec![])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#2-3::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#2-3")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-3")])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("3-1"), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines(args![
                        "^3355FFUnfortunately, you're",
                        "not carrying anything",
                        "that might be able to fit",
                        "into the slot and activate",
                        "this mechanical device.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("3"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hmmm...", "Do I have anything", "that might make this", "weird machine work?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("3"), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("3"), Val::from(255)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn hole_2_3(ctx: &Ctx) -> Script {
    hole_2_3_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_3_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_2_3_onstop_timer(ctx: &Ctx) -> Script {
    hole_2_3_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_3_ontimer22500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-3")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-3")])?;
    return Err(Stop::End);
}

pub fn hole_2_3_ontimer22500(ctx: &Ctx) -> Script {
    hole_2_3_ontimer22500_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp23Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer22500,
}

fn warp_2_3_run(ctx: &Ctx, mut step: Warp23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp23Step::Start => {
                step = Warp23Step::OnInit;
                continue 'machine;
            }
            Warp23Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-3")])?;
                return Err(Stop::End);
            }
            Warp23Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#2-3")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp23Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area2"), Val::from(130), Val::from(105)])?;
                return Err(Stop::End);
            }
            Warp23Step::OnTimer22500 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_2_3(ctx: &Ctx) -> Script {
    warp_2_3_run(ctx, Warp23Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_2_3_oninit(ctx: &Ctx) -> Script {
    warp_2_3_run(ctx, Warp23Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_2_3_onenable(ctx: &Ctx) -> Script {
    warp_2_3_run(ctx, Warp23Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_2_3_ontouch(ctx: &Ctx) -> Script {
    warp_2_3_run(ctx, Warp23Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_2_3_ontimer22500(ctx: &Ctx) -> Script {
    warp_2_3_run(ctx, Warp23Step::OnTimer22500, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm23Step {
    Start,
    OnInit,
    OnTouch,
}

fn red_alarm_2_3_run(ctx: &Ctx, mut step: RedAlarm23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm23Step::Start => {
                step = RedAlarm23Step::OnInit;
                continue 'machine;
            }
            RedAlarm23Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-3")])?;
                return Err(Stop::End);
            }
            RedAlarm23Step::OnTouch => {
                ctx.var("$@juprearea2inuse").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#2-3::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_2_3(ctx: &Ctx) -> Script {
    red_alarm_2_3_run(ctx, RedAlarm23Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_3_oninit(ctx: &Ctx) -> Script {
    red_alarm_2_3_run(ctx, RedAlarm23Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_3_ontouch(ctx: &Ctx) -> Script {
    red_alarm_2_3_run(ctx, RedAlarm23Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn23Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_2_3_run(ctx: &Ctx, mut step: RedAlarmOn23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn23Step::Start => {
                step = RedAlarmOn23Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn23Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-3")])?;
                return Err(Stop::End);
            }
            RedAlarmOn23Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#2-3")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn23Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("These security systems..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn23Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("They're not really for protection."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-3")])?;
                return Err(Stop::End);
            }
            RedAlarmOn23Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("It's sort of just a hobby to pass the time..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-3::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn23Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Being immortal, I have a lot of time on my hands..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn23Step::OnTimer8000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_2_3(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_3_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_2_3_run(ctx, RedAlarmOn23Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster123Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_2_3_run(ctx: &Ctx, mut step: Monster123Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster123Step::Start => {
                step = Monster123Step::OnInit;
                continue 'machine;
            }
            Monster123Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-3")])?;
                return Err(Stop::End);
            }
            Monster123Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-3")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster1#2-3::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster123Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#2-3")])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(126),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(127),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(128),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(129),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(130),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(131),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(132),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(133),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-3::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster123Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Zzzzt...Zzzzt...."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster123Step::OnTimer300002 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-3::OnDisable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-3")])?;
                return Err(Stop::End);
            }
            Monster123Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-3::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-3")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_2_3(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_2_3_oninit(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_2_3_ondisable(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_2_3_onenable(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_2_3_ontimer300000(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_2_3_ontimer300002(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_2_3_onmymobdead(ctx: &Ctx) -> Script {
    monster1_2_3_run(ctx, Monster123Step::OnMyMobDead, Vec::new()).map(|_| ())
}
