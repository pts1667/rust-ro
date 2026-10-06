use std::collections::VecDeque;
use std::sync::mpsc::{SyncSender, TrySendError};
use std::sync::{Mutex, MutexGuard};

use crate::server::model::events::client_notification::Notification;

const FLUSH_BATCH: usize = 64;

/// Notifications waiting for room in the bounded client notification channel.
#[derive(Default)]
pub struct NotificationBacklog {
    queue: Mutex<VecDeque<Notification>>,
}

impl NotificationBacklog {
    fn queue(&self) -> MutexGuard<'_, VecDeque<Notification>> {
        self.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn push(&self, notification: Notification) {
        self.queue().push_back(notification);
    }

    pub fn extend(&self, notifications: impl IntoIterator<Item = Notification>) {
        self.queue().extend(notifications);
    }

    /// Sends up to a batch of queued notifications, stopping at the first full channel.
    pub fn flush(&self, sender: &SyncSender<Notification>) {
        let mut queue = self.queue();
        for _ in 0..FLUSH_BATCH {
            let Some(notification) = queue.pop_front() else {
                break;
            };
            match sender.try_send(notification) {
                Ok(()) => {}
                Err(TrySendError::Full(notification)) => {
                    queue.push_front(notification);
                    break;
                }
                Err(TrySendError::Disconnected(_)) => {
                    queue.clear();
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::sync_channel;

    use super::*;
    use crate::server::model::events::client_notification::CharNotification;

    fn notification(char_id: u32) -> Notification {
        Notification::Char(CharNotification::new(char_id, vec![]))
    }

    #[test]
    fn flush_keeps_what_does_not_fit_in_order() {
        let backlog = NotificationBacklog::default();
        let (sender, receiver) = sync_channel(1);
        backlog.extend([notification(1), notification(2)]);
        backlog.flush(&sender);
        assert!(matches!(receiver.try_recv(), Ok(Notification::Char(char)) if char.char_id() == 1));
        backlog.flush(&sender);
        assert!(matches!(receiver.try_recv(), Ok(Notification::Char(char)) if char.char_id() == 2));
        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn flush_drops_everything_once_the_channel_is_closed() {
        let backlog = NotificationBacklog::default();
        let (sender, receiver) = sync_channel(1);
        drop(receiver);
        backlog.push(notification(1));
        backlog.flush(&sender);
        let (live, live_receiver) = sync_channel(1);
        backlog.flush(&live);
        assert!(live_receiver.try_recv().is_err());
    }
}
