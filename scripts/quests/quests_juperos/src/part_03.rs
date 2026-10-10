use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Monster211Step {
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

fn monster2_1_1_run(ctx: &Ctx, mut step: Monster211Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster211Step::Start => {
                step = Monster211Step::OnInit;
                continue 'machine;
            }
            Monster211Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-1")])?;
                return Err(Stop::End);
            }
            Monster211Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-1")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster2#1-1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster211Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#1-1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Monster211Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("How about now?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                return Err(Stop::End);
            }
            Monster211Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Let me see..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster211Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Just how strong you are!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(31),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(32),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(33),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(34),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(35),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(36),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(37),
                        Val::from(238),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-1::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster211Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Bwahaha! You're only good at running away!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster211Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-1::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster211Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area1"),
                            Val::from("Zzzzt. Zzzzt..... "),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-1")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-1")])?;
                    ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_1_1(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_oninit(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_ondisable(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_onenable(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_ontimer2000(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_ontimer4000(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_ontimer7000(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_ontimer300000(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_ontimer300002(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_1_1_onmymobdead(ctx: &Ctx) -> Script {
    monster2_1_1_run(ctx, Monster211Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn hole_1_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("2"), Val::from(2)])?;
    if ctx.var("$@juprearea1inuse").get()? == 1 {
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
                    if ctx.var("$@juprearea1inuse").get()? == 1 {
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
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#1-2::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#1-2")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-2")])?;
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

pub fn hole_1_2(ctx: &Ctx) -> Script {
    hole_1_2_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_2_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_1_2_onstop_timer(ctx: &Ctx) -> Script {
    hole_1_2_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_2_ontimer22500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-2")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-2")])?;
    return Err(Stop::End);
}

pub fn hole_1_2_ontimer22500(ctx: &Ctx) -> Script {
    hole_1_2_ontimer22500_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp12Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer22500,
}

fn warp_1_2_run(ctx: &Ctx, mut step: Warp12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp12Step::Start => {
                step = Warp12Step::OnInit;
                continue 'machine;
            }
            Warp12Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-2")])?;
                return Err(Stop::End);
            }
            Warp12Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#1-2")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp12Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area1"), Val::from(21), Val::from(191)])?;
                return Err(Stop::End);
            }
            Warp12Step::OnTimer22500 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_1_2(ctx: &Ctx) -> Script {
    warp_1_2_run(ctx, Warp12Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_1_2_oninit(ctx: &Ctx) -> Script {
    warp_1_2_run(ctx, Warp12Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_1_2_onenable(ctx: &Ctx) -> Script {
    warp_1_2_run(ctx, Warp12Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_1_2_ontouch(ctx: &Ctx) -> Script {
    warp_1_2_run(ctx, Warp12Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_1_2_ontimer22500(ctx: &Ctx) -> Script {
    warp_1_2_run(ctx, Warp12Step::OnTimer22500, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm12Step {
    Start,
    OnInit,
    OnTouch,
}

fn red_alarm_1_2_run(ctx: &Ctx, mut step: RedAlarm12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm12Step::Start => {
                step = RedAlarm12Step::OnInit;
                continue 'machine;
            }
            RedAlarm12Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-2")])?;
                return Err(Stop::End);
            }
            RedAlarm12Step::OnTouch => {
                ctx.var("$@juprearea1inuse").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#1-2::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_1_2(ctx: &Ctx) -> Script {
    red_alarm_1_2_run(ctx, RedAlarm12Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_2_oninit(ctx: &Ctx) -> Script {
    red_alarm_1_2_run(ctx, RedAlarm12Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_2_ontouch(ctx: &Ctx) -> Script {
    red_alarm_1_2_run(ctx, RedAlarm12Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn12Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_1_2_run(ctx: &Ctx, mut step: RedAlarmOn12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn12Step::Start => {
                step = RedAlarmOn12Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn12Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-2")])?;
                return Err(Stop::End);
            }
            RedAlarmOn12Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#1-2")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn12Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Come on, come on!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn12Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Yes. Run... Right into my hands!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn12Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Do you want to know who I am?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-2::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn12Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("You will know, once you defeat all of my minions!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn12Step::OnTimer8000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_1_2(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_2_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_1_2_run(ctx, RedAlarmOn12Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster112Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_1_2_run(ctx: &Ctx, mut step: Monster112Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster112Step::Start => {
                step = Monster112Step::OnInit;
                continue 'machine;
            }
            Monster112Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-2")])?;
                return Err(Stop::End);
            }
            Monster112Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-2")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster1#1-2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster112Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#1-2")])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(31),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(32),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(33),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(34),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(35),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(36),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(37),
                        Val::from(176),
                        Val::from("High Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-2::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster112Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I can't believe how cowardly you really are..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster112Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-2::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster112Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-2::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-2")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_1_2(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_1_2_oninit(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_1_2_ondisable(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_1_2_onenable(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_1_2_ontimer300000(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_1_2_ontimer300002(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_1_2_onmymobdead(ctx: &Ctx) -> Script {
    monster1_1_2_run(ctx, Monster112Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster212Step {
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

fn monster2_1_2_run(ctx: &Ctx, mut step: Monster212Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster212Step::Start => {
                step = Monster212Step::OnInit;
                continue 'machine;
            }
            Monster212Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-2")])?;
                return Err(Stop::End);
            }
            Monster212Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-2")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster2#1-2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster212Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#1-2")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Monster212Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I was the head of this underground laboratory."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster212Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("But that was a long time ago, back when I was merely a human."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster212Step::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I was called Vesper Newton. Hahah, they called me a mad man back then."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var(".mymobs").set(Val::from(13))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(31),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(32),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(33),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(34),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(35),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(36),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(37),
                        Val::from(152),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(150),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(31),
                        Val::from(150),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(32),
                        Val::from(150),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(33),
                        Val::from(150),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(34),
                        Val::from(150),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-2::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster212Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("...Not yet."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster212Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-2::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster212Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area1"),
                            Val::from("Not yet!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-2")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-2")])?;
                    ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_1_2(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_oninit(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_ondisable(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_onenable(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_ontimer2000(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_ontimer4000(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_ontimer6000(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_ontimer300000(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_ontimer300002(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_1_2_onmymobdead(ctx: &Ctx) -> Script {
    monster2_1_2_run(ctx, Monster212Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn hole_1_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("3"), Val::from(2)])?;
    if ctx.var("$@juprearea1inuse").get()? == 1 {
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
                    if ctx.var("$@juprearea1inuse").get()? == 1 {
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
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#1-3::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#1-3")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-3")])?;
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

pub fn hole_1_3(ctx: &Ctx) -> Script {
    hole_1_3_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_3_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_1_3_onstop_timer(ctx: &Ctx) -> Script {
    hole_1_3_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_3_ontimer22500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-3")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-3")])?;
    return Err(Stop::End);
}

pub fn hole_1_3_ontimer22500(ctx: &Ctx) -> Script {
    hole_1_3_ontimer22500_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp13Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer22500,
}

fn warp_1_3_run(ctx: &Ctx, mut step: Warp13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp13Step::Start => {
                step = Warp13Step::OnInit;
                continue 'machine;
            }
            Warp13Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-3")])?;
                return Err(Stop::End);
            }
            Warp13Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#1-3")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp13Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area1"), Val::from(33), Val::from(105)])?;
                return Err(Stop::End);
            }
            Warp13Step::OnTimer22500 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_1_3(ctx: &Ctx) -> Script {
    warp_1_3_run(ctx, Warp13Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_1_3_oninit(ctx: &Ctx) -> Script {
    warp_1_3_run(ctx, Warp13Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_1_3_onenable(ctx: &Ctx) -> Script {
    warp_1_3_run(ctx, Warp13Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_1_3_ontouch(ctx: &Ctx) -> Script {
    warp_1_3_run(ctx, Warp13Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_1_3_ontimer22500(ctx: &Ctx) -> Script {
    warp_1_3_run(ctx, Warp13Step::OnTimer22500, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm13Step {
    Start,
    OnInit,
    OnTouch,
}

fn red_alarm_1_3_run(ctx: &Ctx, mut step: RedAlarm13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm13Step::Start => {
                step = RedAlarm13Step::OnInit;
                continue 'machine;
            }
            RedAlarm13Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-3")])?;
                return Err(Stop::End);
            }
            RedAlarm13Step::OnTouch => {
                ctx.var("$@juprearea1inuse").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#1-3::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_1_3(ctx: &Ctx) -> Script {
    red_alarm_1_3_run(ctx, RedAlarm13Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_3_oninit(ctx: &Ctx) -> Script {
    red_alarm_1_3_run(ctx, RedAlarm13Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_3_ontouch(ctx: &Ctx) -> Script {
    red_alarm_1_3_run(ctx, RedAlarm13Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn13Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_1_3_run(ctx: &Ctx, mut step: RedAlarmOn13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn13Step::Start => {
                step = RedAlarmOn13Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn13Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-3")])?;
                return Err(Stop::End);
            }
            RedAlarmOn13Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#1-3")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn13Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("These security systems..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn13Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("They're not really for protection."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn13Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("It's sort of just a hobby to pass the time..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-3::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn13Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Being immortal, I have a lot of time on my hands..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn13Step::OnTimer8000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_1_3(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_3_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_1_3_run(ctx, RedAlarmOn13Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster113Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_1_3_run(ctx: &Ctx, mut step: Monster113Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster113Step::Start => {
                step = Monster113Step::OnInit;
                continue 'machine;
            }
            Monster113Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-3")])?;
                return Err(Stop::End);
            }
            Monster113Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-3")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster1#1-3::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster113Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#1-3")])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(31),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(32),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(33),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(34),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(35),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(36),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(37),
                        Val::from(89),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-3::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster113Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Zzzzt...Zzzzt...."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster113Step::OnTimer300002 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-3::OnDisable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-3")])?;
                return Err(Stop::End);
            }
            Monster113Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-3::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-3")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_1_3(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_1_3_oninit(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_1_3_ondisable(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_1_3_onenable(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_1_3_ontimer300000(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_1_3_ontimer300002(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_1_3_onmymobdead(ctx: &Ctx) -> Script {
    monster1_1_3_run(ctx, Monster113Step::OnMyMobDead, Vec::new()).map(|_| ())
}
