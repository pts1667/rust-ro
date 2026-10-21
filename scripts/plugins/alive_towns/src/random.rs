//! SplitMix64 seeded from everything a result depends on, so any name, look or routine can be recomputed on its own.

pub const PERSONA: u64 = 1;
pub const ROUTINE: u64 = 2;
pub const GATHERING: u64 = 3;
pub const CHAT: u64 = 4;
pub const QUEST: u64 = 5;

pub struct Rng(u64);

impl Rng {
    pub fn new(parts: &[u64]) -> Self {
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        for part in parts {
            state = mix(state ^ part.wrapping_mul(0xD6E8_FEB8_6659_FD93));
        }
        Self(state)
    }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix(self.0)
    }

    pub fn below(&mut self, bound: u64) -> u64 {
        if bound == 0 { 0 } else { self.next() % bound }
    }

    /// Uniform in `low..=high`.
    pub fn range(&mut self, low: u64, high: u64) -> u64 {
        low + self.below(high.saturating_sub(low) + 1)
    }

    pub fn chance(&mut self, percent: u32) -> bool {
        self.below(100) < u64::from(percent)
    }

    pub fn pick<'t, T>(&mut self, items: &'t [T]) -> &'t T {
        &items[self.below(items.len() as u64) as usize]
    }

    /// An index of `weights`, each drawn in proportion to its weight.
    pub fn weighted(&mut self, weights: &[u32]) -> usize {
        let mut roll = self.below(weights.iter().map(|weight| u64::from(*weight)).sum());
        for (index, weight) in weights.iter().enumerate() {
            if roll < u64::from(*weight) {
                return index;
            }
            roll -= u64::from(*weight);
        }
        weights.len() - 1
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for index in (1..items.len()).rev() {
            items.swap(index, self.below(index as u64 + 1) as usize);
        }
    }
}

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
