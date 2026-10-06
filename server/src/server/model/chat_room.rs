use std::collections::{BTreeSet, HashMap};

/// `CHATROOM_TITLE_SIZE` and `CHATROOM_PASS_SIZE` of rathena.
pub const TITLE_SIZE: usize = 60;
pub const PASSWORD_SIZE: usize = 8;
/// `usersd` of rathena's `chat_data`.
pub const MAX_MEMBERS: usize = 20;
/// Chat rooms and NPC ids share the client's object id space, keep clear of both.
const FIRST_ROOM_ID: u32 = 0x7000_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRoom {
    pub id: u32,
    pub title: String,
    pub password: String,
    pub limit: u16,
    pub public: bool,
    /// Owner first.
    pub members: Vec<u32>,
    kicked: BTreeSet<u32>,
}

impl ChatRoom {
    pub fn owner(&self) -> u32 {
        self.members[0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinRefusal {
    Full,
    WrongPassword,
    Kicked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Departure {
    /// The room as it is after the member left; `None` once the room was destroyed.
    pub room: Option<ChatRoom>,
    pub room_id: u32,
    pub new_owner: Option<u32>,
}

#[derive(Default)]
pub struct ChatRooms {
    rooms: HashMap<u32, ChatRoom>,
    room_of_member: HashMap<u32, u32>,
    counter: u32,
}

impl ChatRooms {
    pub fn get(&self, room_id: u32) -> Option<&ChatRoom> {
        self.rooms.get(&room_id)
    }

    pub fn room_of(&self, char_id: u32) -> Option<&ChatRoom> {
        self.room_of_member.get(&char_id).and_then(|id| self.rooms.get(id))
    }

    pub fn owned_by(&self, char_id: u32) -> Option<&ChatRoom> {
        self.room_of(char_id).filter(|room| room.owner() == char_id)
    }

    pub fn create(&mut self, owner: u32, title: &str, password: &str, limit: u16, public: bool) -> Option<&ChatRoom> {
        if self.room_of_member.contains_key(&owner) {
            return None;
        }
        self.counter += 1;
        let id = FIRST_ROOM_ID + self.counter;
        let room = ChatRoom {
            id,
            title: title.chars().take(TITLE_SIZE).collect(),
            password: password.chars().take(PASSWORD_SIZE).collect(),
            limit: limit.min(MAX_MEMBERS as u16),
            public,
            members: vec![owner],
            kicked: BTreeSet::new(),
        };
        self.room_of_member.insert(owner, id);
        self.rooms.insert(id, room);
        self.rooms.get(&id)
    }

    pub fn join(&mut self, char_id: u32, room_id: u32, password: &str, bypass_password: bool) -> Result<&ChatRoom, JoinRefusal> {
        let room = self.rooms.get_mut(&room_id).ok_or(JoinRefusal::Full)?;
        if self.room_of_member.contains_key(&char_id) || room.members.len() >= usize::from(room.limit) {
            return Err(JoinRefusal::Full);
        }
        if !room.public && room.password != password && !bypass_password {
            return Err(JoinRefusal::WrongPassword);
        }
        if room.kicked.contains(&char_id) {
            return Err(JoinRefusal::Kicked);
        }
        room.members.push(char_id);
        self.room_of_member.insert(char_id, room_id);
        Ok(&*room)
    }

    pub fn leave(&mut self, char_id: u32) -> Option<Departure> {
        let room_id = self.room_of_member.remove(&char_id)?;
        let room = self.rooms.get_mut(&room_id)?;
        let was_owner = room.owner() == char_id;
        room.members.retain(|member| *member != char_id);
        if room.members.is_empty() {
            self.rooms.remove(&room_id);
            return Some(Departure { room: None, room_id, new_owner: None });
        }
        Some(Departure {
            room: Some(room.clone()),
            room_id,
            new_owner: was_owner.then(|| room.owner()),
        })
    }

    pub fn kick(&mut self, owner: u32, target: u32) -> Option<Departure> {
        if self.owned_by(owner)?.members.contains(&target) && target != owner {
            let room_id = self.room_of_member[&owner];
            self.rooms.get_mut(&room_id)?.kicked.insert(target);
            return self.leave(target);
        }
        None
    }

    pub fn transfer_ownership(&mut self, owner: u32, target: u32) -> Option<&ChatRoom> {
        let room_id = self.owned_by(owner)?.id;
        let room = self.rooms.get_mut(&room_id)?;
        let index = room.members.iter().position(|member| *member == target).filter(|index| *index > 0)?;
        room.members.swap(0, index);
        Some(&*room)
    }

    pub fn change(&mut self, owner: u32, title: &str, password: &str, limit: u16, public: bool) -> Option<&ChatRoom> {
        let room_id = self.owned_by(owner)?.id;
        let room = self.rooms.get_mut(&room_id)?;
        room.title = title.chars().take(TITLE_SIZE).collect();
        room.password = password.chars().take(PASSWORD_SIZE).collect();
        room.limit = limit.min(MAX_MEMBERS as u16);
        room.public = public;
        Some(&*room)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_follows_the_first_member_and_kicked_characters_cannot_return() {
        let mut rooms = ChatRooms::default();
        let id = rooms.create(1, "title", "secret", 3, false).unwrap().id;
        assert_eq!(rooms.join(2, id, "wrong", false).unwrap_err(), JoinRefusal::WrongPassword);
        rooms.join(2, id, "secret", false).unwrap();
        rooms.join(3, id, "", true).unwrap();
        assert_eq!(rooms.join(4, id, "secret", false).unwrap_err(), JoinRefusal::Full);

        let kicked = rooms.kick(1, 3).unwrap();
        assert_eq!(kicked.room.unwrap().members, vec![1, 2]);
        assert_eq!(rooms.join(3, id, "secret", false).unwrap_err(), JoinRefusal::Kicked);

        let owner_left = rooms.leave(1).unwrap();
        assert_eq!(owner_left.new_owner, Some(2));
        assert!(rooms.leave(2).unwrap().room.is_none());
        assert!(rooms.get(id).is_none());
    }
}
