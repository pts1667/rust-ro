use script_sdk::{Context, Function, Request, Value};

const BROADCAST_MAP: i32 = 1;
const ANNOUNCE_COLOR: &str = "0x00ff00";
const WAITING_ROOM: (&str, i32, i32) = ("bat_room", 154, 150);
const CELL_WALKABLE: i32 = 0;
const CELL_SHOOTABLE: i32 = 1;
const WALL_LENGTH: i32 = 7;
const DIRECTION_NORTH_EAST: i32 = 5;

const PHASE_IDLE: i32 = 0;
const PHASE_PLAYING: i32 = 2;
const PHASE_OVER: i32 = 3;

const GUILLAUME: usize = 0;
const CROIX: usize = 1;

const ROLE_HOST: i32 = 0;
const ROLE_OFFICER: i32 = 1;

struct Camp {
    team: &'static str,
    lobby: (i32, i32),
    field: (i32, i32),
    points: (i32, i32),
}

struct Arena {
    map: &'static str,
    short: &'static str,
    officer_suffix: &'static str,
    win_points: i32,
    camps: [Camp; 2],
    walls: [(i32, i32, i32); 4],
}

const ARENAS: [Arena; 1] = [Arena {
    map: "bat_c01",
    short: "KvM01",
    officer_suffix: "KVM01",
    win_points: 5,
    camps: [
        Camp { team: "Guillaume", lobby: (53, 128), field: (61, 120), points: (5, 1) },
        Camp { team: "Croix", lobby: (146, 55), field: (138, 63), points: (5, 1) },
    ],
    walls: [(54, 122, 6), (55, 122, 5), (140, 56, 6), (140, 57, 5)],
}];

const START_ANNOUNCEMENTS: [(u32, &[&str]); 8] = [
    (10, &["In 1 minute, KVM will start."]),
    (11, &["The maximum time for a KVM battle is 5 minutes."]),
    (12, &["Please prepare for the KVM battle.", "You can buff your people."]),
    (13, &["30 seconds remaining to start KVM battle."]),
    (14, &["15 seconds remaining to start KVM battle."]),
    (15, &["10 seconds remaining to start KVM battle."]),
    (16, &["5 seconds remaining to start KVM battle."]),
    (17, &["KVM is now commencing."]),
];

const FINISH_ANNOUNCEMENTS: [(u32, &str); 5] = [
    (19, "1 minute remaining to finish the KVM battle."),
    (20, "30 seconds remaining to finish the KVM battle."),
    (21, "15 seconds remaining to finish the KVM battle."),
    (22, "10 seconds remaining to finish the KVM battle."),
    (23, "5 seconds remaining to finish the KVM battle."),
];

const OUT_ANNOUNCEMENTS: [(u32, &[&str]); 4] = [
    (31, &["Please apply with the Officer to acquire KVM points."]),
    (32, &["The Officer will grant you the points for 30 seconds.", "In 30 seconds, the Officer will be sent away."]),
    (33, &["Unless you talk to the Officer, you cannot gain the points.", "Please be careful."]),
    (34, &["You will be sent back."]),
];

impl Arena {
    fn host(&self) -> String {
        format!("{}_BG", self.short)
    }

    fn after_party(&self) -> String {
        format!("{}_BG_Out", self.short)
    }

    fn officer(&self, camp: usize) -> String {
        format!("KVM Officer#{}{}", self.officer_suffix, if camp == GUILLAUME { "A" } else { "B" })
    }

    fn variable(&self, suffix: &str) -> String {
        format!("$@{}BG{suffix}", self.short)
    }

    fn team_variable(&self, camp: usize) -> String {
        self.variable(&format!("_id{}", camp + 1))
    }

    fn count_variable(&self, camp: usize) -> String {
        format!("$@{}_{}_Count", self.short, self.camps[camp].team)
    }
}

fn number(ctx: &Context, function: Function, arguments: Vec<Value>) -> Result<i32, String> {
    ctx.call(function, arguments)?.number_value()
}

fn read(ctx: &Context, name: &str) -> Result<i32, String> {
    ctx.read(name)?.number_value()
}

fn announce(ctx: &Context, message: &str) -> Result<(), String> {
    ctx.call(Function::Announce, vec![message.into(), BROADCAST_MAP.into(), ANNOUNCE_COLOR.into()]).map(|_| ())
}

fn announce_all(ctx: &Context, messages: &[&str]) -> Result<(), String> {
    messages.iter().try_for_each(|message| announce(ctx, message))
}

fn set_enabled(ctx: &Context, npc: &str, enabled: bool) -> Result<(), String> {
    ctx.call(if enabled { Function::EnableNpc } else { Function::DisableNpc }, vec![npc.into()]).map(|_| ())
}

fn restart_timer(ctx: &Context, npc: &str) -> Result<(), String> {
    ctx.call(Function::StopNpcTimer, vec![npc.into()])?;
    ctx.call(Function::InitNpcTimer, vec![npc.into()]).map(|_| ())
}

fn warp_team(ctx: &Context, arena: &Arena, camp: usize, (x, y): (i32, i32)) -> Result<(), String> {
    let team = read(ctx, &arena.team_variable(camp))?;
    ctx.call(Function::BgWarp, vec![team.into(), arena.map.into(), x.into(), y.into()]).map(|_| ())
}

fn warp_everyone_out(ctx: &Context, arena: &Arena) -> Result<(), String> {
    ctx.call(Function::MapWarp, vec![arena.map.into(), WAITING_ROOM.0.into(), WAITING_ROOM.1.into(), WAITING_ROOM.2.into()]).map(|_| ())
}

fn disable_officers(ctx: &Context, arena: &Arena) -> Result<(), String> {
    [GUILLAUME, CROIX].iter().try_for_each(|camp| set_enabled(ctx, &arena.officer(*camp), false))
}

fn block_wall(ctx: &Context, arena: &Arena, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), String> {
    for cell in [CELL_WALKABLE, CELL_SHOOTABLE] {
        ctx.call(Function::SetCell, vec![arena.map.into(), x1.into(), y1.into(), x2.into(), y2.into(), cell.into(), 0.into()])?;
    }
    Ok(())
}

fn build_walls(ctx: &Context, arena: &Arena) -> Result<(), String> {
    for (x, y, direction) in arena.walls {
        let step_y = if direction == DIRECTION_NORTH_EAST { -1 } else { 0 };
        for step in 0..WALL_LENGTH {
            let (cell_x, cell_y) = (x + step, y + step * step_y);
            block_wall(ctx, arena, cell_x, cell_y, cell_x, cell_y)?;
        }
    }
    Ok(())
}

fn init(ctx: &Context, arena: &Arena) -> Result<(), String> {
    ctx.call(Function::BgUnbook, vec![arena.map.into()])?;
    warp_everyone_out(ctx, arena)?;
    build_walls(ctx, arena)?;
    disable_officers(ctx, arena)
}

fn report_counts(ctx: &Context, arena: &Arena) -> Result<(), String> {
    for camp in [GUILLAUME, CROIX] {
        let count = read(ctx, &arena.count_variable(camp))?;
        announce(ctx, &format!("The number of {}s is {count}.", arena.camps[camp].team))?;
    }
    Ok(())
}

fn publish_counts(ctx: &Context, arena: &Arena) -> Result<(), String> {
    let guillaume = read(ctx, &arena.count_variable(GUILLAUME))?;
    let croix = read(ctx, &arena.count_variable(CROIX))?;
    ctx.call(Function::BgUpdateScore, vec![arena.map.into(), guillaume.into(), croix.into()]).map(|_| ())
}

fn member_down(ctx: &Context, arena: &Arena, camp: usize) -> Result<(), String> {
    if read(ctx, &arena.variable(""))? != PHASE_PLAYING {
        return Ok(());
    }
    let name = arena.count_variable(camp);
    let remaining = read(ctx, &name)? - 1;
    ctx.write(&name, remaining.into())?;
    publish_counts(ctx, arena)?;
    if remaining < 1 {
        return win(ctx, arena, 1 - camp);
    }
    report_counts(ctx, arena)
}

fn start(ctx: &Context, arena: &Arena) -> Result<(), String> {
    disable_officers(ctx, arena)?;
    ctx.write(&arena.variable("_Victory"), 0.into())?;
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, arena, camp, arena.camps[camp].lobby)?;
    }
    restart_timer(ctx, &arena.host())
}

fn begin_fight(ctx: &Context, arena: &Arena) -> Result<(), String> {
    for camp in [GUILLAUME, CROIX] {
        let team = read(ctx, &arena.team_variable(camp))?;
        let members = number(ctx, Function::BgGetData, vec![team.into(), 0.into()])?;
        ctx.write(&arena.count_variable(camp), members.into())?;
    }
    publish_counts(ctx, arena)?;
    ctx.write(&arena.variable(""), PHASE_PLAYING.into())?;
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, arena, camp, arena.camps[camp].field)?;
    }
    Ok(())
}

fn win(ctx: &Context, arena: &Arena, camp: usize) -> Result<(), String> {
    let team = arena.camps[camp].team;
    ctx.write(&arena.variable(""), PHASE_OVER.into())?;
    ctx.write(&arena.variable("_Victory"), (camp as i32 + 1).into())?;
    announce_all(ctx, &[&format!("{team} wins!"), &format!("Congratulations to {team} members."), "Everyone will be moved to the start point."])?;
    stop(ctx, arena)
}

fn time_up(ctx: &Context, arena: &Arena) -> Result<(), String> {
    announce(ctx, "The KVM battle is over.")?;
    let guillaume = read(ctx, &arena.count_variable(GUILLAUME))?;
    let croix = read(ctx, &arena.count_variable(CROIX))?;
    match croix.cmp(&guillaume) {
        std::cmp::Ordering::Greater => win(ctx, arena, CROIX),
        std::cmp::Ordering::Less => win(ctx, arena, GUILLAUME),
        std::cmp::Ordering::Equal => {
            ctx.write(&arena.variable(""), PHASE_OVER.into())?;
            ctx.write(&arena.variable("_Victory"), 3.into())?;
            report_counts(ctx, arena)?;
            announce(ctx, "This battle has ended in a draw.")?;
            stop(ctx, arena)
        }
    }
}

fn stop(ctx: &Context, arena: &Arena) -> Result<(), String> {
    ctx.call(Function::StopNpcTimer, vec![arena.host().into()])?;
    for camp in [GUILLAUME, CROIX] {
        set_enabled(ctx, &arena.officer(camp), true)?;
    }
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, arena, camp, arena.camps[camp].lobby)?;
    }
    restart_timer(ctx, &arena.after_party())?;
    ctx.call(Function::BgReserve, vec![arena.map.into(), 1.into()]).map(|_| ())
}

fn host_timer(ctx: &Context, arena: &Arena, kind: u32) -> Result<(), String> {
    if let Some((_, messages)) = START_ANNOUNCEMENTS.iter().find(|(id, _)| *id == kind) {
        return announce_all(ctx, messages);
    }
    if let Some((_, message)) = FINISH_ANNOUNCEMENTS.iter().find(|(id, _)| *id == kind) {
        return announce(ctx, message);
    }
    match kind {
        18 => begin_fight(ctx, arena),
        24 => time_up(ctx, arena),
        _ => Err(format!("Unknown battleground arena event {kind}")),
    }
}

fn after_party_end(ctx: &Context, arena: &Arena) -> Result<(), String> {
    ctx.call(Function::StopNpcTimer, vec![arena.after_party().into()])?;
    ctx.call(Function::BgReserve, vec![arena.map.into(), 1.into()])?;
    warp_everyone_out(ctx, arena)?;
    for camp in [GUILLAUME, CROIX] {
        ctx.write(&arena.count_variable(camp), 0.into())?;
    }
    ctx.write(&arena.variable("_Victory"), 0.into())?;
    for camp in [GUILLAUME, CROIX] {
        let name = arena.team_variable(camp);
        let id = read(ctx, &name)?;
        if id != 0 {
            ctx.call(Function::BgDestroy, vec![id.into()])?;
            ctx.write(&name, 0.into())?;
        }
    }
    ctx.call(Function::BgUnbook, vec![arena.map.into()])?;
    disable_officers(ctx, arena)?;
    ctx.write(&arena.variable(""), PHASE_IDLE.into())
}

fn after_party_timer(ctx: &Context, arena: &Arena, kind: u32) -> Result<(), String> {
    if let Some((_, messages)) = OUT_ANNOUNCEMENTS.iter().find(|(id, _)| *id == kind) {
        return announce_all(ctx, messages);
    }
    match kind {
        35 => after_party_end(ctx, arena),
        _ => Err(format!("Unknown battleground arena event {kind}")),
    }
}

pub fn event(ctx: &Context, arena: usize, kind: u32) -> Result<(), String> {
    let arena = ARENAS.get(arena).ok_or("Unknown battleground arena")?;
    match kind {
        0 => init(ctx, arena),
        1 | 2 => member_down(ctx, arena, (kind - 1) as usize),
        3 | 4 => {
            let (x, y) = arena.camps[(kind - 3) as usize].field;
            ctx.call(Function::Warp, vec![arena.map.into(), x.into(), y.into()]).map(|_| ())
        }
        5 => start(ctx, arena),
        10..=24 => host_timer(ctx, arena, kind),
        30 => restart_timer(ctx, &arena.after_party()),
        31..=35 => after_party_timer(ctx, arena, kind),
        _ => Err(format!("Unknown battleground arena event {kind}")),
    }
}

fn arguments(ctx: &Context) -> Result<Vec<Value>, String> {
    match ctx.request(Request::Arguments)? {
        Value::Array(values) => Ok(values),
        _ => Err("NPC arguments are invalid".into()),
    }
}

fn officer(ctx: &Context, arena: &Arena, camp: usize) -> Result<(), String> {
    let victory = read(ctx, &arena.variable("_Victory"))?;
    if victory == 0 {
        return Ok(());
    }
    let battleground = number(ctx, Function::GetCharacterId, vec![4.into()])?;
    let own_team = read(ctx, &arena.team_variable(camp))?;
    let other_team = read(ctx, &arena.team_variable(1 - camp))?;
    if battleground == 0 || (battleground != own_team && battleground != other_team) {
        return Ok(());
    }
    let side = if battleground == own_team { camp } else { 1 - camp };
    let won = victory == side as i32 + 1;
    let points = if won { arena.win_points } else { arena.camps[side].points.1 };
    ctx.mes("[KVM Officer]")?;
    if won {
        ctx.mes(format!("Good Game.\nMay the glory of KVM be with you.\nYou aquire the winning points: {points}"))?;
    } else {
        ctx.mes(format!("I am so sorry.\nI wish you better luck next time.\nYou aquire the losing points: {points}"))?;
    }
    ctx.close()?;
    let total = read(ctx, "kvm_point")? + points;
    ctx.write("kvm_point", total.into())?;
    ctx.call(Function::BgLeave, vec![]).map(|_| ())
}

pub fn npc(ctx: &Context) -> Result<(), String> {
    let args = arguments(ctx)?;
    let arena_index = args.first().ok_or("Arena NPC needs an arena")?.number_value()? as usize;
    let role = args.get(1).ok_or("Arena NPC needs a role")?.number_value()?;
    let arena = ARENAS.get(arena_index).ok_or("Unknown battleground arena")?;
    match role {
        ROLE_HOST => Ok(()),
        role if (ROLE_OFFICER..ROLE_OFFICER + 2).contains(&role) => officer(ctx, arena, (role - ROLE_OFFICER) as usize),
        _ => Err(format!("Unknown battleground NPC role {role}")),
    }
}
