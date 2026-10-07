use std::net::Ipv4Addr;

use database::model::{AccountRecord, IpBanRecord, LoginLogRecord};
use database::{next_id, read, tx_read, tx_required, tx_write};
use sled::transaction::Transactional;

use crate::repository::{Error, LoginRepository, SledRepository};

pub fn ip_ban_patterns(ip: Ipv4Addr) -> [String; 4] {
    let [a, b, c, d] = ip.octets();
    [format!("{a}.*.*.*"), format!("{a}.{b}.*.*"), format!("{a}.{b}.{c}.*"), format!("{a}.{b}.{c}.{d}")]
}

fn log_key(time: i64, sequence: i32) -> [u8; 12] {
    let mut key = [0; 12];
    key[..8].copy_from_slice(&time.max(0).to_be_bytes());
    key[8..].copy_from_slice(&sequence.to_be_bytes());
    key
}

impl LoginRepository for SledRepository {
    fn account_by_name(&self, name: &str) -> Result<Option<AccountRecord>, Error> {
        let Some(id) = read::<u32>(&self.database.account_names, name.as_bytes())? else {
            return Ok(None);
        };
        read(&self.database.accounts, &id.to_be_bytes())
    }

    fn account_by_id(&self, account_id: u32) -> Result<Option<AccountRecord>, Error> {
        read(&self.database.accounts, &account_id.to_be_bytes())
    }

    fn account_create(&self, account: AccountRecord) -> Result<u32, Error> {
        self.database.create_account_record(account)
    }

    fn account_update(&self, account_id: u32, update: &dyn Fn(&mut AccountRecord)) -> Result<AccountRecord, Error> {
        Ok(self.database.accounts.transaction(|accounts| {
            let mut account: AccountRecord = tx_required(accounts, &account_id.to_be_bytes())?;
            update(&mut account);
            tx_write(accounts, &account_id.to_be_bytes(), &account)?;
            Ok(account)
        })?)
    }

    fn account_delete(&self, account_id: u32) -> Result<bool, Error> {
        Ok((&self.database.accounts, &self.database.account_names).transaction(|(accounts, names)| {
            let Some(account) = tx_read::<AccountRecord>(accounts, &account_id.to_be_bytes())? else {
                return Ok(false);
            };
            accounts.remove(account_id.to_be_bytes().to_vec())?;
            names.remove(account.username.as_bytes())?;
            Ok(true)
        })?)
    }

    fn ip_ban_active(&self, ip: Ipv4Addr, now: i64) -> Result<bool, Error> {
        for pattern in ip_ban_patterns(ip) {
            if read::<IpBanRecord>(&self.database.ip_bans, pattern.as_bytes())?.is_some_and(|ban| ban.release > now) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn ip_ban_add(&self, ban: IpBanRecord) -> Result<(), Error> {
        Ok(self.database.ip_bans.transaction(|bans| {
            let release = tx_read::<IpBanRecord>(bans, ban.list.as_bytes())?.map_or(ban.release, |old| old.release.max(ban.release));
            tx_write(bans, ban.list.as_bytes(), &IpBanRecord { release, ..ban.clone() })
        })?)
    }

    fn ip_ban_remove(&self, list: &str) -> Result<bool, Error> {
        Ok(self.database.ip_bans.remove(list.as_bytes())?.is_some())
    }

    fn ip_ban_cleanup(&self, now: i64) -> Result<usize, Error> {
        let mut removed = 0;
        for entry in self.database.ip_bans.iter() {
            let (key, value) = entry?;
            let ban: IpBanRecord = serde_json::from_slice(&value)?;
            if ban.release <= now && self.database.ip_bans.remove(key)?.is_some() {
                removed += 1;
            }
        }
        Ok(removed)
    }

    fn login_log_append(&self, record: LoginLogRecord) -> Result<(), Error> {
        Ok((&self.database.login_log, &self.database.metadata).transaction(|(log, metadata)| {
            let sequence = next_id(metadata, b"login_log_id", 0)?;
            tx_write(log, &log_key(record.time, sequence), &record)
        })?)
    }

    fn login_log_failed_attempts(&self, ip: &str, since: i64) -> Result<u32, Error> {
        let mut failures = 0;
        for entry in self.database.login_log.range(log_key(since, 0)..) {
            let (_, value) = entry?;
            let record: LoginLogRecord = serde_json::from_slice(&value)?;
            if record.ip == ip && record.result_code == 1 {
                failures += 1;
            }
        }
        Ok(failures)
    }

    fn login_log_prune(&self, before: i64) -> Result<usize, Error> {
        let mut removed = 0;
        for entry in self.database.login_log.range(..log_key(before, 0)) {
            let (key, _) = entry?;
            if self.database.login_log.remove(key)?.is_some() {
                removed += 1;
            }
        }
        Ok(removed)
    }

    fn login_log_entries(&self) -> Result<Vec<LoginLogRecord>, Error> {
        self.database
            .login_log
            .iter()
            .map(|entry| Ok(serde_json::from_slice(&entry?.1)?))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository() -> SledRepository {
        SledRepository::temporary().unwrap()
    }

    #[test]
    fn creates_finds_and_updates_accounts_by_name_and_id() {
        let repository = repository();
        let id = repository.account_create(AccountRecord { sex: "F".into(), group_id: 5, ..AccountRecord::new(0, "alice", "secret") }).unwrap();
        assert!(id >= 2_000_000);
        let by_name = repository.account_by_name("alice").unwrap().unwrap();
        assert_eq!((by_name.account_id, by_name.sex.as_str(), by_name.group_id), (id, "F", 5));
        assert_eq!(repository.account_by_id(id).unwrap(), Some(by_name));
        assert!(repository.account_by_name("bob").unwrap().is_none());
        assert!(repository.account_create(AccountRecord::new(0, "alice", "other")).is_err());
        let updated = repository.account_update(id, &|account| account.login_count += 1).unwrap();
        assert_eq!(updated.login_count, 1);
        assert!(repository.account_update(id + 99, &|_| {}).is_err());
    }

    #[test]
    fn ip_bans_cover_exact_addresses_and_wildcard_ranges_until_release() {
        let repository = repository();
        let ban = |list: &str, release| IpBanRecord { list: list.into(), begin: 0, release, reason: "test".into() };
        repository.ip_ban_add(ban("10.1.2.*", 100)).unwrap();
        repository.ip_ban_add(ban("192.168.0.5", 100)).unwrap();
        let in_range: Ipv4Addr = "10.1.2.200".parse().unwrap();
        assert!(repository.ip_ban_active(in_range, 99).unwrap());
        assert!(!repository.ip_ban_active(in_range, 100).unwrap());
        assert!(!repository.ip_ban_active("10.1.3.200".parse().unwrap(), 99).unwrap());
        assert!(repository.ip_ban_active("192.168.0.5".parse().unwrap(), 50).unwrap());
        assert!(!repository.ip_ban_active("192.168.0.6".parse().unwrap(), 50).unwrap());
        repository.ip_ban_add(ban("10.1.2.*", 50)).unwrap();
        assert!(repository.ip_ban_active(in_range, 99).unwrap(), "a shorter ban never shortens an existing one");
        assert_eq!(repository.ip_ban_cleanup(100).unwrap(), 2);
        assert!(!repository.ip_ban_active(in_range, 0).unwrap());
    }

    #[test]
    fn ip_bans_can_be_removed_early() {
        let repository = repository();
        repository.ip_ban_add(IpBanRecord { list: "1.2.3.4".into(), begin: 0, release: i64::MAX, reason: String::new() }).unwrap();
        assert!(repository.ip_ban_remove("1.2.3.4").unwrap());
        assert!(!repository.ip_ban_remove("1.2.3.4").unwrap());
        assert!(!repository.ip_ban_active("1.2.3.4".parse().unwrap(), 0).unwrap());
    }

    #[test]
    fn login_log_counts_only_recent_wrong_password_attempts_from_the_same_ip() {
        let repository = repository();
        let entry = |time, ip: &str, code| LoginLogRecord { time, ip: ip.into(), username: "u".into(), result_code: code, message: String::new() };
        for record in [entry(10, "1.1.1.1", 1), entry(20, "1.1.1.1", 1), entry(21, "1.1.1.1", 0), entry(22, "2.2.2.2", 1), entry(23, "1.1.1.1", 1)] {
            repository.login_log_append(record).unwrap();
        }
        assert_eq!(repository.login_log_failed_attempts("1.1.1.1", 0).unwrap(), 3);
        assert_eq!(repository.login_log_failed_attempts("1.1.1.1", 15).unwrap(), 2);
        assert_eq!(repository.login_log_failed_attempts("2.2.2.2", 0).unwrap(), 1);
        assert_eq!(repository.login_log_prune(21).unwrap(), 2);
        assert_eq!(repository.login_log_entries().unwrap().len(), 3);
    }

    #[test]
    fn same_second_log_entries_do_not_overwrite_each_other() {
        let repository = repository();
        for _ in 0..3 {
            repository
                .login_log_append(LoginLogRecord { time: 5, ip: "1.1.1.1".into(), username: "u".into(), result_code: 1, message: String::new() })
                .unwrap();
        }
        assert_eq!(repository.login_log_entries().unwrap().len(), 3);
    }
}
