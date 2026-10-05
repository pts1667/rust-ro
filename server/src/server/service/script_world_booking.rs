use super::ScriptWorldService;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::party_booking::{BookingAd, BOOKING_JOBS};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const REGISTER_ACK: u16 = 0x0803;
const SEARCH_ACK: u16 = 0x0805;
const DELETE_ACK: u16 = 0x0807;
const NOTIFY_INSERT: u16 = 0x0809;
const NOTIFY_UPDATE: u16 = 0x080A;
const NOTIFY_DELETE: u16 = 0x080B;
const REGISTER_SUCCESS: u16 = 0;
const REGISTER_DUPLICATE: u16 = 2;
const DELETE_SUCCESS: u16 = 0;
const DELETE_NOTHING_REGISTERED: u16 = 3;
const NAME_LENGTH: usize = 24;

fn header(id: u16) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}

fn push_jobs(packet: &mut Vec<u8>, jobs: &[i16; BOOKING_JOBS]) {
    for job in jobs {
        packet.extend_from_slice(&job.to_le_bytes());
    }
}

fn push_ad(packet: &mut Vec<u8>, ad: &BookingAd) {
    packet.extend_from_slice(&ad.index.to_le_bytes());
    let mut name = [0_u8; NAME_LENGTH];
    let length = ad.name.len().min(NAME_LENGTH - 1);
    name[..length].copy_from_slice(&ad.name.as_bytes()[..length]);
    packet.extend_from_slice(&name);
    packet.extend_from_slice(&ad.started.to_le_bytes());
    packet.extend_from_slice(&ad.level.to_le_bytes());
    packet.extend_from_slice(&ad.map_id.to_le_bytes());
    push_jobs(packet, &ad.jobs);
}

fn now_seconds() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as u32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BookingRequest {
    BookingRegister {
            level: i16,
            map_id: i16,
            jobs: [i16; 6],
        },
    BookingSearch {
            level: i16,
            map_id: i16,
            job: i16,
            last_index: u32,
        },
    BookingDelete,
    BookingUpdate([i16; 6]),
}


impl ScriptWorldService {
    fn send_to(&self, char_id: u32, packet: Vec<u8>) {
        let _ = self.notifications.try_send(Notification::Char(CharNotification::new(char_id, packet)));
    }

    fn broadcast_booking(&self, state: &ServerState, owner: &Character, packet: Vec<u8>) {
        self.send_to(owner.char_id, packet.clone());
        for char_id in state.characters().keys() {
            self.send_to(*char_id, packet.clone());
        }
    }

    pub(crate) fn booking_request(&self, state: &mut ServerState, character: &Character, request: BookingRequest) -> Result<(), String> {
        let online: Vec<u32> = state.characters().keys().copied().collect();
        self.party_bookings.prune(|id| online.contains(&id) || id == character.char_id);
        match request {
            BookingRequest::BookingRegister { level, map_id, jobs } => {
                let ad = self.party_bookings.register(character.char_id, &character.name, now_seconds(), level, map_id, jobs);
                let mut ack = header(REGISTER_ACK);
                ack.extend_from_slice(&(if ad.is_some() { REGISTER_SUCCESS } else { REGISTER_DUPLICATE }).to_le_bytes());
                self.send_to(character.char_id, ack);
                if let Some(ad) = ad {
                    let mut notice = header(NOTIFY_INSERT);
                    push_ad(&mut notice, &ad);
                    self.broadcast_booking(state, character, notice);
                }
            }
            BookingRequest::BookingSearch { level, map_id, job, last_index } => {
                let (results, more) = self.party_bookings.search(level, map_id, job, last_index);
                let mut packet = header(SEARCH_ACK);
                packet.extend_from_slice(&((5 + results.len() * 48) as u16).to_le_bytes());
                packet.push(u8::from(more));
                for ad in &results {
                    push_ad(&mut packet, ad);
                }
                self.send_to(character.char_id, packet);
            }
            BookingRequest::BookingDelete => {
                let mut ack = header(DELETE_ACK);
                match self.party_bookings.delete(character.char_id) {
                    Some(index) => {
                        ack.extend_from_slice(&DELETE_SUCCESS.to_le_bytes());
                        self.send_to(character.char_id, ack);
                        let mut notice = header(NOTIFY_DELETE);
                        notice.extend_from_slice(&index.to_le_bytes());
                        self.broadcast_booking(state, character, notice);
                    }
                    None => {
                        ack.extend_from_slice(&DELETE_NOTHING_REGISTERED.to_le_bytes());
                        self.send_to(character.char_id, ack);
                    }
                }
            }
            BookingRequest::BookingUpdate(jobs) => {
                if let Some(ad) = self.party_bookings.update(character.char_id, now_seconds(), jobs) {
                    let mut notice = header(NOTIFY_UPDATE);
                    notice.extend_from_slice(&ad.index.to_le_bytes());
                    push_jobs(&mut notice, &ad.jobs);
                    self.broadcast_booking(state, character, notice);
                }
            }
        }
        Ok(())
    }
}

impl ScriptWorldService {
}
