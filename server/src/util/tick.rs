#![allow(dead_code)]

use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct SimClock {
    wall_start_ms: u128,
    started: Instant,
    speed: f64,
}

static SIM_CLOCK: OnceLock<SimClock> = OnceLock::new();

fn sim_clock() -> &'static SimClock {
    SIM_CLOCK.get_or_init(|| SimClock {
        wall_start_ms: wall_clock_ms(),
        started: Instant::now(),
        speed: 1.0,
    })
}

fn wall_clock_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

/// Sets how many game milliseconds pass per real millisecond. Only the first call is kept, so it must run at boot.
pub fn set_sim_speed(speed: f64) -> bool {
    SIM_CLOCK
        .set(SimClock {
            wall_start_ms: wall_clock_ms(),
            started: Instant::now(),
            speed,
        })
        .is_ok()
}

pub fn sim_speed() -> f64 {
    sim_clock().speed
}

pub fn get_tick_client() -> u32 {
    get_tick() as u32
}

/// Game time in milliseconds: the wall clock, multiplied by the sim speed.
pub fn get_tick() -> u128 {
    let clock = sim_clock();
    clock.wall_start_ms + game_time_ms(clock.started.elapsed(), clock.speed)
}

pub fn get_current_time() -> u128 {
    wall_clock_ms()
}

/// Real time that `game_ms` of game time takes at the current sim speed.
pub fn real_duration(game_ms: u128) -> Duration {
    real_time(game_ms, sim_clock().speed)
}

/// Sleep left in a loop of a `real_tick` period below which the loop is too slow: 5 ms, or a quarter of the period when
/// the sim speed makes it shorter than 20 ms.
pub fn too_little_sleep(sleep: Duration, real_tick: Duration) -> bool {
    sleep < Duration::from_millis(5).min(real_tick / 4)
}

fn game_time_ms(elapsed: Duration, speed: f64) -> u128 {
    (elapsed.as_secs_f64() * 1000.0 * speed) as u128
}

fn real_time(game_ms: u128, speed: f64) -> Duration {
    Duration::from_secs_f64(game_ms as f64 / 1000.0 / speed)
}

pub fn delayed_tick(delay: u128, tick_rate: u128) -> usize {
    (delay as f64 / tick_rate as f64).round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_time_runs_at_the_sim_speed() {
        assert_eq!(game_time_ms(Duration::from_millis(250), 1.0), 250);
        assert_eq!(game_time_ms(Duration::from_millis(250), 10.0), 2500);
    }

    #[test]
    fn a_fast_sim_is_not_too_slow_for_its_short_ticks() {
        assert!(too_little_sleep(Duration::from_millis(4), Duration::from_millis(20)));
        assert!(!too_little_sleep(Duration::from_millis(3), real_time(20, 5.0)));
        assert!(too_little_sleep(Duration::ZERO, real_time(20, 5.0)));
    }

    #[test]
    fn a_game_tick_takes_its_share_of_real_time() {
        assert!((real_time(40, 4.0).as_secs_f64() - 0.01).abs() < 1e-9);
        assert!((real_time(40, 1.0).as_secs_f64() - 0.04).abs() < 1e-9);
    }
}
