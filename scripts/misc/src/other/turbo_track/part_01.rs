use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn entrance_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn entrance_tt_main(ctx: &Ctx) -> Script {
    entrance_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn entrance_tt_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    ctx.call(
        Function::EnableWaitingRoomEvent,
        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
    )?;
    return Err(Stop::End);
}

pub fn entrance_tt_main_onenable(ctx: &Ctx) -> Script {
    entrance_tt_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn entrance_tt_main_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_in = Val::from(0);
    let mut l_in_s = Val::from("");
    if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from("n1")).is_true() {
        l_in_s = Val::from("n");
        l_in = Val::from(1);
    } else {
        l_in_s = (if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("Expert")).is_true() {
            Val::from("e")
        } else {
            Val::from("n")
        });
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("4")).is_true() {
            l_in = Val::from(4);
        }
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("8")).is_true() {
            l_in = Val::from(8);
        }
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("16")).is_true() {
            l_in = Val::from(16);
        }
    }
    ctx.call(
        Function::WarpWaitingPc,
        vec![
            (((Val::from("turbo_") + l_in_s.clone()) + Val::from("_")) + l_in.clone()),
            Val::from(298),
            Val::from(161),
        ],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![(((Val::from("Broadcast#") + l_in_s.clone()) + l_in.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DisableWaitingRoomEvent,
        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
    )?;
    return Err(Stop::End);
}

pub fn entrance_tt_main_onstartarena(ctx: &Ctx) -> Script {
    entrance_tt_main_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn entrance_tt_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_in = Val::from(0);
    if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("main")).is_true() {
        return Err(Stop::End);
    }
    if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from("n1")).is_true() {
        ctx.call(
            Function::WaitingRoom,
            vec![
                Val::from("Solo Mode"),
                Val::from(60),
                Val::from("Solo Mode#n1::OnStartArena"),
                Val::from(1),
                Val::from(1000),
                Val::from(10),
            ],
        )?;
    } else {
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("4")).is_true() {
            l_in = Val::from(4);
        }
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("8")).is_true() {
            l_in = Val::from(8);
        }
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?, &Val::from("16")).is_true() {
            l_in = Val::from(16);
        }
        ctx.call(
            Function::WaitingRoom,
            vec![
                ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?,
                Val::from(60),
                (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnStartArena")),
                l_in.clone(),
                Val::from(1000),
                Val::from(10),
            ],
        )?;
    }
    ctx.call(
        Function::EnableWaitingRoomEvent,
        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
    )?;
    return Err(Stop::End);
}

pub fn entrance_tt_main_oninit(ctx: &Ctx) -> Script {
    entrance_tt_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn helper_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Helper",
        args![
            "You are now in the",
            "Waiting Room. You will",
            "be guided to the Starting Line",
            "after 30 seconds, so please use",
            "this time to prepare your items",
            "and equipment. Thank you."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn helper_tt_main(ctx: &Ctx) -> Script {
    helper_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn point_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn point_tt_main(ctx: &Ctx) -> Script {
    point_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn point_tt_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("tt_point").get()?.number()? < 28999 {
        ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(2)))?;
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(59), Val::from(364)],
        )?;
    } else {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(59), Val::from(364)],
        )?;
    }
    return Err(Stop::End);
}

pub fn point_tt_main_ontouch(ctx: &Ctx) -> Script {
    point_tt_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn point_tt_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Point Give-Away Guy#") + l_w_s.clone())])?;
    return Err(Stop::End);
}

pub fn point_tt_main_oninit(ctx: &Ctx) -> Script {
    point_tt_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BroadcastTtMainStep {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer7000,
    OnTimer10000,
    OnTimer15000,
    OnTimer17000,
    OnTimer27000,
    OnTimer37000,
    OnTimer42000,
    OnTimer43000,
    OnTimer44000,
    OnTimer45000,
    OnTimer46000,
    OnTimer47000,
    OnTimer49000,
    OnTimer50000,
    OnTimer57000,
    OnInit,
}

fn broadcast_tt_main_run(ctx: &Ctx, mut step: BroadcastTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            BroadcastTtMainStep::Start => {
                step = BroadcastTtMainStep::OnEnable;
                continue 'machine;
            }
            BroadcastTtMainStep::OnEnable => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Broadcast#") + l_w_s.clone())])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You are now in the Waiting Room where you can check your items and prepare for the race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You will have 30 seconds before you are transported to the Starting Line."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("Please make sure that you have suitable equipment and items with you."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("The 30 second countdown will begin shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer17000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer27000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("20 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer37000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("10 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer42000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("5 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer43000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("4 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer44000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("3 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer45000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("2 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer46000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("1 second remaining."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer47000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You will be transported to the Starting Line shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer49000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Point Give-Away Guy#") + l_w_s.clone())])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("Master#") + l_w_s.clone()) + Val::from("::OnEnable"))],
                )?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer50000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("snake#") + l_w_s.clone()) + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("hunting#") + l_w_s.clone()) + Val::from("::OnEnable"))],
                )?;
                if ctx.call(Function::StrNpcInfo, vec![Val::from(4)])? != "turbo_n_1" {
                    ctx.call(Function::EnableNpc, vec![(Val::from("bing#") + l_w_s.clone())])?;
                }
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnTimer57000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Point Give-Away Guy#") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
            BroadcastTtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Broadcast#") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn broadcast_tt_main(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_onenable(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer2000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer7000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer10000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer15000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer17000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer17000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer27000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer27000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer37000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer37000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer42000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer42000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer43000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer43000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer44000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer44000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer45000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer45000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer46000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer46000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer47000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer47000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer49000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer49000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer50000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_ontimer57000(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnTimer57000, Vec::new()).map(|_| ())
}

pub fn broadcast_tt_main_oninit(ctx: &Ctx) -> Script {
    broadcast_tt_main_run(ctx, BroadcastTtMainStep::OnInit, Vec::new()).map(|_| ())
}

pub fn master_tt_main(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn master_tt_main_onenable(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ondisable(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer7000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer9000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer11000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer13000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer15000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer17000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer17000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer18000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer18000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer19000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer19000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer20000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer21000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer21000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer22000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer22000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer23000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer23000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer30000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer83000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer83000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer143000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer143000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer203000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer203000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer263000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer263000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer323000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer323000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer383000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer383000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer443000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer443000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer503000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer503000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer563000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer563000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer623000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer623000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer683000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer683000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer743000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer743000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer803000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer803000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer863000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer863000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer893000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer893000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer903000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer903000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer913000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer913000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer918000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer918000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer919000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer919000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer920000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer920000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer921000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer921000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer922000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer922000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer923000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer923000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer925000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer925000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_ontimer927000(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnTimer927000, Vec::new()).map(|_| ())
}

pub fn master_tt_main_oninit(ctx: &Ctx) -> Script {
    master_tt_main_run(ctx, MasterTtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NounfairTtMainStep {
    Start,
    OnTouch,
}

fn nounfair_tt_main_run(ctx: &Ctx, mut step: NounfairTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            NounfairTtMainStep::Start => {
                step = NounfairTtMainStep::OnTouch;
                continue 'machine;
            }
            NounfairTtMainStep::OnTouch => {
                ctx.call(
                    Function::Warp,
                    vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(59), Val::from(364)],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nounfair_tt_main(ctx: &Ctx) -> Script {
    nounfair_tt_main_run(ctx, NounfairTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn nounfair_tt_main_ontouch(ctx: &Ctx) -> Script {
    nounfair_tt_main_run(ctx, NounfairTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LogtrapTtMainStep {
    Start,
    OnTouch,
}

fn logtrap_tt_main_run(ctx: &Ctx, mut step: LogtrapTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LogtrapTtMainStep::Start => {
                step = LogtrapTtMainStep::OnTouch;
                continue 'machine;
            }
            LogtrapTtMainStep::OnTouch => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(72), Val::from(372)],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(72), Val::from(365)],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(72), Val::from(357)],
                    )?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn logtrap_tt_main(ctx: &Ctx) -> Script {
    logtrap_tt_main_run(ctx, LogtrapTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn logtrap_tt_main_ontouch(ctx: &Ctx) -> Script {
    logtrap_tt_main_run(ctx, LogtrapTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SandtrapTtMainStep {
    Start,
    OnTouch,
}

fn sandtrap_tt_main_run(ctx: &Ctx, mut step: SandtrapTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SandtrapTtMainStep::Start => {
                step = SandtrapTtMainStep::OnTouch;
                continue 'machine;
            }
            SandtrapTtMainStep::OnTouch => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                if subject1 == 1 || subject1 == 9 {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_CONFUSION")?, Val::from(8000), Val::from(0)],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_PROFUSELY_SWEAT")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_STONE")?, Val::from(4000), Val::from(0)],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 4 {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_SLEEP")?, Val::from(4000), Val::from(0)],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 6 {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_FREEZE")?, Val::from(4000), Val::from(0)],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_PROFUSELY_SWEAT")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 8 {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_STUN")?, Val::from(4000), Val::from(0)],
                    )?;
                    return Err(Stop::End);
                } else if subject1 == 10 {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_CURSE")?, Val::from(80000), Val::from(0)],
                    )?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn sandtrap_tt_main(ctx: &Ctx) -> Script {
    sandtrap_tt_main_run(ctx, SandtrapTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn sandtrap_tt_main_ontouch(ctx: &Ctx) -> Script {
    sandtrap_tt_main_run(ctx, SandtrapTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapTtMainStep {
    Start,
    OnTouch,
}

fn trap_tt_main_run(ctx: &Ctx, mut step: TrapTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            TrapTtMainStep::Start => {
                step = TrapTtMainStep::OnTouch;
                continue 'machine;
            }
            TrapTtMainStep::OnTouch => {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 1 {
                    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                    ctx.call(Function::Cutin, vec![Val::from("kafra_03"), Val::from(2)])?;
                    ctx.lines(args![
                        "^4d4dffAl De Baran",
                        "Turbo Track",
                        "is brought to you by",
                        "the ^800000Kafra Corporation^4d4dff.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^4d4dffWe wish the best of luck to all",
                        "Turbo Track participants today",
                        "and thank everyone for using the Kafra Services with all our hearts.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args!["^800000Kafra Corporation^4d4dff has been providing Storage Services,", "Save Point Services and Teleport Services^4d4dff to our valued customers for years. Just listen to these real customers...^000000"])?;
                    ctx.next()?;
                    if l_w_s.clone() == "e4" {
                        ctx.lines(args![
                            "^4d4dff[Kachua]",
                            "Oh yes, they provide the best Storage! There's even enough",
                            "room for my Diaaaamonds!",
                            " ",
                            "[Chief Mahnsoo]",
                            "I looove you Kafra!^000000"
                        ])?;
                    } else if (l_w_s.clone() == "e8" || l_w_s.clone() == "n8") {
                        ctx.lines(args![
                            "^4d4dff[Errende]",
                            "Kafra Ladies? Exquisite! Oh, and their service is good too~!^000000",
                            " ",
                            "[Tristram III]",
                            "By my crown! Such low prices!"
                        ])?;
                    } else if (l_w_s.clone() == "e16" || l_w_s.clone() == "n16") {
                        ctx.lines(args![
                            "^4d4dff[Union Staff Kay]",
                            "Of course I love 'em, especially their Pushcart Service~",
                            " ",
                            "[Santa Claus]",
                            "Ho ho ho!",
                            "Such Merry Prices!"
                        ])?;
                    } else if l_w_s.clone() == "n4" {
                        ctx.lines(args![
                            "^4d4dff[Xenophon Zolotas]",
                            "I wouldn't be able to do business without the Kafra Services. Thank you, Kafra!",
                            " ",
                            "[Chief Mahnsoo]",
                            "I looove you Kafra!^000000"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "^4d4dff[Karkatan]",
                            "My land suffered from poor customer service...until Kafra came along!",
                            " ",
                            "[Curator Guiss]",
                            "Oh, Kafra is simply the best!^000000"
                        ])?;
                    }
                    ctx.next()?;
                    ctx.lines(args![
                        "^ff0000Turbo Track",
                        "^ff0000Traps in the Cursed Desert!",
                        "^4d4dffSponsored by ^800000Kafra Corporation^4d4dff",
                        "''We are always by your side.''^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("kafra_03"), Val::from(255)])?;
                    return Err(Stop::End);
                }
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_CONFUSION")?, Val::from(4000), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_tt_main(ctx: &Ctx) -> Script {
    trap_tt_main_run(ctx, TrapTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn trap_tt_main_ontouch(ctx: &Ctx) -> Script {
    trap_tt_main_run(ctx, TrapTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WatertrapTtMainStep {
    Start,
    OnTouch,
}

fn watertrap_tt_main_run(ctx: &Ctx, mut step: WatertrapTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WatertrapTtMainStep::Start => {
                step = WatertrapTtMainStep::OnTouch;
                continue 'machine;
            }
            WatertrapTtMainStep::OnTouch => {
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn watertrap_tt_main(ctx: &Ctx) -> Script {
    watertrap_tt_main_run(ctx, WatertrapTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn watertrap_tt_main_ontouch(ctx: &Ctx) -> Script {
    watertrap_tt_main_run(ctx, WatertrapTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Snake01TtMainStep {
    Start,
    OnTouch,
}

fn snake01_tt_main_run(ctx: &Ctx, mut step: Snake01TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_turbo2 = Val::from(0);
    'machine: loop {
        match step {
            Snake01TtMainStep::Start => {
                step = Snake01TtMainStep::OnTouch;
                continue 'machine;
            }
            Snake01TtMainStep::OnTouch => {
                l_turbo2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                if l_turbo2.clone().number()? < 3 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(370), Val::from(292)],
                    )?;
                }
                if l_turbo2.clone().number()? < 5 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(295), Val::from(293)],
                    )?;
                }
                if l_turbo2.clone().number()? < 7 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(355), Val::from(292)],
                    )?;
                }
                if l_turbo2.clone().number()? < 8 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(279), Val::from(292)],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn snake01_tt_main(ctx: &Ctx) -> Script {
    snake01_tt_main_run(ctx, Snake01TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn snake01_tt_main_ontouch(ctx: &Ctx) -> Script {
    snake01_tt_main_run(ctx, Snake01TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Snake02TtMainStep {
    Start,
    OnTouch,
}

fn snake02_tt_main_run(ctx: &Ctx, mut step: Snake02TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_turbo2 = Val::from(0);
    'machine: loop {
        match step {
            Snake02TtMainStep::Start => {
                step = Snake02TtMainStep::OnTouch;
                continue 'machine;
            }
            Snake02TtMainStep::OnTouch => {
                l_turbo2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(8)])?;
                if l_turbo2.clone().number()? < 3 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(287), Val::from(256)],
                    )?;
                }
                if l_turbo2.clone().number()? < 5 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(303), Val::from(256)],
                    )?;
                }
                if l_turbo2.clone().number()? < 7 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(347), Val::from(256)],
                    )?;
                }
                if l_turbo2.clone().number()? < 9 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(363), Val::from(256)],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn snake02_tt_main(ctx: &Ctx) -> Script {
    snake02_tt_main_run(ctx, Snake02TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn snake02_tt_main_ontouch(ctx: &Ctx) -> Script {
    snake02_tt_main_run(ctx, Snake02TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Snake03TtMainStep {
    Start,
    OnTouch,
}

fn snake03_tt_main_run(ctx: &Ctx, mut step: Snake03TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_turbo2 = Val::from(0);
    'machine: loop {
        match step {
            Snake03TtMainStep::Start => {
                step = Snake03TtMainStep::OnTouch;
                continue 'machine;
            }
            Snake03TtMainStep::OnTouch => {
                l_turbo2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(8)])?;
                if l_turbo2.clone().number()? < 3 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(279), Val::from(292)],
                    )?;
                }
                if l_turbo2.clone().number()? < 5 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(311), Val::from(292)],
                    )?;
                }
                if l_turbo2.clone().number()? < 7 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(347), Val::from(256)],
                    )?;
                }
                if l_turbo2.clone().number()? < 9 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(370), Val::from(292)],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn snake03_tt_main(ctx: &Ctx) -> Script {
    snake03_tt_main_run(ctx, Snake03TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn snake03_tt_main_ontouch(ctx: &Ctx) -> Script {
    snake03_tt_main_run(ctx, Snake03TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Snake04TtMainStep {
    Start,
    OnTouch,
}

fn snake04_tt_main_run(ctx: &Ctx, mut step: Snake04TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_turbo2 = Val::from(0);
    'machine: loop {
        match step {
            Snake04TtMainStep::Start => {
                step = Snake04TtMainStep::OnTouch;
                continue 'machine;
            }
            Snake04TtMainStep::OnTouch => {
                l_turbo2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                if l_turbo2.clone().number()? < 3 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(363), Val::from(256)],
                    )?;
                }
                if l_turbo2.clone().number()? < 5 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(295), Val::from(293)],
                    )?;
                }
                if l_turbo2.clone().number()? < 7 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(355), Val::from(292)],
                    )?;
                }
                if l_turbo2.clone().number()? < 8 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(287), Val::from(256)],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn snake04_tt_main(ctx: &Ctx) -> Script {
    snake04_tt_main_run(ctx, Snake04TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn snake04_tt_main_ontouch(ctx: &Ctx) -> Script {
    snake04_tt_main_run(ctx, Snake04TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

fn snakehunt_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn snakehunt_tt_main(ctx: &Ctx) -> Script {
    snakehunt_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn snakehunt_tt_main_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from("All")],
    )?;
    return Err(Stop::End);
}

pub fn snakehunt_tt_main_onreset(ctx: &Ctx) -> Script {
    snakehunt_tt_main_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn snakehunt_tt_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_n_1: Vec<Val> = Vec::new();
    let mut l_n_1_s: Vec<Val> = Vec::new();
    let mut l_n_2: Vec<Val> = Vec::new();
    let mut l_n_2_s: Vec<Val> = Vec::new();
    let mut l_n_3: Vec<Val> = Vec::new();
    let mut l_n_3_s: Vec<Val> = Vec::new();
    if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from("snake")).is_true() {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(279), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(284), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(279), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(268), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(279), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(260), false);
        runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(287), false);
        runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(288), false);
        runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(287), false);
        runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(280), false);
        runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(287), false);
        runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(264), false);
        runtime::local_set(&mut l_n, &Val::from(base + 12), Val::from(295), false);
        runtime::local_set(&mut l_n, &Val::from(base + 13), Val::from(284), false);
        runtime::local_set(&mut l_n, &Val::from(base + 14), Val::from(295), false);
        runtime::local_set(&mut l_n, &Val::from(base + 15), Val::from(268), false);
        runtime::local_set(&mut l_n, &Val::from(base + 16), Val::from(295), false);
        runtime::local_set(&mut l_n, &Val::from(base + 17), Val::from(260), false);
        runtime::local_set(&mut l_n, &Val::from(base + 18), Val::from(303), false);
        runtime::local_set(&mut l_n, &Val::from(base + 19), Val::from(288), false);
        runtime::local_set(&mut l_n, &Val::from(base + 20), Val::from(303), false);
        runtime::local_set(&mut l_n, &Val::from(base + 21), Val::from(280), false);
        runtime::local_set(&mut l_n, &Val::from(base + 22), Val::from(303), false);
        runtime::local_set(&mut l_n, &Val::from(base + 23), Val::from(264), false);
        runtime::local_set(&mut l_n, &Val::from(base + 24), Val::from(311), false);
        runtime::local_set(&mut l_n, &Val::from(base + 25), Val::from(284), false);
        runtime::local_set(&mut l_n, &Val::from(base + 26), Val::from(311), false);
        runtime::local_set(&mut l_n, &Val::from(base + 27), Val::from(268), false);
        runtime::local_set(&mut l_n, &Val::from(base + 28), Val::from(311), false);
        runtime::local_set(&mut l_n, &Val::from(base + 29), Val::from(260), false);
        runtime::local_set(&mut l_n, &Val::from(base + 30), Val::from(347), false);
        runtime::local_set(&mut l_n, &Val::from(base + 31), Val::from(288), false);
        runtime::local_set(&mut l_n, &Val::from(base + 32), Val::from(347), false);
        runtime::local_set(&mut l_n, &Val::from(base + 33), Val::from(280), false);
        runtime::local_set(&mut l_n, &Val::from(base + 34), Val::from(347), false);
        runtime::local_set(&mut l_n, &Val::from(base + 35), Val::from(264), false);
        runtime::local_set(&mut l_n, &Val::from(base + 36), Val::from(355), false);
        runtime::local_set(&mut l_n, &Val::from(base + 37), Val::from(284), false);
        runtime::local_set(&mut l_n, &Val::from(base + 38), Val::from(355), false);
        runtime::local_set(&mut l_n, &Val::from(base + 39), Val::from(268), false);
        runtime::local_set(&mut l_n, &Val::from(base + 40), Val::from(355), false);
        runtime::local_set(&mut l_n, &Val::from(base + 41), Val::from(260), false);
        runtime::local_set(&mut l_n, &Val::from(base + 42), Val::from(363), false);
        runtime::local_set(&mut l_n, &Val::from(base + 43), Val::from(288), false);
        runtime::local_set(&mut l_n, &Val::from(base + 44), Val::from(363), false);
        runtime::local_set(&mut l_n, &Val::from(base + 45), Val::from(280), false);
        runtime::local_set(&mut l_n, &Val::from(base + 46), Val::from(363), false);
        runtime::local_set(&mut l_n, &Val::from(base + 47), Val::from(264), false);
        runtime::local_set(&mut l_n, &Val::from(base + 48), Val::from(371), false);
        runtime::local_set(&mut l_n, &Val::from(base + 49), Val::from(284), false);
        runtime::local_set(&mut l_n, &Val::from(base + 50), Val::from(371), false);
        runtime::local_set(&mut l_n, &Val::from(base + 51), Val::from(268), false);
        runtime::local_set(&mut l_n, &Val::from(base + 52), Val::from(371), false);
        runtime::local_set(&mut l_n, &Val::from(base + 53), Val::from(260), false);
        runtime::local_set(&mut l_n, &Val::from(base + 54), Val::from(379), false);
        runtime::local_set(&mut l_n, &Val::from(base + 55), Val::from(288), false);
        runtime::local_set(&mut l_n, &Val::from(base + 56), Val::from(379), false);
        runtime::local_set(&mut l_n, &Val::from(base + 57), Val::from(280), false);
        runtime::local_set(&mut l_n, &Val::from(base + 58), Val::from(379), false);
        runtime::local_set(&mut l_n, &Val::from(base + 59), Val::from(264), false);
        l_i = Val::from(0);
        'l1: loop {
            if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
                break 'l1;
            }
            'b1: {
                ctx.call(
                    Function::Monster,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        runtime::local_get(&l_n, &l_i.clone(), false),
                        runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                    ],
                )?;
            }
            l_i = (l_i.clone() + Val::from(2));
        }
    } else {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_1_s, &Val::from(base + 0), Val::from("Munak"), true);
        runtime::local_set(&mut l_n_1_s, &Val::from(base + 1), Val::from("1610"), true);
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_1, &Val::from(base + 0), Val::from(47), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 1), Val::from(87), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 2), Val::from(47), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 3), Val::from(87), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 4), Val::from(24), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 5), Val::from(74), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 6), Val::from(24), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 7), Val::from(74), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 8), Val::from(67), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 9), Val::from(42), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 10), Val::from(67), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 11), Val::from(42), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 12), Val::from(60), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 13), Val::from(70), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 14), Val::from(60), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 15), Val::from(70), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 16), Val::from(32), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 17), Val::from(51), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 18), Val::from(32), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 19), Val::from(51), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 20), Val::from(30), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 21), Val::from(25), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 22), Val::from(30), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 23), Val::from(25), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 24), Val::from(62), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 25), Val::from(20), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 26), Val::from(62), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 27), Val::from(20), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 28), Val::from(216), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 29), Val::from(378), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 30), Val::from(218), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 31), Val::from(360), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 32), Val::from(223), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 33), Val::from(361), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 34), Val::from(243), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 35), Val::from(342), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 36), Val::from(247), false);
        runtime::local_set(&mut l_n_1, &Val::from(base + 37), Val::from(364), false);
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_2_s, &Val::from(base + 0), Val::from("Bongun"), true);
        runtime::local_set(&mut l_n_2_s, &Val::from(base + 1), Val::from("1611"), true);
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_2, &Val::from(base + 0), Val::from(47), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 1), Val::from(87), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 2), Val::from(24), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 3), Val::from(74), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 4), Val::from(67), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 5), Val::from(42), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 6), Val::from(60), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 7), Val::from(70), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 8), Val::from(30), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 9), Val::from(25), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 10), Val::from(62), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 11), Val::from(20), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 12), Val::from(32), false);
        runtime::local_set(&mut l_n_2, &Val::from(base + 13), Val::from(51), false);
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_3_s, &Val::from(base + 0), Val::from("Yao Jun"), true);
        runtime::local_set(&mut l_n_3_s, &Val::from(base + 1), Val::from("1612"), true);
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_3, &Val::from(base + 0), Val::from(68), false);
        runtime::local_set(&mut l_n_3, &Val::from(base + 1), Val::from(56), false);
        runtime::local_set(&mut l_n_3, &Val::from(base + 2), Val::from(26), false);
        runtime::local_set(&mut l_n_3, &Val::from(base + 3), Val::from(46), false);
        'l2: loop {
            if !(l_c.clone().number()? < 3) {
                break 'l2;
            }
            'b2: {
                l_i = Val::from(0);
                'l3: loop {
                    if !(runtime::op(
                        &l_i.clone(),
                        "<",
                        &runtime::getd_size(
                            ctx,
                            &(Val::from(".@n_") + l_c.clone()),
                            &[
                                (".@c", runtime::Local::Scalar(&l_c)),
                                (".@i", runtime::Local::Scalar(&l_i)),
                                (".@n", runtime::Local::Array(&l_n)),
                                (".@n_1", runtime::Local::Array(&l_n_1)),
                                (".@n_1$", runtime::Local::Array(&l_n_1_s)),
                                (".@n_2", runtime::Local::Array(&l_n_2)),
                                (".@n_2$", runtime::Local::Array(&l_n_2_s)),
                                (".@n_3", runtime::Local::Array(&l_n_3)),
                                (".@n_3$", runtime::Local::Array(&l_n_3_s)),
                            ],
                        )?,
                    )?
                    .is_true())
                    {
                        break 'l3;
                    }
                    'b3: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                runtime::getd(
                                    ctx,
                                    &((((Val::from(".@n_") + l_c.clone()) + Val::from("[")) + l_i.clone()) + Val::from("]")),
                                    &[
                                        (".@c", runtime::Local::Scalar(&l_c)),
                                        (".@i", runtime::Local::Scalar(&l_i)),
                                        (".@n", runtime::Local::Array(&l_n)),
                                        (".@n_1", runtime::Local::Array(&l_n_1)),
                                        (".@n_1$", runtime::Local::Array(&l_n_1_s)),
                                        (".@n_2", runtime::Local::Array(&l_n_2)),
                                        (".@n_2$", runtime::Local::Array(&l_n_2_s)),
                                        (".@n_3", runtime::Local::Array(&l_n_3)),
                                        (".@n_3$", runtime::Local::Array(&l_n_3_s)),
                                    ],
                                )?,
                                runtime::getd(
                                    ctx,
                                    &((((Val::from(".@n_") + l_c.clone()) + Val::from("[")) + (l_i.clone() + Val::from(1)))
                                        + Val::from("]")),
                                    &[
                                        (".@c", runtime::Local::Scalar(&l_c)),
                                        (".@i", runtime::Local::Scalar(&l_i)),
                                        (".@n", runtime::Local::Array(&l_n)),
                                        (".@n_1", runtime::Local::Array(&l_n_1)),
                                        (".@n_1$", runtime::Local::Array(&l_n_1_s)),
                                        (".@n_2", runtime::Local::Array(&l_n_2)),
                                        (".@n_2$", runtime::Local::Array(&l_n_2_s)),
                                        (".@n_3", runtime::Local::Array(&l_n_3)),
                                        (".@n_3$", runtime::Local::Array(&l_n_3_s)),
                                    ],
                                )?,
                                runtime::getd(
                                    ctx,
                                    &((Val::from(".@n_") + l_c.clone()) + Val::from("$[0]")),
                                    &[
                                        (".@c", runtime::Local::Scalar(&l_c)),
                                        (".@i", runtime::Local::Scalar(&l_i)),
                                        (".@n", runtime::Local::Array(&l_n)),
                                        (".@n_1", runtime::Local::Array(&l_n_1)),
                                        (".@n_1$", runtime::Local::Array(&l_n_1_s)),
                                        (".@n_2", runtime::Local::Array(&l_n_2)),
                                        (".@n_2$", runtime::Local::Array(&l_n_2_s)),
                                        (".@n_3", runtime::Local::Array(&l_n_3)),
                                        (".@n_3$", runtime::Local::Array(&l_n_3_s)),
                                    ],
                                )?,
                                runtime::atoi(&runtime::getd(
                                    ctx,
                                    &((Val::from(".@n_") + l_c.clone()) + Val::from("$[1]")),
                                    &[
                                        (".@c", runtime::Local::Scalar(&l_c)),
                                        (".@i", runtime::Local::Scalar(&l_i)),
                                        (".@n", runtime::Local::Array(&l_n)),
                                        (".@n_1", runtime::Local::Array(&l_n_1)),
                                        (".@n_1$", runtime::Local::Array(&l_n_1_s)),
                                        (".@n_2", runtime::Local::Array(&l_n_2)),
                                        (".@n_2$", runtime::Local::Array(&l_n_2_s)),
                                        (".@n_3", runtime::Local::Array(&l_n_3)),
                                        (".@n_3$", runtime::Local::Array(&l_n_3_s)),
                                    ],
                                )?),
                                Val::from(1),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                l_c = (l_c.clone() + Val::from(1));
            }
        }
    }
    return Err(Stop::End);
}

pub fn snakehunt_tt_main_onenable(ctx: &Ctx) -> Script {
    snakehunt_tt_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CosTtMainStep {
    Start,
    OnTouch,
}

fn cos_tt_main_run(ctx: &Ctx, mut step: CosTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_n = Val::from(0);
    'machine: loop {
        match step {
            CosTtMainStep::Start => {
                step = CosTtMainStep::OnTouch;
                continue 'machine;
            }
            CosTtMainStep::OnTouch => {
                l_n = runtime::charat(
                    &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
                    &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
                )?;
                'b1: {
                    let subject1 = l_n.clone();
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1))
                        && !subject1.loosely_equals(&Val::from(2))
                        && !subject1.loosely_equals(&Val::from(3))
                        && !subject1.loosely_equals(&Val::from(5))
                        && !subject1.loosely_equals(&Val::from(6))
                        && !subject1.loosely_equals(&Val::from(7));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                    + Val::from(" has just passed the Log Bridge course!")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x70DBDB"),
                            ],
                        )?;
                        let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if subject2 == 1 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(210), Val::from(369)],
                            )?;
                            return Err(Stop::End);
                        } else if subject2 == 2 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(210), Val::from(361)],
                            )?;
                            return Err(Stop::End);
                        } else if subject2 == 3 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(210), Val::from(354)],
                            )?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                    + Val::from(" has just passed the Cube Hills course!")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x70DBDB"),
                            ],
                        )?;
                        ctx.call(
                            Function::Warp,
                            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(316), Val::from(365)],
                        )?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has just passed the Cursed Desert!")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x70DBDB"),
                            ],
                        )?;
                        let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                        if subject3 == 1 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(46), Val::from(254)],
                            )?;
                            return Err(Stop::End);
                        } else if subject3 == 2 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(76), Val::from(227)],
                            )?;
                            return Err(Stop::End);
                        } else if subject3 == 3 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(42), Val::from(197)],
                            )?;
                            return Err(Stop::End);
                        } else if subject3 == 4 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(86), Val::from(220)],
                            )?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(5)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                    + Val::from(" has just passed the Single Snail course!")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x70DBDB"),
                            ],
                        )?;
                        ctx.call(
                            Function::Warp,
                            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(268), Val::from(275)],
                        )?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(6)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                    + Val::from(" has just passed the Snake Dice course!")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x70DBDB"),
                            ],
                        )?;
                        ctx.call(
                            Function::Warp,
                            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(5), Val::from(91)],
                        )?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(7)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                    + Val::from(" has just passed the Small Cave course! Hurry, you're almost at the finish!")),
                                ctx.constant("BC_MAP")?,
                                Val::from("0x70DBDB"),
                            ],
                        )?;
                        let subject4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if subject4 == 1 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(307), Val::from(52)],
                            )?;
                            return Err(Stop::End);
                        } else if subject4 == 2 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(307), Val::from(46)],
                            )?;
                            return Err(Stop::End);
                        } else if subject4 == 3 {
                            ctx.call(
                                Function::Warp,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(307), Val::from(40)],
                            )?;
                            return Err(Stop::End);
                        }
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn cos_tt_main(ctx: &Ctx) -> Script {
    cos_tt_main_run(ctx, CosTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn cos_tt_main_ontouch(ctx: &Ctx) -> Script {
    cos_tt_main_run(ctx, CosTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Turbohint1TtMainStep {
    Start,
    OnTouch,
}

fn turbohint_1_tt_main_run(ctx: &Ctx, mut step: Turbohint1TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Turbohint1TtMainStep::Start => {
                step = Turbohint1TtMainStep::OnTouch;
                continue 'machine;
            }
            Turbohint1TtMainStep::OnTouch => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_QUESTION")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn turbohint_1_tt_main(ctx: &Ctx) -> Script {
    turbohint_1_tt_main_run(ctx, Turbohint1TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn turbohint_1_tt_main_ontouch(ctx: &Ctx) -> Script {
    turbohint_1_tt_main_run(ctx, Turbohint1TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Turbohint2TtMainStep {
    Start,
    OnTouch,
}

fn turbohint_2_tt_main_run(ctx: &Ctx, mut step: Turbohint2TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Turbohint2TtMainStep::Start => {
                step = Turbohint2TtMainStep::OnTouch;
                continue 'machine;
            }
            Turbohint2TtMainStep::OnTouch => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn turbohint_2_tt_main(ctx: &Ctx) -> Script {
    turbohint_2_tt_main_run(ctx, Turbohint2TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn turbohint_2_tt_main_ontouch(ctx: &Ctx) -> Script {
    turbohint_2_tt_main_run(ctx, Turbohint2TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Turbohint3TtMainStep {
    Start,
    OnTouch,
}

fn turbohint_3_tt_main_run(ctx: &Ctx, mut step: Turbohint3TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Turbohint3TtMainStep::Start => {
                step = Turbohint3TtMainStep::OnTouch;
                continue 'machine;
            }
            Turbohint3TtMainStep::OnTouch => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_PROFUSELY_SWEAT")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 3 {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn turbohint_3_tt_main(ctx: &Ctx) -> Script {
    turbohint_3_tt_main_run(ctx, Turbohint3TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn turbohint_3_tt_main_ontouch(ctx: &Ctx) -> Script {
    turbohint_3_tt_main_run(ctx, Turbohint3TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Turbohint4TtMainStep {
    Start,
    OnTouch,
}

fn turbohint_4_tt_main_run(ctx: &Ctx, mut step: Turbohint4TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Turbohint4TtMainStep::Start => {
                step = Turbohint4TtMainStep::OnTouch;
                continue 'machine;
            }
            Turbohint4TtMainStep::OnTouch => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_PROFUSELY_SWEAT")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn turbohint_4_tt_main(ctx: &Ctx) -> Script {
    turbohint_4_tt_main_run(ctx, Turbohint4TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn turbohint_4_tt_main_ontouch(ctx: &Ctx) -> Script {
    turbohint_4_tt_main_run(ctx, Turbohint4TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CosEndTtMainStep {
    Start,
    OnTouch,
    GetNumber,
    AfterGetNumber,
    OnInit,
}

fn cos_end_tt_main_run(ctx: &Ctx, mut step: CosEndTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_pts: Vec<Val> = Vec::new();
    let mut l_w_n_s: Vec<Val> = Vec::new();
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            CosEndTtMainStep::Start => {
                step = CosEndTtMainStep::OnTouch;
                continue 'machine;
            }
            CosEndTtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if l_w_s.clone() == "n1" {
                    ctx.var("$@end_time").set(ctx.call(Function::GetTimeTick, vec![Val::from(0)])?)?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                            + Val::from(" has just arrived at the Finish Line! Congratulations!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                if l_w_s.clone() != "n1" {
                    runtime::setd(
                        ctx,
                        &((Val::from("$ttnames$[") + cos_end_tt_main_run(ctx, CosEndTtMainStep::GetNumber, vec![l_w_s.clone()])?)
                            + Val::from("]")),
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        &mut [
                            (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                            (".@pts", runtime::LocalMut::Array(&mut l_pts)),
                            (".@w_n$", runtime::LocalMut::Array(&mut l_w_n_s)),
                            (".@w$", runtime::LocalMut::Scalar(&mut l_w_s)),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(Val::from("Turbo Track Guide::OnWin_") + l_w_s.clone())],
                    )?;
                }
                ctx.call(
                    Function::Warp,
                    vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(384), Val::from(161)],
                )?;
                if ((l_w_s.clone() == "e4" || l_w_s.clone() == "n4") || l_w_s.clone() == "n1") {
                    if l_w_s.clone() == "e4" {
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![((Val::from("Winner Helper#TBT_") + l_w_s.clone()) + Val::from("::OnEnable"))],
                        )?;
                    } else {
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![((Val::from("Guide#TBT_") + l_w_s.clone()) + Val::from("::OnEnable"))],
                        )?;
                    }
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![((Val::from("Master#") + l_w_s.clone()) + Val::from("::OnDisable"))],
                    )?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("Broadcast#") + l_w_s.clone())])?;
                } else {
                    if (l_w_s.clone() == "e8" || l_w_s.clone() == "n8") {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_pts, &Val::from(base + 0), Val::from(28951), false);
                        runtime::local_set(&mut l_pts, &Val::from(base + 1), Val::from(50), false);
                    }
                    if (l_w_s.clone() == "e16" || l_w_s.clone() == "n16") {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_pts, &Val::from(base + 0), Val::from(28941), false);
                        runtime::local_set(&mut l_pts, &Val::from(base + 1), Val::from(60), false);
                    }
                    if runtime::op(
                        &ctx.var("tt_point").get()?,
                        "<",
                        &runtime::local_get(&l_pts, &Val::from(0), false),
                    )?
                    .is_true()
                    {
                        ctx.var("tt_point")
                            .set((ctx.var("tt_point").get()? + runtime::local_get(&l_pts, &Val::from(1), false)))?;
                    }
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(384), Val::from(161)],
                    )?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Winner Helper#TBT_") + l_w_s.clone())])?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end2"))],
                    )?;
                }
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end"))],
                )?;
                return Err(Stop::End);
                step = CosEndTtMainStep::AfterGetNumber;
                continue 'machine;
            }
            CosEndTtMainStep::GetNumber => {
                let base = Val::from(1).number()?;
                runtime::local_set(&mut l_w_n_s, &Val::from(base + 0), Val::from("n4"), true);
                runtime::local_set(&mut l_w_n_s, &Val::from(base + 1), Val::from("n8"), true);
                runtime::local_set(&mut l_w_n_s, &Val::from(base + 2), Val::from("n16"), true);
                runtime::local_set(&mut l_w_n_s, &Val::from(base + 3), Val::from("e4"), true);
                runtime::local_set(&mut l_w_n_s, &Val::from(base + 4), Val::from("e8"), true);
                runtime::local_set(&mut l_w_n_s, &Val::from(base + 5), Val::from("e16"), true);
                l_i = Val::from(1);
                'l1: loop {
                    if !(runtime::op(&l_i.clone(), "<=", &Val::from(l_w_n_s.len() as i32))?.is_true()) {
                        break 'l1;
                    }
                    'b1: {
                        if runtime::arg(&args, 0, Val::from(0)).loosely_equals(&runtime::local_get(&l_w_n_s, &l_i.clone(), true)) {
                            return Ok(l_i.clone());
                        }
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Ok(Val::from(0));
            }
            CosEndTtMainStep::AfterGetNumber => {
                step = CosEndTtMainStep::OnInit;
                continue 'machine;
            }
            CosEndTtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::EnableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cos_end_tt_main(ctx: &Ctx) -> Script {
    cos_end_tt_main_run(ctx, CosEndTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn cos_end_tt_main_ontouch(ctx: &Ctx) -> Script {
    cos_end_tt_main_run(ctx, CosEndTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn cos_end_tt_main_oninit(ctx: &Ctx) -> Script {
    cos_end_tt_main_run(ctx, CosEndTtMainStep::OnInit, Vec::new()).map(|_| ())
}
