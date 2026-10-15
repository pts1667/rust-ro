use std::net::{Ipv4Addr, ToSocketAddrs};
use std::sync::Mutex;
use std::sync::atomic::{AtomicI64, Ordering};

use configuration::account_config::LoginConfig;
use database::model::{AccountRecord, IpBanRecord, LoginLogRecord};

use crate::repository::LoginRepository;

pub const REFUSE_UNREGISTERED_ID: u32 = 0;
pub const REFUSE_INCORRECT_PASSWORD: u32 = 1;
pub const REFUSE_EXPIRED: u32 = 2;
pub const REFUSE_REJECTED: u32 = 3;
pub const REFUSE_BANNED_UNTIL: u32 = 6;

const LOGIN_LOG_RETENTION_SECONDS: i64 = 90 * 24 * 3600;

pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub client_ip: Ipv4Addr,
    pub now_ms: i64,
}

impl LoginRequest<'_> {
    fn now(&self) -> i64 {
        self.now_ms / 1000
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginOutcome {
    Accepted(AccountRecord),
    /// `AC_REFUSE_LOGIN` with the rathena error code.
    Refused { code: u32, unblock_time: Option<i64> },
    /// `SC_NOTIFY_BAN` result 1.
    ServerClosed,
}

#[derive(Default)]
struct RegistrationLimiter {
    registrations: u32,
    window_end_ms: i64,
    started: bool,
}

#[derive(Default)]
pub struct LoginService {
    registrations: Mutex<RegistrationLimiter>,
    last_cleanup_ms: AtomicI64,
}

pub fn hash_password(config: &LoginConfig, password: &str) -> String {
    if config.use_md5_passwords {
        format!("{:x}", md5::compute(password.as_bytes()))
    } else {
        password.to_string()
    }
}

/// `name_M` / `name_F`: the account name without the suffix, and the sex it names.
fn split_sex_suffix(username: &str) -> Option<(&str, char)> {
    let bytes = username.as_bytes();
    if bytes.len() <= 2 || bytes[bytes.len() - 2] != b'_' {
        return None;
    }
    let sex = (bytes[bytes.len() - 1] as char).to_ascii_uppercase();
    matches!(sex, 'M' | 'F').then(|| (&username[..username.len() - 2], sex))
}

/// `name_M` / `name_F` in the user id field registers an account when `new_account` is on.
pub fn registration_request(config: &LoginConfig, username: &str, password: &str) -> Option<(String, char)> {
    if !config.new_account || password.is_empty() {
        return None;
    }
    split_sex_suffix(username).map(|(name, sex)| (name.to_string(), sex))
}

/// Why a registration was refused; each caller maps it to its own result code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterError {
    Flood,
    NameLength,
    PasswordLength,
    NameTaken,
    Storage,
}

/// Result codes of `AC_CREATE_ACCOUNT_RESULT`, `0` being success.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateAccountError {
    Disabled = 1,
    InvalidName = 2,
    InvalidPassword = 3,
    InvalidSex = 4,
    NameTaken = 5,
    Limited = 6,
    Failed = 7,
    Banned = 8,
}

/// Names a client may create: letters, digits and `_`, and no `name_M` / `name_F` suffix that the login path would read as a sex.
fn valid_account_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && split_sex_suffix(name).is_none()
}

fn random_password() -> String {
    use rand::{Rng, distributions::Alphanumeric};
    rand::thread_rng().sample_iter(Alphanumeric).take(24).map(char::from).collect()
}

fn registration_refusal(error: RegisterError) -> u32 {
    match error {
        RegisterError::Flood => REFUSE_REJECTED,
        RegisterError::Storage => REFUSE_UNREGISTERED_ID,
        RegisterError::NameLength | RegisterError::PasswordLength | RegisterError::NameTaken => REFUSE_INCORRECT_PASSWORD,
    }
}

fn dnsbl_listed(ip: Ipv4Addr, servers: &[String]) -> bool {
    let [a, b, c, d] = ip.octets();
    servers
        .iter()
        .map(|server| server.trim())
        .filter(|server| !server.is_empty())
        .any(|server| format!("{d}.{c}.{b}.{a}.{server}:0").to_socket_addrs().is_ok_and(|mut addresses| addresses.next().is_some()))
}

impl LoginService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authenticate(&self, repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest) -> LoginOutcome {
        self.cleanup(repository, config, request);
        if config.ipban_enable && repository.ip_ban_active(request.client_ip, request.now()).unwrap_or(true) {
            Self::log(repository, config, request, request.username, -3, "ip banned");
            return LoginOutcome::Refused { code: REFUSE_REJECTED, unblock_time: None };
        }
        if config.skips_account_system(request.client_ip) {
            return match Self::skip_account_system(repository, config, request) {
                Ok(account) => LoginOutcome::Accepted(account),
                Err(code) => {
                    Self::log(repository, config, request, request.username, code as i32, refusal_message(code));
                    LoginOutcome::Refused { code, unblock_time: None }
                }
            };
        }
        match self.authenticate_account(repository, config, request) {
            Ok(account) => Self::admit(config, account),
            Err(code) => {
                Self::log(repository, config, request, request.username, code as i32, refusal_message(code));
                if matches!(code, REFUSE_UNREGISTERED_ID | REFUSE_INCORRECT_PASSWORD) && config.ipban_dynamic_pass_failure_ban {
                    Self::ban_repeated_failures(repository, config, request);
                }
                let unblock_time = (code == REFUSE_BANNED_UNTIL)
                    .then(|| {
                        let name = registration_request(config, request.username, request.password)
                            .map_or_else(|| request.username.to_string(), |(name, _)| name);
                        repository.account_by_name(&name).ok().flatten().map(|account| account.unban_time)
                    })
                    .flatten();
                LoginOutcome::Refused { code, unblock_time }
            }
        }
    }

    pub fn log_accepted(repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest, account: &AccountRecord) {
        Self::log(repository, config, request, &account.username, 100, "login ok");
    }

    pub fn log_event(repository: &dyn LoginRepository, config: &LoginConfig, ip: &str, username: &str, code: i32, message: &str, now: i64) {
        if !config.log_login {
            return;
        }
        if let Err(error) = repository.login_log_append(LoginLogRecord {
            time: now,
            ip: ip.to_string(),
            username: username.to_string(),
            result_code: code,
            message: message.to_string(),
        }) {
            warn!("Failed to write the login log: {error}");
        }
    }

    fn log(repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest, username: &str, code: i32, message: &str) {
        Self::log_event(repository, config, &request.client_ip.to_string(), username, code, message, request.now());
    }

    fn cleanup(&self, repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest) {
        let interval_ms = i64::from(config.ipban_cleanup_interval) * 1000;
        let last = self.last_cleanup_ms.load(Ordering::Relaxed);
        if request.now_ms - last < interval_ms.max(1000) || self.last_cleanup_ms.compare_exchange(last, request.now_ms, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return;
        }
        if let Err(error) = repository.ip_ban_cleanup(request.now()) {
            warn!("Failed to remove expired IP bans: {error}");
        }
        if let Err(error) = repository.login_log_prune(request.now() - LOGIN_LOG_RETENTION_SECONDS) {
            warn!("Failed to prune the login log: {error}");
        }
    }

    fn ban_repeated_failures(repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest) {
        if !config.ipban_enable {
            return;
        }
        let since = request.now() - i64::from(config.ipban_dynamic_pass_failure_ban_interval) * 60;
        let failures = repository.login_log_failed_attempts(&request.client_ip.to_string(), since).unwrap_or(0);
        if failures < config.ipban_dynamic_pass_failure_ban_limit {
            return;
        }
        let [a, b, c, _] = request.client_ip.octets();
        let ban = IpBanRecord {
            list: format!("{a}.{b}.{c}.*"),
            begin: request.now(),
            release: request.now() + i64::from(config.ipban_dynamic_pass_failure_ban_duration) * 60,
            reason: "Password error ban".into(),
        };
        if let Err(error) = repository.ip_ban_add(ban) {
            warn!("Failed to record the password error ban: {error}");
        }
    }

    fn admit(config: &LoginConfig, account: AccountRecord) -> LoginOutcome {
        if config.group_id_to_connect >= 0 && i64::from(account.group_id) != i64::from(config.group_id_to_connect) {
            return LoginOutcome::ServerClosed;
        }
        if config.min_group_id_to_connect >= 0
            && config.group_id_to_connect == -1
            && i64::from(account.group_id) < i64::from(config.min_group_id_to_connect)
        {
            return LoginOutcome::ServerClosed;
        }
        LoginOutcome::Accepted(account)
    }

    fn authenticate_account(&self, repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest) -> Result<AccountRecord, u32> {
        if config.use_dnsbl && dnsbl_listed(request.client_ip, &config.dnsbl_servers) {
            return Err(REFUSE_REJECTED);
        }
        let mut username = request.username;
        let registered_name;
        if let Some((name, sex)) = registration_request(config, request.username, request.password) {
            self.register(repository, config, request, &name, sex).map_err(registration_refusal)?;
            registered_name = name;
            username = &registered_name;
        }
        let account = repository.account_by_name(username).map_err(|error| {
            warn!("Account lookup failed: {error}");
            REFUSE_REJECTED
        })?;
        let Some(account) = account else {
            return Err(REFUSE_UNREGISTERED_ID);
        };
        if account.sex == "S" {
            return Err(REFUSE_UNREGISTERED_ID);
        }
        if account.password != hash_password(config, request.password) {
            return Err(REFUSE_INCORRECT_PASSWORD);
        }
        let now = request.now();
        if account.expiration_time != 0 && account.expiration_time < now {
            return Err(REFUSE_EXPIRED);
        }
        if account.unban_time != 0 && account.unban_time > now {
            return Err(REFUSE_BANNED_UNTIL);
        }
        if account.state != 0 {
            return Err(account.state - 1);
        }
        Self::record_login(repository, account.account_id, request)
    }

    /// Logins from `skip_account_ips` need no account: the password, bans, expiry and group gates are not checked. An unknown
    /// name gets an account with a random password, so only the skipped addresses can use it.
    fn skip_account_system(repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest) -> Result<AccountRecord, u32> {
        let (name, sex) = split_sex_suffix(request.username).unwrap_or((request.username, 'M'));
        if name.is_empty() {
            return Err(REFUSE_UNREGISTERED_ID);
        }
        let account_id = match repository.account_by_name(name).map_err(|error| {
            warn!("Account lookup failed: {error}");
            REFUSE_REJECTED
        })? {
            Some(account) if account.sex == "S" => return Err(REFUSE_UNREGISTERED_ID),
            Some(account) => account.account_id,
            None => {
                let template = AccountRecord {
                    sex: sex.to_string(),
                    last_ip: request.client_ip.to_string(),
                    ..AccountRecord::new(0, name, hash_password(config, &random_password()))
                };
                match repository.account_create(template) {
                    Ok(account_id) => account_id,
                    Err(error) => repository.account_by_name(name).ok().flatten().map(|account| account.account_id).ok_or_else(|| {
                        warn!("Account creation for a skipped login failed: {error}");
                        REFUSE_REJECTED
                    })?,
                }
            }
        };
        Self::record_login(repository, account_id, request)
    }

    fn record_login(repository: &dyn LoginRepository, account_id: u32, request: &LoginRequest) -> Result<AccountRecord, u32> {
        let now = request.now();
        repository
            .account_update(account_id, &|stored| {
                stored.last_ip = request.client_ip.to_string();
                stored.last_login = now;
                stored.unban_time = 0;
                stored.login_count = stored.login_count.saturating_add(1);
            })
            .map_err(|error| {
                warn!("Failed to record the login of account {account_id}: {error}");
                REFUSE_REJECTED
            })
    }

    /// `CA_CREATE_ACCOUNT`: registers an account without logging in. Uses the same switch, name and password rules and flood
    /// limit as the registration in the login id.
    pub fn create_account(&self, repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest, sex: char) -> Result<u32, CreateAccountError> {
        if !config.new_account {
            return Err(CreateAccountError::Disabled);
        }
        if config.ipban_enable && repository.ip_ban_active(request.client_ip, request.now()).unwrap_or(true) {
            return Err(CreateAccountError::Banned);
        }
        if !matches!(sex, 'M' | 'F') {
            return Err(CreateAccountError::InvalidSex);
        }
        if !valid_account_name(request.username) {
            return Err(CreateAccountError::InvalidName);
        }
        if request.password.is_empty() {
            return Err(CreateAccountError::InvalidPassword);
        }
        self.register(repository, config, request, request.username, sex).map_err(|error| match error {
            RegisterError::Flood => CreateAccountError::Limited,
            RegisterError::NameLength => CreateAccountError::InvalidName,
            RegisterError::PasswordLength => CreateAccountError::InvalidPassword,
            RegisterError::NameTaken => CreateAccountError::NameTaken,
            RegisterError::Storage => CreateAccountError::Failed,
        })
    }

    fn register(&self, repository: &dyn LoginRepository, config: &LoginConfig, request: &LoginRequest, name: &str, sex: char) -> Result<u32, RegisterError> {
        {
            let mut limiter = self.registrations.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if !limiter.started {
                limiter.started = true;
                limiter.window_end_ms = request.now_ms;
            }
            if request.now_ms < limiter.window_end_ms && limiter.registrations >= config.allowed_regs {
                warn!("Account registration denied (registration limit exceeded)");
                return Err(RegisterError::Flood);
            }
        }
        if name.len() < config.acc_name_min_length {
            return Err(RegisterError::NameLength);
        }
        if request.password.len() < config.password_min_length {
            return Err(RegisterError::PasswordLength);
        }
        if repository.account_by_name(name).map_err(|_| RegisterError::Storage)?.is_some() {
            info!("Attempt of creation of an already existent account (account: {name}, sex: {sex})");
            return Err(RegisterError::NameTaken);
        }
        let now = request.now();
        let account = AccountRecord {
            sex: sex.to_string(),
            expiration_time: if config.start_limited_time != -1 { now + config.start_limited_time } else { 0 },
            last_ip: request.client_ip.to_string(),
            ..AccountRecord::new(0, name, hash_password(config, request.password))
        };
        let account_id = repository.account_create(account).map_err(|error| {
            warn!("Account creation failed: {error}");
            RegisterError::Storage
        })?;
        info!("Account creation (account {name}, id: {account_id}, sex: {sex})");
        let mut limiter = self.registrations.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if request.now_ms > limiter.window_end_ms {
            limiter.registrations = 0;
            limiter.window_end_ms = request.now_ms + i64::from(config.time_allowed) * 1000;
        }
        limiter.registrations += 1;
        Ok(account_id)
    }
}

pub fn refusal_message(code: u32) -> &'static str {
    match code {
        0 => "Unregistered ID.",
        1 => "Incorrect Password.",
        2 => "Account Expired.",
        3 => "Rejected from server.",
        4 => "Blocked by GM.",
        5 => "Not latest game EXE.",
        6 => "Banned.",
        7 => "Server over-population.",
        8 => "Account limit from company",
        9 => "Ban by DBA",
        10 => "Email not confirmed",
        11 => "Ban by GM",
        12 => "Working in DB",
        13 => "Self Lock",
        14 | 15 => "Not Permitted Group",
        _ => "Unknown Error.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::SledRepository;

    fn config() -> LoginConfig {
        LoginConfig::default()
    }

    fn request<'a>(username: &'a str, password: &'a str, now: i64) -> LoginRequest<'a> {
        LoginRequest { username, password, client_ip: "10.0.0.7".parse().unwrap(), now_ms: now * 1000 }
    }

    fn repository_with(account: AccountRecord) -> SledRepository {
        let repository = SledRepository::temporary().unwrap();
        repository.account_create(account).unwrap();
        repository
    }

    fn refused(outcome: LoginOutcome) -> u32 {
        match outcome {
            LoginOutcome::Refused { code, .. } => code,
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[test]
    fn accepts_a_valid_login_and_records_it_on_the_account() {
        let repository = repository_with(AccountRecord { sex: "F".into(), group_id: 3, ..AccountRecord::new(0, "alice", "secret") });
        let outcome = LoginService::new().authenticate(&repository, &config(), &request("alice", "secret", 1000));
        let LoginOutcome::Accepted(account) = outcome else { panic!("login refused") };
        assert_eq!((account.sex.as_str(), account.group_id, account.login_count), ("F", 3, 1));
        let stored = repository.account_by_name("alice").unwrap().unwrap();
        assert_eq!((stored.last_ip.as_str(), stored.last_login, stored.login_count), ("10.0.0.7", 1000, 1));
    }

    #[test]
    fn refuses_unknown_ids_wrong_passwords_and_server_accounts() {
        let repository = SledRepository::temporary().unwrap();
        repository.account_create(AccountRecord::new(0, "alice", "secret")).unwrap();
        repository.account_create(AccountRecord { sex: "S".into(), ..AccountRecord::new(0, "s1", "p1") }).unwrap();
        let service = LoginService::new();
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("nobody", "x", 1))), REFUSE_UNREGISTERED_ID);
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("alice", "wrong", 1))), REFUSE_INCORRECT_PASSWORD);
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("s1", "p1", 1))), REFUSE_UNREGISTERED_ID);
    }

    #[test]
    fn expired_banned_and_blocked_accounts_are_refused_with_rathena_codes() {
        let repository = SledRepository::temporary().unwrap();
        repository.account_create(AccountRecord { expiration_time: 500, ..AccountRecord::new(0, "old", "secret") }).unwrap();
        repository.account_create(AccountRecord { unban_time: 5000, ..AccountRecord::new(0, "banned", "secret") }).unwrap();
        repository.account_create(AccountRecord { unban_time: 900, ..AccountRecord::new(0, "released", "secret") }).unwrap();
        repository.account_create(AccountRecord { state: 5, ..AccountRecord::new(0, "blocked", "secret") }).unwrap();
        let service = LoginService::new();
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("old", "secret", 1000))), REFUSE_EXPIRED);
        assert_eq!(
            service.authenticate(&repository, &config(), &request("banned", "secret", 1000)),
            LoginOutcome::Refused { code: REFUSE_BANNED_UNTIL, unblock_time: Some(5000) }
        );
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("blocked", "secret", 1000))), 4);
        let released = service.authenticate(&repository, &config(), &request("released", "secret", 1000));
        assert!(matches!(released, LoginOutcome::Accepted(_)));
        assert_eq!(repository.account_by_name("released").unwrap().unwrap().unban_time, 0, "an elapsed ban is cleared on login");
    }

    #[test]
    fn group_gating_closes_the_server_to_other_groups() {
        let repository = SledRepository::temporary().unwrap();
        repository.account_create(AccountRecord { group_id: 0, ..AccountRecord::new(0, "player", "secret") }).unwrap();
        repository.account_create(AccountRecord { group_id: 99, ..AccountRecord::new(0, "admin", "secret") }).unwrap();
        let service = LoginService::new();
        let minimum = LoginConfig { min_group_id_to_connect: 10, ..config() };
        assert_eq!(service.authenticate(&repository, &minimum, &request("player", "secret", 1)), LoginOutcome::ServerClosed);
        assert!(matches!(service.authenticate(&repository, &minimum, &request("admin", "secret", 1)), LoginOutcome::Accepted(_)));
        let exact = LoginConfig { group_id_to_connect: 0, min_group_id_to_connect: 50, ..config() };
        assert!(matches!(service.authenticate(&repository, &exact, &request("player", "secret", 1)), LoginOutcome::Accepted(_)));
        assert_eq!(service.authenticate(&repository, &exact, &request("admin", "secret", 1)), LoginOutcome::ServerClosed);
    }

    #[test]
    fn registers_accounts_with_the_sex_suffix_when_enabled() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("newbie_f", "secret", 1))), REFUSE_UNREGISTERED_ID);
        let enabled = LoginConfig { new_account: true, start_limited_time: 3600, ..config() };
        let LoginOutcome::Accepted(account) = service.authenticate(&repository, &enabled, &request("newbie_f", "secret", 1000)) else {
            panic!("registration failed")
        };
        assert_eq!((account.username.as_str(), account.sex.as_str(), account.expiration_time), ("newbie", "F", 4600));
        assert!(matches!(service.authenticate(&repository, &enabled, &request("newbie", "secret", 1001)), LoginOutcome::Accepted(_)));
        assert_eq!(refused(service.authenticate(&repository, &enabled, &request("newbie_m", "secret", 1002))), REFUSE_INCORRECT_PASSWORD);
    }

    #[test]
    fn registration_enforces_the_minimum_name_and_password_lengths() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        let enabled = LoginConfig { new_account: true, ..config() };
        assert_eq!(refused(service.authenticate(&repository, &enabled, &request("abc_m", "secret", 1))), REFUSE_INCORRECT_PASSWORD);
        assert_eq!(refused(service.authenticate(&repository, &enabled, &request("longname_m", "abc", 1))), REFUSE_INCORRECT_PASSWORD);
        assert!(repository.account_by_name("abc").unwrap().is_none());
        assert!(repository.account_by_name("longname").unwrap().is_none());
    }

    #[test]
    fn registration_flood_limit_follows_the_rathena_window() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        let enabled = LoginConfig { new_account: true, allowed_regs: 1, time_allowed: 10, ..config() };
        assert!(matches!(service.authenticate(&repository, &enabled, &request("first_m", "secret", 100)), LoginOutcome::Accepted(_)));
        assert!(
            matches!(service.authenticate(&repository, &enabled, &request("second_m", "secret", 101)), LoginOutcome::Accepted(_)),
            "like rathena, the limit window only opens once the second account is created"
        );
        assert_eq!(refused(service.authenticate(&repository, &enabled, &request("third_m", "secret", 102))), REFUSE_REJECTED);
        assert!(matches!(service.authenticate(&repository, &enabled, &request("third_m", "secret", 112)), LoginOutcome::Accepted(_)));
    }

    #[test]
    fn md5_mode_stores_and_compares_hashed_passwords() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        let md5 = LoginConfig { new_account: true, use_md5_passwords: true, ..config() };
        assert!(matches!(service.authenticate(&repository, &md5, &request("hashed_m", "secret", 1)), LoginOutcome::Accepted(_)));
        let stored = repository.account_by_name("hashed").unwrap().unwrap();
        assert_eq!(stored.password, "5ebe2294ecd0e0f08eab7690d2a6ee69");
        assert!(matches!(service.authenticate(&repository, &md5, &request("hashed", "secret", 20)), LoginOutcome::Accepted(_)));
        assert_eq!(refused(service.authenticate(&repository, &md5, &request("hashed", "5ebe2294ecd0e0f08eab7690d2a6ee69", 21))), REFUSE_INCORRECT_PASSWORD);
    }

    #[test]
    fn repeated_wrong_passwords_ban_the_subnet_for_the_configured_duration() {
        let repository = repository_with(AccountRecord::new(0, "alice", "secret"));
        let service = LoginService::new();
        let strict = LoginConfig { ipban_dynamic_pass_failure_ban_limit: 3, ..config() };
        for second in 0..3 {
            assert_eq!(refused(service.authenticate(&repository, &strict, &request("alice", "bad", 1000 + second))), REFUSE_INCORRECT_PASSWORD);
        }
        assert!(repository.ip_ban_active("10.0.0.99".parse().unwrap(), 1010).unwrap(), "the whole /24 is banned");
        assert_eq!(refused(service.authenticate(&repository, &strict, &request("alice", "secret", 1010))), REFUSE_REJECTED);
        let after_ban = service.authenticate(&repository, &strict, &request("alice", "secret", 1000 + 3 + 5 * 60));
        assert!(matches!(after_ban, LoginOutcome::Accepted(_)), "the ban lapses after ipban_dynamic_pass_failure_ban_duration");
    }

    #[test]
    fn unknown_ids_are_logged_but_do_not_count_towards_the_password_ban() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        let strict = LoginConfig { ipban_dynamic_pass_failure_ban_limit: 1, ..config() };
        for second in 0..3 {
            service.authenticate(&repository, &strict, &request("ghost", "x", 1000 + second));
        }
        assert!(!repository.ip_ban_active("10.0.0.7".parse().unwrap(), 1005).unwrap());
        assert!(repository.login_log_entries().unwrap().iter().all(|entry| entry.result_code == 0));
    }

    #[test]
    fn ip_bans_can_be_disabled() {
        let repository = repository_with(AccountRecord::new(0, "alice", "secret"));
        repository
            .ip_ban_add(IpBanRecord { list: "10.0.0.7".into(), begin: 0, release: i64::MAX, reason: String::new() })
            .unwrap();
        let service = LoginService::new();
        assert_eq!(refused(service.authenticate(&repository, &config(), &request("alice", "secret", 1))), REFUSE_REJECTED);
        let off = LoginConfig { ipban_enable: false, ..config() };
        assert!(matches!(service.authenticate(&repository, &off, &request("alice", "secret", 1)), LoginOutcome::Accepted(_)));
    }

    #[test]
    fn login_log_is_written_only_when_enabled() {
        let repository = repository_with(AccountRecord::new(0, "alice", "secret"));
        let service = LoginService::new();
        service.authenticate(&repository, &config(), &request("alice", "bad", 1));
        assert_eq!(repository.login_log_entries().unwrap().len(), 1);
        let silent = LoginConfig { log_login: false, ..config() };
        service.authenticate(&repository, &silent, &request("alice", "bad", 2));
        assert_eq!(repository.login_log_entries().unwrap().len(), 1);
    }

    #[test]
    fn registration_suffix_requires_a_password_and_a_known_sex() {
        let enabled = LoginConfig { new_account: true, ..config() };
        assert_eq!(registration_request(&enabled, "bob_m", "pw"), Some(("bob".into(), 'M')));
        assert_eq!(registration_request(&enabled, "bob_F", "pw"), Some(("bob".into(), 'F')));
        assert_eq!(registration_request(&enabled, "bob_x", "pw"), None);
        assert_eq!(registration_request(&enabled, "bob_m", ""), None);
        assert_eq!(registration_request(&enabled, "_m", "pw"), None);
        assert_eq!(registration_request(&config(), "bob_m", "pw"), None);
    }

    fn request_from<'a>(ip: &str, username: &'a str, password: &'a str, now: i64) -> LoginRequest<'a> {
        LoginRequest { username, password, client_ip: ip.parse().unwrap(), now_ms: now * 1000 }
    }

    fn skipping_10_0_0_7() -> LoginConfig {
        LoginConfig { skip_account_ips: vec!["10.0.0.7".parse().unwrap()], ..config() }
    }

    #[test]
    fn created_accounts_follow_the_registration_rules_and_can_log_in() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        let enabled = LoginConfig { new_account: true, ..config() };
        let id = service.create_account(&repository, &enabled, &request("maker", "secret", 1000), 'F').unwrap();
        let stored = repository.account_by_name("maker").unwrap().unwrap();
        assert_eq!((stored.account_id, stored.sex.as_str()), (id, "F"));
        assert!(matches!(service.authenticate(&repository, &enabled, &request("maker", "secret", 1001)), LoginOutcome::Accepted(_)));
        assert_eq!(service.create_account(&repository, &enabled, &request("maker", "other", 1002), 'M'), Err(CreateAccountError::NameTaken));
    }

    #[test]
    fn account_creation_refuses_disabled_servers_bad_fields_and_banned_addresses() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        assert_eq!(service.create_account(&repository, &config(), &request("maker", "secret", 1), 'M'), Err(CreateAccountError::Disabled));
        let enabled = LoginConfig { new_account: true, ..config() };
        assert_eq!(service.create_account(&repository, &enabled, &request("maker", "secret", 1), 'S'), Err(CreateAccountError::InvalidSex));
        assert_eq!(service.create_account(&repository, &enabled, &request("ab", "secret", 1), 'M'), Err(CreateAccountError::InvalidName));
        assert_eq!(service.create_account(&repository, &enabled, &request("bad name", "secret", 1), 'M'), Err(CreateAccountError::InvalidName));
        assert_eq!(service.create_account(&repository, &enabled, &request("suffix_m", "secret", 1), 'M'), Err(CreateAccountError::InvalidName));
        assert_eq!(service.create_account(&repository, &enabled, &request("maker", "abc", 1), 'M'), Err(CreateAccountError::InvalidPassword));
        assert_eq!(service.create_account(&repository, &enabled, &request("maker", "", 1), 'M'), Err(CreateAccountError::InvalidPassword));
        assert!(repository.account_by_name("maker").unwrap().is_none());
        repository
            .ip_ban_add(IpBanRecord { list: "10.0.0.7".into(), begin: 0, release: i64::MAX, reason: String::new() })
            .unwrap();
        assert_eq!(service.create_account(&repository, &enabled, &request("maker", "secret", 1), 'M'), Err(CreateAccountError::Banned));
    }

    #[test]
    fn skipped_addresses_log_in_without_a_password_and_get_an_account() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        let skip = skipping_10_0_0_7();
        let LoginOutcome::Accepted(first) = service.authenticate(&repository, &skip, &request("bot_f", "anything", 1000)) else {
            panic!("skipped login refused")
        };
        assert_eq!((first.username.as_str(), first.sex.as_str(), first.last_ip.as_str()), ("bot", "F", "10.0.0.7"));
        let LoginOutcome::Accepted(again) = service.authenticate(&repository, &skip, &request("bot", "wrong", 1001)) else {
            panic!("skipped login refused")
        };
        assert_eq!(again.account_id, first.account_id);
        assert_eq!(refused(service.authenticate(&repository, &config(), &request_from("192.168.0.1", "bot", "anything", 1002))), REFUSE_INCORRECT_PASSWORD);
    }

    #[test]
    fn skipped_logins_ignore_bans_expiry_and_group_gates_but_not_server_accounts() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        repository.account_create(AccountRecord { unban_time: 5000, expiration_time: 500, group_id: 99, ..AccountRecord::new(0, "locked", "secret") }).unwrap();
        repository.account_create(AccountRecord { sex: "S".into(), ..AccountRecord::new(0, "server", "p") }).unwrap();
        let gated = LoginConfig { group_id_to_connect: 0, ..skipping_10_0_0_7() };
        assert!(matches!(service.authenticate(&repository, &gated, &request("locked", "x", 1000)), LoginOutcome::Accepted(_)));
        assert_eq!(refused(service.authenticate(&repository, &gated, &request("server", "p", 1000))), REFUSE_UNREGISTERED_ID);
        assert_eq!(refused(service.authenticate(&repository, &gated, &request("", "x", 1000))), REFUSE_UNREGISTERED_ID);
    }

    #[test]
    fn ip_bans_still_apply_to_skipped_addresses() {
        let repository = SledRepository::temporary().unwrap();
        let service = LoginService::new();
        repository
            .ip_ban_add(IpBanRecord { list: "10.0.0.7".into(), begin: 0, release: i64::MAX, reason: String::new() })
            .unwrap();
        assert_eq!(refused(service.authenticate(&repository, &skipping_10_0_0_7(), &request("bot", "x", 1))), REFUSE_REJECTED);
    }
}
