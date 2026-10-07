use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Client icon the night display uses (`EFST_SKE`).
pub const NIGHT_ICON: u16 = 160;

/// `ZC_MSG_STATE_CHANGE` showing or clearing night on a character of a `nightenabled` map.
pub fn night_packet(char_id: u32, night: bool) -> Vec<u8> {
    let mut packet = 0x0196_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&NIGHT_ICON.to_le_bytes());
    packet.extend_from_slice(&char_id.to_le_bytes());
    packet.push(u8::from(night));
    packet
}

/// The server wide day/night state of rathena: `night_at_start`, `day_duration` and `night_duration` (milliseconds,
/// 0 keeps the current phase forever), moved by `@day`, `@night` and the script commands.
#[derive(Default)]
pub struct DayNight {
    night: AtomicBool,
    /// Tick at which the current cycle started at its day phase.
    origin: AtomicU64,
    started: AtomicBool,
}

impl DayNight {
    pub fn is_night(&self) -> bool {
        self.night.load(Ordering::Relaxed)
    }

    /// Forces a phase and restarts the cycle from it.
    pub fn set(&self, night: bool, tick: u128, day_ms: u64) {
        self.night.store(night, Ordering::Relaxed);
        let origin = if night { tick.saturating_sub(u128::from(day_ms)) } else { tick };
        self.origin.store(origin as u64, Ordering::Relaxed);
    }

    /// Starts the cycle with the phase of `night_at_start`, once.
    pub fn start(&self, night_at_start: bool, tick: u128, day_ms: u64) {
        if !self.started.swap(true, Ordering::Relaxed) {
            self.set(night_at_start, tick, day_ms);
        }
    }

    /// Returns the new phase when the timers switched it.
    pub fn advance(&self, tick: u128, day_ms: u64, night_ms: u64) -> Option<bool> {
        if day_ms == 0 || night_ms == 0 {
            return None;
        }
        let cycle = u128::from(day_ms + night_ms);
        let phase = tick.saturating_sub(u128::from(self.origin.load(Ordering::Relaxed))) % cycle;
        let night = phase >= u128::from(day_ms);
        (self.night.swap(night, Ordering::Relaxed) != night).then_some(night)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cycle_alternates_and_a_forced_phase_restarts_it() {
        let clock = DayNight::default();
        clock.start(false, 1_000, 60_000);
        assert_eq!(clock.advance(30_000, 60_000, 60_000), None);
        assert_eq!(clock.advance(61_000, 60_000, 60_000), Some(true));
        assert_eq!(clock.advance(121_000, 60_000, 60_000), Some(false));
        clock.set(true, 130_000, 60_000);
        assert!(clock.is_night());
        assert_eq!(clock.advance(189_000, 60_000, 60_000), None);
        assert_eq!(clock.advance(191_000, 60_000, 60_000), Some(false));
    }
}
