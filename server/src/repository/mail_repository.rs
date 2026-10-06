use database::{next_id, read, tx_read, tx_write};
use serde::{Deserialize, Serialize};
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use crate::repository::{Error, SledRepository};

/// `MAIL_MAX_INBOX`, `MAIL_TITLE_LENGTH` and the classic `MAIL_BODY_LENGTH` of rathena.
pub const MAIL_MAX_INBOX: usize = 30;
pub const MAIL_TITLE_LENGTH: usize = 40;
pub const MAIL_BODY_LENGTH: usize = 200;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MailItem {
    pub item_id: i32,
    pub amount: i16,
    pub refine: i16,
    pub identified: bool,
    pub damaged: bool,
    pub unique_id: i64,
    pub cards: [i16; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MailMessage {
    pub id: i32,
    /// 0 for mails sent by the server.
    pub sender_id: u32,
    pub sender_name: String,
    pub dest_id: u32,
    pub dest_name: String,
    pub title: String,
    pub body: String,
    pub timestamp: i64,
    pub read: bool,
    pub returned: bool,
    pub zeny: u32,
    pub item: Option<MailItem>,
}

impl MailMessage {
    pub fn has_attachment(&self) -> bool {
        self.zeny > 0 || self.item.is_some()
    }
}

fn mailbox_key(char_id: u32) -> Vec<u8> {
    [b"mailbox/".as_slice(), &char_id.to_be_bytes()].concat()
}

fn mail_key(id: i32) -> Vec<u8> {
    [b"mail/".as_slice(), &id.to_be_bytes()].concat()
}

pub trait MailRepository {
    /// Mails of the character, oldest first. Mails past their return or delete date are processed first.
    fn mail_inbox(&self, _char_id: u32, _now: i64, _return_days: u32, _delete_days: u32) -> Result<Vec<MailMessage>, Error> {
        Ok(Vec::new())
    }
    /// Stores the message in the inbox of its recipient and returns it with its identifier.
    fn mail_deliver(&self, _message: MailMessage) -> Result<MailMessage, Error> {
        Err(Error::InvalidInput("Mail storage is unavailable".into()))
    }
    fn mail_mark_read(&self, _char_id: u32, _mail_id: i32) -> Result<(), Error> {
        Ok(())
    }
    /// Empties the attachments of one mail of the character and returns what it held.
    fn mail_take_attachment(&self, _char_id: u32, _mail_id: i32) -> Result<(u32, Option<MailItem>), Error> {
        Err(Error::InvalidInput("Mail storage is unavailable".into()))
    }
    /// Puts attachments back after a failed retrieval.
    fn mail_restore_attachment(&self, _mail_id: i32, _zeny: u32, _item: Option<MailItem>) -> Result<(), Error> {
        Err(Error::InvalidInput("Mail storage is unavailable".into()))
    }
    /// Deletes a mail without attachment; false when it does not exist or still holds one.
    fn mail_delete(&self, _char_id: u32, _mail_id: i32) -> Result<bool, Error> {
        Err(Error::InvalidInput("Mail storage is unavailable".into()))
    }
    /// Sends the mail back to its sender; the result is the new message, `None` when nothing was sent back.
    fn mail_return(&self, _char_id: u32, _mail_id: i32, _now: i64) -> Result<Option<MailMessage>, Error> {
        Err(Error::InvalidInput("Mail storage is unavailable".into()))
    }
}

fn store_message(
    systems: &TransactionalTree,
    metadata: &TransactionalTree,
    mut message: MailMessage,
) -> ConflictableTransactionResult<MailMessage, Error> {
    message.id = next_id(metadata, b"mail_id", 0)?;
    let mut mailbox: Vec<i32> = tx_read(systems, &mailbox_key(message.dest_id))?.unwrap_or_default();
    mailbox.push(message.id);
    tx_write(systems, &mailbox_key(message.dest_id), &mailbox)?;
    tx_write(systems, &mail_key(message.id), &message)?;
    Ok(message)
}

fn drop_message(systems: &TransactionalTree, message: &MailMessage) -> ConflictableTransactionResult<(), Error> {
    let mut mailbox: Vec<i32> = tx_read(systems, &mailbox_key(message.dest_id))?.unwrap_or_default();
    mailbox.retain(|id| *id != message.id);
    tx_write(systems, &mailbox_key(message.dest_id), &mailbox)?;
    systems.remove(mail_key(message.id))?;
    Ok(())
}

fn returned_copy(message: &MailMessage, now: i64) -> MailMessage {
    MailMessage {
        id: 0,
        sender_id: message.dest_id,
        sender_name: message.dest_name.clone(),
        dest_id: message.sender_id,
        dest_name: message.sender_name.clone(),
        title: format!("RE:{}", message.title).chars().take(MAIL_TITLE_LENGTH).collect(),
        body: message.body.clone(),
        timestamp: now,
        read: false,
        returned: true,
        zeny: message.zeny,
        item: message.item.clone(),
    }
}

impl MailRepository for SledRepository {
    fn mail_inbox(&self, char_id: u32, now: i64, return_days: u32, delete_days: u32) -> Result<Vec<MailMessage>, Error> {
        let ids: Vec<i32> = read(&self.database.game_systems, &mailbox_key(char_id))?.unwrap_or_default();
        let days = |days: u32| i64::from(days) * 24 * 60 * 60;
        let mut inbox = Vec::with_capacity(ids.len());
        for id in ids {
            let Some(message) = read::<MailMessage>(&self.database.game_systems, &mail_key(id))? else {
                continue;
            };
            let age = now.saturating_sub(message.timestamp);
            if message.returned && delete_days > 0 && age > days(delete_days) {
                (&self.database.game_systems,).transaction(|(systems,)| drop_message(systems, &message))?;
            } else if !message.returned && return_days > 0 && age > days(return_days) {
                self.mail_return(char_id, id, now)?;
            } else {
                inbox.push(message);
            }
        }
        Ok(inbox)
    }

    fn mail_deliver(&self, message: MailMessage) -> Result<MailMessage, Error> {
        Ok((&self.database.game_systems, &self.database.metadata)
            .transaction(|(systems, metadata)| store_message(systems, metadata, message.clone()))?)
    }

    fn mail_mark_read(&self, char_id: u32, mail_id: i32) -> Result<(), Error> {
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            if let Some(mut message) = tx_read::<MailMessage>(systems, &mail_key(mail_id))?.filter(|m| m.dest_id == char_id) {
                message.read = true;
                tx_write(systems, &mail_key(mail_id), &message)?;
            }
            Ok(())
        })?)
    }

    fn mail_take_attachment(&self, char_id: u32, mail_id: i32) -> Result<(u32, Option<MailItem>), Error> {
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            let Some(mut message) = tx_read::<MailMessage>(systems, &mail_key(mail_id))?.filter(|m| m.dest_id == char_id) else {
                return Ok((0, None));
            };
            let taken = (std::mem::take(&mut message.zeny), message.item.take());
            tx_write(systems, &mail_key(mail_id), &message)?;
            Ok(taken)
        })?)
    }

    fn mail_restore_attachment(&self, mail_id: i32, zeny: u32, item: Option<MailItem>) -> Result<(), Error> {
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            if let Some(mut message) = tx_read::<MailMessage>(systems, &mail_key(mail_id))? {
                message.zeny = zeny;
                message.item = item.clone();
                tx_write(systems, &mail_key(mail_id), &message)?;
            }
            Ok(())
        })?)
    }

    fn mail_delete(&self, char_id: u32, mail_id: i32) -> Result<bool, Error> {
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            let Some(message) = tx_read::<MailMessage>(systems, &mail_key(mail_id))?.filter(|m| m.dest_id == char_id) else {
                return Ok(false);
            };
            if message.has_attachment() {
                return Ok(false);
            }
            drop_message(systems, &message)?;
            Ok(true)
        })?)
    }

    fn mail_return(&self, char_id: u32, mail_id: i32, now: i64) -> Result<Option<MailMessage>, Error> {
        Ok((&self.database.game_systems, &self.database.metadata).transaction(|(systems, metadata)| {
            let Some(message) = tx_read::<MailMessage>(systems, &mail_key(mail_id))?.filter(|m| m.dest_id == char_id) else {
                return Ok(None);
            };
            drop_message(systems, &message)?;
            if message.sender_id == 0 {
                return Ok(None);
            }
            Ok(Some(store_message(systems, metadata, returned_copy(&message, now))?))
        })?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mail(sender: u32, dest: u32, zeny: u32) -> MailMessage {
        MailMessage {
            sender_id: sender,
            sender_name: format!("char{sender}"),
            dest_id: dest,
            dest_name: format!("char{dest}"),
            title: "hello".into(),
            timestamp: 1_000,
            zeny,
            ..MailMessage::default()
        }
    }

    #[test]
    fn attachments_are_taken_once_and_returned_mail_goes_back_with_them() {
        let repository = SledRepository::temporary().unwrap();
        let sent = repository.mail_deliver(mail(1, 2, 500)).unwrap();
        assert!(!repository.mail_delete(2, sent.id).unwrap(), "attachments must be retrieved first");
        assert_eq!(repository.mail_take_attachment(2, sent.id).unwrap().0, 500);
        assert_eq!(repository.mail_take_attachment(2, sent.id).unwrap().0, 0);
        assert!(repository.mail_delete(2, sent.id).unwrap());

        let second = repository.mail_deliver(mail(1, 2, 10)).unwrap();
        let returned = repository.mail_return(2, second.id, 2_000).unwrap().unwrap();
        assert_eq!((returned.dest_id, returned.zeny, returned.title.as_str(), returned.returned), (1, 10, "RE:hello", true));
        assert!(repository.mail_inbox(2, 2_000, 0, 0).unwrap().is_empty());
        assert_eq!(repository.mail_inbox(1, 2_000, 0, 0).unwrap().len(), 1);
    }

    #[test]
    fn unread_mail_goes_back_after_the_return_delay() {
        let repository = SledRepository::temporary().unwrap();
        repository.mail_deliver(mail(1, 2, 0)).unwrap();
        let after_sixteen_days = 1_000 + 16 * 24 * 60 * 60;
        assert!(repository.mail_inbox(2, after_sixteen_days, 15, 15).unwrap().is_empty());
        assert_eq!(repository.mail_inbox(1, after_sixteen_days, 15, 15).unwrap().len(), 1);
    }
}
