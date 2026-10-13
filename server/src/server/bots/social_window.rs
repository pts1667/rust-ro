//! The friend request and party booking windows of a bot, rebuilt from the packets its client gets.
use serde::Serialize;

use super::social::{BookingAdView, booking_job_name};

const FRIEND_REQUEST: u16 = 0x0207;
const BOOKING_SEARCH_RESULT: u16 = 0x0805;
/// Header, length and the "more results" flag, then one 48 byte entry per ad.
const BOOKING_LIST_START: usize = 5;
const BOOKING_ENTRY_LEN: usize = 48;
const NAME_LEN: usize = 24;
const BOOKING_JOBS: usize = 6;

/// A player asks to be the bot's friend: answer with `friend_accept` or `friend_decline`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FriendRequestView {
    #[serde(skip)]
    pub account_id: u32,
    pub char_id: u32,
    pub name: String,
}

/// The ads the last `booking_search` found.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BookingResults {
    pub ads: Vec<BookingAdView>,
    /// More ads matched than the window shows.
    pub more: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SocialWindow {
    pub friend_request: Option<FriendRequestView>,
    pub booking_results: Option<BookingResults>,
}

fn le_u32(packet: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(packet.get(at..at + 4)?.try_into().ok()?))
}

fn le_i16(packet: &[u8], at: usize) -> Option<i16> {
    Some(i16::from_le_bytes(packet.get(at..at + 2)?.try_into().ok()?))
}

fn name_at(packet: &[u8], at: usize) -> String {
    let field = packet.get(at..at + NAME_LEN).unwrap_or_default();
    String::from_utf8_lossy(field.split(|byte| *byte == 0).next().unwrap_or_default()).into_owned()
}

fn booking_ad(entry: &[u8]) -> Option<BookingAdView> {
    let jobs = (0..BOOKING_JOBS).filter_map(|slot| le_i16(entry, 36 + slot * 2)).filter_map(booking_job_name).collect();
    Some(BookingAdView {
        index: le_u32(entry, 0)?,
        name: name_at(entry, 4),
        started: le_u32(entry, 28)?,
        level: le_i16(entry, 32)?,
        map_id: le_i16(entry, 34)?,
        jobs,
    })
}

/// Applies one server packet to the window; returns whether it was one of its packets.
pub fn decode(window: &mut SocialWindow, packet: &[u8]) -> bool {
    let Some(id) = packet.get(..2).map(|id| u16::from_le_bytes([id[0], id[1]])) else { return false };
    match id {
        FRIEND_REQUEST => {
            let (Some(account_id), Some(char_id)) = (le_u32(packet, 2), le_u32(packet, 6)) else { return false };
            window.friend_request = Some(FriendRequestView { account_id, char_id, name: name_at(packet, 10) });
            true
        }
        BOOKING_SEARCH_RESULT => {
            let more = packet.get(4).is_some_and(|flag| *flag != 0);
            let ads = packet.get(BOOKING_LIST_START..).unwrap_or_default().chunks_exact(BOOKING_ENTRY_LEN).filter_map(booking_ad).collect();
            window.booking_results = Some(BookingResults { ads, more });
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(packet: &mut Vec<u8>, name: &str) {
        let mut field = [0u8; NAME_LEN];
        field[..name.len()].copy_from_slice(name.as_bytes());
        packet.extend_from_slice(&field);
    }

    #[test]
    fn friend_requests_and_booking_results_fill_the_window() {
        let mut window = SocialWindow::default();
        let mut request = vec![0x07, 0x02];
        request.extend_from_slice(&2000001u32.to_le_bytes());
        request.extend_from_slice(&150002u32.to_le_bytes());
        name(&mut request, "Goapy");
        assert!(decode(&mut window, &request));
        assert_eq!(window.friend_request, Some(FriendRequestView { account_id: 2000001, char_id: 150002, name: "Goapy".into() }));

        let mut results = vec![0x05, 0x08, 0, 0, 1];
        results.extend_from_slice(&7u32.to_le_bytes());
        name(&mut results, "Alfa");
        results.extend_from_slice(&1_700_000_000u32.to_le_bytes());
        results.extend_from_slice(&12i16.to_le_bytes());
        results.extend_from_slice(&0i16.to_le_bytes());
        for job in [4i16, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] {
            results.extend_from_slice(&job.to_le_bytes());
        }
        let length = results.len() as u16;
        results[2..4].copy_from_slice(&length.to_le_bytes());
        assert!(decode(&mut window, &results));
        let found = window.booking_results.clone().unwrap();
        assert!(found.more);
        assert_eq!(found.ads.len(), 1);
        assert_eq!((found.ads[0].index, found.ads[0].name.as_str(), found.ads[0].level), (7, "Alfa", 12));
        assert_eq!(found.ads[0].jobs, vec!["Acolyte".to_string()]);
        assert!(!decode(&mut window, &[0x0D, 0x01]));
    }
}
