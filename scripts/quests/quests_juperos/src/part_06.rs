use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Monster223Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster2_2_3_run(ctx: &Ctx, mut step: Monster223Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster223Step::Start => {
                step = Monster223Step::OnInit;
                continue 'machine;
            }
            Monster223Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-3")])?;
                return Err(Stop::End);
            }
            Monster223Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-3")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster2#2-3::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster223Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#2-3")])?;
                return Err(Stop::End);
            }
            Monster223Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Why have you come?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster223Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Were you hoping to find something wonderful? Something miraculous?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster223Step::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("You're wrong! Welcome to Hell!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var(".mymobs").set(Val::from(16))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(114),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(115),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(116),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(117),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(118),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(119),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(120),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(121),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(114),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(115),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(116),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(117),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(118),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(119),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(120),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(121),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-3::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster223Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Do you still have your courage? Come. Prove it."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster223Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-3::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster223Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area2"),
                            Val::from("Do you still have your courage? Come. Prove it."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-3")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-3")])?;
                    ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_2_3(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_oninit(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_ondisable(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_onenable(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_ontimer2000(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_ontimer5000(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_ontimer8000(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_ontimer300000(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_ontimer300002(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_2_3_onmymobdead(ctx: &Ctx) -> Script {
    monster2_2_3_run(ctx, Monster223Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn hole_2_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("4"), Val::from(2)])?;
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
        ctx.call(Function::Cutin, vec![Val::from("4"), Val::from(255)])?;
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
                if ctx.call(Function::CountItem, vec![Val::from(7359)])?.number()? > 0 {
                    ctx.lines(args![
                        "^3355FFYou take out your",
                        "Crest Piece and place",
                        "it into the slot where it",
                        "happens to fit perfectly.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TOPRANK")?])?;
                    ctx.call(Function::Cutin, vec![Val::from("4-1"), Val::from(2)])?;
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
                        ctx.call(Function::Cutin, vec![Val::from("4-1"), Val::from(255)])?;
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
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#2-4::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm#2-4::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-4")])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("4-1"), Val::from(255)])?;
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
                    ctx.call(Function::Cutin, vec![Val::from("4"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hmmm...", "Do I have anything", "that might make this", "weird machine work?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("4"), Val::from(255)])?;
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
        ctx.call(Function::Cutin, vec![Val::from("4"), Val::from(255)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn hole_2_4(ctx: &Ctx) -> Script {
    hole_2_4_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_4_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_2_4_onstop_timer(ctx: &Ctx) -> Script {
    hole_2_4_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_4_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-4")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-4")])?;
    return Err(Stop::End);
}

pub fn hole_2_4_ontimer5000(ctx: &Ctx) -> Script {
    hole_2_4_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp24Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer5000,
}

fn warp_2_4_run(ctx: &Ctx, mut step: Warp24Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp24Step::Start => {
                step = Warp24Step::OnInit;
                continue 'machine;
            }
            Warp24Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-4")])?;
                return Err(Stop::End);
            }
            Warp24Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#2-4")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp24Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area2"), Val::from(80), Val::from(157)])?;
                return Err(Stop::End);
            }
            Warp24Step::OnTimer5000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_2_4(ctx: &Ctx) -> Script {
    warp_2_4_run(ctx, Warp24Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_2_4_oninit(ctx: &Ctx) -> Script {
    warp_2_4_run(ctx, Warp24Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_2_4_onenable(ctx: &Ctx) -> Script {
    warp_2_4_run(ctx, Warp24Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_2_4_ontouch(ctx: &Ctx) -> Script {
    warp_2_4_run(ctx, Warp24Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_2_4_ontimer5000(ctx: &Ctx) -> Script {
    warp_2_4_run(ctx, Warp24Step::OnTimer5000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm24Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
}

fn red_alarm_2_4_run(ctx: &Ctx, mut step: RedAlarm24Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm24Step::Start => {
                step = RedAlarm24Step::OnInit;
                continue 'machine;
            }
            RedAlarm24Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-4")])?;
                return Err(Stop::End);
            }
            RedAlarm24Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#2-4")])?;
                return Err(Stop::End);
            }
            RedAlarm24Step::OnTouch => {
                ctx.var("$@juprearea2inuse").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#2-4::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-4")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_2_4(ctx: &Ctx) -> Script {
    red_alarm_2_4_run(ctx, RedAlarm24Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_4_oninit(ctx: &Ctx) -> Script {
    red_alarm_2_4_run(ctx, RedAlarm24Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_4_onenable(ctx: &Ctx) -> Script {
    red_alarm_2_4_run(ctx, RedAlarm24Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_4_ontouch(ctx: &Ctx) -> Script {
    red_alarm_2_4_run(ctx, RedAlarm24Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn24Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_2_4_run(ctx: &Ctx, mut step: RedAlarmOn24Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn24Step::Start => {
                step = RedAlarmOn24Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn24Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-4")])?;
                return Err(Stop::End);
            }
            RedAlarmOn24Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#2-4")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn24Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I've been waiting for someone strong enough to compete with me."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn24Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("If you hear this, I wish you will be the one..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn24Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Perhaps, a mere shadow of my former self..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-4::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn24Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Is somewhere down here, wandering..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn24Step::OnTimer8000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_2_4(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_4_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_2_4_run(ctx, RedAlarmOn24Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster124Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_2_4_run(ctx: &Ctx, mut step: Monster124Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster124Step::Start => {
                step = Monster124Step::OnInit;
                continue 'machine;
            }
            Monster124Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-4")])?;
                return Err(Stop::End);
            }
            Monster124Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-4")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster1#2-4::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster124Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#2-4")])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(75),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(72),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(71),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(68),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(75),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(72),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(71),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(68),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-4::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster124Step::OnTimer300000 => {
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
            Monster124Step::OnTimer300002 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-4")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-4::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster124Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-4::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-4")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_2_4(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_2_4_oninit(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_2_4_ondisable(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_2_4_onenable(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_2_4_ontimer300000(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_2_4_ontimer300002(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_2_4_onmymobdead(ctx: &Ctx) -> Script {
    monster1_2_4_run(ctx, Monster124Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster224Step {
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

fn monster2_2_4_run(ctx: &Ctx, mut step: Monster224Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster224Step::Start => {
                step = Monster224Step::OnInit;
                continue 'machine;
            }
            Monster224Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-4")])?;
                return Err(Stop::End);
            }
            Monster224Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster2#2-4::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-4")])?;
                return Err(Stop::End);
            }
            Monster224Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#2-4")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Monster224Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I can never rest in peace..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster224Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I'll wait forever or until someone can put me out of my misery..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster224Step::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I will be waiting for you!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var(".mymobs").set(Val::from(12))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(63),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(61),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(59),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(57),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(55),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(53),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(53),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(55),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(57),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(59),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(61),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(63),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#2-4::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster224Step::OnTimer300000 => {
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("It's funny... Isn't it?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster224Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-4")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-4::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster224Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area2"),
                            Val::from("It's funny... Isn't it?"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#2-4")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-4")])?;
                    ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_2_4(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_oninit(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_ondisable(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_onenable(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_ontimer2000(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_ontimer4000(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_ontimer6000(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_ontimer300000(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_ontimer300002(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_2_4_onmymobdead(ctx: &Ctx) -> Script {
    monster2_2_4_run(ctx, Monster224Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn lever_ufe2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFIt's a lever", "whose function", "is not known to you.^000000"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Pull.:Cancel.")])? {
        1 => {
            if ctx.var("$@juprearea2inuse").get()? == 1 {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.call(Function::InitNpcTimer, vec![])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("LeverWarp#ufe2::OnEnable")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Lever#ufe2")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Pull this lever?", "I don't even know", "what will happen..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lever_ufe2(ctx: &Ctx) -> Script {
    lever_ufe2_body(ctx, Vec::new()).map(|_| ())
}

fn lever_ufe2_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Lever#ufe2")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn lever_ufe2_ontimer3000(ctx: &Ctx) -> Script {
    lever_ufe2_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LeverwarpUfe2Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer3000,
}

fn leverwarp_ufe2_run(ctx: &Ctx, mut step: LeverwarpUfe2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LeverwarpUfe2Step::Start => {
                step = LeverwarpUfe2Step::OnInit;
                continue 'machine;
            }
            LeverwarpUfe2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("LeverWarp#ufe2")])?;
                return Err(Stop::End);
            }
            LeverwarpUfe2Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("LeverWarp#ufe2")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            LeverwarpUfe2Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_gate"), Val::from(71), Val::from(29)])?;
                return Err(Stop::End);
            }
            LeverwarpUfe2Step::OnTimer3000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("LeverWarp#ufe2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn leverwarp_ufe2(ctx: &Ctx) -> Script {
    leverwarp_ufe2_run(ctx, LeverwarpUfe2Step::Start, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe2_oninit(ctx: &Ctx) -> Script {
    leverwarp_ufe2_run(ctx, LeverwarpUfe2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe2_onenable(ctx: &Ctx) -> Script {
    leverwarp_ufe2_run(ctx, LeverwarpUfe2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe2_ontouch(ctx: &Ctx) -> Script {
    leverwarp_ufe2_run(ctx, LeverwarpUfe2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe2_ontimer3000(ctx: &Ctx) -> Script {
    leverwarp_ufe2_run(ctx, LeverwarpUfe2Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn switch_ufe_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$@jupeelevatorinuse").get()? == 1 {
        ctx.lines(args![
            "^3355FFIt's some sort of",
            "lever that looks like",
            "it was already pulled",
            "by someone else.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Cutin, vec![Val::from("5"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFIt's some sort of",
            "lever that's located",
            "next to four empty slots.^000000"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Pull the lever.:Leave it alone.")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.mes("^3355FF*Snap Snap*^000000")?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou pull the lever,",
                    "but nothing happened.",
                    "You probably need to",
                    "insert the correct objects",
                    "into the slots in order",
                    "for the lever to operate.^000000"
                ])?;
                ctx.next()?;
                if (((ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0
                    && ctx.call(Function::CountItem, vec![Val::from(7359)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(7357)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(7358)])?.number()? > 0)
                {
                    match runtime::select_values(ctx, &[Val::from("Insert all of your Crest Pieces.")])? {
                        1 => {
                            ctx.lines(args![
                                "^3300FF*Snap!*^000000",
                                "^3300FFStrangely enough,",
                                "all four of the Crest",
                                "Pieces fit perfectly into",
                                "the slots and begin to",
                                "emit a strange light.^000000"
                            ])?;
                            ctx.call(Function::Cutin, vec![Val::from("5-1"), Val::from(2)])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
                            ctx.call(Function::DelItem, vec![Val::from(7356), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7359), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7357), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7358), Val::from(1)])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Pull out the Crest Pieces.:Pull the lever.")])? {
                                1 => {
                                    ctx.call(Function::Cutin, vec![Val::from("5"), Val::from(2)])?;
                                    ctx.lines(args![
                                        "^3355FFYou pull out all",
                                        "the Crest Pieces",
                                        "that you inserted",
                                        "into the slots.^000000"
                                    ])?;
                                    ctx.call(Function::GetItem, vec![Val::from(7356), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(7359), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(7357), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(7358), Val::from(1)])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("5"), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    if ctx.var("$@jupeelevatorinuse").get()? == 1 {
                                        ctx.lines(args![
                                            "^3355FFIt's strange,",
                                            "but this lever has",
                                            "already been pulled.^000000"
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from("5"), Val::from(255)])?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines(args![
                                            "^3355FFOnce you pull the lever,",
                                            "the Crest Piece slots are",
                                            "suddenly covered, making",
                                            "them irretrievable, and the",
                                            "ground begins to shake",
                                            "violently. This isn't normal!^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.var("$@jupeelevatorinuse").set(Val::from(1))?;
                                        ctx.call(Function::DisableNpc, vec![Val::from("Switch#ufe")])?;
                                        ctx.call(Function::EnableNpc, vec![Val::from("Switch On#ufe")])?;
                                        ctx.call(Function::InitNpcTimer, vec![])?;
                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SCREEN_QUAKE")?])?;
                                        ctx.call(Function::SoundEffectAll, vec![Val::from("earth_quake.wav"), Val::from(0)])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from("5-1"), Val::from(255)])?;
                                        return Err(Stop::End);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else if (((ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0
                    || ctx.call(Function::CountItem, vec![Val::from(7359)])?.number()? > 0)
                    || ctx.call(Function::CountItem, vec![Val::from(7357)])?.number()? > 0)
                    || ctx.call(Function::CountItem, vec![Val::from(7358)])?.number()? > 0)
                {
                    let choice = runtime::select_values(ctx, &[Val::from("Insert Crest Pieces.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines(args![
                        "^3355FFRight now, you don't",
                        "have enough Crest Pieces",
                        "to place into all four of these",
                        "slots. You'll need to find and^FFFFFF ^3355FF bring them all to make this work.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("5-1"), Val::from(255)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "^3355FFYou need to find",
                        "some kind of object",
                        "that you can fit into",
                        "each of these four slots...^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("5-1"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "^3355FFWho knows what",
                    "this lever may do?",
                    "You'll never know unless",
                    "you have the courage to try.^000000"
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("5"), Val::from(255)])?;
                return Err(Stop::End);
            }
        }
    }
    return Err(Stop::End);
}

pub fn switch_ufe(ctx: &Ctx) -> Script {
    switch_ufe_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("My descendents..."),
            ctx.constant("BC_MAP")?,
            Val::from("0x66FF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer2000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("Do you want to know why this city was buried beneath the earth...?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x66FF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer3000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer7000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("If so, follow my voice..."),
            ctx.constant("BC_MAP")?,
            Val::from("0x66FF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer7000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer7000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("I shall let you see for yourself what you desire to know..."),
            ctx.constant("BC_MAP")?,
            Val::from("0x66FF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer10000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer17000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("Overcome all the hallucinations."),
            ctx.constant("BC_MAP")?,
            Val::from("0xCC6600"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer17000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer17000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer20000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("Open your eyes and see past all of the lies."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFF0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer20000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer20000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer23000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("I can only maintain this vision for you for 20 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFF0000"),
        ],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Elevator Guard1#ufe::OnEnable")])?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer23000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer23000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_ufe_ontimer27000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("jupe_ele_r"),
            Val::from("Look! And remember!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xFF0000"),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn switch_ufe_ontimer27000(ctx: &Ctx) -> Script {
    switch_ufe_ontimer27000_body(ctx, Vec::new()).map(|_| ())
}

fn switch_on_ufe_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFIt seems like", "someone else is", "using this machine...^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn switch_on_ufe(ctx: &Ctx) -> Script {
    switch_on_ufe_body(ctx, Vec::new()).map(|_| ())
}

fn switch_on_ufe_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Switch On#ufe")])?;
    return Err(Stop::End);
}

pub fn switch_on_ufe_oninit(ctx: &Ctx) -> Script {
    switch_on_ufe_oninit_body(ctx, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::Start, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_oninit(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_onenable(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer1000(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer1200(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer1200, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer1400(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer1400, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer1600(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer1600, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer1800(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer1800, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer2000(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer2200(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer2200, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer2400(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer2400, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer2600(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer2600, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer120000(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_ontimer120005(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnTimer120005, Vec::new()).map(|_| ())
}

pub fn elevator_guard1_ufe_onmymobdead(ctx: &Ctx) -> Script {
    elevator_guard1_ufe_run(ctx, ElevatorGuard1UfeStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ElevatorSafetyUfeStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer5000,
    OnTimer8000,
    OnTimer10000,
}

fn elevator_safety_ufe_run(ctx: &Ctx, mut step: ElevatorSafetyUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ElevatorSafetyUfeStep::Start => {
                step = ElevatorSafetyUfeStep::OnInit;
                continue 'machine;
            }
            ElevatorSafetyUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator Safety#ufe")])?;
                return Err(Stop::End);
            }
            ElevatorSafetyUfeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Elevator Safety#ufe")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ElevatorSafetyUfeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from("Those of you who have defeated the hallucination, step forward."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("earth_quake.wav"), Val::from(0)])?;
                return Err(Stop::End);
            }
            ElevatorSafetyUfeStep::OnTimer8000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Elevator On#ufe::OnEnable")])?;
                return Err(Stop::End);
            }
            ElevatorSafetyUfeStep::OnTimer10000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator Safety#ufe")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn elevator_safety_ufe(ctx: &Ctx) -> Script {
    elevator_safety_ufe_run(ctx, ElevatorSafetyUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn elevator_safety_ufe_oninit(ctx: &Ctx) -> Script {
    elevator_safety_ufe_run(ctx, ElevatorSafetyUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn elevator_safety_ufe_onenable(ctx: &Ctx) -> Script {
    elevator_safety_ufe_run(ctx, ElevatorSafetyUfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn elevator_safety_ufe_ontimer5000(ctx: &Ctx) -> Script {
    elevator_safety_ufe_run(ctx, ElevatorSafetyUfeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn elevator_safety_ufe_ontimer8000(ctx: &Ctx) -> Script {
    elevator_safety_ufe_run(ctx, ElevatorSafetyUfeStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn elevator_safety_ufe_ontimer10000(ctx: &Ctx) -> Script {
    elevator_safety_ufe_run(ctx, ElevatorSafetyUfeStep::OnTimer10000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AnnihilationUfeStep {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer1000,
    OnTimer1600,
    OnTimer3000,
}

fn annihilation_ufe_run(ctx: &Ctx, mut step: AnnihilationUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AnnihilationUfeStep::Start => {
                step = AnnihilationUfeStep::OnInit;
                continue 'machine;
            }
            AnnihilationUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Annihilation#ufe")])?;
                return Err(Stop::End);
            }
            AnnihilationUfeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Annihilation#ufe")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            AnnihilationUfeStep::OnTouch => {
                ctx.call(Function::PercentHeal, vec![Val::from(-99), Val::from(-100)])?;
                return Err(Stop::End);
            }
            AnnihilationUfeStep::OnTimer1000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("jupe_ele_r"), Val::from("jupe_gate"), Val::from(49), Val::from(138)],
                )?;
                return Err(Stop::End);
            }
            AnnihilationUfeStep::OnTimer1600 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Elevator Escape#ufe")])?;
                return Err(Stop::End);
            }
            AnnihilationUfeStep::OnTimer3000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Annihilation#ufe")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator Escape#ufe")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn annihilation_ufe(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn annihilation_ufe_oninit(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn annihilation_ufe_onenable(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn annihilation_ufe_ontouch(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn annihilation_ufe_ontimer1000(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn annihilation_ufe_ontimer1600(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::OnTimer1600, Vec::new()).map(|_| ())
}

pub fn annihilation_ufe_ontimer3000(ctx: &Ctx) -> Script {
    annihilation_ufe_run(ctx, AnnihilationUfeStep::OnTimer3000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ElevatorEscapeUfeStep {
    Start,
    OnInit,
    OnTouch,
}

fn elevator_escape_ufe_run(ctx: &Ctx, mut step: ElevatorEscapeUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ElevatorEscapeUfeStep::Start => {
                step = ElevatorEscapeUfeStep::OnInit;
                continue 'machine;
            }
            ElevatorEscapeUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator Escape#ufe")])?;
                return Err(Stop::End);
            }
            ElevatorEscapeUfeStep::OnTouch => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("jupe_ele_r"), Val::from("jupe_gate"), Val::from(49), Val::from(138)],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn elevator_escape_ufe(ctx: &Ctx) -> Script {
    elevator_escape_ufe_run(ctx, ElevatorEscapeUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn elevator_escape_ufe_oninit(ctx: &Ctx) -> Script {
    elevator_escape_ufe_run(ctx, ElevatorEscapeUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn elevator_escape_ufe_ontouch(ctx: &Ctx) -> Script {
    elevator_escape_ufe_run(ctx, ElevatorEscapeUfeStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ElevatorOnUfeStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer10000,
}

fn elevator_on_ufe_run(ctx: &Ctx, mut step: ElevatorOnUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ElevatorOnUfeStep::Start => {
                step = ElevatorOnUfeStep::OnInit;
                continue 'machine;
            }
            ElevatorOnUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator On#ufe")])?;
                return Err(Stop::End);
            }
            ElevatorOnUfeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Elevator On#ufe")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ElevatorOnUfeStep::OnTimer1000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("jupe_ele_r"), Val::from("jupe_ele"), Val::from(42), Val::from(47)],
                )?;
                ctx.var("$@jupeelevatorinuse2").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("TimeOut#ufe::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-1#ufe::OnEnable")])?;
                return Err(Stop::End);
            }
            ElevatorOnUfeStep::OnTimer10000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator On#ufe")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn elevator_on_ufe(ctx: &Ctx) -> Script {
    elevator_on_ufe_run(ctx, ElevatorOnUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn elevator_on_ufe_oninit(ctx: &Ctx) -> Script {
    elevator_on_ufe_run(ctx, ElevatorOnUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn elevator_on_ufe_onenable(ctx: &Ctx) -> Script {
    elevator_on_ufe_run(ctx, ElevatorOnUfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn elevator_on_ufe_ontimer1000(ctx: &Ctx) -> Script {
    elevator_on_ufe_run(ctx, ElevatorOnUfeStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn elevator_on_ufe_ontimer10000(ctx: &Ctx) -> Script {
    elevator_on_ufe_run(ctx, ElevatorOnUfeStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_onenable(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ondisable(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer59000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer59000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer120000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer122000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer122000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer125000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer125000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer127000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer127000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer129000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer129000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer131000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer131000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer133000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer133000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer134000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer134000, Vec::new()).map(|_| ())
}
