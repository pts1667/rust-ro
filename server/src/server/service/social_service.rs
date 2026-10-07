use crate::repository::social_repository::{FriendAdd, FriendRecord, MAX_FRIENDS};
use crate::server::model::chat_room::{ChatRoom, Departure, JoinRefusal};
use crate::server::model::events::game_event::{GameEvent, ScriptBroadcast, SocialAction};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::permission_groups::Permission;
use crate::server::service::social_packets as wire;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::{PLAYER_FOV, Server};
use models::enums::EnumWithMaskValueU32;
use models::enums::script::BroadcastFlag;

/// `CHAT_SIZE_MAX` of rathena.
const CHAT_SIZE_MAX: usize = 255;
/// `MAX_IGNORE_LIST` of rathena.
const MAX_IGNORE_LIST: usize = 20;
const NAME_SEPARATOR: &[u8] = b" : ";
const BASIC_SKILL_CHAT_LEVEL: u8 = 4;
const ADMIN_LEVEL: u8 = 99;

const WHISPER_TYPE_DENY: u8 = 0;
const IGNORE_FAILED: u8 = 1;
const IGNORE_LIST_FULL: u8 = 2;

const FRIEND_ADDED: u16 = 0;
const FRIEND_REFUSED: u16 = 1;
const FRIEND_OWN_LIST_FULL: u16 = 2;

/// The text of a `Name : text` chat packet, when the speaker prefix is the character's own name.
fn message_after_speaker<'a>(text: &'a [u8], name: &str) -> Option<&'a [u8]> {
    let text = text.split(|byte| *byte == 0).next().unwrap_or_default();
    text.strip_prefix(name.as_bytes())?.strip_prefix(NAME_SEPARATOR)
}

fn same_name(left: &str, right: &str) -> bool {
    left == right
}

/// The title packet for a character that owns a chat room, sent to whoever sees that character appear.
pub(crate) fn chat_room_entry_packet(state: &ServerState, owner_id: u32) -> Option<Vec<u8>> {
    state.chat_rooms.owned_by(owner_id).map(wire::room_entry)
}

impl Server {
    pub(crate) fn run_social_action(&self, state: &mut ServerState, char_id: u32, action: SocialAction, tick: u128) -> Result<(), String> {
        if state.get_character(char_id).is_none() {
            return Err("Character is not in game".into());
        }
        match action {
            SocialAction::PublicChat(text) => self.public_chat(state, char_id, &text),
            SocialAction::Whisper { target, message } => self.whisper(state, char_id, &target, &message, tick),
            SocialAction::IgnoreName { name, block } => self.ignore_name(state, char_id, name, block),
            SocialAction::IgnoreAll { block } => self.ignore_all(state, char_id, block),
            SocialAction::IgnoreList => {
                let names = state.get_character(char_id).map(|c| c.game_systems.ignored_names.clone()).unwrap_or_default();
                self.send_raw(char_id, wire::whisper_list(&names));
            }
            SocialAction::CreateRoom { title, password, limit, public } => self.create_chat_room(state, char_id, &title, &password, limit, public),
            SocialAction::EnterRoom { room_id, password } => self.enter_chat_room(state, char_id, room_id, &password),
            SocialAction::ChangeRoom { title, password, limit, public } => self.change_chat_room(state, char_id, &title, &password, limit, public),
            SocialAction::RoomOwner { name } => self.change_chat_room_owner(state, char_id, &name),
            SocialAction::KickFromRoom { name } => self.kick_from_chat_room(state, char_id, &name),
            SocialAction::LeaveRoom => self.leave_chat_room(state, char_id, false),
            SocialAction::FriendRequest { name } => self.friend_request(state, char_id, &name),
            SocialAction::FriendReply { inviter_account_id, accepted } => self.friend_reply(state, char_id, inviter_account_id, accepted),
            SocialAction::FriendRemove { account_id, char_id: friend_id } => self.friend_remove(state, char_id, account_id, friend_id),
            SocialAction::Mail(action) => self.run_mail_action(state, char_id, action, tick),
        }
        Ok(())
    }

    pub(crate) fn tell(&self, char_id: u32, text: &str) {
        self.send_raw(char_id, wire::notify_player_chat(text.as_bytes()));
    }

    fn online_character_named<'a>(state: &'a ServerState, name: &str) -> Option<&'a Character> {
        state
            .characters()
            .values()
            .find(|character| character.loaded_from_client_side && same_name(&character.name, name))
    }

    fn group_level(state: &ServerState, character: &Character) -> u8 {
        state.permission_groups().level(state.group_id_of(character.account_id))
    }

    /// Public chat: heard by the characters around that are not inside a chat room, or by the room members when the speaker is in one.
    fn public_chat(&self, state: &ServerState, char_id: u32, text: &[u8]) {
        let Some(speaker) = state.get_character(char_id) else { return };
        let Some(message) = message_after_speaker(text, &speaker.name) else {
            warn!("Chat message of {} does not start with the character name", speaker.name);
            return;
        };
        if message.len() + 1 > CHAT_SIZE_MAX || speaker.manner < 0 {
            return;
        }
        let text = text.split(|byte| *byte == 0).next().unwrap_or_default();
        let waiting_room = state.waiting_rooms.room_of(char_id);
        let listeners: Vec<u32> = match (state.chat_rooms.room_of(char_id), waiting_room) {
            (Some(room), _) => room.members.iter().copied().filter(|member| *member != char_id).collect(),
            (None, Some(room)) => room.members.iter().copied().filter(|member| *member != char_id).collect(),
            (None, None) => state
                .directory()
                .in_fov(speaker.current_map_name(), speaker.current_map_instance(), speaker.x(), speaker.y(), PLAYER_FOV, Some(char_id))
                .into_iter()
                .filter(|listener| state.chat_rooms.room_of(*listener).is_none() && state.waiting_rooms.room_of(*listener).is_none())
                .collect(),
        };
        let heard = wire::notify_chat(char_id, text);
        for listener in listeners {
            self.send_raw(listener, heard.clone());
        }
        self.send_raw(char_id, wire::notify_player_chat(text));
    }

    fn whisper(&self, state: &mut ServerState, char_id: u32, target: &str, message: &[u8], tick: u128) {
        let message = message.split(|byte| *byte == 0).next().unwrap_or_default();
        if target.starts_with('#') && self.channel_chat(state, char_id, target, message, tick) {
            return;
        }
        let Some(sender) = state.get_character(char_id) else { return };
        if message.len() + 1 > CHAT_SIZE_MAX || sender.manner < 0 {
            return;
        }
        let Some(destination) = Self::online_character_named(state, target) else {
            self.send_raw(char_id, wire::whisper_result(wire::WHISPER_TARGET_OFFLINE));
            return;
        };
        let sender_level = Self::group_level(state, sender);
        let destination_level = Self::group_level(state, destination);
        if sender_level <= destination_level {
            if destination.game_systems.ignore_all {
                self.send_raw(char_id, wire::whisper_result(wire::WHISPER_ALL_IGNORED));
                return;
            }
            if destination.game_systems.ignored_names.iter().any(|ignored| same_name(ignored, &sender.name)) {
                self.send_raw(char_id, wire::whisper_result(wire::WHISPER_IGNORED));
                return;
            }
        }
        self.send_raw(char_id, wire::whisper_result(wire::WHISPER_SUCCESS));
        self.send_raw(destination.char_id, wire::whisper(&sender.name, sender_level == ADMIN_LEVEL, message));
    }

    fn ignore_name(&self, state: &mut ServerState, char_id: u32, name: String, block: bool) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        let list = &mut character.game_systems.ignored_names;
        let known = list.iter().position(|ignored| same_name(ignored, &name));
        let result = match (block, known) {
            (true, Some(_)) => 0,
            (true, None) if list.len() >= MAX_IGNORE_LIST => IGNORE_LIST_FULL,
            (true, None) => {
                list.push(name);
                0
            }
            (false, Some(index)) => {
                list.remove(index);
                0
            }
            (false, None) => IGNORE_FAILED,
        };
        let kind = if block { WHISPER_TYPE_DENY } else { 1 };
        self.send_raw(char_id, wire::whisper_pc_setting(kind, result));
    }

    fn ignore_all(&self, state: &mut ServerState, char_id: u32, block: bool) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        let systems = &mut character.game_systems;
        let failure = if block {
            std::mem::replace(&mut systems.ignore_all, true)
        } else if systems.ignore_all {
            systems.ignore_all = false;
            false
        } else if !systems.ignored_names.is_empty() {
            systems.ignored_names.clear();
            false
        } else {
            true
        };
        self.send_raw(char_id, wire::whisper_state_setting(u8::from(!block), failure));
    }

    fn room_member_names(state: &ServerState, room: &ChatRoom) -> Vec<String> {
        room.members
            .iter()
            .map(|member| state.get_character(*member).map(|c| c.name.clone()).unwrap_or_default())
            .collect()
    }

    /// Shows or refreshes the room title above its owner for everyone around, except the owner.
    fn show_chat_room(&self, state: &ServerState, room: &ChatRoom) {
        if let Some(owner) = state.get_character(room.owner()) {
            self.send_area_raw(owner, wire::room_entry(room), false);
        }
    }

    fn create_chat_room(&self, state: &mut ServerState, char_id: u32, title: &str, password: &str, limit: u16, public: bool) {
        let Some(character) = state.get_character(char_id) else { return };
        if state.chat_rooms.room_of(char_id).is_some()
            || character.game_systems.vending_store.is_some()
            || character.game_systems.buying_store.is_some()
        {
            return;
        }
        if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoChat) {
            self.tell(char_id, "You can't create chat rooms on this map.");
            return;
        }
        if self.configuration.game.basic_skill_check && Self::basic_skill_level(character) < BASIC_SKILL_CHAT_LEVEL {
            self.send_raw(char_id, Self::basic_skill_required(3));
            return;
        }
        let Some(room) = state.chat_rooms.create(char_id, title, password, limit, public).cloned() else {
            self.send_raw(char_id, wire::create_chat_room_result(1));
            return;
        };
        self.send_raw(char_id, wire::create_chat_room_result(0));
        self.show_chat_room(state, &room);
        if let Some(character) = state.characters_mut().get_mut(&char_id) {
            character.clear_attack();
        }
    }

    /// `ZC_ACK_TOUSESKILL` telling the client that Basic Skill is too low for the requested feature.
    fn basic_skill_required(feature: u32) -> Vec<u8> {
        let mut packet = 0x0110_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&1u16.to_le_bytes());
        packet.extend_from_slice(&feature.to_le_bytes());
        packet.extend_from_slice(&[0, 0]);
        packet
    }

    fn enter_chat_room(&self, state: &mut ServerState, char_id: u32, room_id: u32, password: &str) {
        if self.join_waiting_room(state, char_id, room_id) {
            return;
        }
        let Some(character) = state.get_character(char_id) else { return };
        let name = character.name.clone();
        let bypass_password = state.has_permission(character.account_id, Permission::JoinChat);
        let busy = character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some();
        let on_owner_map = state
            .chat_rooms
            .get(room_id)
            .and_then(|room| state.get_character(room.owner()))
            .is_some_and(|owner| owner.map_instance_key == character.map_instance_key);
        if busy || !on_owner_map {
            self.send_raw(char_id, wire::room_refused(0));
            return;
        }
        match state.chat_rooms.join(char_id, room_id, password, bypass_password).cloned() {
            Err(refusal) => {
                let result = match refusal {
                    JoinRefusal::Full => 0,
                    JoinRefusal::WrongPassword => 1,
                    JoinRefusal::Kicked => 2,
                };
                self.send_raw(char_id, wire::room_refused(result));
            }
            Ok(room) => {
                self.send_raw(char_id, wire::room_entered(room.id, &Self::room_member_names(state, &room)));
                for member in room.members.iter().filter(|member| **member != char_id) {
                    self.send_raw(*member, wire::room_member_joined(room.members.len(), &name));
                }
                self.show_chat_room(state, &room);
            }
        }
    }

    fn change_chat_room(&self, state: &mut ServerState, char_id: u32, title: &str, password: &str, limit: u16, public: bool) {
        let Some(room) = state.chat_rooms.change(char_id, title, password, limit, public).cloned() else { return };
        let changed = wire::room_changed(&room);
        for member in &room.members {
            self.send_raw(*member, changed.clone());
        }
        self.show_chat_room(state, &room);
    }

    fn change_chat_room_owner(&self, state: &mut ServerState, char_id: u32, name: &str) {
        let Some(room) = state.chat_rooms.owned_by(char_id) else { return };
        let Some(target) = room
            .members
            .iter()
            .skip(1)
            .copied()
            .find(|member| state.get_character(*member).is_some_and(|c| same_name(&c.name, name)))
        else {
            return;
        };
        let Some(old_owner) = state.get_character(char_id) else { return };
        self.send_area_raw(old_owner, wire::room_destroyed(room.id), false);
        let Some(room) = state.chat_rooms.transfer_ownership(char_id, target).cloned() else { return };
        for member in &room.members {
            for (index, role_holder) in [target, char_id].into_iter().enumerate() {
                if let Some(holder) = state.get_character(role_holder) {
                    self.send_raw(*member, wire::room_role(index == 0, &holder.name));
                }
            }
        }
        self.show_chat_room(state, &room);
    }

    fn kick_from_chat_room(&self, state: &mut ServerState, char_id: u32, name: &str) {
        let Some(room) = state.chat_rooms.owned_by(char_id) else { return };
        let Some(target) = room
            .members
            .iter()
            .copied()
            .find(|member| state.get_character(*member).is_some_and(|c| same_name(&c.name, name)))
        else {
            return;
        };
        let protected = state
            .get_character(target)
            .is_some_and(|target| state.has_permission(target.account_id, Permission::KickChat));
        if !protected {
            self.depart_chat_room(state, target, true);
        }
    }

    pub(crate) fn leave_chat_room(&self, state: &mut ServerState, char_id: u32, kicked: bool) {
        self.depart_chat_room(state, char_id, kicked);
    }

    fn depart_chat_room(&self, state: &mut ServerState, char_id: u32, kicked: bool) {
        if self.depart_waiting_room(state, char_id, kicked) {
            return;
        }
        let Some(leaver) = state.get_character(char_id) else { return };
        let name = leaver.name.clone();
        let owner_id = state.chat_rooms.room_of(char_id).map(ChatRoom::owner);
        let departure = if kicked {
            owner_id.and_then(|owner| state.chat_rooms.kick(owner, char_id))
        } else {
            state.chat_rooms.leave(char_id)
        };
        let Some(Departure { room, room_id, new_owner }) = departure else { return };
        let Some(leaver) = state.get_character(char_id) else { return };
        let users_after = room.as_ref().map_or(0, |room| room.members.len());
        let left = wire::room_member_left(users_after, &name, kicked);
        self.send_raw(char_id, left.clone());
        for member in room.iter().flat_map(|room| room.members.iter()) {
            self.send_raw(*member, left.clone());
        }
        let Some(room) = room else {
            self.send_area_raw(leaver, wire::room_destroyed(room_id), false);
            return;
        };
        if let Some(owner_id) = new_owner {
            self.send_area_raw(leaver, wire::room_destroyed(room_id), false);
            if let Some(owner) = state.get_character(owner_id) {
                for member in &room.members {
                    self.send_raw(*member, wire::room_role(true, &owner.name));
                }
            }
        }
        self.show_chat_room(state, &room);
    }

    fn friend_name(&self, state: &ServerState, char_id: u32) -> Option<String> {
        if let Some(character) = state.get_character(char_id) {
            return Some(character.name.clone());
        }
        self.repository.char_find(char_id).ok().flatten().map(|record| record.name)
    }

    /// Sends the friend list and online notices when a character enters a map, and tells the friends who are online.
    pub(crate) fn friend_login(&self, state: &mut ServerState, char_id: u32) {
        let friends = self.repository.friends(char_id).unwrap_or_else(|error| {
            warn!("Friend list of {char_id} could not be loaded: {error}");
            Vec::new()
        });
        let entries: Vec<(u32, u32, String)> = friends
            .iter()
            .filter_map(|friend| Some((friend.account_id, friend.char_id, self.friend_name(state, friend.char_id)?)))
            .collect();
        self.send_raw(char_id, wire::friends_list(&entries));
        for friend in &friends {
            if state.get_character(friend.char_id).is_some_and(|friend| friend.loaded_from_client_side) {
                self.send_raw(char_id, wire::friend_state(friend.account_id, friend.char_id, true));
            }
        }
        if let Some(character) = state.characters_mut().get_mut(&char_id) {
            character.game_systems.friends = friends;
        }
        self.notify_friends(state, char_id, true);
    }

    pub(crate) fn notify_friends(&self, state: &ServerState, char_id: u32, online: bool) {
        let Some(account_id) = state.get_character(char_id).map(|character| character.account_id) else { return };
        for other in state.characters().values() {
            if other.game_systems.friends.iter().any(|friend| friend.char_id == char_id) {
                self.send_raw(other.char_id, wire::friend_state(account_id, char_id, online));
            }
        }
    }

    fn friend_request(&self, state: &mut ServerState, char_id: u32, name: &str) {
        let Some(me) = state.get_character(char_id) else { return };
        let Some(target) = Self::online_character_named(state, name) else {
            self.tell(char_id, "Character not found.");
            return;
        };
        if target.char_id == char_id {
            return;
        }
        let (target_id, target_account, target_name) = (target.char_id, target.account_id, target.name.clone());
        if me.game_systems.friends.len() >= MAX_FRIENDS {
            self.send_raw(char_id, wire::friend_request_result(FRIEND_OWN_LIST_FULL, target_account, target_id, &target_name));
            return;
        }
        if target.game_systems.no_ask {
            self.tell(char_id, "Your request has been rejected by autoreject option.");
            self.tell(target_id, &format!("Autorejected friend request from {}.", me.name));
            return;
        }
        if me.game_systems.friends.iter().any(|friend| friend.char_id == target_id) {
            self.tell(char_id, "Friend already exists.");
            return;
        }
        let request = wire::friend_request(me.account_id, char_id, &me.name);
        for (id, other) in [(char_id, target_id), (target_id, char_id)] {
            if let Some(character) = state.characters_mut().get_mut(&id) {
                character.game_systems.friend_request = other;
            }
        }
        self.send_raw(target_id, request);
    }

    fn friend_reply(&self, state: &mut ServerState, char_id: u32, inviter_account_id: u32, accepted: bool) {
        let Some(me) = state.get_character(char_id) else { return };
        let Some(inviter) = state
            .characters()
            .values()
            .find(|character| character.account_id == inviter_account_id && character.game_systems.friend_request == char_id)
        else {
            return;
        };
        let inviter_id = inviter.char_id;
        let (my_account, my_name) = (me.account_id, me.name.clone());
        let (inviter_account, inviter_name) = (inviter.account_id, inviter.name.clone());
        let mutual = me.game_systems.friend_request == inviter_id;
        for id in [char_id, inviter_id] {
            if let Some(character) = state.characters_mut().get_mut(&id) {
                character.game_systems.friend_request = 0;
            }
        }
        if !accepted || !mutual {
            self.send_raw(inviter_id, wire::friend_request_result(FRIEND_REFUSED, my_account, char_id, &my_name));
            return;
        }
        let befriended = FriendRecord { account_id: my_account, char_id };
        if !self.add_friend(state, inviter_id, befriended) {
            self.send_raw(inviter_id, wire::friend_request_result(FRIEND_OWN_LIST_FULL, my_account, char_id, &my_name));
            return;
        }
        self.send_raw(inviter_id, wire::friend_request_result(FRIEND_ADDED, my_account, char_id, &my_name));
        if self.configuration.game.friend_auto_add {
            let requester = FriendRecord { account_id: inviter_account, char_id: inviter_id };
            let result = if self.add_friend(state, char_id, requester) { FRIEND_ADDED } else { FRIEND_OWN_LIST_FULL };
            self.send_raw(char_id, wire::friend_request_result(result, inviter_account, inviter_id, &inviter_name));
        }
    }

    /// Persists the friend and updates the online copy; false when the list is full.
    fn add_friend(&self, state: &mut ServerState, char_id: u32, friend: FriendRecord) -> bool {
        match self.repository.friend_add(char_id, friend) {
            Ok(FriendAdd::Added) => {
                if let Some(character) = state.characters_mut().get_mut(&char_id) {
                    character.game_systems.friends.push(friend);
                }
                true
            }
            Ok(FriendAdd::AlreadyFriends) => true,
            Ok(FriendAdd::ListFull) => false,
            Err(error) => {
                warn!("Friend of {char_id} could not be saved: {error}");
                false
            }
        }
    }

    fn friend_remove(&self, state: &mut ServerState, char_id: u32, account_id: u32, friend_id: u32) {
        let Some(me) = state.get_character(char_id) else { return };
        if !me.game_systems.friends.iter().any(|friend| friend.char_id == friend_id && friend.account_id == account_id) {
            self.tell(char_id, "Name not found in list.");
            return;
        }
        let my_account = me.account_id;
        if let Err(error) = self.repository.friend_remove(friend_id, char_id).and(self.repository.friend_remove(char_id, friend_id)) {
            warn!("Friend removal failed: {error}");
            self.tell(char_id, "This action can't be performed at the moment. Please try again later.");
            return;
        }
        if let Some(other) = state.characters_mut().get_mut(&friend_id) {
            other.game_systems.friends.retain(|friend| friend.char_id != char_id);
            self.send_raw(friend_id, wire::friend_removed(my_account, char_id));
        }
        if let Some(me) = state.characters_mut().get_mut(&char_id) {
            me.game_systems.friends.retain(|friend| friend.char_id != friend_id);
        }
        self.tell(char_id, "Friend removed.");
        self.send_raw(char_id, wire::friend_removed(account_id, friend_id));
    }

    /// `@kami`, `@kamib`, `@kamic` and `@lkami`; returns the reply for the caller.
    pub(crate) fn command_broadcast(&self, char_id: u32, command: &str, argument: &str) -> String {
        let argument = argument.trim();
        let argument: String = argument.chars().take(CHAT_SIZE_MAX).collect();
        let usage = |text: &str| text.to_string();
        let (packet, flags) = match command {
            "kami" | "lkami" if !argument.is_empty() => {
                let flags = if command == "lkami" { BroadcastFlag::Map.as_flag() } else { 0 };
                (wire::broadcast(&argument), flags)
            }
            "kamib" if !argument.is_empty() => (wire::broadcast(&format!("blue{argument}")), 0),
            "kamic" => {
                let Some((color, message)) = argument.split_once(' ').filter(|(_, message)| !message.trim().is_empty()) else {
                    return usage("Please enter color and message (usage: @kamic <color> <message>).");
                };
                let Some(color) = u32::from_str_radix(color, 16).ok().filter(|color| *color <= 0xFF_FFFF) else {
                    return usage("Invalid color.");
                };
                (wire::broadcast_colored(color, message.trim()), 0)
            }
            _ => return usage("Please enter a message (usage: @kami <message>)."),
        };
        self.add_to_next_tick(GameEvent::ScriptBroadcast(ScriptBroadcast { char_id, flags, packet }));
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_text_must_start_with_the_speaker_name() {
        assert_eq!(message_after_speaker(b"Alice : hello\0", "Alice"), Some(b"hello".as_slice()));
        assert_eq!(message_after_speaker(b"Mallory : hello\0", "Alice"), None);
        assert_eq!(message_after_speaker(b"Alice: hello\0", "Alice"), None);
    }
}
