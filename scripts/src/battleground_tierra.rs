use script_sdk::{Context, Function, Value};

use crate::battleground_arena::{Reward, announce, arguments, give_badges, number, read, restart_timer, set_enabled, therapist};

const MAP: &str = "bat_a01";
const WAITING_ROOM: (&str, i32, i32) = ("bat_room", 154, 150);
const THERAPIST_PERIOD_MS: i32 = 26_500;
const REWARD: Reward = Reward { badge: 7828, win: 3, loss: 1 };
const STONE: i32 = 7049;
const STONES_REQUIRED: i32 = 50;
const EF_REPAIR_WEAPON: i32 = 101;
const EF_SANCTUARY: i32 = 83;
const EF_HEAL: i32 = 312;
const BARRICADE: i32 = 1906;
const BARRICADE_COUNT_AFTER_BREAK: i32 = 17;
const NEUTRAL_SPAWN: (i32, i32) = (273, 203);
const NEUTRAL_RESPAWN: (i32, i32) = (56, 212);
const GUARDIANS: [(i32, i32, i32); 3] = [(272, 204, 1949), (272, 213, 1949), (273, 197, 1950)];
const GHOST: &str = "Valley Ghost#bat_a01_n";
const GHOST_AREA: [i32; 4] = [45, 203, 65, 223];
const GHOST_WARP: (i32, i32) = (301, 209);
const ANNOUNCE_COLOR: &str = "0xFFCE00";

const GUILLAUME: usize = 0;
const CROIX: usize = 1;

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
    heal_area: [i32; 4],
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
        heal_area: [41, 365, 61, 385],
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
        heal_area: [33, 7, 53, 27],
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

fn spawn(ctx: &Context, team: i32, x: i32, y: i32, name: &str, class: i32, label: &str) -> Result<(), String> {
    ctx.call(Function::BgMonster, vec![team.into(), MAP.into(), x.into(), y.into(), name.into(), class.into(), label.into()]).map(|_| ())
}

fn kill(ctx: &Context, label: String) -> Result<(), String> {
    ctx.call(Function::KillMonster, vec![MAP.into(), label.into()]).map(|_| ())
}

fn warp_team(ctx: &Context, camp: usize, (x, y): (i32, i32)) -> Result<(), String> {
    let team = read(ctx, &team_variable(camp))?;
    ctx.call(Function::BgWarp, vec![team.into(), MAP.into(), x.into(), y.into()]).map(|_| ())
}

fn set_barricade_wall(ctx: &Context, camp: usize, blocked: bool) -> Result<(), String> {
    let data = &CAMPS[camp];
    let (x1, x2) = (data.barricade_start + 1, data.barricade_start + 16);
    ctx.call(Function::SetCell, vec![
        MAP.into(),
        x1.into(),
        data.barricade_row.into(),
        x2.into(),
        data.barricade_row.into(),
        0.into(),
        i32::from(!blocked).into(),
    ])
    .map(|_| ())
}

fn spawn_barricade(ctx: &Context, camp: usize) -> Result<(), String> {
    let data = &CAMPS[camp];
    let team = read(ctx, &team_variable(camp))?;
    let label = barricade_label(camp);
    for x in data.barricade_start..data.barricade_start + BARRICADE_COUNT_AFTER_BREAK {
        spawn(ctx, team, x, data.barricade_row, "Barricade", BARRICADE, &label)?;
    }
    set_barricade_wall(ctx, camp, true)
}

fn clear_barricade(ctx: &Context, camp: usize) -> Result<(), String> {
    kill(ctx, barricade_label(camp))?;
    set_barricade_wall(ctx, camp, false)
}

fn restart_ghost(ctx: &Context) -> Result<(), String> {
    restart_timer(ctx, GHOST)
}

fn therapists(ctx: &Context, running: bool) -> Result<(), String> {
    for camp in &CAMPS {
        if running {
            restart_timer(ctx, camp.therapist)?;
        } else {
            ctx.call(Function::StopNpcTimer, vec![camp.therapist.into()])?;
        }
        set_enabled(ctx, camp.therapist, running)?;
    }
    Ok(())
}

fn reset_objectives(ctx: &Context) -> Result<(), String> {
    for camp in [GUILLAUME, CROIX] {
        kill(ctx, food_label(camp))?;
        let (name, class, x, y) = CAMPS[camp].food;
        spawn(ctx, read(ctx, &team_variable(camp))?, x, y, name, class, &food_label(camp))?;
    }
    for camp in [GUILLAUME, CROIX] {
        clear_barricade(ctx, camp)?;
        spawn_barricade(ctx, camp)?;
    }
    kill(ctx, neutral_label())?;
    ctx.call(Function::Monster, vec![
        MAP.into(),
        NEUTRAL_SPAWN.0.into(),
        NEUTRAL_SPAWN.1.into(),
        "Neutrality Flag".into(),
        1911.into(),
        1.into(),
        neutral_label().into(),
    ])?;
    kill(ctx, neutral_guard_label())
}

fn ready_check(ctx: &Context) -> Result<(), String> {
    if read(ctx, &variable(""))? != 0 {
        return Ok(());
    }
    ctx.write(&variable(""), 1.into())?;
    reset_objectives(ctx)?;
    therapists(ctx, true)?;
    restart_ghost(ctx)?;
    restart_timer(ctx, &countdown())?;
    for camp in &CAMPS {
        set_enabled(ctx, camp.blacksmith, false)?;
        set_enabled(ctx, camp.vintenar, false)?;
    }
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, camp, CAMPS[camp].base)?;
    }
    restart_timer(ctx, &start_npc())
}

fn food_destroyed(ctx: &Context, victim: usize) -> Result<(), String> {
    if number(ctx, Function::MobCount, vec![MAP.into(), food_label(victim).into()])? >= 1 {
        return Ok(());
    }
    therapists(ctx, false)?;
    ctx.write(&variable("_Victory"), (2 - victim as i32).into())?;
    for camp in &CAMPS {
        set_enabled(ctx, camp.vintenar, true)?;
    }
    let message = if victim == GUILLAUME {
        "Croix Vintenar Swandery: We destroyed Guillaume's Food Storage. We won that! Wow!"
    } else {
        "Guillaume Vintenar Axl Rose : We destroyed Croix's Food Storage. We won that! Wow!"
    };
    ctx.call(Function::Announce, vec![message.into(), 1.into(), ANNOUNCE_COLOR.into()])?;
    for camp in [GUILLAUME, CROIX] {
        warp_team(ctx, camp, CAMPS[camp].hall)?;
    }
    ctx.call(Function::BgReserve, vec![MAP.into(), 1.into()]).map(|_| ())
}

fn barricade_destroyed(ctx: &Context, camp: usize) -> Result<(), String> {
    if number(ctx, Function::MobCount, vec![MAP.into(), barricade_label(camp).into()])? >= BARRICADE_COUNT_AFTER_BREAK {
        return Ok(());
    }
    clear_barricade(ctx, camp)?;
    set_enabled(ctx, CAMPS[camp].blacksmith, true)?;
    ctx.call(Function::Announce, vec![CAMPS[camp].guard_message.into(), 1.into(), ANNOUNCE_COLOR.into()]).map(|_| ())
}

fn neutral_flag_captured(ctx: &Context) -> Result<(), String> {
    let capturer = number(ctx, Function::GetCharacterId, vec![4.into()])?;
    let Some(camp) = [GUILLAUME, CROIX].into_iter().find(|camp| capturer != 0 && read(ctx, &team_variable(*camp)).is_ok_and(|id| id == capturer)) else {
        return Ok(());
    };
    ctx.call(Function::BgTeamSetXy, vec![capturer.into(), NEUTRAL_RESPAWN.0.into(), NEUTRAL_RESPAWN.1.into()])?;
    kill(ctx, neutral_guard_label())?;
    let team = CAMPS[camp].team;
    for (x, y, class) in GUARDIANS {
        spawn(ctx, capturer, x, y, &format!("{team} Camp Guardian"), class, &neutral_guard_label())?;
    }
    announce(ctx, &format!("{team} captured a Neutrality Flag, so they have an advantage."), ANNOUNCE_COLOR)
}

fn heal_cycle(ctx: &Context, npc: &str, area: [i32; 4], destination: (i32, i32), timer: i32) -> Result<(), String> {
    if timer == THERAPIST_PERIOD_MS {
        restart_timer(ctx, npc)?;
        return set_enabled(ctx, npc, true);
    }
    let [x1, y1, x2, y2] = area;
    ctx.call(Function::SpecialEffect, vec![EF_SANCTUARY.into()])?;
    ctx.call(Function::AreaPercentHeal, vec![MAP.into(), x1.into(), y1.into(), x2.into(), y2.into(), 100.into(), 100.into()])?;
    ctx.call(Function::AreaWarp, vec![
        MAP.into(),
        x1.into(),
        y1.into(),
        x2.into(),
        y2.into(),
        MAP.into(),
        destination.0.into(),
        destination.1.into(),
    ])
    .map(|_| ())
}

fn countdown_event(ctx: &Context, kind: u32) -> Result<(), String> {
    let (message, color) = COUNTDOWN[(kind - 40) as usize];
    if message.is_empty() {
        ctx.call(Function::MapWarp, vec![MAP.into(), WAITING_ROOM.0.into(), WAITING_ROOM.1.into(), WAITING_ROOM.2.into()])?;
        return ctx.call(Function::StopNpcTimer, vec![countdown().into()]).map(|_| ());
    }
    announce(ctx, message, color)
}

fn cleanup_poll(ctx: &Context) -> Result<(), String> {
    let cleanup = cleanup_timer();
    ctx.call(Function::StopNpcTimer, vec![cleanup.as_str().into()])?;
    let occupied = |camp: usize| -> Result<bool, String> {
        let id = read(ctx, &team_variable(camp))?;
        Ok(id != 0 && number(ctx, Function::BgGetData, vec![id.into(), 0.into()])? != 0)
    };
    if occupied(GUILLAUME)? || occupied(CROIX)? {
        return ctx.call(Function::InitNpcTimer, vec![cleanup.into()]).map(|_| ());
    }
    ctx.call(Function::StopNpcTimer, vec![countdown().into()])?;
    ctx.call(Function::StopNpcTimer, vec![GHOST.into()])?;
    ctx.call(Function::BgReserve, vec![MAP.into(), 1.into()])?;
    ctx.write(&variable(""), 0.into())?;
    ctx.write(&variable("_Victory"), 0.into())?;
    for camp in [GUILLAUME, CROIX] {
        let name = team_variable(camp);
        let id = read(ctx, &name)?;
        if id != 0 {
            ctx.call(Function::BgDestroy, vec![id.into()])?;
            ctx.write(&name, 0.into())?;
        }
    }
    ctx.call(Function::BgUnbook, vec![MAP.into()]).map(|_| ())
}

pub fn event(ctx: &Context, arena: usize, kind: u32) -> Result<(), String> {
    if arena != 0 {
        return Err("Unknown battleground arena".into());
    }
    match kind {
        0 => {
            ctx.call(Function::BgUnbook, vec![MAP.into()])?;
            ctx.call(Function::MapWarp, vec![MAP.into(), WAITING_ROOM.0.into(), WAITING_ROOM.1.into(), WAITING_ROOM.2.into()]).map(|_| ())
        }
        1 => ready_check(ctx),
        2 | 3 => {
            let (x, y) = CAMPS[(kind - 2) as usize].base;
            ctx.call(Function::Warp, vec![MAP.into(), x.into(), y.into()]).map(|_| ())
        }
        5 => {
            ctx.call(Function::StopNpcTimer, vec![start_npc().into()])?;
            ctx.call(Function::InitNpcTimer, vec![cleanup_timer().into()]).map(|_| ())
        }
        10 | 11 => food_destroyed(ctx, (kind - 10) as usize),
        12 => neutral_flag_captured(ctx),
        14 | 15 => barricade_destroyed(ctx, (kind - 14) as usize),
        20..=23 => {
            let camp = ((kind - 20) / 2) as usize;
            let timer = if kind % 2 == 0 { 25_000 } else { THERAPIST_PERIOD_MS };
            heal_cycle(ctx, CAMPS[camp].therapist, CAMPS[camp].heal_area, CAMPS[camp].base, timer)
        }
        24 | 25 => heal_cycle(ctx, GHOST, GHOST_AREA, GHOST_WARP, if kind == 24 { 25_000 } else { THERAPIST_PERIOD_MS }),
        30 | 31 => set_enabled(ctx, CAMPS[(kind - 30) as usize].vintenar, false),
        32 | 33 => set_enabled(ctx, CAMPS[(kind - 32) as usize].blacksmith, false),
        40..=47 => countdown_event(ctx, kind),
        50 => cleanup_poll(ctx),
        _ => Err(format!("Unknown battleground arena event {kind}")),
    }
}

fn vintenar(ctx: &Context, camp: usize) -> Result<(), String> {
    let battleground = number(ctx, Function::GetCharacterId, vec![4.into()])?;
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
    ctx.call(Function::BgLeave, vec![]).map(|_| ())
}

fn blacksmith(ctx: &Context, camp: usize) -> Result<(), String> {
    let data = &CAMPS[camp];
    let own_team = read(ctx, &team_variable(camp))?;
    let title = format!("[{} Blacksmith]", data.team);
    if own_team == 0 || number(ctx, Function::GetCharacterId, vec![4.into()])? != own_team {
        ctx.mes(title)?;
        ctx.mes("There the enemy is coming!")?;
        return ctx.close();
    }
    ctx.mes(title.as_str())?;
    ctx.mes("We are in urgency! The Barricade has been destroyed!")?;
    ctx.mes("We can repair the Barricade with ^3131FF50 Stones, 3 Sinew of Bear, 500 Metal Fragments, 30 Rough Elunium and 100 Gold.^000000")?;
    ctx.mes("We have it all except for the 50 Stones!")?;
    ctx.next()?;
    if ctx.select(&["Repair.".to_string(), "Leave it.".to_string()])? == 1 {
        ctx.mes(title.as_str())?;
        ctx.mes("There are enemies coming! Let's evacuate from here!")?;
        return ctx.close();
    }
    if number(ctx, Function::CountItem, vec![STONE.into()])? < STONES_REQUIRED {
        ctx.mes(title.as_str())?;
        ctx.mes("You don't have enough Stones!")?;
        ctx.next()?;
        ctx.mes(title.as_str())?;
        ctx.mes(format!("^3131FFWe need {STONES_REQUIRED} Stones.^000000\nWe are busy, so please hurry."))?;
        return ctx.close();
    }
    ctx.mes(title.as_str())?;
    ctx.mes("You brought enough stones! Let's go and repair.")?;
    ctx.next()?;
    ctx.mes(title.as_str())?;
    ctx.mes("Combine Stones and Gold in the proper percentage and shape the Barricade, then add Rough Elunium to make it stronger. Decorate with Metal Fragments, and plait stones with Sinew of Bear!")?;
    ctx.next()?;
    ctx.call(Function::SpecialEffect, vec![EF_REPAIR_WEAPON.into()])?;
    ctx.mes(title.as_str())?;
    ctx.mes("Wow! It's done.\nWe are relieved.")?;
    if number(ctx, Function::MobCount, vec![MAP.into(), barricade_label(camp).into()])? < BARRICADE_COUNT_AFTER_BREAK {
        ctx.call(Function::DelItem, vec![STONE.into(), STONES_REQUIRED.into()])?;
        clear_barricade(ctx, camp)?;
        spawn_barricade(ctx, camp)?;
    }
    ctx.close()?;
    set_enabled(ctx, data.blacksmith, false)
}

fn ghost(ctx: &Context) -> Result<(), String> {
    ctx.call(Function::SpecialEffect, vec![EF_HEAL.into()])?;
    ctx.mes("[Valley Ghost]")?;
    ctx.mes("Boo...Boo...")?;
    ctx.close()
}

pub fn npc(ctx: &Context) -> Result<(), String> {
    let args: Vec<Value> = arguments(ctx)?;
    let arena = args.first().ok_or("Arena NPC needs an arena")?.number_value()?;
    let role = args.get(1).ok_or("Arena NPC needs a role")?.number_value()?;
    if arena != 0 {
        return Err("Unknown battleground arena".into());
    }
    match role {
        ROLE_DECORATION => Ok(()),
        ROLE_THERAPIST => therapist(ctx),
        ROLE_GHOST => ghost(ctx),
        role if (ROLE_VINTENAR..ROLE_VINTENAR + 2).contains(&role) => vintenar(ctx, (role - ROLE_VINTENAR) as usize),
        role if (ROLE_BLACKSMITH..ROLE_BLACKSMITH + 2).contains(&role) => blacksmith(ctx, (role - ROLE_BLACKSMITH) as usize),
        _ => Err(format!("Unknown battleground NPC role {role}")),
    }
}
