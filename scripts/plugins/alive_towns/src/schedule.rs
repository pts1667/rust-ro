//! What each townsperson does at any moment: a routine of activities repeated in a loop, interrupted by gatherings on the
//! plazas. All of it is a function of (settings, town, index, time), so the director, a resync and the dialogue agree
//! without storing anything.

use std::cell::RefCell;
use std::collections::HashMap;

use script_sdk_2::constants::*;

use crate::persona::{Kind, Persona};
use crate::random::{CHAT, GATHERING, ROUTINE, Rng};
use crate::towns::{Cell, Counter, Door, TOWNS, Town, direction, octile};
use crate::Settings;

/// Gatherings form and break up at multiples of this.
pub const SLOT_MS: i64 = 150_000;
/// An `EnableNpc` is applied by the map thread after the game loop checks the next walk, so the walk out of a door waits.
const SHOW_BEFORE_WALK_MS: u64 = 2_000;
/// Cells of a ring of radius 2, counterclockwise from east.
const RING: [(i32, i32); 8] = [(2, 0), (2, 2), (0, 2), (-2, 2), (-2, 0), (-2, -2), (0, -2), (2, -2)];

const ASKING: &[i32] = &[ET_QUESTION, ET_MONEY, ET_THINK, ET_HUM];
const THANKING: &[i32] = &[ET_THANKS, ET_OK, ET_SMILE, ET_DELIGHT];
const IDLE: &[i32] = &[ET_HUM, ET_THINK, ET_SMILE, ET_SLEEPY, ET_STARE_ABOUT, ET_SWEAT];
const PLAYFUL: &[i32] = &[ET_DELIGHT, ET_KIK, ET_MERONG, ET_SMILE, ET_HUK];
const ON_GUARD: &[i32] = &[ET_STARE_ABOUT, ET_OK, ET_HUM];
const CHATTY: &[i32] = &[ET_SMILE, ET_KIK, ET_HUK, ET_OK, ET_DELIGHT, ET_BLABLA, ET_AHA, ET_KEK, ET_SURPRISE, ET_QUESTION];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Walks to `to`; through a door when `door`, which hides the walker on arrival.
    Walk { to: Cell, door: bool },
    /// Turns where it stands.
    Face(u8),
    Emote(i32),
    /// Comes back out of the door it went through.
    Show,
}

/// What the person is busy with, for the dialogue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Doing {
    Shopping(&'static str),
    Entering(&'static str),
    Strolling,
    Loitering,
    Patrolling,
    GoingHome,
    Gathering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timed {
    /// Offset from the start of the loop, in ms.
    pub at: u64,
    pub step: Step,
    pub doing: Doing,
}

/// Where someone is, assuming every walk so far arrived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub cell: Cell,
    pub hidden: bool,
    pub dir: u8,
    pub doing: Doing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Routine {
    pub length: u64,
    /// A loop starts `phase` ms before each multiple of `length`, so neighbours do not move in step.
    pub phase: u64,
    pub steps: Vec<Timed>,
    pub home: Cell,
    pub dir: u8,
}

/// Time a walk is given before the next step: the octile distance at the walker's speed with room for detours.
pub fn walk_budget(from: Cell, to: Cell, speed: u16) -> u64 {
    octile(from, to) * u64::from(speed) * 16 / 100 + 2_000
}

struct Builder {
    rng: Rng,
    town: &'static Town,
    speed: u16,
    at: Cell,
    time: u64,
    doing: Doing,
    steps: Vec<Timed>,
}

impl Builder {
    fn push(&mut self, step: Step) {
        self.steps.push(Timed { at: self.time, step, doing: self.doing });
    }

    fn walk(&mut self, to: Cell, door: bool) {
        if to == self.at && !door {
            return;
        }
        self.push(Step::Walk { to, door });
        self.time += walk_budget(self.at, to, self.speed);
        self.at = to;
    }

    fn wait(&mut self, low: u64, high: u64) {
        self.time += self.rng.range(low, high);
    }

    fn face(&mut self, dir: u8) {
        self.push(Step::Face(dir));
    }

    fn look_around(&mut self) {
        let dir = self.rng.below(8) as u8;
        self.face(dir);
    }

    fn emote(&mut self, emotes: &[i32]) {
        let emote = *self.rng.pick(emotes);
        self.push(Step::Emote(emote));
    }

    /// The closest to where the person stands of three places drawn at random.
    fn nearby<T>(&mut self, places: &'static [T], cell: impl Fn(&T) -> Cell) -> &'static T {
        let at = self.at;
        (0..3).map(|_| self.rng.pick(places)).min_by_key(|place| octile(at, cell(place))).unwrap()
    }

    fn shop(&mut self) {
        let counter: &Counter = self.nearby(self.town.counters, |counter| counter.front);
        self.doing = Doing::Shopping(counter.npc);
        self.walk(counter.front, false);
        self.face(counter.face);
        self.wait(800, 1_500);
        self.emote(ASKING);
        self.wait(8_000, 15_000);
        self.emote(THANKING);
        self.wait(1_500, 3_000);
    }

    fn enter(&mut self) {
        let door: &Door = self.nearby(self.town.doors, |door| door.front);
        self.doing = Doing::Entering(door.inside);
        self.walk(door.front, false);
        self.face(direction(door.front, door.cell));
        self.wait(1_500, 2_500);
        self.walk(door.cell, true);
        self.wait(20_000, 60_000);
        self.push(Step::Show);
        self.time += SHOW_BEFORE_WALK_MS;
        self.walk(door.front, false);
        self.wait(1_000, 3_000);
    }

    fn stroll(&mut self) {
        self.doing = Doing::Strolling;
        let near = *self.nearby(self.town.spots, |spot| *spot);
        let far = (0..3).map(|_| *self.rng.pick(self.town.spots)).max_by_key(|spot| octile(near, *spot)).unwrap();
        let legs = self.rng.range(2, 3);
        for leg in 0..legs {
            self.walk(if leg % 2 == 0 { near } else { far }, false);
            self.wait(2_500, 6_000);
            self.look_around();
            self.wait(1_000, 3_000);
        }
    }

    fn loiter(&mut self, emotes: &[i32]) {
        self.doing = Doing::Loitering;
        let spot = *self.nearby(self.town.spots, |spot| *spot);
        self.walk(spot, false);
        for _ in 0..self.rng.range(3, 5) {
            self.wait(4_000, 8_000);
            self.look_around();
            if self.rng.chance(30) {
                self.emote(emotes);
            }
        }
        self.wait(2_000, 4_000);
    }

    fn patrol(&mut self) {
        self.doing = Doing::Patrolling;
        for _ in 0..4 {
            let post = *self.rng.pick(self.town.spots);
            self.walk(post, false);
            self.wait(1_000, 2_000);
            self.look_around();
            self.wait(6_000, 12_000);
            if self.rng.chance(20) {
                self.emote(ON_GUARD);
            }
        }
    }
}

pub fn routine(settings: &Settings, town: usize, index: u32, persona: &Persona) -> Routine {
    let place = &TOWNS[town];
    let rng = Rng::new(&[u64::from(settings.seed), ROUTINE, town as u64, u64::from(index)]);
    let mut builder = Builder { rng, town: place, speed: persona.speed, at: persona.home, time: 0, doing: Doing::Loitering, steps: vec![] };
    if persona.kind == Kind::Guard {
        builder.patrol();
    } else {
        let child = persona.kind == Kind::Child;
        for _ in 0..builder.rng.range(3, 5) {
            let shops = if place.counters.is_empty() { 0 } else if child { 10 } else { 35 };
            let doors = if place.doors.is_empty() || child { 0 } else { 25 };
            match builder.rng.weighted(&[shops, doors, if child { 50 } else { 20 }, if child { 40 } else { 20 }]) {
                0 => builder.shop(),
                1 => builder.enter(),
                2 => builder.stroll(),
                _ => builder.loiter(if child { PLAYFUL } else { IDLE }),
            }
        }
    }
    builder.doing = Doing::GoingHome;
    builder.walk(persona.home, false);
    builder.wait(3_000, 10_000);
    let length = builder.time;
    let phase = builder.rng.below(length);
    Routine { length, phase, steps: builder.steps, home: persona.home, dir: persona.dir as u8 }
}

impl Routine {
    fn loop_start(&self, time: i64) -> i64 {
        let length = self.length as i64;
        (time + self.phase as i64).div_euclid(length) * length - self.phase as i64
    }

    /// Steps after `from` up to `to` included, at their absolute times.
    pub fn steps_between(&self, from: i64, to: i64) -> impl Iterator<Item = (i64, Timed)> + '_ {
        let first = self.loop_start(from + 1);
        let last = self.loop_start(to);
        let length = self.length as i64;
        (0..=((last - first) / length).max(-1))
            .flat_map(move |round| {
                let start = first + round * length;
                self.steps.iter().map(move |timed| (start + timed.at as i64, *timed))
            })
            .filter(move |(time, _)| *time > from && *time <= to)
    }

    pub fn state_at(&self, time: i64) -> State {
        let rel = (time - self.loop_start(time)) as u64;
        let mut state = State { cell: self.home, hidden: false, dir: self.dir, doing: Doing::GoingHome };
        for timed in self.steps.iter().take_while(|timed| timed.at <= rel) {
            state.doing = timed.doing;
            match timed.step {
                Step::Walk { to, door } => {
                    state.dir = direction(state.cell, to);
                    state.cell = to;
                    state.hidden = door;
                }
                Step::Face(dir) => state.dir = dir,
                Step::Show => state.hidden = false,
                Step::Emote(_) => {}
            }
        }
        state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gathering {
    pub center: Cell,
    pub cell: Cell,
}

/// The groups of `slot`: for each townsperson, the place in a group it joins, if any. Guards never join.
pub fn gatherings(settings: &Settings, town: usize, slot: i64) -> Vec<Option<Gathering>> {
    let place = &TOWNS[town];
    let mut groups = vec![None; settings.npcs_per_town as usize];
    let mut rng = Rng::new(&[u64::from(settings.seed), GATHERING, town as u64, slot as u64]);
    let mut candidates: Vec<u32> = (settings.guards_per_town.min(settings.npcs_per_town)..settings.npcs_per_town).collect();
    rng.shuffle(&mut candidates);
    let mut plazas: Vec<Cell> = place.plazas.to_vec();
    rng.shuffle(&mut plazas);
    let mut wanted = candidates.len() * settings.gathering_percent.min(100) as usize / 100;
    let mut members = candidates.into_iter();
    for center in plazas {
        let size = (rng.range(3, 5) as usize).min(wanted);
        if size < 2 {
            break;
        }
        wanted -= size;
        let rotation = rng.below(8) as usize;
        for (seat, index) in members.by_ref().take(size).enumerate() {
            let (dx, dy) = RING[(rotation + seat * RING.len() / size) % RING.len()];
            let cell = ((i32::from(center.0) + dx) as u16, (i32::from(center.1) + dy) as u16);
            groups[index as usize] = Some(Gathering { center, cell });
        }
    }
    groups
}

/// One town's townsfolk and the gatherings asked about so far.
pub struct Plan<'s> {
    settings: &'s Settings,
    town: usize,
    pub personas: Vec<Persona>,
    pub routines: Vec<Routine>,
    slots: RefCell<HashMap<i64, Vec<Option<Gathering>>>>,
}

impl<'s> Plan<'s> {
    pub fn new(settings: &'s Settings, town: usize) -> Self {
        let personas: Vec<Persona> = (0..settings.npcs_per_town).map(|index| crate::persona::persona(settings, town, index)).collect();
        let routines = personas.iter().enumerate().map(|(index, persona)| routine(settings, town, index as u32, persona)).collect();
        Self { settings, town, personas, routines, slots: RefCell::new(HashMap::new()) }
    }

    fn gathering(&self, index: u32, slot: i64) -> Option<Gathering> {
        let mut slots = self.slots.borrow_mut();
        slots.entry(slot).or_insert_with(|| gatherings(self.settings, self.town, slot))[index as usize]
    }

    fn gathering_steps(&self, index: u32, slot: i64, gathering: Gathering) -> Vec<(i64, Step)> {
        let start = slot * SLOT_MS;
        let before = self.state_at(index, start - 1);
        let mut rng = Rng::new(&[u64::from(self.settings.seed), CHAT, self.town as u64, u64::from(index), slot as u64]);
        let mut steps = Vec::new();
        let mut time = start;
        if before.hidden {
            steps.push((time, Step::Show));
            time += SHOW_BEFORE_WALK_MS as i64;
        }
        steps.push((time, Step::Walk { to: gathering.cell, door: false }));
        time += walk_budget(before.cell, gathering.cell, self.personas[index as usize].speed) as i64;
        steps.push((time, Step::Face(direction(gathering.cell, gathering.center))));
        time += rng.range(3_000, 8_000) as i64;
        while time < start + SLOT_MS - 3_000 {
            steps.push((time, Step::Emote(*rng.pick(CHATTY))));
            time += rng.range(5_000, 12_000) as i64;
        }
        steps
    }

    /// What townsperson `index` does after `from` up to `to` included, in order.
    pub fn steps_between(&self, index: u32, from: i64, to: i64) -> Vec<(i64, Step)> {
        let mut steps = Vec::new();
        for slot in (from + 1).div_euclid(SLOT_MS)..=to.div_euclid(SLOT_MS) {
            let (low, high) = (from.max(slot * SLOT_MS - 1), to.min((slot + 1) * SLOT_MS - 1));
            match self.gathering(index, slot) {
                Some(gathering) => steps.extend(self.gathering_steps(index, slot, gathering).into_iter().filter(|(time, _)| *time > low && *time <= high)),
                None => steps.extend(self.routines[index as usize].steps_between(low, high).map(|(time, timed)| (time, timed.step))),
            }
        }
        steps
    }

    /// Where townsperson `index` stands at `time`, `None` while inside or heading in: along the walk under way at its
    /// speed, where [`state_at`](Self::state_at) gives where the walk ends. Only an estimate: the server walks them around
    /// obstacles, so the controller asks it where they are.
    #[cfg(test)]
    pub fn position_at(&self, index: u32, time: i64) -> Option<Cell> {
        let state = self.state_at(index, time);
        if state.hidden {
            return None;
        }
        let walk = self.steps_between(index, time - 60_000, time).into_iter().rev().find_map(|(at, step)| match step {
            Step::Walk { to, .. } => Some((at, to)),
            _ => None,
        });
        let Some((started, to)) = walk else { return Some(state.cell) };
        let from = self.state_at(index, started - 1).cell;
        let duration = (octile(from, to) as i64 * i64::from(self.personas[index as usize].speed) / 10).max(1);
        let done = (time - started).clamp(0, duration);
        let between = |a: u16, b: u16| (i64::from(a) + (i64::from(b) - i64::from(a)) * done / duration) as u16;
        Some((between(from.0, to.0), between(from.1, to.1)))
    }

    pub fn state_at(&self, index: u32, time: i64) -> State {
        let slot = time.div_euclid(SLOT_MS);
        let gathered = |gathering: Gathering| State {
            cell: gathering.cell,
            hidden: false,
            dir: direction(gathering.cell, gathering.center),
            doing: Doing::Gathering,
        };
        if let Some(gathering) = self.gathering(index, slot) {
            return gathered(gathering);
        }
        let routine = &self.routines[index as usize];
        let walked_since = routine.steps_between(slot * SLOT_MS - 1, time).any(|(_, timed)| matches!(timed.step, Step::Walk { .. }));
        match self.gathering(index, slot - 1) {
            Some(gathering) if !walked_since => gathered(gathering),
            _ => routine.state_at(time),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::settings;
    use crate::towns::find;

    fn prontera() -> usize {
        find("prontera").unwrap()
    }

    #[test]
    fn a_routine_is_a_loop_that_starts_and_ends_at_home() {
        let settings = settings();
        let plan = Plan::new(&settings, prontera());
        for routine in &plan.routines {
            assert!(routine.phase < routine.length);
            assert!(routine.steps.windows(2).all(|pair| pair[0].at <= pair[1].at), "steps are in order");
            assert!(routine.steps.iter().all(|timed| timed.at < routine.length));
            let Some(Step::Walk { to, door: false }) = routine.steps.iter().rev().map(|timed| timed.step).find(|step| matches!(step, Step::Walk { .. })) else {
                panic!("the loop ends walking home");
            };
            assert_eq!(to, routine.home);
            let mut hidden = false;
            for timed in &routine.steps {
                match timed.step {
                    Step::Walk { door, .. } => {
                        assert!(!hidden, "nobody walks while inside");
                        hidden = door;
                    }
                    Step::Show => {
                        assert!(hidden);
                        hidden = false;
                    }
                    Step::Face(_) | Step::Emote(_) => assert!(!hidden),
                }
            }
            assert!(!hidden);
        }
    }

    #[test]
    fn windows_split_anywhere_give_the_same_steps() {
        let settings = settings();
        let plan = Plan::new(&settings, prontera());
        let (start, end) = (1_000_000_i64, 1_000_000 + 2 * SLOT_MS + 7_777);
        for index in [0, 5, 17, 42, 99] {
            let whole = plan.steps_between(index, start, end);
            let mut pieces = Vec::new();
            let mut from = start;
            for width in [1_000, 1_040, 3_000, 999, 4_999].into_iter().cycle() {
                let to = (from + width).min(end);
                pieces.extend(plan.steps_between(index, from, to));
                from = to;
                if from == end {
                    break;
                }
            }
            assert_eq!(whole, pieces, "townsperson {index}");
            assert!(whole.windows(2).all(|pair| pair[0].0 <= pair[1].0));
        }
    }

    #[test]
    fn a_long_routine_window_repeats_the_loop() {
        let settings = settings();
        let plan = Plan::new(&settings, prontera());
        let routine = &plan.routines[10];
        let steps: Vec<_> = routine.steps_between(0, 3 * routine.length as i64).collect();
        assert!(steps.len() >= 3 * routine.steps.len() - 1 && steps.len() <= 3 * routine.steps.len() + 1, "{} for {}", steps.len(), routine.steps.len());
    }

    #[test]
    fn groups_gather_on_distinct_plazas_without_guards_and_ring_their_centre() {
        let settings = settings();
        let town = prontera();
        for slot in 0..20 {
            let groups = gatherings(&settings, town, slot);
            let members: Vec<(usize, Gathering)> = groups.iter().enumerate().filter_map(|(index, group)| group.map(|group| (index, group))).collect();
            assert!(members.iter().all(|(index, _)| *index as u32 >= settings.guards_per_town));
            let wanted = (settings.npcs_per_town - settings.guards_per_town) as usize * settings.gathering_percent as usize / 100;
            assert!(members.len() <= wanted && members.len() + 5 >= wanted, "{} of {wanted}", members.len());
            let mut cells = std::collections::HashSet::new();
            let mut by_centre: HashMap<Cell, usize> = HashMap::new();
            for (_, gathering) in &members {
                assert!(cells.insert(gathering.cell), "two members share {:?}", gathering.cell);
                assert_eq!(octile(gathering.cell, gathering.center) / 10, 2);
                *by_centre.entry(gathering.center).or_default() += 1;
            }
            assert!(by_centre.values().all(|size| (2..=5).contains(size)), "{by_centre:?}");
        }
    }

    #[test]
    fn a_member_walks_to_its_seat_faces_the_centre_and_chats() {
        let settings = settings();
        let town = prontera();
        let plan = Plan::new(&settings, town);
        let slot = 7_000;
        let (index, gathering) = gatherings(&settings, town, slot).into_iter().enumerate().find_map(|(index, group)| group.map(|group| (index as u32, group))).unwrap();
        let steps = plan.steps_between(index, slot * SLOT_MS - 1, (slot + 1) * SLOT_MS - 1);
        let walk = steps.iter().position(|(_, step)| *step == Step::Walk { to: gathering.cell, door: false }).unwrap();
        assert_eq!(steps[walk + 1].1, Step::Face(direction(gathering.cell, gathering.center)));
        assert!(steps[walk + 2..].iter().all(|(_, step)| matches!(step, Step::Emote(_))) && steps.len() > walk + 4);
        let state = plan.state_at(index, slot * SLOT_MS + 60_000);
        assert_eq!((state.cell, state.doing, state.hidden), (gathering.cell, Doing::Gathering, false));
    }

    #[test]
    fn state_follows_the_steps() {
        let settings = settings();
        let plan = Plan::new(&settings, prontera());
        for index in [3, 30, 60] {
            let routine = &plan.routines[index];
            let start = routine.loop_start(5_000_000);
            for timed in &routine.steps {
                let state = routine.state_at(start + timed.at as i64);
                match timed.step {
                    Step::Walk { to, door } => assert_eq!((state.cell, state.hidden), (to, door)),
                    Step::Face(dir) => assert_eq!(state.dir, dir),
                    Step::Show => assert!(!state.hidden),
                    Step::Emote(_) => {}
                }
            }
        }
    }
}
