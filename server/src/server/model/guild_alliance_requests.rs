use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

/// Pending guild alliance invitations by invited character, holding the inviter and the inviting guild.
/// Cheap to clone; every clone shares one table, locked per call.
#[derive(Debug, Default, Clone)]
pub struct GuildAllianceRequests {
    pending: Arc<Mutex<HashMap<u32, (u32, u32)>>>,
}

impl GuildAllianceRequests {
    fn pending(&self) -> MutexGuard<'_, HashMap<u32, (u32, u32)>> {
        self.pending.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Returns false, leaving the existing invitation untouched, when the target already has one.
    pub fn request(&self, target: u32, inviter: u32, guild_id: u32) -> bool {
        let mut pending = self.pending();
        if pending.contains_key(&target) {
            return false;
        }
        pending.insert(target, (inviter, guild_id));
        true
    }

    pub fn take(&self, target: u32) -> Option<(u32, u32)> {
        self.pending().remove(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_target_holds_one_invitation_until_it_is_taken() {
        let requests = GuildAllianceRequests::default();
        assert!(requests.request(2, 1, 10));
        assert!(!requests.request(2, 3, 11));
        assert_eq!(requests.take(2), Some((1, 10)));
        assert_eq!(requests.take(2), None);
    }
}
