use script_sdk_2::constants::{CELL_WALKABLE, EF_HEAL, EF_REPAIRWEAPON};
use script_sdk_2::{Area, Ctx, Script, Spot, Stop, args};

use crate::battleground_arena::{Reward, WAITING_ROOM, announce, destroy_teams, give_badges, heal_cycle, read, therapist};

const MAP: &str = "bat_a01";
const THERAPIST_PERIOD_MS: i32 = 26_500;
const REWARD: Reward = Reward { badge: 7828, win: 3, loss: 1 };
const STONE: i32 = 7049;
const STONES_REQUIRED: i32 = 50;
const BARRICADE: i32 = 1906;
const NEUTRALITY_FLAG: i32 = 1911;
const BARRICADE_COUNT_AFTER_BREAK: i32 = 17;
const NEUTRAL_SPAWN: (i32, i32) = (273, 203);
const NEUTRAL_RESPAWN: (i32, i32) = (56, 212);
const GUARDIANS: [(i32, i32, i32); 3] = [(272, 204, 1949), (272, 213, 1949), (273, 197, 1950)];
const GHOST: &str = "Valley Ghost#bat_a01_n";
const GHOST_AREA: Area = Area { x1: 45, y1: 203, x2: 65, y2: 223 };
const GHOST_WARP: (i32, i32) = (301, 209);
const ANNOUNCE_COLOR: &str = "0xFFCE00";

const GUILLAUME: usize = 0;
const CROIX: usize = 1;

/// Invisible touch areas across the valley barricades: `(owning camp, exit cell)`.
const GATES: [(usize, (i32, i32)); 4] = [(GUILLAUME, (194, 261)), (GUILLAUME, (194, 270)), (CROIX, (178, 125)), (CROIX, (178, 134))];

const ROLE_DECORATION: i32 = 0;
const ROLE_THERAPIST: i32 = 1;
const ROLE_GHOST: i32 = 2;
const ROLE_VINTENAR: i32 = 3;
const ROLE_BLACKSMITH: i32 = 5;

struct Camp {
    team: &'static str,
    officer: &'static str,
    food: (&'static str, i32, i32, i32),
    barricade_row: i32,
    barricade_start: i32,
    hall: (i32, i32),
    base: (i32, i32),
    heal_area: Area,
    vintenar: &'static str,
    blacksmith: &'static str,
    therapist: &'static str,
    guard_message: &'static str,
}

const CAMPS: [Camp; 2] = [
    Camp {
        team: "Guillaume",
        officer: "Axl Rose",
        food: ("Food Storage", 1909, 177, 345),
        barricade_row: 266,
        barricade_start: 185,
        hall: (50, 374),
        base: (352, 342),
        heal_area: Area { x1: 41, y1: 365, x2: 61, y2: 385 },
        vintenar: "Guillaume Vintenar#a01_a",
        blacksmith: "Guillaume Blacksmith#a01",
        therapist: "Battle Therapist#a01_a",
        guard_message: "Guillaume Vintenar Axl Rose : The Barricade in the valley has been destroyed! Where's the Blacksmith? We need to rebuild the Barricade!",
    },
    Camp {
        team: "Croix",
        officer: "Swandery",
        food: ("Food Depot", 1910, 167, 50),
        barricade_row: 129,
        barricade_start: 169,
        hall: (42, 16),
        base: (353, 52),
        heal_area: Area { x1: 33, y1: 7, x2: 53, y2: 27 },
        vintenar: "Croix Vintenar#a01_b",
        blacksmith: "Croix Blacksmith#bat_a01",
        therapist: "Battle Therapist#a01_b",
        guard_message: "Croix Vintenar Swandery : The Barricade in the valley has been destroyed! Where's the Blacksmith? We need to rebuild the Barricade!",
    },
];

const COUNTDOWN: [(&str, &str); 8] = [
    ("Guillaume Vintenar Axl Rose : Let's attack to burn down Croix's Food Depot!", "0xFF9900"),
    ("Croix Vintenar Swandery : Master of Valhalla! Let us be gifted with unfailing faith and courage!", "0xFF99CC"),
    ("Marollo VII : Guillaume Marollo, Croix Marollo! Marollo followers!", "0x99CC00"),
    ("Marollo VII : Both camps are competitive, so no camp would be destroyed easily. That means the Marollo kingdoms will never be defeated!", "0x99CC00"),
    ("Marollo VII : I think we'd better terminate the battle, and call it a draw.", "0x99CC00"),
    ("Marollo VII : Hold your royalty and faith for a moment, and let's settle up the battle of Tierra Gorge.", "0x99CC00"),
    ("Axl Rose, Swandery : Yes sir.", "0x99CC00"),
    ("", ""),
];

fn variable(suffix: &str) -> String {
    format!("$@TierraBG1{suffix}")
}

fn team_variable(camp: usize) -> String {
    variable(&format!("_id{}", camp + 1))
}

fn food_label(camp: usize) -> String {
    format!("OBJ#{MAP}_{}::OnMyMobDead", if camp == GUILLAUME { "a" } else { "b" })
}

fn barricade_label(camp: usize) -> String {
    format!("barricade#{MAP}_{}::OnMyMobDead", if camp == GUILLAUME { "a" } else { "b" })
}

fn neutral_label() -> String {
    format!("OBJ#{MAP}_n::OnMyMobDead")
}

fn neutral_guard_label() -> String {
    format!("NOBJ_mob#{MAP}_a::OnMyMobDead")
}

fn countdown() -> String {
    format!("countdown#{MAP}")
}

fn cleanup_timer() -> String {
    format!("#{MAP}_timer")
}

fn start_npc() -> String {
    format!("start#{MAP}")
}

fn spot(x: i32, y: i32) -> Spot<'static> {
    Spot { map: MAP, x, y }
}

fn spawn(ctx: &Ctx, team: i32, x: i32, y: i32, name: &str, class: i32, label: &str) -> Script {
    ctx.battleground().monster(team, spot(x, y), name, class, Some(label)).map(|_| ())
}

fn warp_team(ctx: &Ctx, camp: usize, (x, y): (i32, i32)) -> Script {
    let team = read(ctx, &team_variable(camp))?;
    ctx.battleground().warp(team, spot(x, y))
}

/// Blocks the row of a camp's valley barricade, or opens it once the barricade falls.
fn set_barricade_wall(ctx: &Ctx, camp: usize, blocked: bool) -> Script {
    let data = &CAMPS[camp];
    let row = Area { x1: data.barricade_start + 1, y1: data.barricade_row, x2: data.barricade_start + 16, y2: data.barricade_row };
    ctx.set_cell(MAP, row, CELL_WALKABLE, !blocked)
}

fn spawn_barricade(ctx: &Ctx, camp: usize) -> Script {
    let data = &CAMPS[camp];
    let team = read(ctx, &team_variable(camp))?;
    let label = barricade_label(camp);
    for x in data.barricade_start..data.barricade_start + BARRICADE_COUNT_AFTER_BREAK {
        spawn(ctx, team, x, data.barricade_row, "Barricade", BARRICADE, &label)?;
    }
    set_barricade_wall(ctx, camp, true)
}

fn clear_barricade(ctx: &Ctx, camp: usize) -> Script {
    ctx.kill_monster(MAP, &barricade_label(camp))?;
    set_barricade_wall(ctx, camp, false)
}

fn therapists(ctx: &Ctx, running: bool) -> Script {
    for camp in &CAMPS {
        if running {
            ctx.timers().restart(Some(camp.therapist))?;
        } else {
            ctx.timers().stop(Some(camp.therapist))?;
        }
        ctx.set_npc_visible(camp.therapist, running)?;
    }
    Ok(())
}

fn reset_objectives(ctx: &Ctx) -> Script {
    for camp in [GUILLAUME, CROIX] {
        ctx.kill_monster(MAP, &food_label(camp))?;
        let (name, class, x, y) = CAMPS[camp].food;
        spawn(ctx, read(ctx, &team_variable(camp))?, x, y, name, class, &food_label(camp))?;
    }
    for camp in [GUILLAUME, CROIX] {
        clear_barricade(ctx, camp)?;
        spawn_barricade(ctx, camp)?;
    }
    ctx.kill_monster(MAP, &neutral_label())?;
    ctx.monster(MAP, NEUTRAL_SPAWN.0, NEUTRAL_SPAWN.1, "Neutrality Flag", NEUTRALITY_FLAG, 1, Some(&neutral_label()))?;
    ctx.kill_monster(MAP, &neutral_guard_label())
}

fn ready_check(ctx: &Ctx) -> Script {
    if read(ctx, &variable(""))? != 0 {
        return Ok(());
    }
    ctx.var(&variable("")).set(1)?;
    reset_objectives(ctx)?;
    therapists(ctx, true)?;
    ctx.timers().restart(Some(GHOST))?;
    ctx.timers().restart(Some(&countdown()))?;
    for camp in &CAMPS {
        ctx.set_npc_visible(camp.blacksmith, false)?;
        ctx.set_npc_visible(camp.vintenar, false)?;
    }
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, camp, CAMPS[camp].base)?;
    }
    ctx.timers().restart(Some(&start_npc()))
}

/// A camp's food storage died, which ends the battle in the other camp's favour.
fn food_destroyed(ctx: &Ctx, victim: usize) -> Script {
    if ctx.mob_count(MAP, &food_label(victim))? >= 1 {
        return Ok(());
    }
    therapists(ctx, false)?;
    ctx.var(&variable("_Victory")).set(2 - victim as i32)?;
    for camp in &CAMPS {
        ctx.set_npc_visible(camp.vintenar, true)?;
    }
    let message = if victim == GUILLAUME {
        "Croix Vintenar Swandery: We destroyed Guillaume's Food Storage. We won that! Wow!"
    } else {
        "Guillaume Vintenar Axl Rose : We destroyed Croix's Food Storage. We won that! Wow!"
    };
    announce(ctx, message, ANNOUNCE_COLOR)?;
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, camp, CAMPS[camp].hall)?;
    }
    ctx.battleground().reserve(MAP, true).map(|_| ())
}

fn barricade_destroyed(ctx: &Ctx, camp: usize) -> Script {
    if ctx.mob_count(MAP, &barricade_label(camp))? >= BARRICADE_COUNT_AFTER_BREAK {
        return Ok(());
    }
    clear_barricade(ctx, camp)?;
    ctx.set_npc_visible(CAMPS[camp].blacksmith, true)?;
    announce(ctx, CAMPS[camp].guard_message, ANNOUNCE_COLOR)
}

/// The camp that captured the neutrality flag respawns closer to the front, guarded by camp guardians.
fn neutral_flag_captured(ctx: &Ctx) -> Script {
    let capturer = ctx.player().battleground_id()?;
    let mut owner = None;
    for camp in [GUILLAUME, CROIX] {
        if capturer != 0 && read(ctx, &team_variable(camp)).is_ok_and(|team| team == capturer) {
            owner = Some(camp);
            break;
        }
    }
    let Some(camp) = owner else {
        return Ok(());
    };
    ctx.battleground().set_cemetery(capturer, NEUTRAL_RESPAWN.0, NEUTRAL_RESPAWN.1)?;
    ctx.kill_monster(MAP, &neutral_guard_label())?;
    let team = CAMPS[camp].team;
    for (x, y, class) in GUARDIANS {
        spawn(ctx, capturer, x, y, &format!("{team} Camp Guardian"), class, &neutral_guard_label())?;
    }
    announce(ctx, &format!("{team} captured a Neutrality Flag, so they have an advantage."), ANNOUNCE_COLOR)
}

fn countdown_event(ctx: &Ctx, kind: u32) -> Script {
    let (message, color) = COUNTDOWN[(kind - 40) as usize];
    if message.is_empty() {
        ctx.map_warp(MAP, WAITING_ROOM)?;
        return ctx.timers().stop(Some(&countdown()));
    }
    announce(ctx, message, color)
}

/// Polls until both teams are empty, then frees the arena for the next battle.
fn cleanup_poll(ctx: &Ctx) -> Script {
    let cleanup = cleanup_timer();
    ctx.timers().stop(Some(&cleanup))?;
    let occupied = |camp: usize| -> Result<bool, Stop> {
        let team = read(ctx, &team_variable(camp))?;
        Ok(team != 0 && ctx.battleground().member_count(team)? != 0)
    };
    if occupied(GUILLAUME)? || occupied(CROIX)? {
        return ctx.timers().init(Some(&cleanup));
    }
    ctx.timers().stop(Some(&countdown()))?;
    ctx.timers().stop(Some(GHOST))?;
    ctx.battleground().reserve(MAP, true)?;
    ctx.var(&variable("")).set(0)?;
    ctx.var(&variable("_Victory")).set(0)?;
    destroy_teams(ctx, [team_variable(GUILLAUME), team_variable(CROIX)])?;
    ctx.battleground().unbook(MAP).map(|_| ())
}

/// Runs event `kind` of Tierra Gorge: the battleground queue and the valley's NPCs and monsters send these.
pub fn event(ctx: &Ctx, arena: usize, kind: u32) -> Script {
    if arena != 0 {
        return Err("Unknown battleground arena".into());
    }
    match kind {
        0 => {
            ctx.battleground().unbook(MAP)?;
            ctx.map_warp(MAP, WAITING_ROOM)
        }
        1 => ready_check(ctx),
        2 | 3 => {
            let (x, y) = CAMPS[(kind - 2) as usize].base;
            ctx.warp(MAP, x, y)
        }
        5 => {
            ctx.timers().stop(Some(&start_npc()))?;
            ctx.timers().init(Some(&cleanup_timer()))
        }
        10 | 11 => food_destroyed(ctx, (kind - 10) as usize),
        12 => neutral_flag_captured(ctx),
        14 | 15 => barricade_destroyed(ctx, (kind - 14) as usize),
        20..=23 => {
            let camp = &CAMPS[((kind - 20) / 2) as usize];
            let timer = if kind % 2 == 0 { 25_000 } else { THERAPIST_PERIOD_MS };
            heal_cycle(ctx, MAP, camp.therapist, camp.heal_area, camp.base, timer)
        }
        24 | 25 => heal_cycle(ctx, MAP, GHOST, GHOST_AREA, GHOST_WARP, if kind == 24 { 25_000 } else { THERAPIST_PERIOD_MS }),
        30 | 31 => ctx.set_npc_visible(CAMPS[(kind - 30) as usize].vintenar, false),
        32 | 33 => ctx.set_npc_visible(CAMPS[(kind - 32) as usize].blacksmith, false),
        40..=47 => countdown_event(ctx, kind),
        50 => cleanup_poll(ctx),
        60..=63 => barricade_gate(ctx, (kind - 60) as usize),
        _ => Err(Stop::Error(format!("Unknown battleground arena event {kind}"))),
    }
}

/// Lets the members of the gate's camp through their own barricade.
fn barricade_gate(ctx: &Ctx, gate: usize) -> Script {
    let (camp, (x, y)) = GATES[gate];
    let own_team = read(ctx, &team_variable(camp))?;
    if own_team != 0 && ctx.player().battleground_id()? == own_team {
        ctx.warp(MAP, x, y)?;
    }
    Ok(())
}

fn vintenar(ctx: &Ctx, camp: usize) -> Script {
    let battleground = ctx.player().battleground_id()?;
    let guillaume = read(ctx, &team_variable(GUILLAUME))?;
    let croix = read(ctx, &team_variable(CROIX))?;
    let own = if battleground != 0 && battleground == guillaume {
        GUILLAUME
    } else if battleground != 0 && battleground == croix {
        CROIX
    } else {
        return Ok(());
    };
    let victory = read(ctx, &variable("_Victory"))?;
    let data = &CAMPS[camp];
    give_badges(ctx, data.team, data.officer, victory == own as i32 + 1, &REWARD)?;
    ctx.battleground().leave(None)
}

/// Rebuilds the camp's barricade for 50 stones.
fn blacksmith(ctx: &Ctx, camp: usize) -> Script {
    let data = &CAMPS[camp];
    let own_team = read(ctx, &team_variable(camp))?;
    let title = format!("{} Blacksmith", data.team);
    if own_team == 0 || ctx.player().battleground_id()? != own_team {
        ctx.mes_as(&title, "There the enemy is coming!")?;
        return ctx.close();
    }
    ctx.lines_as(title.as_str(), args![
        "We are in urgency! The Barricade has been destroyed!",
        "We can repair the Barricade with ^3131FF50 Stones, 3 Sinew of Bear, 500 Metal Fragments, 30 Rough Elunium and 100 Gold.^000000",
        "We have it all except for the 50 Stones!",
    ])?;
    ctx.next()?;
    if ctx.menu(&["Repair.", "Leave it."])? == 1 {
        ctx.mes_as(&title, "There are enemies coming! Let's evacuate from here!")?;
        return ctx.close();
    }
    if ctx.items().count(STONE)? < STONES_REQUIRED {
        ctx.mes_as(&title, "You don't have enough Stones!")?;
        ctx.next()?;
        ctx.mes_as(&title, &format!("^3131FFWe need {STONES_REQUIRED} Stones.^000000\nWe are busy, so please hurry."))?;
        return ctx.close();
    }
    ctx.mes_as(&title, "You brought enough stones! Let's go and repair.")?;
    ctx.next()?;
    ctx.mes_as(&title, "Combine Stones and Gold in the proper percentage and shape the Barricade, then add Rough Elunium to make it stronger. Decorate with Metal Fragments, and plait stones with Sinew of Bear!")?;
    ctx.next()?;
    ctx.fx().special_effect(EF_REPAIRWEAPON)?;
    ctx.mes_as(&title, "Wow! It's done.\nWe are relieved.")?;
    if ctx.mob_count(MAP, &barricade_label(camp))? < BARRICADE_COUNT_AFTER_BREAK {
        ctx.items().take(STONE, STONES_REQUIRED)?;
        clear_barricade(ctx, camp)?;
        spawn_barricade(ctx, camp)?;
    }
    ctx.close_window()?;
    ctx.set_npc_visible(data.blacksmith, false)
}

fn ghost(ctx: &Ctx) -> Script {
    ctx.fx().special_effect(EF_HEAL)?;
    ctx.mes_as("Valley Ghost", "Boo...Boo...")?;
    ctx.close()
}

/// An NPC of Tierra Gorge. Its placement gives the arena and the role.
pub fn npc(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let arena = arguments.first().ok_or("Arena NPC needs an arena")?.number()?;
    let role = arguments.get(1).ok_or("Arena NPC needs a role")?.number()?;
    if arena != 0 {
        return Err("Unknown battleground arena".into());
    }
    match role {
        ROLE_DECORATION => Ok(()),
        ROLE_THERAPIST => therapist(ctx),
        ROLE_GHOST => ghost(ctx),
        role if (ROLE_VINTENAR..ROLE_VINTENAR + 2).contains(&role) => vintenar(ctx, (role - ROLE_VINTENAR) as usize),
        role if (ROLE_BLACKSMITH..ROLE_BLACKSMITH + 2).contains(&role) => blacksmith(ctx, (role - ROLE_BLACKSMITH) as usize),
        _ => Err(Stop::Error(format!("Unknown battleground NPC role {role}"))),
    }
}
