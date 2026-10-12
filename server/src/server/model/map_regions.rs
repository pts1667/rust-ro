//! Walkable areas ("regions") of a map: the parts a character can walk between without a warp. An interior map such as
//! `geffen_in` is several rooms, and fields can have closed-off parts; a warp is only usable from its own region.
//!
//! Walking never cuts a corner (a diagonal step needs both side cells walkable), so two cells are in the same region
//! exactly when they are joined by orthogonal steps.
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;

/// How far from a cell that is not walkable (a warp landing on a wall) the nearest walkable one is looked for.
const LANDING_SEARCH: u16 = 3;

/// Regions of one map, numbered from 1 in the order of the cells (row `y = 0` first), so the numbers are stable.
pub struct MapRegions {
    width: u16,
    height: u16,
    /// Region of every cell, 0 for a cell that can not be walked on.
    labels: Vec<u32>,
    /// Number of cells of each region, `sizes[0]` being region 1.
    sizes: Vec<u32>,
}

impl MapRegions {
    pub fn compute(cells: &[u16], width: u16, height: u16) -> Self {
        let (w, h) = (usize::from(width), usize::from(height));
        let walkable = |index: usize| cells.get(index).is_some_and(|cell| cell & CellType::Walkable.as_flag() != 0);
        let mut labels = vec![0_u32; w * h];
        let mut sizes = Vec::new();
        let mut stack = Vec::new();
        for start in 0..w * h {
            if labels[start] != 0 || !walkable(start) {
                continue;
            }
            let region = sizes.len() as u32 + 1;
            let mut size = 0_u32;
            labels[start] = region;
            stack.push(start);
            while let Some(index) = stack.pop() {
                size += 1;
                let (x, y) = (index % w, index / w);
                let neighbours = [
                    (x > 0).then(|| index - 1),
                    (x + 1 < w).then_some(index + 1),
                    (y > 0).then(|| index - w),
                    (y + 1 < h).then_some(index + w),
                ];
                for next in neighbours.into_iter().flatten() {
                    if labels[next] == 0 && walkable(next) {
                        labels[next] = region;
                        stack.push(next);
                    }
                }
            }
            sizes.push(size);
        }
        Self { width, height, labels, sizes }
    }

    pub fn count(&self) -> usize {
        self.sizes.len()
    }

    /// Cells of each region, the first being region 1.
    pub fn sizes(&self) -> &[u32] {
        &self.sizes
    }

    /// The region of a cell, `None` when it can not be walked on.
    pub fn at(&self, x: u16, y: u16) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let label = self.labels[usize::from(y) * usize::from(self.width) + usize::from(x)];
        (label != 0).then_some(label)
    }

    /// The region of a cell, or of the nearest walkable cell when the cell itself is not: where a warp landing there
    /// puts a character.
    pub fn landing(&self, x: u16, y: u16) -> Option<u32> {
        (0..=LANDING_SEARCH).find_map(|ring| self.most_common(x, y, ring, ring))
    }

    /// The region of a warp: the one most of the walkable cells of its trigger area belong to (the area is
    /// `x - half_width .. x + half_width`, as the map flags it), or the cell itself for a warp of size 0.
    pub fn of_area(&self, x: u16, y: u16, half_width: u16, half_height: u16) -> Option<u32> {
        self.most_common(x, y, half_width, half_height).or_else(|| self.landing(x, y))
    }

    fn most_common(&self, x: u16, y: u16, half_width: u16, half_height: u16) -> Option<u32> {
        let mut counts: HashMap<u32, u32> = HashMap::new();
        let xs = x.saturating_sub(half_width)..=x.saturating_add(half_width);
        for cx in xs {
            for cy in y.saturating_sub(half_height)..=y.saturating_add(half_height) {
                if let Some(region) = self.at(cx, cy) {
                    *counts.entry(region).or_default() += 1;
                }
            }
        }
        counts.into_iter().max_by_key(|(region, count)| (*count, std::cmp::Reverse(*region))).map(|(region, _)| region)
    }
}

static REGIONS: OnceLock<Mutex<HashMap<String, Option<Arc<MapRegions>>>>> = OnceLock::new();

/// The regions of a map from its map cache, computed once and kept. `None` when the map has no cache file.
pub fn of_map(name: &str) -> Option<Arc<MapRegions>> {
    let name = crate::server::model::map::Map::name_without_ext(name);
    let cache = REGIONS.get_or_init(Default::default);
    if let Some(known) = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).get(&name) {
        return known.clone();
    }
    // Read outside the lock: a cache file takes a few milliseconds to inflate
    // SAFETY: MAP_DIR is only written once, at boot and in test setup, before any map is read
    let dir = unsafe { crate::MAP_DIR };
    let regions = map_cache::read_mcache(Path::new(dir), &name)
        .ok()
        .map(|map| Arc::new(MapRegions::compute(&map.cells, map.xs, map.ys)));
    cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(name, regions.clone());
    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `rows[0]` is `y = 0`; `.` is walkable.
    fn grid(rows: &[&str]) -> MapRegions {
        let width = rows[0].len() as u16;
        let cells: Vec<u16> = rows
            .iter()
            .flat_map(|row| row.chars().map(|c| if c == '.' { CellType::Walkable.as_flag() } else { 0 }))
            .collect();
        MapRegions::compute(&cells, width, rows.len() as u16)
    }

    #[test]
    fn rooms_split_by_a_wall_are_two_regions() {
        let regions = grid(&["..#..", "..#..", "..#.."]);
        assert_eq!(regions.count(), 2);
        assert_eq!(regions.at(0, 0), Some(1));
        assert_eq!(regions.at(4, 2), Some(2));
        assert_eq!(regions.at(2, 1), None);
        assert_eq!(regions.sizes(), &[6, 6]);
    }

    #[test]
    fn a_diagonal_gap_does_not_join_two_regions() {
        // Walking never cuts the corner between (1, 0) and (2, 1)
        let regions = grid(&["..#", "#.."]);
        assert_eq!(regions.at(0, 0), regions.at(1, 1));
        let regions = grid(&[".#", "#."]);
        assert_ne!(regions.at(0, 0), regions.at(1, 1));
    }

    #[test]
    fn a_landing_on_a_wall_counts_for_the_nearest_walkable_cell() {
        let regions = grid(&["...##", "...##", "#####"]);
        assert_eq!(regions.landing(3, 1), Some(1));
        assert_eq!(regions.landing(1, 1), Some(1));
    }

    #[test]
    fn the_rooms_of_an_interior_map_are_regions_of_their_own() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/maps/pre-re");
        let map = map_cache::read_mcache(&dir, "geffen_in").unwrap();
        let regions = MapRegions::compute(&map.cells, map.xs, map.ys);
        assert!(regions.count() > 5, "geffen_in has {} regions", regions.count());
        let field = map_cache::read_mcache(&dir, "prt_fild08").unwrap();
        let regions = MapRegions::compute(&field.cells, field.xs, field.ys);
        let largest = *regions.sizes().iter().max().unwrap();
        assert!(largest as usize * 2 > regions.sizes().iter().sum::<u32>() as usize, "one region holds most of a field");
    }

    #[test]
    fn a_warp_belongs_to_the_region_most_of_its_cells_are_in() {
        let regions = grid(&["...#.", "...#.", "...#."]);
        assert_eq!(regions.of_area(2, 1, 1, 1), Some(1));
        assert_eq!(regions.of_area(4, 1, 0, 0), Some(2));
    }
}
