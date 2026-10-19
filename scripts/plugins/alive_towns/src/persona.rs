//! Who each townsperson is: name, sprite, look and speed, drawn from (seed, town, index).

use script_sdk_2::NpcAppearance;

use crate::random::{PERSONA, Rng};
use crate::towns::{Cell, TOWNS};
use crate::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Folk,
    Child,
    Guard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Persona {
    pub display: &'static str,
    /// A job class id or a sprite constant.
    pub sprite: String,
    pub kind: Kind,
    pub speed: u16,
    pub appearance: Option<NpcAppearance>,
    /// Where the routine starts and ends.
    pub home: Cell,
    pub dir: u16,
}

const FEMALE: &[&str] = &[
    "Alina", "Bella", "Clara", "Dalia", "Elise", "Fiona", "Greta", "Hanna", "Iris", "Julia", "Karin", "Lena", "Marie", "Nadia",
    "Olga", "Petra", "Rosa", "Sofia", "Tessa", "Vera", "Wanda", "Yuna", "Zoe", "Mina", "Suri", "Elin", "Noa", "Ilse",
];
const MALE: &[&str] = &[
    "Aldo", "Bram", "Cedric", "Dorian", "Emil", "Felix", "Gunter", "Henrik", "Ivo", "Jonas", "Kurt", "Lukas", "Marco", "Niels",
    "Otto", "Pavel", "Rolf", "Stefan", "Tobias", "Ulrich", "Viktor", "Walt", "Yorick", "Hiro", "Tae", "Bruno", "Erik", "Oskar",
];

/// (job class, weight, only for sex) of adults drawn as players; mounted and wedding sprites are left out.
const JOBS: &[(u16, u32, Option<u8>)] = &[
    (0, 8, None), (1, 6, None), (2, 6, None), (3, 6, None), (4, 7, None), (5, 8, None), (6, 5, None), (7, 2, None),
    (8, 3, None), (9, 2, None), (10, 3, None), (11, 2, None), (12, 1, None), (15, 1, None), (16, 2, None), (17, 1, None),
    (18, 2, None), (19, 1, Some(1)), (20, 1, Some(0)),
];
/// Baby Novice to Baby Thief.
const BABY_JOBS: std::ops::RangeInclusive<u16> = 4023..=4029;
/// Hair styles 1 to 27 and colours 0 to 8 exist for both sexes in the client; clothes palettes 0 to 3 for every job above.
const HAIR_STYLES: (u64, u64) = (1, 27);
const HAIR_COLORS: u64 = 8;
const CLOTHES_COLORS: u64 = 3;
/// Head gear view ids, (view, only for sex), from `config/items.json`.
const HEAD_TOP: &[(u16, Option<u8>)] = &[
    (2, None), (4, Some(0)), (5, Some(0)), (6, None), (7, None), (9, None), (11, None), (14, None), (15, Some(0)), (17, Some(0)),
    (18, None), (28, Some(0)), (32, None), (34, None), (37, None), (48, None), (58, Some(1)), (60, None), (62, None), (70, None),
    (74, None), (87, None), (95, Some(1)), (102, Some(1)), (104, None), (111, None), (117, None), (126, Some(0)), (127, None),
];
const HEAD_MID: &[(u16, Option<u8>)] = &[(3, None), (12, None), (47, None), (125, Some(0))];
const HEAD_BOTTOM: &[(u16, Option<u8>)] = &[(8, None), (24, Some(1)), (25, Some(1)), (54, Some(1)), (55, Some(1)), (56, Some(0)), (57, None)];
const MALE_SPRITES: &[&str] = &["4W_M_01", "4W_M_02", "4W_M_03"];
const GUARD_SPRITES: &[&str] = &["8W_SOLDIER", "4_M_GEF_SOLDIER"];
const PORT_TOWNS: &[&str] = &["alberta", "izlude"];

pub fn persona(settings: &Settings, town: usize, index: u32) -> Persona {
    let mut rng = Rng::new(&[u64::from(settings.seed), PERSONA, town as u64, u64::from(index)]);
    let [fast, slow] = settings.walk_speed.map(u64::from);
    let home = *rng.pick(TOWNS[town].spots);
    let dir = rng.below(8) as u16;
    if index < settings.guards_per_town {
        return Persona {
            display: "Guard",
            sprite: rng.pick(GUARD_SPRITES).to_string(),
            kind: Kind::Guard,
            speed: rng.range(fast, slow) as u16,
            appearance: None,
            home,
            dir,
        };
    }
    let sex = rng.below(2) as u8;
    let display = *rng.pick(if sex == 0 { FEMALE } else { MALE });
    let child = rng.chance(settings.children_percent);
    let player_look = rng.chance(settings.player_look_percent);
    let speed = if child {
        rng.range(fast.saturating_sub(30).max(20), fast + 20)
    } else {
        rng.range(fast, slow)
    } as u16;
    let (sprite, appearance) = match (child, player_look) {
        (true, true) => {
            let job = BABY_JOBS.start() + rng.below(u64::from(BABY_JOBS.end() - BABY_JOBS.start() + 1)) as u16;
            (job.to_string(), Some(appearance(&mut rng, sex, false)))
        }
        (true, false) => ("4W_KID".to_string(), None),
        (false, true) => {
            let jobs: Vec<&(u16, u32, Option<u8>)> = JOBS.iter().filter(|(_, _, only)| only.is_none_or(|only| only == sex)).collect();
            let weights: Vec<u32> = jobs.iter().map(|(_, weight, _)| *weight).collect();
            (jobs[rng.weighted(&weights)].0.to_string(), Some(appearance(&mut rng, sex, true)))
        }
        (false, false) if sex == 0 => ("4W_F_01".to_string(), None),
        (false, false) if PORT_TOWNS.contains(&TOWNS[town].map) && rng.chance(40) => ("4W_SAILOR".to_string(), None),
        (false, false) => (rng.pick(MALE_SPRITES).to_string(), None),
    };
    Persona { display, sprite, kind: if child { Kind::Child } else { Kind::Folk }, speed, appearance, home, dir }
}

fn appearance(rng: &mut Rng, sex: u8, adult: bool) -> NpcAppearance {
    let mut gear = |list: &[(u16, Option<u8>)], percent: u32| {
        let fitting: Vec<u16> = list.iter().filter(|(_, only)| only.is_none_or(|only| only == sex)).map(|(view, _)| *view).collect();
        if rng.chance(percent) { *rng.pick(&fitting) } else { 0 }
    };
    let (head_top, head_mid, head_bottom) = (gear(HEAD_TOP, 45), gear(HEAD_MID, 15), gear(HEAD_BOTTOM, if adult { 10 } else { 0 }));
    NpcAppearance {
        sex,
        hair: rng.range(HAIR_STYLES.0, HAIR_STYLES.1) as u16,
        hair_color: rng.range(0, HAIR_COLORS) as u16,
        clothes_color: if adult { rng.range(0, CLOTHES_COLORS) as u16 } else { 0 },
        head_top,
        head_mid,
        head_bottom,
        ..Default::default()
    }
}

/// `Display#atTTIII`: the client shows the part before `#`.
pub fn unique_name(display: &str, town: usize, index: u32) -> String {
    format!("{display}#at{town:02}{index:03}")
}

/// The town and index of a name made by [`unique_name`].
pub fn parse_name(name: &str) -> Option<(usize, u32)> {
    let (_, code) = name.rsplit_once("#at")?;
    if code.len() != 5 || !code.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let town = code[..2].parse::<usize>().ok().filter(|town| *town < TOWNS.len())?;
    Some((town, code[2..].parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::settings;
    use crate::towns::find;

    #[test]
    fn personas_are_reproducible_and_named_within_the_name_limit() {
        let settings = settings();
        let prontera = find("prontera").unwrap();
        let mut names = std::collections::HashSet::new();
        for index in 0..settings.npcs_per_town {
            let persona = persona(&settings, prontera, index);
            assert_eq!(persona, super::persona(&settings, prontera, index));
            let name = unique_name(persona.display, prontera, index);
            assert!(name.len() <= 24, "{name}");
            assert_eq!(parse_name(&name), Some((prontera, index)));
            assert!(names.insert(name));
            assert_eq!(persona.kind == Kind::Guard, index < settings.guards_per_town);
            assert!(TOWNS[prontera].spots.contains(&persona.home));
            if let Some(look) = persona.appearance {
                assert!((1..=27).contains(&look.hair) && look.hair_color <= 8 && look.clothes_color <= 3);
                let job: u16 = persona.sprite.parse().unwrap();
                assert!(job != 19 || look.sex == 1, "bards are men");
                assert!(job != 20 || look.sex == 0, "dancers are women");
            }
        }
        assert_eq!(parse_name("Rate Clerk"), None);
    }

    #[test]
    fn the_settings_shape_the_crowd() {
        let mut settings = settings();
        settings.player_look_percent = 0;
        settings.children_percent = 0;
        let prontera = find("prontera").unwrap();
        for index in settings.guards_per_town..settings.npcs_per_town {
            let persona = persona(&settings, prontera, index);
            assert!(persona.appearance.is_none() && persona.sprite.starts_with("4W_") && persona.kind == Kind::Folk, "{persona:?}");
            assert!((settings.walk_speed[0]..=settings.walk_speed[1]).contains(&persona.speed));
        }
    }
}
