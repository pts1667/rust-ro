//! Long-distance pathfinding. Walkable cells are grouped into the connected components of 8×8 chunks, and the graph of
//! those components into the connected components of 32×32 segments. A route is searched on the segments, then on the
//! chunk components inside the segments found, and comes out as waypoints close enough together for the client's own
//! A* (which gives up after 100 iterations) to walk each leg.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

pub const CHUNK: u16 = 8;
const SEGMENT_CHUNKS: u16 = 4;
/// 8-connected components of an 8×8 chunk: at most 16, cells two apart.
const CHUNK_NODES: u32 = 16;
const SEGMENT_NODES: u32 = (SEGMENT_CHUNKS * SEGMENT_CHUNKS) as u32 * CHUNK_NODES;
/// Longest straight leg kept when waypoints are merged.
const MAX_LEG: u16 = 12;
/// Length of the legs cut from [`local_route`] when the client's A* cannot walk a leg.
pub const SUB_LEG: usize = 6;

pub type Cell = (u16, u16);

const NEIGHBOURS: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Link {
    to: u32,
    /// The cell of the target component that this crossing enters.
    to_cell: Cell,
    cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChunkNode {
    representative: Cell,
    links: Vec<Link>,
    /// The segment component holding this one.
    group: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GroupNode {
    representative: Cell,
    links: Vec<(u32, u32)>,
    region: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavHierarchy {
    x_size: u16,
    y_size: u16,
    chunks_x: u16,
    chunks_y: u16,
    segments_x: u16,
    segments_y: u16,
    /// Per cell: 0 when it cannot be walked, else its component in its chunk, from 1.
    component: Vec<u8>,
    chunks: Vec<Vec<ChunkNode>>,
    segments: Vec<Vec<GroupNode>>,
}

pub fn octile((ax, ay): Cell, (bx, by): Cell) -> u32 {
    let (dx, dy) = (u32::from(ax.abs_diff(bx)), u32::from(ay.abs_diff(by)));
    10 * dx.max(dy) + 4 * dx.min(dy)
}

fn walkable(cells: &[u16], x_size: u16, (x, y): Cell) -> bool {
    cells.get(usize::from(x) + usize::from(y) * usize::from(x_size)).is_some_and(|cell| cell & 1 == 1)
}

fn neighbour(x_size: u16, y_size: u16, (x, y): Cell, (dx, dy): (i32, i32)) -> Option<Cell> {
    let (nx, ny) = (i32::from(x) + dx, i32::from(y) + dy);
    (nx >= 0 && ny >= 0 && nx < i32::from(x_size) && ny < i32::from(y_size)).then(|| (nx as u16, ny as u16))
}

/// The walkable cell of `cells` closest to their centroid.
fn representative(cells: &[Cell]) -> Cell {
    let count = cells.len() as u32;
    let centroid = (
        (cells.iter().map(|cell| u32::from(cell.0)).sum::<u32>() / count) as u16,
        (cells.iter().map(|cell| u32::from(cell.1)).sum::<u32>() / count) as u16,
    );
    *cells.iter().min_by_key(|cell| octile(**cell, centroid)).unwrap()
}

/// A* over node ids; `neighbours` pushes `(node, cost)` pairs.
fn search(start: u32, goal: u32, mut neighbours: impl FnMut(u32, &mut Vec<(u32, u32)>), heuristic: impl Fn(u32) -> u32) -> Option<Vec<u32>> {
    let mut best: HashMap<u32, (u32, u32)> = HashMap::from([(start, (0, start))]);
    let mut open = BinaryHeap::from([Reverse((heuristic(start), 0, start))]);
    let mut next = Vec::new();
    while let Some(Reverse((_, cost, node))) = open.pop() {
        if node == goal {
            let mut path = vec![goal];
            while let Some(&(_, parent)) = best.get(path.last().unwrap()).filter(|(_, parent)| *parent != *path.last().unwrap()) {
                path.push(parent);
            }
            path.reverse();
            return Some(path);
        }
        if best.get(&node).is_some_and(|(known, _)| *known < cost) {
            continue;
        }
        next.clear();
        neighbours(node, &mut next);
        for &(to, step) in &next {
            let reached = cost + step;
            if best.get(&to).is_none_or(|(known, _)| reached < *known) {
                best.insert(to, (reached, node));
                open.push(Reverse((reached + heuristic(to), reached, to)));
            }
        }
    }
    None
}

impl NavHierarchy {
    pub fn build(x_size: u16, y_size: u16, cells: &[u16]) -> Self {
        let chunks_x = x_size.div_ceil(CHUNK);
        let chunks_y = y_size.div_ceil(CHUNK);
        let segments_x = chunks_x.div_ceil(SEGMENT_CHUNKS);
        let segments_y = chunks_y.div_ceil(SEGMENT_CHUNKS);
        let mut nav = Self {
            x_size,
            y_size,
            chunks_x,
            chunks_y,
            segments_x,
            segments_y,
            component: vec![0; usize::from(x_size) * usize::from(y_size)],
            chunks: vec![Vec::new(); usize::from(chunks_x) * usize::from(chunks_y)],
            segments: vec![Vec::new(); usize::from(segments_x) * usize::from(segments_y)],
        };
        let chunks: Vec<usize> = (0..nav.chunks.len()).collect();
        let segments: Vec<usize> = (0..nav.segments.len()).collect();
        nav.rebuild(cells, &chunks, &chunks, &segments, &segments);
        nav
    }

    /// Takes the walkability changes of `cells` in the rectangle into account, rebuilding only what they touch.
    pub fn update_area(&mut self, cells: &[u16], (x1, y1): Cell, (x2, y2): Cell) {
        let clamp = |value: u16, size: u16| value.min(size.saturating_sub(1)) / CHUNK;
        let (cx1, cx2) = (clamp(x1.min(x2), self.x_size), clamp(x1.max(x2), self.x_size));
        let (cy1, cy2) = (clamp(y1.min(y2), self.y_size), clamp(y1.max(y2), self.y_size));
        let chunk = |cx: u16, cy: u16| usize::from(cx) + usize::from(cy) * usize::from(self.chunks_x);
        let refilled: Vec<usize> = (cy1..=cy2).flat_map(|cy| (cx1..=cx2).map(move |cx| (cx, cy))).map(|(cx, cy)| chunk(cx, cy)).collect();
        let around = |from: u16, to: u16, count: u16| from.saturating_sub(1)..=(to + 1).min(count - 1);
        let relinked: Vec<usize> = around(cy1, cy2, self.chunks_y)
            .flat_map(|cy| around(cx1, cx2, self.chunks_x).map(move |cx| (cx, cy)))
            .map(|(cx, cy)| chunk(cx, cy))
            .collect();
        let regrouped: Vec<usize> = relinked.iter().map(|chunk| self.segment_of_chunk(*chunk)).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
        let segment_links: Vec<usize> = regrouped
            .iter()
            .flat_map(|segment| self.segment_and_neighbours(*segment))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        self.rebuild(cells, &refilled, &relinked, &regrouped, &segment_links);
    }

    fn rebuild(&mut self, cells: &[u16], refilled: &[usize], relinked: &[usize], regrouped: &[usize], segment_links: &[usize]) {
        for chunk in refilled {
            self.fill_chunk(cells, *chunk);
        }
        for chunk in relinked {
            self.link_chunk(*chunk);
        }
        for segment in regrouped {
            self.group_segment(*segment);
        }
        for segment in segment_links {
            self.link_segment(*segment);
        }
        self.label_regions();
    }

    /// True when a walk from `a` can reach `b`.
    pub fn reachable(&self, a: Cell, b: Cell) -> bool {
        match (self.node_of(a), self.node_of(b)) {
            (Some(a), Some(b)) => self.group(self.chunk_node(a).group).region == self.group(self.chunk_node(b).group).region,
            _ => false,
        }
    }

    /// Waypoints from `from` to `to`, `to` last, each close enough to the previous one for the client's A* in most cases;
    /// `None` when `to` cannot be reached.
    pub fn route(&self, cells: &[u16], from: Cell, to: Cell) -> Option<Vec<Cell>> {
        if !self.reachable(from, to) {
            return None;
        }
        if from == to {
            return Some(vec![]);
        }
        let (start, goal) = (self.node_of(from)?, self.node_of(to)?);
        let mut waypoints = Vec::new();
        if start != goal {
            let corridor = self.corridor(self.chunk_node(start).group, self.chunk_node(goal).group)?;
            let goal_cell = self.chunk_node(goal).representative;
            let path = search(
                start,
                goal,
                |node, next| {
                    next.extend(self.chunk_node(node).links.iter().filter(|link| corridor.contains(&self.chunk_node(link.to).group)).map(|link| (link.to, link.cost)))
                },
                |node| octile(self.chunk_node(node).representative, goal_cell),
            )?;
            for pair in path.windows(2) {
                waypoints.push(self.chunk_node(pair[0]).links.iter().find(|link| link.to == pair[1])?.to_cell);
            }
        }
        waypoints.push(to);
        Some(self.merge_straight_legs(cells, from, &waypoints))
    }

    /// The segment components a route from `start` to `goal` goes through.
    fn corridor(&self, start: u32, goal: u32) -> Option<HashSet<u32>> {
        if start == goal {
            return Some(HashSet::from([start]));
        }
        let goal_cell = self.group(goal).representative;
        let path = search(
            start,
            goal,
            |group, next| next.extend(self.group(group).links.iter().copied()),
            |group| octile(self.group(group).representative, goal_cell),
        )?;
        Some(path.into_iter().collect())
    }

    /// Drops the waypoints a straight, walkable line of at most [`MAX_LEG`] cells can skip.
    fn merge_straight_legs(&self, cells: &[u16], from: Cell, waypoints: &[Cell]) -> Vec<Cell> {
        let mut merged = Vec::new();
        let mut anchor = from;
        let mut index = 0;
        while index < waypoints.len() {
            let mut farthest = index;
            for candidate in index + 1..waypoints.len() {
                let distance = anchor.0.abs_diff(waypoints[candidate].0).max(anchor.1.abs_diff(waypoints[candidate].1));
                if distance > 2 * MAX_LEG {
                    break;
                }
                if distance <= MAX_LEG && self.straight_walkable(cells, anchor, waypoints[candidate]) {
                    farthest = candidate;
                }
            }
            if waypoints[farthest] != anchor {
                merged.push(waypoints[farthest]);
            }
            anchor = waypoints[farthest];
            index = farthest + 1;
        }
        merged
    }

    fn straight_walkable(&self, cells: &[u16], (x0, y0): Cell, (x1, y1): Cell) -> bool {
        let (mut x, mut y) = (i32::from(x0), i32::from(y0));
        let (dx, dy) = ((i32::from(x1) - x).abs(), -(i32::from(y1) - y).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let mut error = dx + dy;
        loop {
            if !walkable(cells, self.x_size, (x as u16, y as u16)) {
                return false;
            }
            if (x, y) == (i32::from(x1), i32::from(y1)) {
                return true;
            }
            let doubled = 2 * error;
            if doubled >= dy {
                error += dy;
                x += sx;
            }
            if doubled <= dx {
                error += dx;
                y += sy;
            }
        }
    }

    fn chunk_of(&self, (x, y): Cell) -> usize {
        usize::from(x / CHUNK) + usize::from(y / CHUNK) * usize::from(self.chunks_x)
    }

    fn node_of(&self, cell: Cell) -> Option<u32> {
        if cell.0 >= self.x_size || cell.1 >= self.y_size {
            return None;
        }
        let component = self.component[usize::from(cell.0) + usize::from(cell.1) * usize::from(self.x_size)];
        (component > 0).then(|| self.chunk_of(cell) as u32 * CHUNK_NODES + u32::from(component) - 1)
    }

    fn chunk_node(&self, id: u32) -> &ChunkNode {
        &self.chunks[(id / CHUNK_NODES) as usize][(id % CHUNK_NODES) as usize]
    }

    fn group(&self, id: u32) -> &GroupNode {
        &self.segments[(id / SEGMENT_NODES) as usize][(id % SEGMENT_NODES) as usize]
    }

    fn chunk_bounds(&self, chunk: usize) -> (u16, u16, u16, u16) {
        let (cx, cy) = ((chunk % usize::from(self.chunks_x)) as u16, (chunk / usize::from(self.chunks_x)) as u16);
        (cx * CHUNK, cy * CHUNK, (cx * CHUNK + CHUNK).min(self.x_size), (cy * CHUNK + CHUNK).min(self.y_size))
    }

    fn segment_of_chunk(&self, chunk: usize) -> usize {
        let (cx, cy) = ((chunk % usize::from(self.chunks_x)) as u16, (chunk / usize::from(self.chunks_x)) as u16);
        usize::from(cx / SEGMENT_CHUNKS) + usize::from(cy / SEGMENT_CHUNKS) * usize::from(self.segments_x)
    }

    fn segment_chunks(&self, segment: usize) -> impl Iterator<Item = usize> + '_ {
        let (sx, sy) = ((segment % usize::from(self.segments_x)) as u16, (segment / usize::from(self.segments_x)) as u16);
        let xs = sx * SEGMENT_CHUNKS..(sx * SEGMENT_CHUNKS + SEGMENT_CHUNKS).min(self.chunks_x);
        let ys = sy * SEGMENT_CHUNKS..(sy * SEGMENT_CHUNKS + SEGMENT_CHUNKS).min(self.chunks_y);
        ys.flat_map(move |cy| xs.clone().map(move |cx| usize::from(cx) + usize::from(cy) * usize::from(self.chunks_x)))
    }

    fn segment_and_neighbours(&self, segment: usize) -> Vec<usize> {
        let (sx, sy) = ((segment % usize::from(self.segments_x)) as i32, (segment / usize::from(self.segments_x)) as i32);
        let mut segments = Vec::new();
        for (dx, dy) in [(0, 0)].into_iter().chain(NEIGHBOURS) {
            let (nx, ny) = (sx + dx, sy + dy);
            if nx >= 0 && ny >= 0 && nx < i32::from(self.segments_x) && ny < i32::from(self.segments_y) {
                segments.push(nx as usize + ny as usize * usize::from(self.segments_x));
            }
        }
        segments
    }

    fn fill_chunk(&mut self, cells: &[u16], chunk: usize) {
        let (x0, y0, x1, y1) = self.chunk_bounds(chunk);
        let index = |(x, y): Cell| usize::from(x) + usize::from(y) * usize::from(self.x_size);
        for y in y0..y1 {
            for x in x0..x1 {
                self.component[index((x, y))] = 0;
            }
        }
        let mut nodes = Vec::new();
        for y in y0..y1 {
            for x in x0..x1 {
                if self.component[index((x, y))] != 0 || !walkable(cells, self.x_size, (x, y)) {
                    continue;
                }
                let id = nodes.len() as u8 + 1;
                self.component[index((x, y))] = id;
                let mut members = vec![(x, y)];
                let mut pending = vec![(x, y)];
                while let Some(cell) = pending.pop() {
                    for offset in NEIGHBOURS {
                        let Some(next) = neighbour(self.x_size, self.y_size, cell, offset) else { continue };
                        let inside = (x0..x1).contains(&next.0) && (y0..y1).contains(&next.1);
                        if inside && self.component[index(next)] == 0 && walkable(cells, self.x_size, next) {
                            self.component[index(next)] = id;
                            members.push(next);
                            pending.push(next);
                        }
                    }
                }
                nodes.push(ChunkNode { representative: representative(&members), links: Vec::new(), group: 0 });
            }
        }
        self.chunks[chunk] = nodes;
    }

    fn link_chunk(&mut self, chunk: usize) {
        let (x0, y0, x1, y1) = self.chunk_bounds(chunk);
        let mut crossings: BTreeMap<(u32, u32), Vec<(Cell, Cell)>> = BTreeMap::new();
        for y in y0..y1 {
            for x in x0..x1 {
                if !(x == x0 || y == y0 || x + 1 == x1 || y + 1 == y1) {
                    continue;
                }
                let Some(from) = self.node_of((x, y)) else { continue };
                for offset in NEIGHBOURS {
                    let Some(next) = neighbour(self.x_size, self.y_size, (x, y), offset) else { continue };
                    if (x0..x1).contains(&next.0) && (y0..y1).contains(&next.1) {
                        continue;
                    }
                    if let Some(to) = self.node_of(next) {
                        crossings.entry((from, to)).or_default().push(((x, y), next));
                    }
                }
            }
        }
        for node in &mut self.chunks[chunk] {
            node.links.clear();
        }
        for ((from, to), pairs) in crossings {
            let (from_cell, to_cell) = pairs[pairs.len() / 2];
            let cost = octile(self.chunk_node(from).representative, from_cell) + octile(from_cell, to_cell) + octile(to_cell, self.chunk_node(to).representative);
            self.chunks[chunk][(from % CHUNK_NODES) as usize].links.push(Link { to, to_cell, cost });
        }
    }

    fn group_segment(&mut self, segment: usize) {
        let members: Vec<u32> = self
            .segment_chunks(segment)
            .flat_map(|chunk| (0..self.chunks[chunk].len() as u32).map(move |index| chunk as u32 * CHUNK_NODES + index))
            .collect();
        let inside: HashSet<u32> = members.iter().copied().collect();
        let mut group_of: HashMap<u32, u32> = HashMap::new();
        let mut groups: Vec<Vec<Cell>> = Vec::new();
        for &member in &members {
            if group_of.contains_key(&member) {
                continue;
            }
            let index = groups.len() as u32;
            let mut representatives = vec![];
            let mut pending = vec![member];
            group_of.insert(member, index);
            while let Some(node) = pending.pop() {
                representatives.push(self.chunk_node(node).representative);
                for link in &self.chunk_node(node).links {
                    if inside.contains(&link.to) && !group_of.contains_key(&link.to) {
                        group_of.insert(link.to, index);
                        pending.push(link.to);
                    }
                }
            }
            groups.push(representatives);
        }
        for (node, index) in group_of {
            self.chunks[(node / CHUNK_NODES) as usize][(node % CHUNK_NODES) as usize].group = segment as u32 * SEGMENT_NODES + index;
        }
        self.segments[segment] = groups.iter().map(|representatives| GroupNode { representative: representative(representatives), links: Vec::new(), region: 0 }).collect();
    }

    fn link_segment(&mut self, segment: usize) {
        let mut links: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        for chunk in self.segment_chunks(segment) {
            for node in &self.chunks[chunk] {
                for link in &node.links {
                    let to = self.chunk_node(link.to).group;
                    if (to / SEGMENT_NODES) as usize != segment {
                        let cost = octile(self.group(node.group).representative, self.group(to).representative);
                        links.insert((node.group, to), cost);
                    }
                }
            }
        }
        for group in &mut self.segments[segment] {
            group.links.clear();
        }
        for ((from, to), cost) in links {
            self.segments[segment][(from % SEGMENT_NODES) as usize].links.push((to, cost));
        }
    }

    fn label_regions(&mut self) {
        let mut labelled: HashSet<u32> = HashSet::new();
        let mut region = 0;
        for segment in 0..self.segments.len() {
            for index in 0..self.segments[segment].len() as u32 {
                let start = segment as u32 * SEGMENT_NODES + index;
                if !labelled.insert(start) {
                    continue;
                }
                let mut pending = vec![start];
                while let Some(group) = pending.pop() {
                    self.segments[(group / SEGMENT_NODES) as usize][(group % SEGMENT_NODES) as usize].region = region;
                    for &(to, _) in &self.group(group).links {
                        if labelled.insert(to) {
                            pending.push(to);
                        }
                    }
                }
                region += 1;
            }
        }
    }
}

/// The cells from `from` to `to` (excluding `from`), searched within `margin` cells of the box they span.
pub fn local_route(x_size: u16, y_size: u16, cells: &[u16], from: Cell, to: Cell, margin: u16) -> Option<Vec<Cell>> {
    let (min_x, max_x) = (from.0.min(to.0).saturating_sub(margin), from.0.max(to.0).saturating_add(margin).min(x_size - 1));
    let (min_y, max_y) = (from.1.min(to.1).saturating_sub(margin), from.1.max(to.1).saturating_add(margin).min(y_size - 1));
    let id = |(x, y): Cell| u32::from(x) + u32::from(y) * u32::from(x_size);
    let cell = |id: u32| ((id % u32::from(x_size)) as u16, (id / u32::from(x_size)) as u16);
    let path = search(
        id(from),
        id(to),
        |node, next| {
            for offset in NEIGHBOURS {
                let Some(candidate) = neighbour(x_size, y_size, cell(node), offset) else { continue };
                if (min_x..=max_x).contains(&candidate.0) && (min_y..=max_y).contains(&candidate.1) && walkable(cells, x_size, candidate) {
                    next.push((id(candidate), if offset.0 != 0 && offset.1 != 0 { 14 } else { 10 }));
                }
            }
        },
        |node| octile(cell(node), to),
    )?;
    Some(path.into_iter().skip(1).map(cell).collect())
}

/// Every [`SUB_LEG`]th cell of `path`, and its last.
pub fn sub_legs(path: &[Cell]) -> Vec<Cell> {
    path.chunks(SUB_LEG).filter_map(|legs| legs.last().copied()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::path_search_client_side_algorithm;

    struct Grid {
        x_size: u16,
        y_size: u16,
        cells: Vec<u16>,
    }

    impl Grid {
        fn open(x_size: u16, y_size: u16) -> Self {
            Self { x_size, y_size, cells: vec![1; usize::from(x_size) * usize::from(y_size)] }
        }

        fn wall(&mut self, (x1, y1): Cell, (x2, y2): Cell) {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    self.cells[usize::from(x) + usize::from(y) * usize::from(self.x_size)] = 0;
                }
            }
        }

        fn nav(&self) -> NavHierarchy {
            NavHierarchy::build(self.x_size, self.y_size, &self.cells)
        }

        fn client_walks(&self, from: Cell, to: Cell) -> bool {
            from == to || !path_search_client_side_algorithm(self.x_size, self.y_size, &self.cells, from.0, from.1, to.0, to.1).is_empty()
        }

        /// Walks the route leg by leg as the server would, falling back to [`local_route`] when a leg fails.
        fn walk(&self, nav: &NavHierarchy, from: Cell, to: Cell) -> (usize, usize) {
            let waypoints = nav.route(&self.cells, from, to).expect("reachable");
            assert_eq!(waypoints.last().copied(), Some(to));
            let (mut at, mut fallbacks) = (from, 0);
            for waypoint in &waypoints {
                assert!(walkable(&self.cells, self.x_size, *waypoint), "{waypoint:?} is not walkable");
                assert_ne!(*waypoint, at, "a leg from {from:?} to {to:?} goes nowhere");
                if !self.client_walks(at, *waypoint) {
                    fallbacks += 1;
                    let path = local_route(self.x_size, self.y_size, &self.cells, at, *waypoint, CHUNK).expect("local route");
                    let mut sub_at = at;
                    for sub in sub_legs(&path) {
                        assert!(self.client_walks(sub_at, sub), "sub-leg {sub_at:?} -> {sub:?}");
                        sub_at = sub;
                    }
                }
                at = *waypoint;
            }
            (waypoints.len(), fallbacks)
        }
    }

    #[test]
    fn a_chunk_cut_by_a_wall_holds_two_components() {
        let mut grid = Grid::open(8, 8);
        grid.wall((4, 0), (4, 7));
        let nav = grid.nav();
        assert_eq!(nav.chunks[0].len(), 2);
        assert!(!nav.reachable((0, 0), (7, 7)));
        assert!(nav.reachable((0, 0), (3, 7)));
        assert!(nav.route(&grid.cells, (0, 0), (7, 7)).is_none());
    }

    #[test]
    fn diagonal_steps_connect_like_the_client() {
        let mut grid = Grid::open(16, 16);
        grid.wall((0, 8), (15, 8));
        grid.cells[7 + 8 * 16] = 1;
        assert!(grid.nav().reachable((0, 0), (15, 15)));
    }

    #[test]
    fn a_long_corridor_is_walked_in_client_sized_legs() {
        let mut grid = Grid::open(200, 20);
        grid.wall((0, 5), (199, 5));
        grid.wall((0, 15), (199, 15));
        let nav = grid.nav();
        let (legs, fallbacks) = grid.walk(&nav, (1, 10), (198, 10));
        assert!(legs >= 200 / usize::from(MAX_LEG), "{legs} legs");
        assert_eq!(fallbacks, 0);
        assert!(grid.client_walks((1, 10), (12, 10)));
        assert!(!grid.client_walks((1, 10), (198, 4)));
    }

    #[test]
    fn no_leg_of_a_walk_across_prontera_goes_nowhere() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/maps/pre-re");
        let Ok(map) = map_cache::read_mcache(&dir, "prontera") else { return };
        let grid = Grid { x_size: map.xs, y_size: map.ys, cells: map.cells };
        let nav = grid.nav();
        let spawn = (155, 183);
        let mut seed = 0x9E37_79B9_u32;
        let mut next = |bound: u16| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed % u32::from(bound)) as u16
        };
        let mut cells = Vec::new();
        while cells.len() < 400 {
            let cell = (next(grid.x_size), next(grid.y_size));
            if nav.reachable(spawn, cell) {
                cells.push(cell);
            }
        }
        for pair in cells.windows(2) {
            grid.walk(&nav, pair[0], pair[1]);
        }
    }

    #[test]
    fn a_maze_is_crossed() {
        let mut grid = Grid::open(96, 96);
        for (index, x) in (8..96).step_by(8).enumerate() {
            if index % 2 == 0 {
                grid.wall((x, 0), (x, 90));
            } else {
                grid.wall((x, 5), (x, 95));
            }
        }
        let nav = grid.nav();
        assert!(nav.reachable((1, 1), (94, 94)));
        let (legs, _) = grid.walk(&nav, (1, 1), (94, 94));
        assert!(legs > 20, "the route winds through every gap: {legs} legs");
    }

    #[test]
    fn a_wall_with_a_gap_at_the_map_edge_is_walked_round() {
        let mut grid = Grid::open(100, 100);
        grid.wall((45, 0), (45, 98));
        let nav = grid.nav();
        grid.walk(&nav, (40, 50), (50, 50));
    }

    #[test]
    fn disjoint_regions_are_not_reachable() {
        let mut grid = Grid::open(100, 100);
        grid.wall((50, 0), (50, 99));
        let nav = grid.nav();
        assert!(!nav.reachable((10, 10), (90, 90)));
        assert!(nav.reachable((10, 10), (40, 90)));
        assert!(!nav.reachable((10, 10), (50, 50)), "a wall cell has no component");
    }

    #[test]
    fn updating_an_area_matches_a_full_rebuild() {
        let mut grid = Grid::open(100, 70);
        grid.wall((30, 0), (30, 60));
        let mut nav = grid.nav();
        grid.wall((0, 40), (29, 40));
        nav.update_area(&grid.cells, (0, 40), (29, 40));
        assert_eq!(nav, grid.nav());
        assert!(!nav.reachable((5, 5), (5, 60)));
        for x in 0..30u16 {
            grid.cells[usize::from(x) + 40 * 100] = 1;
        }
        nav.update_area(&grid.cells, (29, 40), (0, 40));
        assert_eq!(nav, grid.nav());
        assert!(nav.reachable((5, 5), (5, 60)));
    }

    #[test]
    fn crossing_prontera_is_quick() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/maps/pre-re");
        let Ok(map) = map_cache::read_mcache(&dir, "prontera") else { return };
        let grid = Grid { x_size: map.xs, y_size: map.ys, cells: map.cells };
        let started = std::time::Instant::now();
        let nav = grid.nav();
        let built = started.elapsed();
        let spawn = (155, 183);
        let near = |(x, y): Cell| {
            (0..30u16)
                .flat_map(|radius| (x - radius..=x + radius).flat_map(move |cx| (y - radius..=y + radius).map(move |cy| (cx, cy))))
                .find(|cell| nav.reachable(spawn, *cell))
                .unwrap()
        };
        let (from, to) = (near((156, 30)), near((156, 360)));
        let started = std::time::Instant::now();
        let waypoints = nav.route(&grid.cells, from, to).unwrap();
        let routed = started.elapsed();
        grid.walk(&nav, from, to);
        if !cfg!(debug_assertions) {
            assert!(built.as_millis() < 50, "built in {built:?}");
            assert!(routed.as_micros() < 1000, "routed in {routed:?}");
        }
        assert!(waypoints.len() > 320 / usize::from(MAX_LEG));
    }
}
