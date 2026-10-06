use database::{read, tx_read, tx_write};
use serde::{Deserialize, Serialize};
use sled::transaction::Transactional;

use crate::repository::{Error, SledRepository};

/// `MAX_FRIENDS` of rathena.
pub const MAX_FRIENDS: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FriendRecord {
    pub account_id: u32,
    pub char_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendAdd {
    Added,
    AlreadyFriends,
    ListFull,
}

fn friends_key(char_id: u32) -> Vec<u8> {
    [b"friends/".as_slice(), &char_id.to_be_bytes()].concat()
}

pub trait SocialRepository {
    fn friends(&self, _char_id: u32) -> Result<Vec<FriendRecord>, Error> {
        Ok(Vec::new())
    }
    fn friend_add(&self, _char_id: u32, _friend: FriendRecord) -> Result<FriendAdd, Error> {
        Err(Error::InvalidInput("Friend storage is unavailable".into()))
    }
    fn friend_remove(&self, _char_id: u32, _friend_char_id: u32) -> Result<bool, Error> {
        Err(Error::InvalidInput("Friend storage is unavailable".into()))
    }
}

impl SocialRepository for SledRepository {
    fn friends(&self, char_id: u32) -> Result<Vec<FriendRecord>, Error> {
        Ok(read(&self.database.game_systems, &friends_key(char_id))?.unwrap_or_default())
    }

    fn friend_add(&self, char_id: u32, friend: FriendRecord) -> Result<FriendAdd, Error> {
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            let mut friends: Vec<FriendRecord> = tx_read(systems, &friends_key(char_id))?.unwrap_or_default();
            if friends.iter().any(|known| known.char_id == friend.char_id) {
                return Ok(FriendAdd::AlreadyFriends);
            }
            if friends.len() >= MAX_FRIENDS {
                return Ok(FriendAdd::ListFull);
            }
            friends.push(friend);
            tx_write(systems, &friends_key(char_id), &friends)?;
            Ok(FriendAdd::Added)
        })?)
    }

    fn friend_remove(&self, char_id: u32, friend_char_id: u32) -> Result<bool, Error> {
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            let mut friends: Vec<FriendRecord> = tx_read(systems, &friends_key(char_id))?.unwrap_or_default();
            let Some(index) = friends.iter().position(|known| known.char_id == friend_char_id) else {
                return Ok(false);
            };
            friends.remove(index);
            tx_write(systems, &friends_key(char_id), &friends)?;
            Ok(true)
        })?)
    }
}
