//! The forum of the bot API: text threads, comments that answer threads or other comments, and a WebSocket that reports the replies
//! to the character it is opened for. It is served under `/forum` on the bot API port, see `SKILL.md`.
//!
//! A player signs in with the account and the character, as in game, and gets a token. The bot API key stands for any character:
//! a request carrying it names the character it acts as. Either way, starting threads and commenting are rate limited per character.
mod http;
mod websocket;

pub use http::router;

use std::collections::{HashMap, VecDeque};
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use configuration::account_config::LoginConfig;
use database::DatabaseError;
use rand::RngCore;
use serde::Serialize;

use crate::repository::Repository;
use crate::repository::forum_repository::{ForumComment, ForumNotice, ForumThread, NewComment, NewThread};
use crate::server::service::login_service::{LoginOutcome, LoginRequest, LoginService, refusal_message, registration_request};

const DEFAULT_PAGE: usize = 25;
const MAX_PAGE: usize = 100;
/// Notices read from the forum at once, the rest follow on the next poll.
const NOTICE_BATCH: usize = 100;
const TITLE_LENGTH: usize = 150;
const THREAD_LENGTH: usize = 10_000;
const COMMENT_LENGTH: usize = 2_000;
const SESSION_SECONDS: i64 = 24 * 60 * 60;
const THREAD_INTERVAL: i64 = 15 * 60;
const COMMENT_INTERVAL: i64 = 60;
/// Comments a character may write within `COMMENT_INTERVAL`.
const COMMENTS_PER_INTERVAL: usize = 5;

/// The character a request writes as or reads the inbox of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Author {
    pub char_id: u32,
    pub name: String,
}

/// What a sign-in hands out: the token for the `Authorization` header and the character it stands for.
#[derive(Debug, Serialize)]
pub struct Issued {
    pub token: String,
    pub character: Author,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Thread,
    Comment,
}

impl Action {
    /// How many times a character may do the action within how many seconds.
    fn allowance(self) -> (usize, i64) {
        match self {
            Action::Thread => (1, THREAD_INTERVAL),
            Action::Comment => (COMMENTS_PER_INTERVAL, COMMENT_INTERVAL),
        }
    }

    fn refusal(self, wait: i64) -> String {
        match self {
            Action::Thread => format!("A character starts a thread every 15 minutes, wait {wait} more seconds"),
            Action::Comment => format!("A character comments {COMMENTS_PER_INTERVAL} times in 60 seconds, wait {wait} more seconds"),
        }
    }
}

/// When each action of a character went through lately, in seconds, the oldest first.
#[derive(Default)]
struct Limits {
    thread: VecDeque<i64>,
    comment: VecDeque<i64>,
}

impl Limits {
    fn slot(&mut self, action: Action) -> &mut VecDeque<i64> {
        match action {
            Action::Thread => &mut self.thread,
            Action::Comment => &mut self.comment,
        }
    }
}

pub struct Forum {
    repository: Arc<dyn Repository>,
    login: LoginService,
    login_config: LoginConfig,
    limits: Mutex<HashMap<u32, Limits>>,
    /// Token to the character it signed in as, and when the token ends.
    sessions: Mutex<HashMap<String, (Author, i64)>>,
}

#[derive(Debug, Serialize)]
pub struct ThreadView {
    #[serde(flatten)]
    pub thread: ForumThread,
    /// Every comment in the order it was written. `parent` links a reply to what it answers, `depth` counts the replies above it.
    pub comments: Vec<CommentView>,
}

#[derive(Debug, Serialize)]
pub struct CommentView {
    #[serde(flatten)]
    pub comment: ForumComment,
    pub depth: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    Invalid,
    Unauthorized,
    /// Seconds to wait before the same action may go through.
    RateLimited(u64),
    Unavailable,
}

#[derive(Debug)]
pub struct ForumError {
    pub kind: ErrorKind,
    pub message: String,
}

impl ForumError {
    fn invalid(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Invalid, message: message.into() }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::NotFound, message: message.into() }
    }

    fn unauthorized(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Unauthorized, message: message.into() }
    }

    fn unavailable(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Unavailable, message: message.into() }
    }
}

impl From<DatabaseError> for ForumError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound => Self::not_found("Not found"),
            DatabaseError::InvalidInput(message) | DatabaseError::Conflict(message) => Self::invalid(message),
            other => {
                error!("The forum storage failed: {other}");
                Self::unavailable("The forum storage failed")
            }
        }
    }
}

impl Forum {
    pub fn new(repository: Arc<dyn Repository>, login_config: LoginConfig) -> Self {
        Self {
            repository,
            login: LoginService::new(),
            login_config,
            limits: Mutex::default(),
            sessions: Mutex::default(),
        }
    }

    /// Signs in as a character of an account, with the password and the checks of the game login.
    pub fn authenticate(&self, account: &str, password: &str, character: &str, client_ip: Ipv4Addr, now: i64) -> Result<Issued, ForumError> {
        let config = &self.login_config;
        // Those names create the account at the game login. The forum signs in to existing ones only.
        if registration_request(config, account, password).is_some() {
            return Err(ForumError::invalid("The forum signs in to an existing account, not to a new one"));
        }
        let request = LoginRequest { username: account, password, client_ip, now_ms: now * 1000 };
        match self.login.authenticate(self.repository.as_ref(), config, &request) {
            LoginOutcome::Accepted(record) => {
                LoginService::log_accepted(self.repository.as_ref(), config, &request, &record);
                let found = self
                    .repository
                    .char_account_characters(record.account_id)?
                    .into_iter()
                    .find(|candidate| candidate.delete_date == 0 && candidate.name.eq_ignore_ascii_case(character))
                    .ok_or_else(|| ForumError::unauthorized("This account has no character of that name"))?;
                let author = Author { char_id: found.char_id as u32, name: found.name };
                Ok(self.issue(author, now))
            }
            LoginOutcome::Refused { code, .. } => Err(ForumError::unauthorized(refusal_message(code))),
            LoginOutcome::ServerClosed => Err(ForumError::unauthorized("The server is closed to this account")),
        }
    }

    /// The character a token stands for, `None` for an unknown or expired token.
    pub fn session(&self, token: &str, now: i64) -> Option<Author> {
        lock(&self.sessions).get(token).filter(|(_, expires_at)| *expires_at > now).map(|(author, _)| author.clone())
    }

    /// The character a request names when it carries the bot API key. Any character of the server will do.
    pub fn act_as(&self, name: &str) -> Result<Author, ForumError> {
        let record = self
            .repository
            .char_find_by_name(name.trim())?
            .ok_or_else(|| ForumError::not_found(format!("There is no character named {}", name.trim())))?;
        Ok(Author { char_id: record.char_id as u32, name: record.name })
    }

    /// Threads in the viewing order, top first. `before` is the `bumped` of the last thread of the previous page.
    pub fn threads(&self, before: Option<u64>, limit: Option<usize>) -> Result<Vec<ForumThread>, ForumError> {
        let limit = limit.unwrap_or(DEFAULT_PAGE).clamp(1, MAX_PAGE);
        Ok(self.repository.forum_threads(before.unwrap_or(u64::MAX), limit)?)
    }

    pub fn thread(&self, id: u64) -> Result<ThreadView, ForumError> {
        let thread = self.repository.forum_thread(id)?.ok_or_else(|| ForumError::not_found(format!("There is no thread {id}")))?;
        let comments = flatten(self.repository.forum_comments(id)?);
        Ok(ThreadView { thread, comments })
    }

    pub fn create_thread(&self, author: &Author, title: &str, body: &str) -> Result<ForumThread, ForumError> {
        let now = now();
        let new = NewThread {
            author: author.name.clone(),
            title: single_line("The title", title, TITLE_LENGTH)?,
            body: multi_line("The body", body, THREAD_LENGTH)?,
            created_at: now,
        };
        self.limited(author.char_id, Action::Thread, now, || Ok(self.repository.forum_thread_create(new)?))
    }

    pub fn comment(&self, author: &Author, thread_id: u64, body: &str, parent: Option<u64>) -> Result<ForumComment, ForumError> {
        let now = now();
        let new = NewComment {
            thread_id,
            parent,
            author: author.name.clone(),
            body: multi_line("The comment", body, COMMENT_LENGTH)?,
            created_at: now,
        };
        self.limited(author.char_id, Action::Comment, now, || match self.repository.forum_comment_create(new) {
            Err(DatabaseError::NotFound) => Err(ForumError::not_found(format!("There is no thread {thread_id}"))),
            result => Ok(result?),
        })
    }

    /// The notices of a character after the sequence `after`, the oldest first.
    pub fn notices(&self, author: &Author, after: u64) -> Result<Vec<ForumNotice>, ForumError> {
        Ok(self.repository.forum_notices(&author.name, after, NOTICE_BATCH)?)
    }

    /// Runs a write once the character may do this action, and gives the place back when the write fails.
    fn limited<T>(&self, char_id: u32, action: Action, now: i64, write: impl FnOnce() -> Result<T, ForumError>) -> Result<T, ForumError> {
        self.reserve(char_id, action, now)?;
        write().inspect_err(|_| self.release(char_id, action, now))
    }

    /// Takes a place in the allowance of the action for the character, or says how long to wait.
    fn reserve(&self, char_id: u32, action: Action, now: i64) -> Result<(), ForumError> {
        let (count, interval) = action.allowance();
        let mut limits = lock(&self.limits);
        let done = limits.entry(char_id).or_default().slot(action);
        done.retain(|at| at + interval > now);
        if done.len() >= count {
            let wait = done.iter().min().map_or(0, |oldest| oldest + interval - now);
            return Err(ForumError { kind: ErrorKind::RateLimited(wait as u64), message: action.refusal(wait) });
        }
        done.push_back(now);
        Ok(())
    }

    fn release(&self, char_id: u32, action: Action, reserved_at: i64) {
        let mut limits = lock(&self.limits);
        let done = limits.entry(char_id).or_default().slot(action);
        if let Some(i) = done.iter().rposition(|at| *at == reserved_at) {
            done.remove(i);
        }
    }

    fn issue(&self, author: Author, now: i64) -> Issued {
        let mut bytes = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut bytes);
        let token: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        let expires_at = now + SESSION_SECONDS;
        let mut sessions = lock(&self.sessions);
        sessions.retain(|_, (_, ends)| *ends > now);
        sessions.insert(token.clone(), (author.clone(), expires_at));
        Issued { token, character: author, expires_at }
    }

}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// A comment's depth follows from its parent, which was written first, so one pass over the comments in order is enough.
fn flatten(comments: Vec<ForumComment>) -> Vec<CommentView> {
    let mut depths: HashMap<u64, u32> = HashMap::with_capacity(comments.len());
    comments
        .into_iter()
        .map(|comment| {
            let depth = comment.parent.and_then(|parent| depths.get(&parent)).map_or(0, |depth| depth + 1);
            depths.insert(comment.id, depth);
            CommentView { comment, depth }
        })
        .collect()
}

/// A title or a line of text: trimmed, no line breaks.
fn single_line(field: &str, text: &str, max: usize) -> Result<String, ForumError> {
    checked(field, text, max, false)
}

fn multi_line(field: &str, text: &str, max: usize) -> Result<String, ForumError> {
    checked(field, text, max, true)
}

/// Plain text only: control characters are refused, line breaks allowed where the text may have several lines.
fn checked(field: &str, text: &str, max: usize, multi_line: bool) -> Result<String, ForumError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(ForumError::invalid(format!("{field} is empty")));
    }
    if text.chars().count() > max {
        return Err(ForumError::invalid(format!("{field} is longer than {max} characters")));
    }
    if text.chars().any(|c| c.is_control() && !(multi_line && matches!(c, '\n' | '\r' | '\t'))) {
        return Err(ForumError::invalid(format!("{field} has a control character")));
    }
    Ok(text.to_string())
}

fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| i64::try_from(since.as_secs()).unwrap_or(i64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::{CharServerRepository, SledRepository};
    use database::CharacterCreation;
    use database::model::{AccountRecord, CharacterRecord};

    fn author(char_id: u32, name: &str) -> Author {
        Author { char_id, name: name.into() }
    }

    fn forum() -> Forum {
        Forum::new(Arc::new(SledRepository::temporary().unwrap()), LoginConfig::default())
    }

    /// Clears the limits between steps of a test that writes as one character.
    fn forget_limits(forum: &Forum) {
        lock(&forum.limits).clear();
    }

    fn account_with_character(repository: &SledRepository, account: &str, password: &str, character: &str) {
        use crate::repository::LoginRepository;
        let account_id = repository.account_create(AccountRecord::new(0, account, password)).unwrap();
        repository
            .char_create(&CharacterCreation {
                character: CharacterRecord { account_id: account_id as i32, name: character.into(), inventory_slots: 100, ..Default::default() },
                items: Vec::new(),
                slot_limit: 12,
                case_sensitive_names: false,
            })
            .unwrap();
    }

    #[test]
    fn names_and_texts_are_plain_and_within_their_limits() {
        assert!(single_line("Title", "Two\nlines", TITLE_LENGTH).is_err());
        assert!(multi_line("Body", "Two\nlines\tand a tab", THREAD_LENGTH).is_ok());
        assert!(multi_line("Body", "nul\0byte", THREAD_LENGTH).is_err());
        assert!(multi_line("Body", "   ", THREAD_LENGTH).is_err());
        assert!(multi_line("Comment", &"x".repeat(COMMENT_LENGTH), COMMENT_LENGTH).is_ok());
        assert!(multi_line("Comment", &"x".repeat(COMMENT_LENGTH + 1), COMMENT_LENGTH).is_err());
    }

    #[test]
    fn a_thread_lists_its_comments_in_order_with_their_depth() {
        let forum = forum();
        let thread = forum.create_thread(&author(1, "Alice"), "Hello", "First post").unwrap();
        let top = forum.comment(&author(2, "Bob"), thread.id, "A comment", None).unwrap();
        let reply = forum.comment(&author(1, "Alice"), thread.id, "A reply", Some(top.id)).unwrap();
        forget_limits(&forum);
        let deeper = forum.comment(&author(2, "Bob"), thread.id, "A reply to the reply", Some(reply.id)).unwrap();
        let view = forum.thread(thread.id).unwrap();
        assert_eq!(view.thread.comments, 3);
        assert_eq!(view.comments.iter().map(|comment| comment.comment.id).collect::<Vec<_>>(), vec![top.id, reply.id, deeper.id]);
        assert_eq!(view.comments.iter().map(|comment| comment.depth).collect::<Vec<_>>(), vec![0, 1, 2]);
    }

    #[test]
    fn a_reply_reaches_the_character_it_answers() {
        let forum = forum();
        let thread = forum.create_thread(&author(1, "Alice"), "Hello", "First post").unwrap();
        let top = forum.comment(&author(2, "Bob"), thread.id, "A comment", None).unwrap();
        forum.comment(&author(3, "Cara"), thread.id, "A reply", Some(top.id)).unwrap();
        let bob = forum.notices(&author(2, "Bob"), 0).unwrap();
        assert_eq!(bob.len(), 1);
        assert_eq!(bob[0].from, "Cara");
        assert_eq!(bob[0].body, "A reply");
    }

    #[test]
    fn a_missing_thread_and_bad_text_are_refused_with_their_kind() {
        let forum = forum();
        assert_eq!(forum.thread(77).unwrap_err().kind, ErrorKind::NotFound);
        assert_eq!(forum.comment(&author(2, "Bob"), 77, "Hi", None).unwrap_err().kind, ErrorKind::NotFound);
        assert_eq!(forum.create_thread(&author(1, "Alice"), "Title", "  ").unwrap_err().kind, ErrorKind::Invalid);
    }

    #[test]
    fn a_character_starts_a_thread_once_in_fifteen_minutes_and_comments_five_times_a_minute() {
        let forum = forum();
        assert!(forum.reserve(1, Action::Thread, 1000).is_ok());
        let refused = forum.reserve(1, Action::Thread, 1000 + 899).unwrap_err();
        assert_eq!(refused.kind, ErrorKind::RateLimited(1));
        assert!(forum.reserve(1, Action::Thread, 1000 + 900).is_ok());
        assert!(forum.reserve(1, Action::Comment, 1000).is_ok(), "the thread limit does not hold back comments");
        for at in 1001..1005 {
            assert!(forum.reserve(1, Action::Comment, at).is_ok());
        }
        assert_eq!(forum.reserve(1, Action::Comment, 1059).unwrap_err().kind, ErrorKind::RateLimited(1));
        assert!(forum.reserve(1, Action::Comment, 1060).is_ok(), "the oldest comment left the minute");
        assert!(forum.reserve(2, Action::Thread, 1000).is_ok(), "another character is not held back");
    }

    #[test]
    fn a_write_that_fails_gives_its_slot_back() {
        let forum = forum();
        let failed = forum.limited(1, Action::Thread, 500, || Err::<(), _>(ForumError::not_found("gone")));
        assert!(failed.is_err());
        assert!(forum.reserve(1, Action::Thread, 501).is_ok());
    }

    #[test]
    fn the_thread_limit_is_per_character_and_writes_as_the_character_named() {
        let forum = forum();
        let alice = author(1, "Alice");
        forum.create_thread(&alice, "First", "Body").unwrap();
        assert_eq!(forum.create_thread(&alice, "Second", "Body").unwrap_err().kind, ErrorKind::RateLimited(900));
        let listed = forum.threads(None, None).unwrap();
        assert_eq!(listed.iter().map(|thread| thread.author.as_str()).collect::<Vec<_>>(), vec!["Alice"]);
    }

    #[test]
    fn a_player_signs_in_with_the_account_password_and_character_as_in_game() {
        let repository = Arc::new(SledRepository::temporary().unwrap());
        account_with_character(&repository, "alice", "secret", "Hero");
        let forum = Forum::new(repository, LoginConfig::default());
        let ip: Ipv4Addr = "10.0.0.7".parse().unwrap();
        let issued = forum.authenticate("alice", "secret", "hero", ip, 1000).unwrap();
        assert_eq!(issued.character.name, "Hero");
        assert_eq!(forum.session(&issued.token, 1001), Some(issued.character.clone()));
        assert_eq!(forum.session(&issued.token, issued.expires_at), None, "a token ends with its time");
        assert_eq!(forum.session("unknown", 1001), None);
        assert_eq!(forum.authenticate("alice", "wrong", "Hero", ip, 1000).unwrap_err().kind, ErrorKind::Unauthorized);
        assert_eq!(forum.authenticate("alice", "secret", "Nobody", ip, 1000).unwrap_err().kind, ErrorKind::Unauthorized);
        assert_eq!(forum.authenticate("alice_M", "secret", "Hero", ip, 1000).unwrap_err().kind, ErrorKind::Unauthorized);
        let registering = Forum::new(forum.repository.clone(), LoginConfig { new_account: true, ..LoginConfig::default() });
        assert_eq!(registering.authenticate("bob_M", "secret", "Hero", ip, 1000).unwrap_err().kind, ErrorKind::Invalid);
        assert!(forum.repository.account_by_name("bob").unwrap().is_none(), "the forum never creates an account");
    }

    #[test]
    fn the_bot_key_acts_as_any_named_character() {
        let repository = Arc::new(SledRepository::temporary().unwrap());
        account_with_character(&repository, "alice", "secret", "Hero");
        let forum = Forum::new(repository, LoginConfig::default());
        assert_eq!(forum.act_as(" hero ").unwrap().name, "Hero");
        assert_eq!(forum.act_as("Nobody").unwrap_err().kind, ErrorKind::NotFound);
    }

    #[test]
    fn the_listing_defaults_to_a_page_and_never_asks_for_more_than_the_maximum() {
        let forum = forum();
        for index in 0..3 {
            forum.create_thread(&author(index, &format!("Name {index}")), &format!("Title {index}"), "Body").unwrap();
        }
        assert_eq!(forum.threads(None, None).unwrap().len(), 3);
        assert_eq!(forum.threads(None, Some(0)).unwrap().len(), 1);
        assert_eq!(forum.threads(Some(u64::MAX), Some(usize::MAX)).unwrap().len(), 3);
    }
}
