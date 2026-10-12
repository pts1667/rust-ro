//! The party window of a bot, rebuilt from the packets the server sends to its client: the members, who leads, who is
//! online and on which map, the sharing options, and for the members on the bot's map their position and, while they
//! are in sight, their HP. A bot knows its party this way only, as a player does.
use serde::Serialize;

const PARTY_ROSTER: u16 = 0x00FB;
const PARTY_MEMBER: u16 = 0x01E9;
const PARTY_WITHDRAWAL: u16 = 0x0105;
const PARTY_POSITION: u16 = 0x0107;
const PARTY_HEALTH: u16 = 0x080E;
const PARTY_OPTIONS: u16 = 0x07D8;
const PARTY_LEADER: u16 = 0x07FC;

/// Bytes of a member in the roster: account id, name, map, leader and offline flags.
const ROSTER_ENTRY: usize = 46;
const ROSTER_START: usize = 28;
/// The position the server sends for a member on another map.
const OFF_MAP: u16 = u16::MAX;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct PartyWindow {
    pub name: String,
    pub exp_share: bool,
    /// The last attempt to turn experience sharing on was refused: the levels are too far apart.
    pub exp_share_denied: bool,
    pub item_pickup: bool,
    pub item_share: bool,
    pub members: Vec<WindowMember>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WindowMember {
    #[serde(skip)]
    pub account_id: u32,
    pub name: String,
    /// The map the member is on, or was on when it left the game.
    pub map: String,
    pub leader: bool,
    pub online: bool,
    /// On the bot's map only, as the minimap shows it.
    pub x: Option<u16>,
    pub y: Option<u16>,
    /// While the member is in sight on the bot's map.
    pub hp: Option<u32>,
    pub max_hp: Option<u32>,
}

impl WindowMember {
    fn new(account_id: u32) -> Self {
        Self { account_id, name: String::new(), map: String::new(), leader: false, online: false, x: None, y: None, hp: None, max_hp: None }
    }

    fn moved_to(&mut self, map: String) {
        if map != self.map {
            self.map = map;
            self.x = None;
            self.y = None;
            self.hp = None;
            self.max_hp = None;
        }
    }
}

impl PartyWindow {
    fn member(&mut self, account_id: u32) -> &mut WindowMember {
        if let Some(index) = self.members.iter().position(|member| member.account_id == account_id) {
            return &mut self.members[index];
        }
        self.members.push(WindowMember::new(account_id));
        self.members.last_mut().expect("member was just added")
    }
}

fn u16_at(packet: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(packet.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(packet: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(packet.get(at..at + 4)?.try_into().ok()?))
}

fn text_at(packet: &[u8], at: usize, length: usize) -> String {
    let field = packet.get(at..at + length).unwrap_or_default();
    String::from_utf8_lossy(field.split(|byte| *byte == 0).next().unwrap_or_default()).into_owned()
}

fn map_at(packet: &[u8], at: usize) -> String {
    let map = text_at(packet, at, 16);
    map.strip_suffix(".gat").map(str::to_string).unwrap_or(map)
}

/// Applies a packet to the window of the bot of `own_account`. Returns whether it was a party packet.
pub fn decode(window: &mut Option<PartyWindow>, own_account: u32, packet: &[u8]) -> bool {
    let Some(id) = u16_at(packet, 0) else { return false };
    match id {
        PARTY_ROSTER => {
            let current = window.get_or_insert_with(PartyWindow::default);
            current.name = text_at(packet, 4, 24);
            let previous = std::mem::take(&mut current.members);
            let mut at = ROSTER_START;
            while let Some(account_id) = u32_at(packet, at).filter(|_| packet.len() >= at + ROSTER_ENTRY) {
                let mut member = previous.iter().find(|known| known.account_id == account_id).cloned().unwrap_or_else(|| WindowMember::new(account_id));
                member.name = text_at(packet, at + 4, 24);
                member.moved_to(map_at(packet, at + 28));
                member.leader = packet[at + 44] == 0;
                member.online = packet[at + 45] == 0;
                if !member.online {
                    member.x = None;
                    member.y = None;
                    member.hp = None;
                    member.max_hp = None;
                }
                current.members.push(member);
                at += ROSTER_ENTRY;
            }
            true
        }
        PARTY_MEMBER => {
            let Some(account_id) = u32_at(packet, 2) else { return false };
            let current = window.get_or_insert_with(PartyWindow::default);
            current.name = text_at(packet, 15, 24);
            if let (Some(pickup), Some(share)) = (packet.get(79), packet.get(80)) {
                current.item_pickup = *pickup != 0;
                current.item_share = *share != 0;
            }
            let leader = u32_at(packet, 6) == Some(0);
            let member = current.member(account_id);
            member.name = text_at(packet, 39, 24);
            member.moved_to(map_at(packet, 63));
            member.leader = leader;
            member.online = true;
            true
        }
        PARTY_WITHDRAWAL => {
            let Some(account_id) = u32_at(packet, 2) else { return false };
            if account_id == own_account {
                *window = None;
            } else if let Some(current) = window.as_mut() {
                current.members.retain(|member| member.account_id != account_id);
            }
            true
        }
        PARTY_POSITION => {
            let (Some(account_id), Some(x), Some(y)) = (u32_at(packet, 2), u16_at(packet, 6), u16_at(packet, 8)) else { return false };
            if let Some(current) = window.as_mut() {
                let member = current.member(account_id);
                if x == OFF_MAP || y == OFF_MAP {
                    member.x = None;
                    member.y = None;
                    member.hp = None;
                    member.max_hp = None;
                } else {
                    member.x = Some(x);
                    member.y = Some(y);
                }
            }
            true
        }
        PARTY_HEALTH => {
            let (Some(account_id), Some(hp), Some(max_hp)) = (u32_at(packet, 2), u32_at(packet, 6), u32_at(packet, 10)) else { return false };
            if let Some(current) = window.as_mut() {
                let member = current.member(account_id);
                member.hp = Some(hp);
                member.max_hp = Some(max_hp);
            }
            true
        }
        PARTY_OPTIONS => {
            let Some(exp) = u32_at(packet, 2) else { return false };
            let current = window.get_or_insert_with(PartyWindow::default);
            current.exp_share_denied = exp == 2;
            if exp != 2 {
                current.exp_share = exp == 1;
            }
            if let (Some(pickup), Some(share)) = (packet.get(6), packet.get(7)) {
                current.item_pickup = *pickup != 0;
                current.item_share = *share != 0;
            }
            true
        }
        PARTY_LEADER => {
            let (Some(old), Some(new)) = (u32_at(packet, 2), u32_at(packet, 6)) else { return false };
            if let Some(current) = window.as_mut() {
                for member in &mut current.members {
                    if member.account_id == old {
                        member.leader = false;
                    }
                    if member.account_id == new {
                        member.leader = true;
                    }
                }
            }
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(packet: &mut Vec<u8>, value: &str, length: usize) {
        let start = packet.len();
        packet.resize(start + length, 0);
        packet[start..start + value.len()].copy_from_slice(value.as_bytes());
    }

    fn roster(members: &[(u32, &str, &str, bool, bool)]) -> Vec<u8> {
        let mut packet = PARTY_ROSTER.to_le_bytes().to_vec();
        packet.extend_from_slice(&(28u16 + members.len() as u16 * 46).to_le_bytes());
        name(&mut packet, "Wanderers", 24);
        for (account_id, member, map, leader, online) in members {
            packet.extend_from_slice(&account_id.to_le_bytes());
            name(&mut packet, member, 24);
            name(&mut packet, &format!("{map}.gat"), 16);
            packet.push(u8::from(!leader));
            packet.push(u8::from(!online));
        }
        packet
    }

    fn position(account_id: u32, x: u16, y: u16) -> Vec<u8> {
        let mut packet = PARTY_POSITION.to_le_bytes().to_vec();
        packet.extend_from_slice(&account_id.to_le_bytes());
        packet.extend_from_slice(&x.to_le_bytes());
        packet.extend_from_slice(&y.to_le_bytes());
        packet
    }

    fn health(account_id: u32, hp: u32, max_hp: u32) -> Vec<u8> {
        let mut packet = PARTY_HEALTH.to_le_bytes().to_vec();
        packet.extend_from_slice(&account_id.to_le_bytes());
        packet.extend_from_slice(&hp.to_le_bytes());
        packet.extend_from_slice(&max_hp.to_le_bytes());
        packet
    }

    #[test]
    fn the_roster_positions_and_hp_bars_make_the_window() {
        let mut window = None;
        assert!(decode(&mut window, 1, &roster(&[(1, "Alpha", "prontera", true, true), (2, "Beta", "prt_fild08", false, true), (3, "Gamma", "geffen", false, false)])));
        assert!(decode(&mut window, 1, &position(2, 150, 160)));
        assert!(decode(&mut window, 1, &health(2, 30, 120)));
        let current = window.clone().unwrap();
        assert_eq!(current.name, "Wanderers");
        let beta = &current.members[1];
        assert_eq!((beta.name.as_str(), beta.map.as_str(), beta.leader, beta.online), ("Beta", "prt_fild08", false, true));
        assert_eq!((beta.x, beta.y, beta.hp, beta.max_hp), (Some(150), Some(160), Some(30), Some(120)));
        assert!(current.members[0].leader);
        assert!(!current.members[2].online);

        // Gone to another map: no position nor HP bar any more
        assert!(decode(&mut window, 1, &position(2, OFF_MAP, OFF_MAP)));
        assert_eq!(window.as_ref().unwrap().members[1].hp, None);
        // A new roster keeps what it does not say, on the same map
        decode(&mut window, 1, &position(2, 151, 161));
        decode(&mut window, 1, &roster(&[(1, "Alpha", "prontera", true, true), (2, "Beta", "prt_fild08", false, true)]));
        assert_eq!(window.as_ref().unwrap().members[1].x, Some(151));
        assert_eq!(window.as_ref().unwrap().members.len(), 2);
    }

    #[test]
    fn options_leader_changes_and_leaving_update_the_window() {
        let mut window = None;
        decode(&mut window, 1, &roster(&[(1, "Alpha", "prontera", true, true), (2, "Beta", "prontera", false, true)]));
        let mut options = PARTY_OPTIONS.to_le_bytes().to_vec();
        options.extend_from_slice(&1u32.to_le_bytes());
        options.extend_from_slice(&[1, 0]);
        decode(&mut window, 1, &options);
        assert!(window.as_ref().unwrap().exp_share && window.as_ref().unwrap().item_pickup);
        options[2] = 2;
        decode(&mut window, 1, &options);
        assert!(window.as_ref().unwrap().exp_share && window.as_ref().unwrap().exp_share_denied);

        let mut leader = PARTY_LEADER.to_le_bytes().to_vec();
        leader.extend_from_slice(&1u32.to_le_bytes());
        leader.extend_from_slice(&2u32.to_le_bytes());
        decode(&mut window, 1, &leader);
        assert!(!window.as_ref().unwrap().members[0].leader && window.as_ref().unwrap().members[1].leader);

        let mut left = PARTY_WITHDRAWAL.to_le_bytes().to_vec();
        left.extend_from_slice(&2u32.to_le_bytes());
        name(&mut left, "Beta", 24);
        left.push(1);
        decode(&mut window, 1, &left);
        assert_eq!(window.as_ref().unwrap().members.len(), 1);
        left[2] = 1;
        decode(&mut window, 1, &left);
        assert!(window.is_none());
    }
}
