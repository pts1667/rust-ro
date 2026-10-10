use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Monster213Step {
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

fn monster2_1_3_run(ctx: &Ctx, mut step: Monster213Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster213Step::Start => {
                step = Monster213Step::OnInit;
                continue 'machine;
            }
            Monster213Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-3")])?;
                return Err(Stop::End);
            }
            Monster213Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-3")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster2#1-3::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster213Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#1-3")])?;
                return Err(Stop::End);
            }
            Monster213Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Why have you come?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster213Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Were you hoping to find something wonderful? Something miraculous?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster213Step::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("You're wrong! Welcome to Hell!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var(".mymobs").set(Val::from(15))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(42),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(43),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(44),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(45),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(46),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(47),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(48),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(49),
                        Val::from(64),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(42),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(43),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(44),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(45),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(46),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(47),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(48),
                        Val::from(62),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-3::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster213Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Do you still have your courage? Come. Prove it."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster213Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-3::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster213Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area1"),
                            Val::from("Do you still have your courage? Come. Prove it."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-3")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-3")])?;
                    ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_1_3(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_oninit(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_ondisable(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_onenable(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_ontimer2000(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_ontimer5000(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_ontimer8000(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_ontimer300000(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_ontimer300002(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_1_3_onmymobdead(ctx: &Ctx) -> Script {
    monster2_1_3_run(ctx, Monster213Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn hole_1_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("4"), Val::from(2)])?;
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
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#1-4::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm#1-4::OnEnable")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-4")])?;
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

pub fn hole_1_4(ctx: &Ctx) -> Script {
    hole_1_4_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_4_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_1_4_onstop_timer(ctx: &Ctx) -> Script {
    hole_1_4_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_4_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-4")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-4")])?;
    return Err(Stop::End);
}

pub fn hole_1_4_ontimer5000(ctx: &Ctx) -> Script {
    hole_1_4_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp14Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer5000,
}

fn warp_1_4_run(ctx: &Ctx, mut step: Warp14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp14Step::Start => {
                step = Warp14Step::OnInit;
                continue 'machine;
            }
            Warp14Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-4")])?;
                return Err(Stop::End);
            }
            Warp14Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#1-4")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp14Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area1"), Val::from(83), Val::from(157)])?;
                return Err(Stop::End);
            }
            Warp14Step::OnTimer5000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_1_4(ctx: &Ctx) -> Script {
    warp_1_4_run(ctx, Warp14Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_1_4_oninit(ctx: &Ctx) -> Script {
    warp_1_4_run(ctx, Warp14Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_1_4_onenable(ctx: &Ctx) -> Script {
    warp_1_4_run(ctx, Warp14Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_1_4_ontouch(ctx: &Ctx) -> Script {
    warp_1_4_run(ctx, Warp14Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_1_4_ontimer5000(ctx: &Ctx) -> Script {
    warp_1_4_run(ctx, Warp14Step::OnTimer5000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm14Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
}

fn red_alarm_1_4_run(ctx: &Ctx, mut step: RedAlarm14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm14Step::Start => {
                step = RedAlarm14Step::OnInit;
                continue 'machine;
            }
            RedAlarm14Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-4")])?;
                return Err(Stop::End);
            }
            RedAlarm14Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#1-4")])?;
                return Err(Stop::End);
            }
            RedAlarm14Step::OnTouch => {
                ctx.var("$@juprearea1inuse").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#1-4::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-4")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_1_4(ctx: &Ctx) -> Script {
    red_alarm_1_4_run(ctx, RedAlarm14Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_4_oninit(ctx: &Ctx) -> Script {
    red_alarm_1_4_run(ctx, RedAlarm14Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_4_onenable(ctx: &Ctx) -> Script {
    red_alarm_1_4_run(ctx, RedAlarm14Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_4_ontouch(ctx: &Ctx) -> Script {
    red_alarm_1_4_run(ctx, RedAlarm14Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn14Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_1_4_run(ctx: &Ctx, mut step: RedAlarmOn14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn14Step::Start => {
                step = RedAlarmOn14Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn14Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-4")])?;
                return Err(Stop::End);
            }
            RedAlarmOn14Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#1-4")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn14Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I've been waiting for someone strong enough to compete with me."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn14Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("If you hear this, I wish you will be the one..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn14Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Perhaps, a mere shadow of my former self..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-4::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn14Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Is somewhere down here, wandering..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn14Step::OnTimer8000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_1_4(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_4_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_1_4_run(ctx, RedAlarmOn14Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster114Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_1_4_run(ctx: &Ctx, mut step: Monster114Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster114Step::Start => {
                step = Monster114Step::OnInit;
                continue 'machine;
            }
            Monster114Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-4")])?;
                return Err(Stop::End);
            }
            Monster114Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-4")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster1#1-4::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster114Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#1-4")])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(92),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(96),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(100),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(104),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(92),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(96),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(100),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(104),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-4::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster114Step::OnTimer300000 => {
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
            Monster114Step::OnTimer300002 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-4")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-4::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster114Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-4::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-4")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_1_4(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_1_4_oninit(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_1_4_ondisable(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_1_4_onenable(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_1_4_ontimer300000(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_1_4_ontimer300002(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_1_4_onmymobdead(ctx: &Ctx) -> Script {
    monster1_1_4_run(ctx, Monster114Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster214Step {
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

fn monster2_1_4_run(ctx: &Ctx, mut step: Monster214Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster214Step::Start => {
                step = Monster214Step::OnInit;
                continue 'machine;
            }
            Monster214Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-4")])?;
                return Err(Stop::End);
            }
            Monster214Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster2#1-4::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-4")])?;
                return Err(Stop::End);
            }
            Monster214Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster2#1-4")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Monster214Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I can never rest in peace..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster214Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I'll wait forever or until someone can put me out of my misery..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster214Step::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I will be waiting for you!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var(".mymobs").set(Val::from(10))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(104),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(108),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(111),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(112),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(115),
                        Val::from(161),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(104),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(108),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(111),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(112),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(115),
                        Val::from(154),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster2#1-4::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster214Step::OnTimer300000 => {
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("It's funny... Isn't it?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster214Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-4")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-4::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster214Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("jupe_area1"),
                            Val::from("It's funny... Isn't it?"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster2#1-4")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-4")])?;
                    ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster2_1_4(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::Start, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_oninit(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_ondisable(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_onenable(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_ontimer2000(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_ontimer4000(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_ontimer6000(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_ontimer300000(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_ontimer300002(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster2_1_4_onmymobdead(ctx: &Ctx) -> Script {
    monster2_1_4_run(ctx, Monster214Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn lever_ufe_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFIt's a lever", "whose function", "is not known to you.^000000"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Pull.:Cancel.")])? {
        1 => {
            if ctx.var("$@juprearea1inuse").get()? == 1 {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.call(Function::InitNpcTimer, vec![])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("LeverWarp#ufe::OnEnable")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Lever#ufe")])?;
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

pub fn lever_ufe(ctx: &Ctx) -> Script {
    lever_ufe_body(ctx, Vec::new()).map(|_| ())
}

fn lever_ufe_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Lever#ufe")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn lever_ufe_ontimer3000(ctx: &Ctx) -> Script {
    lever_ufe_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LeverwarpUfeStep {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer3000,
}

fn leverwarp_ufe_run(ctx: &Ctx, mut step: LeverwarpUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LeverwarpUfeStep::Start => {
                step = LeverwarpUfeStep::OnInit;
                continue 'machine;
            }
            LeverwarpUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("LeverWarp#ufe")])?;
                return Err(Stop::End);
            }
            LeverwarpUfeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("LeverWarp#ufe")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            LeverwarpUfeStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_gate"), Val::from(28), Val::from(30)])?;
                return Err(Stop::End);
            }
            LeverwarpUfeStep::OnTimer3000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("LeverWarp#ufe")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn leverwarp_ufe(ctx: &Ctx) -> Script {
    leverwarp_ufe_run(ctx, LeverwarpUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe_oninit(ctx: &Ctx) -> Script {
    leverwarp_ufe_run(ctx, LeverwarpUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe_onenable(ctx: &Ctx) -> Script {
    leverwarp_ufe_run(ctx, LeverwarpUfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe_ontouch(ctx: &Ctx) -> Script {
    leverwarp_ufe_run(ctx, LeverwarpUfeStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn leverwarp_ufe_ontimer3000(ctx: &Ctx) -> Script {
    leverwarp_ufe_run(ctx, LeverwarpUfeStep::OnTimer3000, Vec::new()).map(|_| ())
}

fn hole_2_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(2)])?;
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
        ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
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
                if ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0 {
                    ctx.lines(args![
                        "^3355FFYou take out your",
                        "Crest Piece and place",
                        "it into the slot where it",
                        "happens to fit perfectly.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TOPRANK")?])?;
                    ctx.call(Function::Cutin, vec![Val::from("1-1"), Val::from(2)])?;
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
                        ctx.call(Function::Cutin, vec![Val::from("1-1"), Val::from(255)])?;
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
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#2-1::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#2-1")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-1")])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("1-1"), Val::from(255)])?;
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
                    ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hmmm...", "Do I have anything", "that might make this", "weird machine work?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
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
        ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
        return Err(Stop::End);
    }
}

pub fn hole_2_1(ctx: &Ctx) -> Script {
    hole_2_1_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_1_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_2_1_onstop_timer(ctx: &Ctx) -> Script {
    hole_2_1_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_2_1_ontimer22500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-1")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-1")])?;
    return Err(Stop::End);
}

pub fn hole_2_1_ontimer22500(ctx: &Ctx) -> Script {
    hole_2_1_ontimer22500_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp21Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer22500,
}

fn warp_2_1_run(ctx: &Ctx, mut step: Warp21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp21Step::Start => {
                step = Warp21Step::OnInit;
                continue 'machine;
            }
            Warp21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-1")])?;
                return Err(Stop::End);
            }
            Warp21Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#2-1")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp21Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area2"), Val::from(116), Val::from(259)])?;
                return Err(Stop::End);
            }
            Warp21Step::OnTimer22500 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#2-1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_2_1(ctx: &Ctx) -> Script {
    warp_2_1_run(ctx, Warp21Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_2_1_oninit(ctx: &Ctx) -> Script {
    warp_2_1_run(ctx, Warp21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_2_1_onenable(ctx: &Ctx) -> Script {
    warp_2_1_run(ctx, Warp21Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_2_1_ontouch(ctx: &Ctx) -> Script {
    warp_2_1_run(ctx, Warp21Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_2_1_ontimer22500(ctx: &Ctx) -> Script {
    warp_2_1_run(ctx, Warp21Step::OnTimer22500, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm21Step {
    Start,
    OnInit,
    OnTouch,
}

fn red_alarm_2_1_run(ctx: &Ctx, mut step: RedAlarm21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm21Step::Start => {
                step = RedAlarm21Step::OnInit;
                continue 'machine;
            }
            RedAlarm21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-1")])?;
                return Err(Stop::End);
            }
            RedAlarm21Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#2-1::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#2-1")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#hole#2-1::OnStop_Timer")])?;
                ctx.var("$@juprearea2inuse").set(Val::from(1))?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#2-1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_2_1(ctx: &Ctx) -> Script {
    red_alarm_2_1_run(ctx, RedAlarm21Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_1_oninit(ctx: &Ctx) -> Script {
    red_alarm_2_1_run(ctx, RedAlarm21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_2_1_ontouch(ctx: &Ctx) -> Script {
    red_alarm_2_1_run(ctx, RedAlarm21Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn21Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_2_1_run(ctx: &Ctx, mut step: RedAlarmOn21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn21Step::Start => {
                step = RedAlarmOn21Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn21Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-1")])?;
                return Err(Stop::End);
            }
            RedAlarmOn21Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#2-1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn21Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Those of you who have come here..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn21Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("I do not intend to stop you."),
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
            RedAlarmOn21Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("But I assume you are prepared for a few obstacles..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-1::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn21Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("After all, you are venturing through a forbidden area!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn21Step::OnTimer8000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#2-1")])?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area2")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area2")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_2_1(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_2_1_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_2_1_run(ctx, RedAlarmOn21Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster121Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_2_1_run(ctx: &Ctx, mut step: Monster121Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster121Step::Start => {
                step = Monster121Step::OnInit;
                continue 'machine;
            }
            Monster121Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-1")])?;
                return Err(Stop::End);
            }
            Monster121Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-1")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area2"), Val::from("Monster1#2-1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster121Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#2-1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(126),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(127),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(128),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(129),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(130),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(131),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(132),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from(133),
                        Val::from(252),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#2-1::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster121Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area2"),
                        Val::from("Do you realize this is a hallucination?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea2inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster121Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#2-1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#2-1::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster121Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#2-1::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#2-1")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_2_1(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_2_1_oninit(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_2_1_ondisable(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_2_1_onenable(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_2_1_ontimer300000(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_2_1_ontimer300002(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_2_1_onmymobdead(ctx: &Ctx) -> Script {
    monster1_2_1_run(ctx, Monster121Step::OnMyMobDead, Vec::new()).map(|_| ())
}
