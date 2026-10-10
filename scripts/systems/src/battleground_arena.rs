use script_sdk_2::constants::{BC_MAP, CELL_WALKABLE, EF_HEAL, EF_SANCTUARY};
use script_sdk_2::{Area, Ctx, Script, Spot, Stop};

const BADGE_LIMIT: i32 = 500;
const FLAVIUS_REWARD: Reward = Reward { badge: 7829, win: 9, loss: 3 };
const CELL_BASILICA: i32 = 4;
pub(crate) const WAITING_ROOM: Spot = Spot { map: "bat_room", x: 154, y: 150 };
const THERAPIST_PERIOD_MS: i32 = 26_500;
const GUARDIAN: i32 = 1949;

const GUILLAUME: usize = 0;
const CROIX: usize = 1;

const ROLE_DECORATION: i32 = 0;
const ROLE_THERAPIST: i32 = 1;
const ROLE_VINTENAR: i32 = 2;
const ROLE_VINTENAR_OVER: i32 = 4;

struct Camp {
    team: &'static str,
    suffix: &'static str,
    officer: &'static str,
    crystal: (&'static str, i32, i32, i32),
    guardian_name: &'static str,
    guardians: [(i32, i32); 2],
    cell: Area,
    base: (i32, i32),
    hall: (i32, i32),
    heal_area: Area,
    heal_warp: (i32, i32),
}

struct Arena {
    index: i32,
    map: &'static str,
    short: &'static str,
    camps: [Camp; 2],
}

const ARENAS: [Arena; 1] = [Arena {
    index: 1,
    map: "bat_b01",
    short: "b01",
    camps: [
        Camp {
            team: "Guillaume",
            suffix: "a",
            officer: "Axl Rose",
            crystal: ("Pink Crystal", 1915, 61, 150),
            guardian_name: "Guillaume Camp Guardian",
            guardians: [(108, 159), (108, 141)],
            cell: Area { x1: 62, y1: 149, x2: 60, y2: 151 },
            base: (87, 75),
            hall: (10, 290),
            heal_area: Area { x1: 0, y1: 280, x2: 20, y2: 300 },
            heal_warp: (87, 73),
        },
        Camp {
            team: "Croix",
            suffix: "b",
            officer: "Swandery",
            crystal: ("Blue Crystal", 1914, 328, 150),
            guardian_name: "Croix Camp Guardian",
            guardians: [(307, 160), (307, 138)],
            cell: Area { x1: 327, y1: 151, x2: 329, y2: 149 },
            base: (311, 224),
            hall: (390, 10),
            heal_area: Area { x1: 379, y1: 0, x2: 399, y2: 20 },
            heal_warp: (312, 225),
        },
    ],
}];

const COUNTDOWN_ANNOUNCEMENTS: [(&str, &str); 5] = [
    ("Marollo VII : Guillaume Marollo, Croix Marollo! And their followers!", "0x99CC00"),
    ("Marollo VII : Both camps are competitive, so it's hard to judge which team is superior.", "0x99CC00"),
    ("Marollo VII : This battle of Flavian is such a waste of time. I will decide victory and defeat by your progress.", "0x99CC00"),
    ("Marollo VII : If you can't accept the results, try again in another valley battle!", "0x99CC00"),
    ("Axl Rose, Swandery : Yes, sir.", "0x99CC00"),
];

impl Arena {
    fn variable(&self, suffix: &str) -> String {
        format!("$@FlaviusBG{}{suffix}", self.index)
    }

    fn score_variable(&self, camp: usize) -> String {
        format!("${}_ScoreBG{}", if camp == GUILLAUME { "@Guill" } else { "@Croix" }, self.index)
    }

    fn team_variable(&self, camp: usize) -> String {
        self.variable(&format!("_id{}", camp + 1))
    }

    fn crystal_label(&self, camp: usize) -> String {
        format!("OBJ#{}_{}::OnMyMobDead", self.map, self.camps[camp].suffix)
    }

    fn guardian_label(&self, camp: usize) -> String {
        format!("guardian#{}_{}::OnMyMobDead", self.map, self.camps[camp].suffix)
    }

    fn therapist(&self, camp: usize) -> String {
        format!("Battle Therapist#{}_{}", self.short, self.camps[camp].suffix)
    }

    fn vintenar(&self, camp: usize) -> String {
        format!("{} Vintenar#{}_{}", self.camps[camp].team, self.short, self.camps[camp].suffix)
    }

    fn overseer(&self, camp: usize) -> String {
        format!("Vintenar#{}_{}over", self.map, self.camps[camp].suffix)
    }

    fn cleanup_timer(&self) -> String {
        format!("#{}_timer", self.map)
    }

    fn countdown(&self) -> String {
        format!("countdown#{}", self.map)
    }

    fn start_npc(&self) -> String {
        format!("start#{}", self.map)
    }

    fn spot(&self, (x, y): (i32, i32)) -> Spot<'static> {
        Spot { map: self.map, x, y }
    }
}

pub(crate) fn read(ctx: &Ctx, name: &str) -> Result<i32, Stop> {
    ctx.var(name).get()?.number()
}

/// Announces `message` on the map of the NPC that runs the script, in `color`.
pub(crate) fn announce(ctx: &Ctx, message: &str, color: &str) -> Script {
    ctx.announce_colored(message, BC_MAP, color)
}

/// Opens a camp's crystal cell for attackers, or closes it again.
fn set_cell(ctx: &Ctx, arena: &Arena, camp: usize, blocked: bool) -> Script {
    let area = arena.camps[camp].cell;
    ctx.set_cell(arena.map, area, CELL_BASILICA, blocked)?;
    ctx.set_cell(arena.map, area, CELL_WALKABLE, !blocked)
}

fn spawn_crystal(ctx: &Ctx, arena: &Arena, camp: usize) -> Script {
    let (name, class, x, y) = arena.camps[camp].crystal;
    let label = arena.crystal_label(camp);
    let team = read(ctx, &arena.team_variable(camp))?;
    ctx.battleground().monster(team, arena.spot((x, y)), name, class, Some(&label))?;
    ctx.set_mob_immunity(arena.map, &label, true)
}

fn spawn_guardians(ctx: &Ctx, arena: &Arena, camp: usize) -> Script {
    let data = &arena.camps[camp];
    for position in data.guardians {
        let team = read(ctx, &arena.team_variable(camp))?;
        ctx.battleground().monster(team, arena.spot(position), data.guardian_name, GUARDIAN, Some(&arena.guardian_label(camp)))?;
    }
    Ok(())
}

fn reset_objectives(ctx: &Ctx, arena: &Arena) -> Script {
    for camp in [GUILLAUME, CROIX] {
        ctx.kill_monster(arena.map, &arena.crystal_label(camp))?;
        spawn_crystal(ctx, arena, camp)?;
    }
    for camp in [GUILLAUME, CROIX] {
        ctx.kill_monster(arena.map, &arena.guardian_label(camp))?;
    }
    for camp in [GUILLAUME, CROIX] {
        spawn_guardians(ctx, arena, camp)?;
    }
    for camp in [GUILLAUME, CROIX] {
        set_cell(ctx, arena, camp, true)?;
    }
    Ok(())
}

fn therapists(ctx: &Ctx, arena: &Arena, running: bool) -> Script {
    for camp in [GUILLAUME, CROIX] {
        let name = arena.therapist(camp);
        if running {
            ctx.timers().restart(Some(&name))?;
        } else {
            ctx.timers().stop(Some(&name))?;
        }
        ctx.set_npc_visible(&name, running)?;
    }
    Ok(())
}

fn warp_teams(ctx: &Ctx, arena: &Arena, position: impl Fn(&Camp) -> (i32, i32)) -> Script {
    for camp in [GUILLAUME, CROIX] {
        let team = read(ctx, &arena.team_variable(camp))?;
        ctx.battleground().warp(team, arena.spot(position(&arena.camps[camp])))?;
    }
    Ok(())
}

fn publish_scores(ctx: &Ctx, arena: &Arena) -> Script {
    let guillaume = read(ctx, &arena.score_variable(GUILLAUME))?;
    let croix = read(ctx, &arena.score_variable(CROIX))?;
    ctx.battleground().update_score(arena.map, guillaume, croix)
}

fn ready_check(ctx: &Ctx, arena: &Arena) -> Script {
    if read(ctx, &arena.variable(""))? != 0 {
        return Ok(());
    }
    ctx.var(&arena.variable("")).set(1)?;
    ctx.var(&arena.variable("_Victory")).set(0)?;
    ctx.var(&arena.score_variable(GUILLAUME)).set(0)?;
    ctx.var(&arena.score_variable(CROIX)).set(0)?;
    publish_scores(ctx, arena)?;
    reset_objectives(ctx, arena)?;
    therapists(ctx, arena, true)?;
    for camp in [GUILLAUME, CROIX] {
        ctx.set_npc_visible(&arena.vintenar(camp), false)?;
        ctx.set_npc_visible(&arena.overseer(camp), false)?;
        let team = read(ctx, &arena.team_variable(camp))?;
        ctx.battleground().warp(team, arena.spot(arena.camps[camp].base))?;
    }
    ctx.timers().restart(Some(&arena.countdown()))?;
    ctx.timers().restart(Some(&arena.start_npc()))
}

/// A camp's crystal died. The first one only scores a point and resets the field; the second one ends the battle.
fn crystal_destroyed(ctx: &Ctx, arena: &Arena, victim: usize) -> Script {
    if ctx.mob_count(arena.map, &arena.crystal_label(victim))? >= 1 {
        return Ok(());
    }
    let scorer = 1 - victim;
    announce(ctx, &format!("{}'s Crystal has been destroyed.", arena.camps[victim].team), "0xFFCE00")?;
    let score = arena.score_variable(scorer);
    if read(ctx, &score)? > 0 {
        ctx.var(&arena.variable("_Victory")).set(scorer as i32 + 1)?;
        ctx.var(&score).set(read(ctx, &score)? + 1)?;
        for camp in [GUILLAUME, CROIX] {
            ctx.set_npc_visible(&arena.vintenar(camp), true)?;
        }
        therapists(ctx, arena, false)?;
        ctx.battleground().reserve(arena.map, true)?;
    } else {
        ctx.var(&score).set(1)?;
        therapists(ctx, arena, true)?;
        reset_objectives(ctx, arena)?;
    }
    ctx.timers().stop(Some(&arena.cleanup_timer()))?;
    publish_scores(ctx, arena)?;
    warp_teams(ctx, arena, |camp| camp.hall)?;
    ctx.timers().init(Some(&arena.cleanup_timer()))
}

fn guardians_slain(ctx: &Ctx, arena: &Arena, camp: usize) -> Script {
    if ctx.mob_count(arena.map, &arena.guardian_label(camp))? >= 1 {
        return Ok(());
    }
    set_cell(ctx, arena, camp, false)?;
    announce(ctx, &format!("The Guardian protecting {}'s Crystal has been slain.", arena.camps[camp].team), "0xFFCE00")?;
    ctx.set_mob_immunity(arena.map, &arena.crystal_label(camp), false)
}

/// The timer of a camp's therapist: it heals and sends back the players in the camp's hall, then restarts.
pub(crate) fn heal_cycle(ctx: &Ctx, map: &'static str, npc: &str, area: Area, destination: (i32, i32), timer: i32) -> Script {
    if timer == THERAPIST_PERIOD_MS {
        ctx.timers().restart(Some(npc))?;
        return ctx.set_npc_visible(npc, true);
    }
    ctx.fx().special_effect(EF_SANCTUARY)?;
    ctx.area_heal(map, area, 100, 100)?;
    ctx.area_warp(map, area, Spot { map, x: destination.0, y: destination.1 })
}

fn match_timeout(ctx: &Ctx, arena: &Arena) -> Script {
    therapists(ctx, arena, false)?;
    warp_teams(ctx, arena, |camp| camp.hall)?;
    for camp in [GUILLAUME, CROIX] {
        ctx.set_npc_visible(&arena.overseer(camp), true)?;
    }
    ctx.battleground().reserve(arena.map, true).map(|_| ())
}

fn countdown_event(ctx: &Ctx, arena: &Arena, kind: u32) -> Script {
    match kind {
        40 => announce(ctx, "Guillaume Vintenar Axl Rose : Let's attack to destroy that Crystal!", "0xFF9900"),
        41 => announce(ctx, "Croix Vintenar Swandery : Even though Guillaume is struggling to win against us, the victory is ours. Let's show them our power.", "0xFF99CC"),
        42..=46 => {
            let (message, color) = COUNTDOWN_ANNOUNCEMENTS[(kind - 42) as usize];
            announce(ctx, message, color)
        }
        47 => match_timeout(ctx, arena),
        _ => {
            ctx.map_warp(arena.map, WAITING_ROOM)?;
            ctx.timers().stop(Some(&arena.countdown()))
        }
    }
}

/// Polls until both teams are empty, then frees the arena for the next battle.
fn cleanup_poll(ctx: &Ctx, arena: &Arena) -> Script {
    let cleanup = arena.cleanup_timer();
    ctx.timers().stop(Some(&cleanup))?;
    let occupied = |camp: usize| -> Result<bool, Stop> {
        let team = read(ctx, &arena.team_variable(camp))?;
        Ok(team != 0 && ctx.battleground().member_count(team)? != 0)
    };
    if occupied(GUILLAUME)? || occupied(CROIX)? {
        return ctx.timers().init(Some(&cleanup));
    }
    ctx.timers().stop(Some(&arena.countdown()))?;
    ctx.battleground().reserve(arena.map, true)?;
    ctx.var(&arena.variable("")).set(0)?;
    destroy_teams(ctx, [arena.team_variable(GUILLAUME), arena.team_variable(CROIX)])?;
    ctx.battleground().unbook(arena.map).map(|_| ())
}

/// Destroys the team each variable holds, if any, and clears the variable.
pub(crate) fn destroy_teams(ctx: &Ctx, variables: [String; 2]) -> Script {
    for name in variables {
        let team = read(ctx, &name)?;
        if team != 0 {
            ctx.battleground().destroy(team)?;
            ctx.var(&name).set(0)?;
        }
    }
    Ok(())
}

/// Runs event `kind` of arena `arena`: the battleground queue and the arena's NPCs and monsters send these.
pub fn event(ctx: &Ctx, arena: usize, kind: u32) -> Script {
    let arena = ARENAS.get(arena).ok_or("Unknown battleground arena")?;
    match kind {
        0 => {
            ctx.battleground().unbook(arena.map)?;
            ctx.map_warp(arena.map, WAITING_ROOM)
        }
        1 => ready_check(ctx, arena),
        2 => reset_objectives(ctx, arena),
        3 | 4 => {
            let (x, y) = arena.camps[(kind - 3) as usize].base;
            ctx.warp(arena.map, x, y)
        }
        10 | 11 => crystal_destroyed(ctx, arena, (kind - 10) as usize),
        12 | 13 => guardians_slain(ctx, arena, (kind - 12) as usize),
        20..=23 => {
            let camp = &arena.camps[((kind - 20) / 2) as usize];
            let timer = if kind % 2 == 0 { 25_000 } else { THERAPIST_PERIOD_MS };
            heal_cycle(ctx, arena.map, &arena.therapist(((kind - 20) / 2) as usize), camp.heal_area, camp.heal_warp, timer)
        }
        30..=33 => {
            let name = if kind < 32 { arena.vintenar((kind - 30) as usize) } else { arena.overseer((kind - 32) as usize) };
            ctx.set_npc_visible(&name, false)
        }
        40..=48 => countdown_event(ctx, arena, kind),
        50 => cleanup_poll(ctx, arena),
        51 => {
            ctx.timers().stop(Some(&arena.start_npc()))?;
            ctx.timers().init(Some(&arena.cleanup_timer()))
        }
        _ => Err(Stop::Error(format!("Unknown battleground arena event {kind}"))),
    }
}

pub(crate) struct Reward {
    pub badge: i32,
    pub win: i32,
    pub loss: i32,
}

/// The officer's farewell after a battle, and the badges of the battle: more for the winners, up to the badge limit.
pub(crate) fn give_badges(ctx: &Ctx, team: &str, officer: &str, won: bool, reward: &Reward) -> Script {
    let name = ctx.player().name()?;
    let farewell = if won {
        format!("Blessed {team}!\nLet's enjoy our glorious victory!\n{name}, it's a sign reflecting victory.")
    } else if team == "Guillaume" {
        "You lost, but you're dedicated to this battle.\nThis is a reward for your great dedication by Guillaume Marollo!\nJust take this defeat as a lesson, and next time you will definitely win.".into()
    } else {
        format!("Oh, {name} Don't be sad.\nEven though we didn't win, we did our best.\nThis is a Royal gift from Croix, and please don't forget this battle. We will win the next one.")
    };
    ctx.mes_as(officer, &farewell)?;
    ctx.close_window()?;
    let room = BADGE_LIMIT - ctx.items().count(reward.badge)?;
    let amount = if won { reward.win } else { reward.loss }.min(room);
    if amount > 0 {
        ctx.items().give(reward.badge, amount)?;
    }
    Ok(())
}

fn vintenar(ctx: &Ctx, arena: &Arena, camp: usize, overseer: bool) -> Script {
    let own_team = read(ctx, &arena.team_variable(camp))?;
    let battleground = ctx.player().battleground_id()?;
    if own_team == 0 || own_team != battleground {
        ctx.mes_as(arena.camps[camp].officer, &format!("Why are you here, {} mercenary? You will be sent to where you should be!", arena.camps[1 - camp].team))?;
        return ctx.close();
    }
    let won = if overseer {
        let gap = read(ctx, &arena.score_variable(GUILLAUME))? - read(ctx, &arena.score_variable(CROIX))?;
        if camp == GUILLAUME { gap > 0 } else { gap < 0 }
    } else {
        read(ctx, &arena.variable("_Victory"))? == camp as i32 + 1
    };
    let data = &arena.camps[camp];
    give_badges(ctx, data.team, data.officer, won, &FLAVIUS_REWARD)?;
    ctx.battleground().leave(None)
}

pub(crate) fn therapist(ctx: &Ctx) -> Script {
    ctx.fx().special_effect(EF_HEAL)?;
    ctx.mes_as("Battle Therapist", "Just close your eyes,\nand take a deep breath.\nYou can be free from pain.")?;
    ctx.close()
}

/// An NPC of the arena. Its placement gives the arena and the role.
pub fn npc(ctx: &Ctx) -> Script {
    let arguments = ctx.arguments()?;
    let arena = arguments.first().ok_or("Arena NPC needs an arena")?.number()? as usize;
    let role = arguments.get(1).ok_or("Arena NPC needs a role")?.number()?;
    let arena = ARENAS.get(arena).ok_or("Unknown battleground arena")?;
    match role {
        ROLE_DECORATION => Ok(()),
        ROLE_THERAPIST => therapist(ctx),
        ROLE_VINTENAR | 3 => vintenar(ctx, arena, (role - ROLE_VINTENAR) as usize, false),
        ROLE_VINTENAR_OVER | 5 => vintenar(ctx, arena, (role - ROLE_VINTENAR_OVER) as usize, true),
        _ => Err(Stop::Error(format!("Unknown battleground NPC role {role}"))),
    }
}
