//! The places of each town, generated into `towns_data.rs` by `tools/alive-towns-data` from the map cells, city warps and
//! placed NPCs. Every cell is reachable from the town's arrival cell.

pub use crate::towns_data::TOWNS;

pub type Cell = (u16, u16);

pub struct Town {
    pub map: &'static str,
    pub name: &'static str,
    pub doors: &'static [Door],
    pub counters: &'static [Counter],
    /// Cells with free neighbours, about 8 cells apart.
    pub spots: &'static [Cell],
    /// Centres of open 7×7 areas, so a ring of radius 2 around them is walkable.
    pub plazas: &'static [Cell],
}

/// A warp into a building.
pub struct Door {
    pub cell: Cell,
    /// Where someone coming out stands, outside the warp.
    pub front: Cell,
    pub inside: &'static str,
}

/// A placed NPC someone can walk up to.
pub struct Counter {
    /// The name the NPC shows.
    pub npc: &'static str,
    pub front: Cell,
    /// Direction from `front` towards the NPC.
    pub face: u8,
}

pub fn find(map: &str) -> Option<usize> {
    TOWNS.iter().position(|town| town.map == map)
}

/// Tenths of cells, diagonals counted 1.4.
pub fn octile((ax, ay): Cell, (bx, by): Cell) -> u64 {
    let (dx, dy) = (u64::from(ax.abs_diff(bx)), u64::from(ay.abs_diff(by)));
    10 * dx.max(dy) + 4 * dx.min(dy)
}

/// RO direction from `from` towards `to`: 0 north (+y), turning counterclockwise.
pub fn direction(from: Cell, to: Cell) -> u8 {
    let dx = i32::from(to.0) - i32::from(from.0);
    let dy = i32::from(to.1) - i32::from(from.1);
    match (dx.signum(), dy.signum()) {
        (0, 1) | (0, 0) => 0,
        (-1, 1) => 1,
        (-1, 0) => 2,
        (-1, -1) => 3,
        (0, -1) => 4,
        (1, -1) => 5,
        (1, 0) => 6,
        _ => 7,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_town_has_room_for_routines_and_gatherings() {
        for town in TOWNS {
            assert!(town.spots.len() >= 8 && !town.plazas.is_empty(), "{}", town.map);
            assert!(town.counters.iter().all(|counter| counter.face < 8 && !counter.npc.is_empty()), "{}", town.map);
            assert!(town.doors.iter().all(|door| door.cell != door.front), "{}", town.map);
        }
        assert!(find("prontera").is_some() && find("prt_fild08").is_none());
    }

    #[test]
    fn directions_follow_the_client() {
        assert_eq!(direction((10, 10), (10, 12)), 0);
        assert_eq!(direction((10, 10), (8, 10)), 2);
        assert_eq!(direction((10, 10), (10, 3)), 4);
        assert_eq!(direction((10, 10), (15, 15)), 7);
    }
}
