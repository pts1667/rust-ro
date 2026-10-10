use std::cmp::Ordering;

use script_sdk_2::constants::{CELL_SHOOTABLE, CELL_WALKABLE};
use script_sdk_2::{Area, Ctx, Script, Spot, Stop};

use crate::battleground_arena::{WAITING_ROOM, destroy_teams, read};

const ANNOUNCE_COLOR: &str = "0x00ff00";
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

fn announce(ctx: &Ctx, message: &str) -> Script {
    crate::battleground_arena::announce(ctx, message, ANNOUNCE_COLOR)
}

fn announce_all(ctx: &Ctx, messages: &[&str]) -> Script {
    messages.iter().try_for_each(|message| announce(ctx, message))
}

fn warp_team(ctx: &Ctx, arena: &Arena, camp: usize, (x, y): (i32, i32)) -> Script {
    let team = read(ctx, &arena.team_variable(camp))?;
    ctx.battleground().warp(team, Spot { map: arena.map, x, y })
}

fn disable_officers(ctx: &Ctx, arena: &Arena) -> Script {
    [GUILLAUME, CROIX].iter().try_for_each(|camp| ctx.set_npc_visible(&arena.officer(*camp), false))
}

/// Blocks the diagonal walls that keep the teams in their lobbies until the fight starts.
fn build_walls(ctx: &Ctx, arena: &Arena) -> Script {
    for (x, y, direction) in arena.walls {
        let step_y = if direction == DIRECTION_NORTH_EAST { -1 } else { 0 };
        for step in 0..WALL_LENGTH {
            let (x, y) = (x + step, y + step * step_y);
            for cell in [CELL_WALKABLE, CELL_SHOOTABLE] {
                ctx.set_cell(arena.map, Area { x1: x, y1: y, x2: x, y2: y }, cell, false)?;
            }
        }
    }
    Ok(())
}

fn init(ctx: &Ctx, arena: &Arena) -> Script {
    ctx.battleground().unbook(arena.map)?;
    ctx.map_warp(arena.map, WAITING_ROOM)?;
    build_walls(ctx, arena)?;
    disable_officers(ctx, arena)
}

fn report_counts(ctx: &Ctx, arena: &Arena) -> Script {
    for camp in [GUILLAUME, CROIX] {
        let count = read(ctx, &arena.count_variable(camp))?;
        announce(ctx, &format!("The number of {}s is {count}.", arena.camps[camp].team))?;
    }
    Ok(())
}

fn publish_counts(ctx: &Ctx, arena: &Arena) -> Script {
    let guillaume = read(ctx, &arena.count_variable(GUILLAUME))?;
    let croix = read(ctx, &arena.count_variable(CROIX))?;
    ctx.battleground().update_score(arena.map, guillaume, croix)
}

/// A member of `camp` died during the fight: the last one standing loses the battle for their camp.
fn member_down(ctx: &Ctx, arena: &Arena, camp: usize) -> Script {
    if read(ctx, &arena.variable(""))? != PHASE_PLAYING {
        return Ok(());
    }
    let count = arena.count_variable(camp);
    let remaining = read(ctx, &count)? - 1;
    ctx.var(&count).set(remaining)?;
    publish_counts(ctx, arena)?;
    if remaining < 1 {
        return win(ctx, arena, 1 - camp);
    }
    report_counts(ctx, arena)
}

fn start(ctx: &Ctx, arena: &Arena) -> Script {
    disable_officers(ctx, arena)?;
    ctx.var(&arena.variable("_Victory")).set(0)?;
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, arena, camp, arena.camps[camp].lobby)?;
    }
    ctx.timers().restart(Some(&arena.host()))
}

fn begin_fight(ctx: &Ctx, arena: &Arena) -> Script {
    for camp in [GUILLAUME, CROIX] {
        let team = read(ctx, &arena.team_variable(camp))?;
        let members = ctx.battleground().member_count(team)?;
        ctx.var(&arena.count_variable(camp)).set(members)?;
    }
    publish_counts(ctx, arena)?;
    ctx.var(&arena.variable("")).set(PHASE_PLAYING)?;
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, arena, camp, arena.camps[camp].field)?;
    }
    Ok(())
}

fn win(ctx: &Ctx, arena: &Arena, camp: usize) -> Script {
    let team = arena.camps[camp].team;
    ctx.var(&arena.variable("")).set(PHASE_OVER)?;
    ctx.var(&arena.variable("_Victory")).set(camp as i32 + 1)?;
    announce_all(ctx, &[&format!("{team} wins!"), &format!("Congratulations to {team} members."), "Everyone will be moved to the start point."])?;
    stop(ctx, arena)
}

/// The fight ran out of time: the camp with more members standing wins.
fn time_up(ctx: &Ctx, arena: &Arena) -> Script {
    announce(ctx, "The KVM battle is over.")?;
    let guillaume = read(ctx, &arena.count_variable(GUILLAUME))?;
    let croix = read(ctx, &arena.count_variable(CROIX))?;
    match croix.cmp(&guillaume) {
        Ordering::Greater => win(ctx, arena, CROIX),
        Ordering::Less => win(ctx, arena, GUILLAUME),
        Ordering::Equal => {
            ctx.var(&arena.variable("")).set(PHASE_OVER)?;
            ctx.var(&arena.variable("_Victory")).set(3)?;
            report_counts(ctx, arena)?;
            announce(ctx, "This battle has ended in a draw.")?;
            stop(ctx, arena)
        }
    }
}

fn stop(ctx: &Ctx, arena: &Arena) -> Script {
    ctx.timers().stop(Some(&arena.host()))?;
    for camp in [GUILLAUME, CROIX] {
        ctx.set_npc_visible(&arena.officer(camp), true)?;
    }
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, arena, camp, arena.camps[camp].lobby)?;
    }
    ctx.timers().restart(Some(&arena.after_party()))?;
    ctx.battleground().reserve(arena.map, true).map(|_| ())
}

fn host_timer(ctx: &Ctx, arena: &Arena, kind: u32) -> Script {
    if let Some((_, messages)) = START_ANNOUNCEMENTS.iter().find(|(id, _)| *id == kind) {
        return announce_all(ctx, messages);
    }
    if let Some((_, message)) = FINISH_ANNOUNCEMENTS.iter().find(|(id, _)| *id == kind) {
        return announce(ctx, message);
    }
    match kind {
        18 => begin_fight(ctx, arena),
        24 => time_up(ctx, arena),
        _ => Err(Stop::Error(format!("Unknown battleground arena event {kind}"))),
    }
}

/// The officers left, so the arena is emptied and freed for the next battle.
fn after_party_end(ctx: &Ctx, arena: &Arena) -> Script {
    ctx.timers().stop(Some(&arena.after_party()))?;
    ctx.battleground().reserve(arena.map, true)?;
    ctx.map_warp(arena.map, WAITING_ROOM)?;
    for camp in [GUILLAUME, CROIX] {
        ctx.var(&arena.count_variable(camp)).set(0)?;
    }
    ctx.var(&arena.variable("_Victory")).set(0)?;
    destroy_teams(ctx, [arena.team_variable(GUILLAUME), arena.team_variable(CROIX)])?;
    ctx.battleground().unbook(arena.map)?;
    disable_officers(ctx, arena)?;
    ctx.var(&arena.variable("")).set(PHASE_IDLE)
}

fn after_party_timer(ctx: &Ctx, arena: &Arena, kind: u32) -> Script {
    if let Some((_, messages)) = OUT_ANNOUNCEMENTS.iter().find(|(id, _)| *id == kind) {
        return announce_all(ctx, messages);
    }
    match kind {
        35 => after_party_end(ctx, arena),
        _ => Err(Stop::Error(format!("Unknown battleground arena event {kind}"))),
    }
}

/// Runs event `kind` of arena `arena`: the battleground queue and the arena's NPCs send these.
pub fn event(ctx: &Ctx, arena: usize, kind: u32) -> Script {
    let arena = ARENAS.get(arena).ok_or("Unknown battleground arena")?;
    match kind {
        0 => init(ctx, arena),
        1 | 2 => member_down(ctx, arena, (kind - 1) as usize),
        3 | 4 => {
            let (x, y) = arena.camps[(kind - 3) as usize].field;
            ctx.warp(arena.map, x, y)
        }
        5 => start(ctx, arena),
        10..=24 => host_timer(ctx, arena, kind),
        30 => ctx.timers().restart(Some(&arena.after_party())),
        31..=35 => after_party_timer(ctx, arena, kind),
        _ => Err(Stop::Error(format!("Unknown battleground arena event {kind}"))),
    }
}

/// Gives the KvM points of the battle to a member of either camp, then lets them leave.
fn officer(ctx: &Ctx, arena: &Arena, camp: usize) -> Script {
    let victory = read(ctx, &arena.variable("_Victory"))?;
    if victory == 0 {
        return Ok(());
    }
    let battleground = ctx.player().battleground_id()?;
    let own_team = read(ctx, &arena.team_variable(camp))?;
    let other_team = read(ctx, &arena.team_variable(1 - camp))?;
    if battleground == 0 || (battleground != own_team && battleground != other_team) {
        return Ok(());
    }
    let side = if battleground == own_team { camp } else { 1 - camp };
    let won = victory == side as i32 + 1;
    let points = if won { arena.win_points } else { arena.camps[side].points.1 };
    let farewell = if won {
        format!("Good Game.\nMay the glory of KVM be with you.\nYou aquire the winning points: {points}")
    } else {
        format!("I am so sorry.\nI wish you better luck next time.\nYou aquire the losing points: {points}")
    };
    ctx.mes_as("KVM Officer", &farewell)?;
    ctx.close_window()?;
    let total = read(ctx, "kvm_point")? + points;
    ctx.var("kvm_point").set(total)?;
    ctx.battleground().leave(None)
}

/// An NPC of the arena. Its placement gives the arena and the role.
pub fn npc(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let arena = arguments.first().ok_or("Arena NPC needs an arena")?.number()? as usize;
    let role = arguments.get(1).ok_or("Arena NPC needs a role")?.number()?;
    let arena = ARENAS.get(arena).ok_or("Unknown battleground arena")?;
    match role {
        ROLE_HOST => Ok(()),
        role if (ROLE_OFFICER..ROLE_OFFICER + 2).contains(&role) => officer(ctx, arena, (role - ROLE_OFFICER) as usize),
        _ => Err(Stop::Error(format!("Unknown battleground NPC role {role}"))),
    }
}
