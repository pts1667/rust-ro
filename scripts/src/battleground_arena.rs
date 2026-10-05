use script_sdk::{Context, Function, Request, Value};

const BADGE_LIMIT: i32 = 500;
const FLAVIUS_REWARD: Reward = Reward { badge: 7829, win: 9, loss: 3 };
const BROADCAST_MAP: i32 = 1;
const CELL_BASILICA: i32 = 4;
const EF_HEAL: i32 = 312;
const EF_SANCTUARY: i32 = 83;
const WAITING_ROOM: (&str, i32, i32) = ("bat_room", 154, 150);
const THERAPIST_PERIOD_MS: i32 = 26_500;

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
    cell: [i32; 4],
    base: (i32, i32),
    hall: (i32, i32),
    heal_area: [i32; 4],
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
            cell: [62, 149, 60, 151],
            base: (87, 75),
            hall: (10, 290),
            heal_area: [0, 280, 20, 300],
            heal_warp: (87, 73),
        },
        Camp {
            team: "Croix",
            suffix: "b",
            officer: "Swandery",
            crystal: ("Blue Crystal", 1914, 328, 150),
            guardian_name: "Croix Camp Guardian",
            guardians: [(307, 160), (307, 138)],
            cell: [327, 151, 329, 149],
            base: (311, 224),
            hall: (390, 10),
            heal_area: [379, 0, 399, 20],
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
}

pub(crate) fn number(ctx: &Context, function: Function, arguments: Vec<Value>) -> Result<i32, String> {
    ctx.call(function, arguments)?.number_value()
}

pub(crate) fn read(ctx: &Context, name: &str) -> Result<i32, String> {
    ctx.read(name)?.number_value()
}

pub(crate) fn announce(ctx: &Context, message: &str, color: &str) -> Result<(), String> {
    ctx.call(Function::Announce, vec![message.into(), BROADCAST_MAP.into(), color.into()]).map(|_| ())
}

pub(crate) fn set_enabled(ctx: &Context, npc: &str, enabled: bool) -> Result<(), String> {
    ctx.call(if enabled { Function::EnableNpc } else { Function::DisableNpc }, vec![npc.into()]).map(|_| ())
}

pub(crate) fn restart_timer(ctx: &Context, npc: &str) -> Result<(), String> {
    ctx.call(Function::StopNpcTimer, vec![npc.into()])?;
    ctx.call(Function::InitNpcTimer, vec![npc.into()]).map(|_| ())
}

fn set_cell(ctx: &Context, arena: &Arena, camp: usize, blocked: bool) -> Result<(), String> {
    let [x1, y1, x2, y2] = arena.camps[camp].cell;
    let area = |cell: i32, enabled: bool| vec![arena.map.into(), x1.into(), y1.into(), x2.into(), y2.into(), cell.into(), i32::from(enabled).into()];
    ctx.call(Function::SetCell, area(CELL_BASILICA, blocked))?;
    ctx.call(Function::SetCell, area(0, !blocked)).map(|_| ())
}

fn spawn_crystal(ctx: &Context, arena: &Arena, camp: usize) -> Result<(), String> {
    let (name, class, x, y) = arena.camps[camp].crystal;
    let label = arena.crystal_label(camp);
    ctx.call(Function::BgMonster, vec![
        read(ctx, &arena.team_variable(camp))?.into(),
        arena.map.into(),
        x.into(),
        y.into(),
        name.into(),
        class.into(),
        label.as_str().into(),
    ])?;
    ctx.call(Function::SetMobImmunity, vec![arena.map.into(), label.into(), 1.into()]).map(|_| ())
}

fn spawn_guardians(ctx: &Context, arena: &Arena, camp: usize) -> Result<(), String> {
    let data = &arena.camps[camp];
    for (x, y) in data.guardians {
        ctx.call(Function::BgMonster, vec![
            read(ctx, &arena.team_variable(camp))?.into(),
            arena.map.into(),
            x.into(),
            y.into(),
            data.guardian_name.into(),
            1949.into(),
            arena.guardian_label(camp).into(),
        ])?;
    }
    Ok(())
}

fn reset_objectives(ctx: &Context, arena: &Arena) -> Result<(), String> {
    for camp in [GUILLAUME, CROIX] {
        ctx.call(Function::KillMonster, vec![arena.map.into(), arena.crystal_label(camp).into()])?;
        spawn_crystal(ctx, arena, camp)?;
    }
    for camp in [GUILLAUME, CROIX] {
        ctx.call(Function::KillMonster, vec![arena.map.into(), arena.guardian_label(camp).into()])?;
    }
    for camp in [GUILLAUME, CROIX] {
        spawn_guardians(ctx, arena, camp)?;
    }
    for camp in [GUILLAUME, CROIX] {
        set_cell(ctx, arena, camp, true)?;
    }
    Ok(())
}

fn therapists(ctx: &Context, arena: &Arena, running: bool) -> Result<(), String> {
    for camp in [GUILLAUME, CROIX] {
        let name = arena.therapist(camp);
        if running {
            restart_timer(ctx, &name)?;
        } else {
            ctx.call(Function::StopNpcTimer, vec![name.as_str().into()])?;
        }
        set_enabled(ctx, &name, running)?;
    }
    Ok(())
}

fn warp_teams_home(ctx: &Context, arena: &Arena) -> Result<(), String> {
    for camp in [GUILLAUME, CROIX] {
        let (x, y) = arena.camps[camp].hall;
        ctx.call(Function::BgWarp, vec![read(ctx, &arena.team_variable(camp))?.into(), arena.map.into(), x.into(), y.into()])?;
    }
    Ok(())
}

fn publish_scores(ctx: &Context, arena: &Arena) -> Result<(), String> {
    ctx.call(Function::BgUpdateScore, vec![
        arena.map.into(),
        read(ctx, &arena.score_variable(GUILLAUME))?.into(),
        read(ctx, &arena.score_variable(CROIX))?.into(),
    ])
    .map(|_| ())
}

fn ready_check(ctx: &Context, arena: &Arena) -> Result<(), String> {
    if read(ctx, &arena.variable(""))? != 0 {
        return Ok(());
    }
    ctx.write(&arena.variable(""), 1.into())?;
    ctx.write(&arena.variable("_Victory"), 0.into())?;
    ctx.write(&arena.score_variable(GUILLAUME), 0.into())?;
    ctx.write(&arena.score_variable(CROIX), 0.into())?;
    publish_scores(ctx, arena)?;
    reset_objectives(ctx, arena)?;
    therapists(ctx, arena, true)?;
    for camp in [GUILLAUME, CROIX] {
        set_enabled(ctx, &arena.vintenar(camp), false)?;
        set_enabled(ctx, &arena.overseer(camp), false)?;
        let (x, y) = arena.camps[camp].base;
        ctx.call(Function::BgWarp, vec![read(ctx, &arena.team_variable(camp))?.into(), arena.map.into(), x.into(), y.into()])?;
    }
    restart_timer(ctx, &arena.countdown())?;
    restart_timer(ctx, &arena.start_npc())
}

fn crystal_destroyed(ctx: &Context, arena: &Arena, victim: usize) -> Result<(), String> {
    if number(ctx, Function::MobCount, vec![arena.map.into(), arena.crystal_label(victim).into()])? >= 1 {
        return Ok(());
    }
    let scorer = 1 - victim;
    announce(ctx, &format!("{}'s Crystal has been destroyed.", arena.camps[victim].team), "0xFFCE00")?;
    let score_name = arena.score_variable(scorer);
    if read(ctx, &score_name)? > 0 {
        ctx.write(&arena.variable("_Victory"), (scorer as i32 + 1).into())?;
        ctx.write(&score_name, (read(ctx, &score_name)? + 1).into())?;
        for camp in [GUILLAUME, CROIX] {
            set_enabled(ctx, &arena.vintenar(camp), true)?;
        }
        therapists(ctx, arena, false)?;
        ctx.call(Function::BgReserve, vec![arena.map.into(), 1.into()])?;
    } else {
        ctx.write(&score_name, 1.into())?;
        therapists(ctx, arena, true)?;
        reset_objectives(ctx, arena)?;
    }
    ctx.call(Function::StopNpcTimer, vec![arena.cleanup_timer().into()])?;
    publish_scores(ctx, arena)?;
    warp_teams_home(ctx, arena)?;
    ctx.call(Function::InitNpcTimer, vec![arena.cleanup_timer().into()]).map(|_| ())
}

fn guardians_slain(ctx: &Context, arena: &Arena, camp: usize) -> Result<(), String> {
    if number(ctx, Function::MobCount, vec![arena.map.into(), arena.guardian_label(camp).into()])? >= 1 {
        return Ok(());
    }
    set_cell(ctx, arena, camp, false)?;
    announce(ctx, &format!("The Guardian protecting {}'s Crystal has been slain.", arena.camps[camp].team), "0xFFCE00")?;
    ctx.call(Function::SetMobImmunity, vec![arena.map.into(), arena.crystal_label(camp).into(), 0.into()]).map(|_| ())
}

fn therapist_cycle(ctx: &Context, arena: &Arena, camp: usize, timer: i32) -> Result<(), String> {
    let name = arena.therapist(camp);
    if timer == THERAPIST_PERIOD_MS {
        restart_timer(ctx, &name)?;
        return set_enabled(ctx, &name, true);
    }
    let data = &arena.camps[camp];
    let [x1, y1, x2, y2] = data.heal_area;
    ctx.call(Function::SpecialEffect, vec![EF_SANCTUARY.into()])?;
    ctx.call(Function::AreaPercentHeal, vec![arena.map.into(), x1.into(), y1.into(), x2.into(), y2.into(), 100.into(), 100.into()])?;
    ctx.call(Function::AreaWarp, vec![
        arena.map.into(),
        x1.into(),
        y1.into(),
        x2.into(),
        y2.into(),
        arena.map.into(),
        data.heal_warp.0.into(),
        data.heal_warp.1.into(),
    ])
    .map(|_| ())
}

fn match_timeout(ctx: &Context, arena: &Arena) -> Result<(), String> {
    therapists(ctx, arena, false)?;
    warp_teams_home(ctx, arena)?;
    for camp in [GUILLAUME, CROIX] {
        set_enabled(ctx, &arena.overseer(camp), true)?;
    }
    ctx.call(Function::BgReserve, vec![arena.map.into(), 1.into()]).map(|_| ())
}

fn countdown_event(ctx: &Context, arena: &Arena, kind: u32) -> Result<(), String> {
    match kind {
        40 => announce(ctx, "Guillaume Vintenar Axl Rose : Let's attack to destroy that Crystal!", "0xFF9900"),
        41 => announce(ctx, "Croix Vintenar Swandery : Even though Guillaume is struggling to win against us, the victory is ours. Let's show them our power.", "0xFF99CC"),
        42..=46 => {
            let (message, color) = COUNTDOWN_ANNOUNCEMENTS[(kind - 42) as usize];
            announce(ctx, message, color)
        }
        47 => match_timeout(ctx, arena),
        _ => {
            ctx.call(Function::MapWarp, vec![arena.map.into(), WAITING_ROOM.0.into(), WAITING_ROOM.1.into(), WAITING_ROOM.2.into()])?;
            ctx.call(Function::StopNpcTimer, vec![arena.countdown().into()]).map(|_| ())
        }
    }
}

fn cleanup_poll(ctx: &Context, arena: &Arena) -> Result<(), String> {
    let cleanup = arena.cleanup_timer();
    ctx.call(Function::StopNpcTimer, vec![cleanup.as_str().into()])?;
    let team = |camp: usize| read(ctx, &arena.team_variable(camp));
    let occupied = |id: i32| -> Result<bool, String> { Ok(id != 0 && number(ctx, Function::BgGetData, vec![id.into(), 0.into()])? != 0) };
    if occupied(team(GUILLAUME)?)? || occupied(team(CROIX)?)? {
        return ctx.call(Function::InitNpcTimer, vec![cleanup.into()]).map(|_| ());
    }
    ctx.call(Function::StopNpcTimer, vec![arena.countdown().into()])?;
    ctx.call(Function::BgReserve, vec![arena.map.into(), 1.into()])?;
    ctx.write(&arena.variable(""), 0.into())?;
    for camp in [GUILLAUME, CROIX] {
        let name = arena.team_variable(camp);
        let id = read(ctx, &name)?;
        if id != 0 {
            ctx.call(Function::BgDestroy, vec![id.into()])?;
            ctx.write(&name, 0.into())?;
        }
    }
    ctx.call(Function::BgUnbook, vec![arena.map.into()]).map(|_| ())
}

pub fn event(ctx: &Context, arena: usize, kind: u32) -> Result<(), String> {
    let arena = ARENAS.get(arena).ok_or("Unknown battleground arena")?;
    match kind {
        0 => {
            ctx.call(Function::BgUnbook, vec![arena.map.into()])?;
            ctx.call(Function::MapWarp, vec![arena.map.into(), WAITING_ROOM.0.into(), WAITING_ROOM.1.into(), WAITING_ROOM.2.into()]).map(|_| ())
        }
        1 => ready_check(ctx, arena),
        2 => reset_objectives(ctx, arena),
        3 | 4 => {
            let (x, y) = arena.camps[(kind - 3) as usize].base;
            ctx.call(Function::Warp, vec![arena.map.into(), x.into(), y.into()]).map(|_| ())
        }
        10 | 11 => crystal_destroyed(ctx, arena, (kind - 10) as usize),
        12 | 13 => guardians_slain(ctx, arena, (kind - 12) as usize),
        20..=23 => therapist_cycle(ctx, arena, ((kind - 20) / 2) as usize, if kind % 2 == 0 { 25_000 } else { THERAPIST_PERIOD_MS }),
        30..=33 => {
            let name = if kind < 32 { arena.vintenar((kind - 30) as usize) } else { arena.overseer((kind - 32) as usize) };
            set_enabled(ctx, &name, false)
        }
        40..=48 => countdown_event(ctx, arena, kind),
        50 => cleanup_poll(ctx, arena),
        51 => {
            ctx.call(Function::StopNpcTimer, vec![arena.start_npc().into()])?;
            ctx.call(Function::InitNpcTimer, vec![arena.cleanup_timer().into()]).map(|_| ())
        }
        _ => Err(format!("Unknown battleground arena event {kind}")),
    }
}

pub(crate) fn arguments(ctx: &Context) -> Result<Vec<Value>, String> {
    match ctx.request(Request::Arguments)? {
        Value::Array(values) => Ok(values),
        _ => Err("NPC arguments are invalid".into()),
    }
}

pub(crate) struct Reward {
    pub badge: i32,
    pub win: i32,
    pub loss: i32,
}

pub(crate) fn give_badges(ctx: &Context, team: &str, officer: &str, won: bool, reward: &Reward) -> Result<(), String> {
    let name = ctx.call(Function::StrCharInfo, vec![0.into()])?.text();
    ctx.mes(format!("[{officer}]"))?;
    if won {
        ctx.mes(format!("Blessed {team}!
Let's enjoy our glorious victory!
{name}, it's a sign reflecting victory."))?;
    } else if team == "Guillaume" {
        ctx.mes("You lost, but you're dedicated to this battle.
This is a reward for your great dedication by Guillaume Marollo!
Just take this defeat as a lesson, and next time you will definitely win.")?;
    } else {
        ctx.mes(format!("Oh, {name} Don't be sad.
Even though we didn't win, we did our best.
This is a Royal gift from Croix, and please don't forget this battle. We will win the next one."))?;
    }
    ctx.close()?;
    let room = BADGE_LIMIT - number(ctx, Function::CountItem, vec![reward.badge.into()])?;
    let amount = if won { reward.win } else { reward.loss }.min(room);
    if amount > 0 {
        ctx.call(Function::GetItem, vec![reward.badge.into(), amount.into()])?;
    }
    Ok(())
}

fn vintenar(ctx: &Context, arena: &Arena, camp: usize, overseer: bool) -> Result<(), String> {
    let own_team = read(ctx, &arena.team_variable(camp))?;
    let battleground = number(ctx, Function::GetCharacterId, vec![4.into()])?;
    if own_team == 0 || own_team != battleground {
        let data = &arena.camps[1 - camp];
        ctx.mes(format!("[{}]", arena.camps[camp].officer))?;
        ctx.mes(format!("Why are you here, {} mercenary? You will be sent to where you should be!", data.team))?;
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
    ctx.call(Function::BgLeave, vec![]).map(|_| ())
}

pub(crate) fn therapist(ctx: &Context) -> Result<(), String> {
    ctx.call(Function::SpecialEffect, vec![EF_HEAL.into()])?;
    ctx.mes("[Battle Therapist]")?;
    ctx.mes("Just close your eyes,\nand take a deep breath.\nYou can be free from pain.")?;
    ctx.close()
}

pub fn npc(ctx: &Context) -> Result<(), String> {
    let args = arguments(ctx)?;
    let arena_index = args.first().ok_or("Arena NPC needs an arena")?.number_value()? as usize;
    let role = args.get(1).ok_or("Arena NPC needs a role")?.number_value()?;
    let arena = ARENAS.get(arena_index).ok_or("Unknown battleground arena")?;
    match role {
        ROLE_DECORATION => Ok(()),
        ROLE_THERAPIST => therapist(ctx),
        ROLE_VINTENAR | 3 => vintenar(ctx, arena, (role - ROLE_VINTENAR) as usize, false),
        ROLE_VINTENAR_OVER | 5 => vintenar(ctx, arena, (role - ROLE_VINTENAR_OVER) as usize, true),
        _ => Err(format!("Unknown battleground NPC role {role}")),
    }
}
