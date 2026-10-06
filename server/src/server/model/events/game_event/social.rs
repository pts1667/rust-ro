use super::*;
use crate::server::Server;
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub enum SocialAction {
    /// `Name : text` as typed by the player, undecoded.
    PublicChat(Vec<u8>),
    Whisper { target: String, message: Vec<u8> },
    IgnoreName { name: String, block: bool },
    IgnoreAll { block: bool },
    IgnoreList,
    CreateRoom { title: String, password: String, limit: u16, public: bool },
    EnterRoom { room_id: u32, password: String },
    ChangeRoom { title: String, password: String, limit: u16, public: bool },
    RoomOwner { name: String },
    KickFromRoom { name: String },
    LeaveRoom,
    FriendRequest { name: String },
    FriendReply { inviter_account_id: u32, accepted: bool },
    FriendRemove { account_id: u32, char_id: u32 },
    Mail(MailAction),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MailAction {
    /// `CZ_MAIL_RESET_ITEM`: 0 everything, 1 the item, 2 the zeny.
    ResetDraft(u16),
    List,
    Open(i32),
    Delete(i32),
    TakeAttachment(i32),
    Attach { index: u16, amount: u32 },
    Send { recipient: String, title: String, body: String },
    Return(i32),
}

/// Chat, whisper, chat room and friend requests of one character.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSocial {
    pub char_id: u32,
    pub action: SocialAction,
}

impl GameEventHandler for CharacterSocial {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        server.run_social_action(state, self.char_id, self.action, tick)
    }
}
