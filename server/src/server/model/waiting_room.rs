use std::collections::HashMap;

/// Chat rooms use `0x7000_0000 + n`, NPC waiting rooms the range above.
const FIRST_ROOM_ID: u32 = 0x7800_0000;
pub const TITLE_SIZE: usize = 60;
pub const MAX_LEVEL: u32 = 175;

/// A chat room owned by an NPC (`waitingroom`): players wait in it until enough of them have gathered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitingRoom {
    pub id: u32,
    pub npc_id: u32,
    pub map: String,
    pub instance: u8,
    pub x: u16,
    pub y: u16,
    pub title: String,
    /// The NPC counts as one of the users, as in rathena.
    pub limit: u16,
    pub trigger: u16,
    pub event: String,
    pub event_disabled: bool,
    pub zeny: u32,
    pub min_level: u32,
    pub max_level: u32,
    /// In the order they joined: the longest waiting first.
    pub members: Vec<u32>,
}

impl WaitingRoom {
    pub fn is_full(&self) -> bool {
        self.members.len() + 1 >= usize::from(self.limit)
    }

    pub fn trigger_reached(&self) -> bool {
        !self.event_disabled && !self.event.is_empty() && self.members.len() >= usize::from(self.trigger)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    Full,
    NoZeny,
    TooLowLevel,
    TooHighLevel,
}

impl Refusal {
    /// `ZC_REFUSE_ENTER_ROOM` result.
    pub fn code(self) -> u8 {
        match self {
            Refusal::Full => 0,
            Refusal::NoZeny => 4,
            Refusal::TooLowLevel => 5,
            Refusal::TooHighLevel => 6,
        }
    }
}

#[derive(Default)]
pub struct WaitingRooms {
    by_npc: HashMap<u32, WaitingRoom>,
    room_of_member: HashMap<u32, u32>,
    counter: u32,
}

impl WaitingRooms {
    pub fn of_npc(&self, npc_id: u32) -> Option<&WaitingRoom> {
        self.by_npc.get(&npc_id)
    }

    pub fn of_npc_mut(&mut self, npc_id: u32) -> Option<&mut WaitingRoom> {
        self.by_npc.get_mut(&npc_id)
    }

    pub fn of_chat(&self, chat_id: u32) -> Option<&WaitingRoom> {
        self.by_npc.values().find(|room| room.id == chat_id)
    }

    pub fn room_of(&self, char_id: u32) -> Option<&WaitingRoom> {
        self.room_of_member.get(&char_id).and_then(|npc_id| self.by_npc.get(npc_id))
    }

    /// One room per NPC; `None` when the NPC already has one.
    #[allow(clippy::too_many_arguments)]
    pub fn create(&mut self, mut room: WaitingRoom) -> Option<&WaitingRoom> {
        if self.by_npc.contains_key(&room.npc_id) {
            return None;
        }
        self.counter += 1;
        room.id = FIRST_ROOM_ID + self.counter;
        room.title = room.title.chars().take(TITLE_SIZE).collect();
        let npc_id = room.npc_id;
        self.by_npc.insert(npc_id, room);
        self.by_npc.get(&npc_id)
    }

    pub fn delete(&mut self, npc_id: u32) -> Option<WaitingRoom> {
        let room = self.by_npc.remove(&npc_id)?;
        for member in &room.members {
            self.room_of_member.remove(member);
        }
        Some(room)
    }

    pub fn check_join(&self, npc_id: u32, level: u32, zeny: u32) -> Result<(), Refusal> {
        let room = self.by_npc.get(&npc_id).ok_or(Refusal::Full)?;
        if room.is_full() {
            return Err(Refusal::Full);
        }
        if level < room.min_level {
            return Err(Refusal::TooLowLevel);
        }
        if level > room.max_level {
            return Err(Refusal::TooHighLevel);
        }
        if zeny < room.zeny {
            return Err(Refusal::NoZeny);
        }
        Ok(())
    }

    pub fn join(&mut self, npc_id: u32, char_id: u32) -> Option<&WaitingRoom> {
        if self.room_of_member.contains_key(&char_id) {
            return None;
        }
        let room = self.by_npc.get_mut(&npc_id)?;
        room.members.push(char_id);
        self.room_of_member.insert(char_id, npc_id);
        Some(&*room)
    }

    /// The room as it is after the member left.
    pub fn leave(&mut self, char_id: u32) -> Option<&WaitingRoom> {
        let npc_id = self.room_of_member.remove(&char_id)?;
        let room = self.by_npc.get_mut(&npc_id)?;
        room.members.retain(|member| *member != char_id);
        Some(&*room)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room() -> WaitingRoom {
        WaitingRoom {
            id: 0, npc_id: 7, map: "prontera".into(), instance: 0, x: 1, y: 1, title: "Arena".into(), limit: 3, trigger: 2, event: "Gate::OnStart".into(),
            event_disabled: false, zeny: 100, min_level: 10, max_level: 50, members: vec![],
        }
    }

    #[test]
    fn the_npc_counts_toward_the_limit_and_entry_requires_level_and_zeny() {
        let mut rooms = WaitingRooms::default();
        rooms.create(room()).unwrap();
        assert!(rooms.create(room()).is_none());
        assert_eq!(rooms.check_join(7, 9, 500), Err(Refusal::TooLowLevel));
        assert_eq!(rooms.check_join(7, 51, 500), Err(Refusal::TooHighLevel));
        assert_eq!(rooms.check_join(7, 20, 99), Err(Refusal::NoZeny));
        rooms.check_join(7, 20, 100).unwrap();
        assert!(!rooms.join(7, 1).unwrap().trigger_reached());
        assert!(rooms.join(7, 2).unwrap().trigger_reached());
        assert_eq!(rooms.check_join(7, 20, 100), Err(Refusal::Full));
        assert_eq!(rooms.leave(1).unwrap().members, vec![2]);
        assert!(rooms.delete(7).is_some());
        assert!(rooms.room_of(2).is_none());
    }
}
