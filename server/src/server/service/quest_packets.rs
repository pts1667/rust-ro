//! Wire encoders of the quest log packets (PACKETVER 20070622 to 20141021, `clif_quest_*` of rathena).

use crate::server::service::social_packets::name_field;

const ZC_ALL_QUEST_LIST: u16 = 0x02B1;
const ZC_ALL_QUEST_MISSION: u16 = 0x02B2;
const ZC_ADD_QUEST: u16 = 0x02B3;
const ZC_DEL_QUEST: u16 = 0x02B4;
const ZC_UPDATE_MISSION_HUNT: u16 = 0x02B5;
const ZC_ACTIVE_QUEST: u16 = 0x02B7;
const ZC_QUEST_NOTIFY_EFFECT: u16 = 0x0446;

/// Objectives the client shows per quest; the packets always reserve room for them.
const OBJECTIVE_SLOTS: usize = 3;

pub struct ObjectiveView {
    pub mob_id: u32,
    pub name: String,
    pub done: u16,
    pub total: u16,
}

pub struct QuestView {
    pub quest_id: u32,
    pub active: bool,
    pub start_time: u32,
    pub expire_time: u32,
    pub objectives: Vec<ObjectiveView>,
}

fn variable(id: u16, count: u32, body: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(8 + body.len());
    packet.extend_from_slice(&id.to_le_bytes());
    packet.extend_from_slice(&((8 + body.len()) as u16).to_le_bytes());
    packet.extend_from_slice(&count.to_le_bytes());
    packet.extend_from_slice(body);
    packet
}

/// Most quests a variable length packet can carry within its 16 bit length.
fn fit(entry_size: usize, quests: usize) -> usize {
    quests.min((usize::from(u16::MAX) - 8) / entry_size)
}

/// The mission block shared by `ZC_ALL_QUEST_MISSION` and `ZC_ADD_QUEST`: start and expiry times, objective count, three objective slots.
fn mission(quest: &QuestView) -> Vec<u8> {
    let mut body = Vec::with_capacity(10 + OBJECTIVE_SLOTS * 30);
    body.extend_from_slice(&quest.start_time.to_le_bytes());
    body.extend_from_slice(&quest.expire_time.to_le_bytes());
    body.extend_from_slice(&(quest.objectives.len() as u16).to_le_bytes());
    for slot in 0..OBJECTIVE_SLOTS {
        match quest.objectives.get(slot) {
            Some(objective) => {
                body.extend_from_slice(&objective.mob_id.to_le_bytes());
                body.extend_from_slice(&objective.done.to_le_bytes());
                body.extend_from_slice(&name_field(&objective.name));
            }
            None => body.extend_from_slice(&[0; 30]),
        }
    }
    body
}

/// `ZC_ALL_QUEST_LIST`
pub fn quest_list(quests: &[QuestView]) -> Vec<u8> {
    let quests = &quests[..fit(5, quests.len())];
    let mut body = Vec::with_capacity(quests.len() * 5);
    for quest in quests {
        body.extend_from_slice(&quest.quest_id.to_le_bytes());
        body.push(u8::from(quest.active));
    }
    variable(ZC_ALL_QUEST_LIST, quests.len() as u32, &body)
}

/// `ZC_ALL_QUEST_MISSION`
pub fn quest_missions(quests: &[QuestView]) -> Vec<u8> {
    let quests = &quests[..fit(104, quests.len())];
    let mut body = Vec::with_capacity(quests.len() * 104);
    for quest in quests {
        body.extend_from_slice(&quest.quest_id.to_le_bytes());
        body.extend_from_slice(&mission(quest));
    }
    variable(ZC_ALL_QUEST_MISSION, quests.len() as u32, &body)
}

/// `ZC_ADD_QUEST`
pub fn quest_added(quest: &QuestView) -> Vec<u8> {
    let mut packet = ZC_ADD_QUEST.to_le_bytes().to_vec();
    packet.extend_from_slice(&quest.quest_id.to_le_bytes());
    packet.push(u8::from(quest.active));
    packet.extend_from_slice(&mission(quest));
    packet
}

/// `ZC_DEL_QUEST`
pub fn quest_deleted(quest_id: u32) -> Vec<u8> {
    let mut packet = ZC_DEL_QUEST.to_le_bytes().to_vec();
    packet.extend_from_slice(&quest_id.to_le_bytes());
    packet
}

/// `ZC_UPDATE_MISSION_HUNT`
pub fn quest_progress(quest: &QuestView) -> Vec<u8> {
    let mut packet = ZC_UPDATE_MISSION_HUNT.to_le_bytes().to_vec();
    packet.extend_from_slice(&((6 + quest.objectives.len() * 12) as u16).to_le_bytes());
    packet.extend_from_slice(&(quest.objectives.len() as u16).to_le_bytes());
    for objective in &quest.objectives {
        packet.extend_from_slice(&quest.quest_id.to_le_bytes());
        packet.extend_from_slice(&objective.mob_id.to_le_bytes());
        packet.extend_from_slice(&objective.total.to_le_bytes());
        packet.extend_from_slice(&objective.done.to_le_bytes());
    }
    packet
}

/// `ZC_ACTIVE_QUEST`
pub fn quest_activated(quest_id: u32, active: bool) -> Vec<u8> {
    let mut packet = ZC_ACTIVE_QUEST.to_le_bytes().to_vec();
    packet.extend_from_slice(&quest_id.to_le_bytes());
    packet.push(u8::from(active));
    packet
}

/// `ZC_QUEST_NOTIFY_EFFECT`: the icon above an NPC.
pub fn quest_icon(npc_id: u32, x: u16, y: u16, icon: u16, color: u16) -> Vec<u8> {
    let mut packet = ZC_QUEST_NOTIFY_EFFECT.to_le_bytes().to_vec();
    packet.extend_from_slice(&npc_id.to_le_bytes());
    packet.extend_from_slice(&x.to_le_bytes());
    packet.extend_from_slice(&y.to_le_bytes());
    packet.extend_from_slice(&icon.to_le_bytes());
    packet.extend_from_slice(&color.to_le_bytes());
    packet
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> QuestView {
        QuestView {
            quest_id: 7000,
            active: true,
            start_time: 100,
            expire_time: 200,
            objectives: vec![ObjectiveView { mob_id: 1002, name: "Poring".into(), done: 3, total: 10 }],
        }
    }

    #[test]
    fn packets_have_the_lengths_the_client_expects() {
        assert_eq!(quest_added(&sample()).len(), 107);
        assert_eq!(quest_deleted(7000).len(), 6);
        assert_eq!(quest_activated(7000, true).len(), 7);
        assert_eq!(quest_icon(1, 2, 3, 1, 1).len(), 14);
        let list = quest_list(&[sample()]);
        assert_eq!(u16::from_le_bytes([list[2], list[3]]) as usize, list.len());
        assert_eq!(list.len(), 8 + 5);
        let missions = quest_missions(&[sample(), sample()]);
        assert_eq!(missions.len(), 8 + 2 * 104);
        assert_eq!(u16::from_le_bytes([missions[2], missions[3]]) as usize, missions.len());
        let progress = quest_progress(&sample());
        assert_eq!((progress.len(), u16::from_le_bytes([progress[2], progress[3]]) as usize), (18, 18));
    }
}
