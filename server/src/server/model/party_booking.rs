use std::collections::BTreeMap;

pub const BOOKING_JOBS: usize = 6;
pub const BOOKING_RESULTS: usize = 10;
const NO_JOB: i16 = -1;

#[derive(Debug, Clone, PartialEq)]
pub struct BookingAd {
    pub index: u32,
    pub name: String,
    pub started: u32,
    pub level: i16,
    pub map_id: i16,
    pub jobs: [i16; BOOKING_JOBS],
}

#[derive(Debug, Default)]
pub struct PartyBookings {
    next_index: u32,
    ads: BTreeMap<u32, BookingAd>,
}

fn normalized(jobs: [i16; BOOKING_JOBS]) -> [i16; BOOKING_JOBS] {
    jobs.map(|job| if job == 0xFF { NO_JOB } else { job })
}

impl PartyBookings {
    pub fn register(&mut self, char_id: u32, name: &str, now: u32, level: i16, map_id: i16, jobs: [i16; BOOKING_JOBS]) -> Option<BookingAd> {
        if self.ads.contains_key(&char_id) {
            return None;
        }
        self.next_index += 1;
        let ad = BookingAd {
            index: self.next_index,
            name: name.to_string(),
            started: now,
            level,
            map_id,
            jobs: normalized(jobs),
        };
        self.ads.insert(char_id, ad.clone());
        Some(ad)
    }

    pub fn update(&mut self, char_id: u32, now: u32, jobs: [i16; BOOKING_JOBS]) -> Option<BookingAd> {
        let ad = self.ads.get_mut(&char_id)?;
        ad.started = now;
        ad.jobs = normalized(jobs);
        Some(ad.clone())
    }

    pub fn delete(&mut self, char_id: u32) -> Option<u32> {
        self.ads.remove(&char_id).map(|ad| ad.index)
    }

    pub fn prune(&mut self, is_online: impl Fn(u32) -> bool) {
        self.ads.retain(|char_id, _| is_online(*char_id));
    }

    /// Mirrors rathena: a map or a job filter, never both; results are capped and flagged when more remain.
    pub fn search(&self, level: i16, map_id: i16, job: i16, last_index: u32) -> (Vec<BookingAd>, bool) {
        let mut results = Vec::new();
        for ad in self.ads.values() {
            if ad.index < last_index || (level != 0 && (ad.level < level - 15 || ad.level > level)) {
                continue;
            }
            if results.len() >= BOOKING_RESULTS {
                return (results, true);
            }
            let matches = match (map_id, job) {
                (0, NO_JOB) => true,
                (0, job) => ad.jobs.contains(&job),
                (map_id, NO_JOB) => ad.map_id == map_id,
                _ => false,
            };
            if matches {
                results.push(ad.clone());
            }
        }
        (results, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booking_lifecycle_and_search_filters() {
        let mut bookings = PartyBookings::default();
        let jobs = [7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let ad = bookings.register(1, "Knight", 100, 50, 3, jobs).unwrap();
        assert!(bookings.register(1, "Knight", 100, 50, 3, jobs).is_none());
        assert_eq!(ad.jobs[1], -1);
        let (found, more) = bookings.search(55, 0, -1, 0);
        assert_eq!((found.len(), more), (1, false));
        assert!(bookings.search(80, 0, -1, 0).0.is_empty());
        assert_eq!(bookings.search(0, 0, 7, 0).0.len(), 1);
        assert_eq!(bookings.search(0, 3, -1, 0).0.len(), 1);
        assert!(bookings.search(0, 3, 7, 0).0.is_empty());
        assert_eq!(bookings.delete(1), Some(ad.index));
        assert!(bookings.update(1, 200, jobs).is_none());
    }
}
