//! Server rates plugins scale through `SetRate`. Exp is rated once per kill in the game loop, drops in the map instance
//! threads, so the effective percent of each rate is kept in an atomic and read without a lock.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

use script_sdk::Rate;

const HIGHEST_PERCENT: u32 = 100_000;

static SHARES: Mutex<BTreeMap<(String, Rate), u32>> = Mutex::new(BTreeMap::new());
static EFFECTIVE: [AtomicU32; 4] = [AtomicU32::new(100), AtomicU32::new(100), AtomicU32::new(100), AtomicU32::new(100)];

/// What the plugins together apply to `rate`, in percent.
pub fn percent(rate: Rate) -> u32 {
    EFFECTIVE[rate as usize].load(Ordering::Relaxed)
}

pub fn factor(rate: Rate) -> f32 {
    percent(rate) as f32 / 100.0
}

/// Sets `owner`'s share of `rate`. The shares of every owner multiply.
pub fn set(owner: &str, rate: Rate, percent: u32) -> Result<(), String> {
    if percent > HIGHEST_PERCENT {
        return Err(format!("Rate {} cannot exceed {HIGHEST_PERCENT}%", rate.name()));
    }
    let mut shares = SHARES.lock().unwrap();
    shares.insert((owner.to_string(), rate), percent);
    let effective = shares
        .iter()
        .filter(|((_, kind), _)| *kind == rate)
        .fold(1.0_f64, |product, (_, share)| product * f64::from(*share) / 100.0);
    EFFECTIVE[rate as usize].store((effective * 100.0).round().min(f64::from(HIGHEST_PERCENT)) as u32, Ordering::Relaxed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shares_of_several_plugins_multiply() {
        assert_eq!(percent(Rate::JobExp), 100);
        set("first", Rate::JobExp, 200).unwrap();
        set("second", Rate::JobExp, 150).unwrap();
        assert_eq!(percent(Rate::JobExp), 300);
        set("first", Rate::JobExp, 100).unwrap();
        assert_eq!(percent(Rate::JobExp), 150);
        assert!(set("first", Rate::JobExp, HIGHEST_PERCENT + 1).is_err());
        set("second", Rate::JobExp, 100).unwrap();
        assert_eq!(factor(Rate::JobExp), 1.0);
    }
}
