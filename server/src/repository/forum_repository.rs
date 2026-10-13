use database::{DatabaseError, abort, read, tx_read, tx_required, tx_write};
use serde::{Deserialize, Serialize};
use sled::IVec;
use sled::transaction::{ConflictableTransactionError, ConflictableTransactionResult, Transactional, TransactionalTree};

use crate::repository::{Error, SledRepository};

/// Threads, comments and notices draw their identifiers from one counter, so a notice's `seq` is the id of the comment behind it.
const SEQUENCE_KEY: &[u8] = b"forum/sequence";
const THREAD_PREFIX: &[u8] = b"forum/thread/";
const COMMENT_PREFIX: &[u8] = b"forum/comment/";
const NOTICE_PREFIX: &[u8] = b"forum/notice/";
/// The viewing order: `bumped` sequence to thread id, so the last key is the thread on top.
const ORDER_PREFIX: &[u8] = b"forum/order/";

/// Threads kept. Starting a thread or replying to one puts it on top, and the thread at the bottom goes when a new one would pass this.
pub const MAX_THREADS: usize = 300;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumThread {
    pub id: u64,
    pub author: String,
    pub title: String,
    pub body: String,
    pub created_at: i64,
    /// Sequence of the last thing that happened in the thread, its place in the viewing order.
    pub bumped: u64,
    pub comments: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumComment {
    pub id: u64,
    pub thread_id: u64,
    /// The comment this one answers, `None` for a comment on the thread itself.
    pub parent: Option<u64>,
    pub author: String,
    pub body: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeKind {
    /// A comment on a thread of the recipient.
    ThreadReply,
    /// A reply to a comment of the recipient.
    CommentReply,
}

/// Tells the author of a thread or of a comment that someone answered it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumNotice {
    pub seq: u64,
    pub kind: NoticeKind,
    pub thread_id: u64,
    pub comment_id: u64,
    pub parent: Option<u64>,
    pub from: String,
    pub body: String,
    pub created_at: i64,
}

pub struct NewThread {
    pub author: String,
    pub title: String,
    pub body: String,
    pub created_at: i64,
}

pub struct NewComment {
    pub thread_id: u64,
    pub parent: Option<u64>,
    pub author: String,
    pub body: String,
    pub created_at: i64,
}

fn unavailable() -> Error {
    Error::InvalidInput("Forum storage is unavailable".into())
}

fn thread_key(id: u64) -> Vec<u8> {
    [THREAD_PREFIX, &id.to_be_bytes()].concat()
}

fn order_key(bumped: u64) -> Vec<u8> {
    [ORDER_PREFIX, &bumped.to_be_bytes()].concat()
}

fn comments_prefix(thread_id: u64) -> Vec<u8> {
    [COMMENT_PREFIX, &thread_id.to_be_bytes()].concat()
}

fn comment_key(thread_id: u64, id: u64) -> Vec<u8> {
    let mut key = comments_prefix(thread_id);
    key.extend_from_slice(&id.to_be_bytes());
    key
}

/// Names are ASCII and never contain `/`, so one name's notices are never under another's prefix.
fn notices_prefix(name: &str) -> Vec<u8> {
    let mut prefix = NOTICE_PREFIX.to_vec();
    prefix.extend_from_slice(name.to_ascii_lowercase().as_bytes());
    prefix.push(b'/');
    prefix
}

fn notice_key(name: &str, seq: u64) -> Vec<u8> {
    let mut key = notices_prefix(name);
    key.extend_from_slice(&seq.to_be_bytes());
    key
}

fn tx_next_id(systems: &TransactionalTree) -> ConflictableTransactionResult<u64, DatabaseError> {
    let next = tx_read::<u64>(systems, SEQUENCE_KEY)?
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| ConflictableTransactionError::Abort(DatabaseError::new("Forum sequence overflow".into())))?;
    tx_write(systems, SEQUENCE_KEY, &next)?;
    Ok(next)
}

/// Removes the thread at the bottom of the viewing order, with its comments, once the forum holds more than `MAX_THREADS`.
/// Only one thread is added at a time, so one removal keeps the count at the cap. Callers hold the forum write lock.
fn evict_past_capacity(systems: &sled::Tree) -> Result<(), Error> {
    let order: Vec<(IVec, IVec)> = systems.scan_prefix(ORDER_PREFIX).take(MAX_THREADS + 1).collect::<Result<_, _>>()?;
    if order.len() <= MAX_THREADS {
        return Ok(());
    }
    let (oldest_order, thread_bytes) = &order[0];
    let thread_id: u64 = serde_json::from_slice(thread_bytes)?;
    let comments: Vec<IVec> = systems.scan_prefix(comments_prefix(thread_id)).keys().collect::<Result<_, _>>()?;
    Ok((systems,).transaction(|(systems,)| {
        systems.remove(oldest_order.as_ref())?;
        systems.remove(thread_key(thread_id))?;
        for comment in &comments {
            systems.remove(comment.as_ref())?;
        }
        Ok::<_, ConflictableTransactionError<DatabaseError>>(())
    })?)
}

pub trait ForumRepository {
    /// Threads in the viewing order, top first, those with a `bumped` below `before`.
    fn forum_threads(&self, _before: u64, _limit: usize) -> Result<Vec<ForumThread>, Error> {
        Err(unavailable())
    }
    fn forum_thread(&self, _id: u64) -> Result<Option<ForumThread>, Error> {
        Err(unavailable())
    }
    /// The comments of a thread in the order they were written.
    fn forum_comments(&self, _thread_id: u64) -> Result<Vec<ForumComment>, Error> {
        Err(unavailable())
    }
    /// Puts the thread on top, and removes the one at the bottom when the forum passes `MAX_THREADS`.
    fn forum_thread_create(&self, _thread: NewThread) -> Result<ForumThread, Error> {
        Err(unavailable())
    }
    /// Stores the comment, counts it on its thread, puts the thread on top and tells whoever it answers, unless that is its author.
    /// `NotFound` when the thread does not exist, `InvalidInput` when the parent is not in the thread.
    fn forum_comment_create(&self, _comment: NewComment) -> Result<ForumComment, Error> {
        Err(unavailable())
    }
    /// Notices of a name, from the first one after `after`, at most `limit`.
    fn forum_notices(&self, _name: &str, _after: u64, _limit: usize) -> Result<Vec<ForumNotice>, Error> {
        Err(unavailable())
    }
}

impl ForumRepository for SledRepository {
    fn forum_threads(&self, before: u64, limit: usize) -> Result<Vec<ForumThread>, Error> {
        let systems = &self.database.game_systems;
        let mut threads = Vec::with_capacity(limit);
        for entry in systems.range(order_key(0)..order_key(before)).rev() {
            if threads.len() == limit {
                break;
            }
            let (_, id_bytes) = entry?;
            let id: u64 = serde_json::from_slice(&id_bytes)?;
            if let Some(thread) = read::<ForumThread>(systems, &thread_key(id))? {
                threads.push(thread);
            }
        }
        Ok(threads)
    }

    fn forum_thread(&self, id: u64) -> Result<Option<ForumThread>, Error> {
        read(&self.database.game_systems, &thread_key(id))
    }

    fn forum_comments(&self, thread_id: u64) -> Result<Vec<ForumComment>, Error> {
        self.database
            .game_systems
            .scan_prefix(comments_prefix(thread_id))
            .map(|entry| {
                let (_, bytes) = entry?;
                Ok(serde_json::from_slice(&bytes)?)
            })
            .collect()
    }

    fn forum_thread_create(&self, new: NewThread) -> Result<ForumThread, Error> {
        let _writes = self.forum_writes.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let thread = (&self.database.game_systems,).transaction(|(systems,)| {
            let id = tx_next_id(systems)?;
            let thread = ForumThread {
                id,
                author: new.author.clone(),
                title: new.title.clone(),
                body: new.body.clone(),
                created_at: new.created_at,
                bumped: id,
                comments: 0,
            };
            tx_write(systems, &thread_key(id), &thread)?;
            tx_write(systems, &order_key(id), &id)?;
            Ok(thread)
        })?;
        evict_past_capacity(&self.database.game_systems)?;
        Ok(thread)
    }

    fn forum_comment_create(&self, new: NewComment) -> Result<ForumComment, Error> {
        let _writes = self.forum_writes.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        Ok((&self.database.game_systems,).transaction(|(systems,)| {
            let mut thread: ForumThread = tx_required(systems, &thread_key(new.thread_id))?;
            let answered = match new.parent {
                Some(parent) => match tx_read::<ForumComment>(systems, &comment_key(new.thread_id, parent))? {
                    Some(comment) => Some(comment.author),
                    None => return abort("That comment is not in this thread"),
                },
                None => None,
            };
            let id = tx_next_id(systems)?;
            let comment = ForumComment {
                id,
                thread_id: new.thread_id,
                parent: new.parent,
                author: new.author.clone(),
                body: new.body.clone(),
                created_at: new.created_at,
            };
            tx_write(systems, &comment_key(new.thread_id, id), &comment)?;
            systems.remove(order_key(thread.bumped))?;
            thread.bumped = id;
            tx_write(systems, &order_key(id), &new.thread_id)?;
            thread.comments = thread.comments.saturating_add(1);
            tx_write(systems, &thread_key(new.thread_id), &thread)?;
            let (recipient, kind) = match answered {
                Some(author) => (author, NoticeKind::CommentReply),
                None => (thread.author.clone(), NoticeKind::ThreadReply),
            };
            if !recipient.eq_ignore_ascii_case(&comment.author) {
                let notice = ForumNotice {
                    seq: id,
                    kind,
                    thread_id: new.thread_id,
                    comment_id: id,
                    parent: new.parent,
                    from: comment.author.clone(),
                    body: comment.body.clone(),
                    created_at: comment.created_at,
                };
                tx_write(systems, &notice_key(&recipient, id), &notice)?;
            }
            Ok(comment)
        })?)
    }

    fn forum_notices(&self, name: &str, after: u64, limit: usize) -> Result<Vec<ForumNotice>, Error> {
        let prefix = notices_prefix(name);
        let mut notices = Vec::new();
        for entry in self.database.game_systems.range(notice_key(name, after.saturating_add(1))..) {
            let (key, bytes) = entry?;
            if !key.starts_with(&prefix) || notices.len() == limit {
                break;
            }
            notices.push(serde_json::from_slice(&bytes)?);
        }
        Ok(notices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository() -> SledRepository {
        SledRepository::temporary().unwrap()
    }

    fn thread(repository: &SledRepository, author: &str) -> ForumThread {
        repository
            .forum_thread_create(NewThread { author: author.into(), title: "Title".into(), body: "Body".into(), created_at: 1 })
            .unwrap()
    }

    fn comment(repository: &SledRepository, thread_id: u64, parent: Option<u64>, author: &str) -> Result<ForumComment, Error> {
        repository.forum_comment_create(NewComment {
            thread_id,
            parent,
            author: author.into(),
            body: format!("from {author}"),
            created_at: 2,
        })
    }

    fn listed(repository: &SledRepository, before: u64) -> Vec<u64> {
        repository.forum_threads(before, 1000).unwrap().iter().map(|thread| thread.id).collect()
    }

    #[test]
    fn threads_are_listed_newest_first_and_paged_back_with_before() {
        let repository = repository();
        let first = thread(&repository, "Alice");
        let second = thread(&repository, "Bob");
        let third = thread(&repository, "Cara");
        assert_eq!(listed(&repository, u64::MAX), vec![third.id, second.id, first.id]);
        assert_eq!(repository.forum_threads(third.bumped, 1).unwrap()[0].id, second.id);
        assert_eq!(listed(&repository, second.bumped), vec![first.id]);
    }

    #[test]
    fn a_comment_moves_its_thread_to_the_top_and_counts_on_it() {
        let repository = repository();
        let old = thread(&repository, "Alice");
        let newer = thread(&repository, "Bob");
        comment(&repository, old.id, None, "Cara").unwrap();
        assert_eq!(listed(&repository, u64::MAX), vec![old.id, newer.id]);
        let bumped = repository.forum_thread(old.id).unwrap().unwrap();
        assert_eq!((bumped.comments, bumped.bumped > newer.bumped), (1, true));
    }

    #[test]
    fn the_thread_at_the_bottom_is_removed_with_its_comments_past_the_cap() {
        let repository = repository();
        let oldest = thread(&repository, "Alice");
        let kept = comment(&repository, oldest.id, None, "Bob").unwrap();
        let others: Vec<u64> = (0..MAX_THREADS).map(|_| thread(&repository, "Cara").id).collect();
        let order = listed(&repository, u64::MAX);
        assert_eq!(order.len(), MAX_THREADS);
        assert!(!order.contains(&oldest.id));
        assert_eq!(order.first(), others.last());
        assert!(repository.forum_thread(oldest.id).unwrap().is_none());
        assert!(repository.forum_comments(oldest.id).unwrap().is_empty());
        assert!(repository.forum_comment_create(NewComment { thread_id: oldest.id, parent: Some(kept.id), author: "Dan".into(), body: "late".into(), created_at: 3 }).is_err());
    }

    #[test]
    fn a_reply_to_a_comment_tells_the_author_of_that_comment() {
        let repository = repository();
        let post = thread(&repository, "Alice");
        let question = comment(&repository, post.id, None, "Bob").unwrap();
        let answer = comment(&repository, post.id, Some(question.id), "Alice").unwrap();
        let bob = repository.forum_notices("Bob", 0, 10).unwrap();
        assert_eq!(bob.len(), 1);
        assert_eq!(bob[0].kind, NoticeKind::CommentReply);
        assert_eq!(bob[0].parent, Some(question.id));
        assert_eq!(bob[0].comment_id, answer.id);
        let alice = repository.forum_notices("Alice", 0, 10).unwrap();
        assert_eq!(alice.iter().map(|notice| notice.kind).collect::<Vec<_>>(), vec![NoticeKind::ThreadReply]);
    }

    #[test]
    fn a_top_level_comment_tells_the_thread_author_and_not_the_author_themselves() {
        let repository = repository();
        let post = thread(&repository, "Alice");
        let reply = comment(&repository, post.id, None, "Bob").unwrap();
        comment(&repository, post.id, None, "Alice").unwrap();
        let notices = repository.forum_notices("alice", 0, 10).unwrap();
        assert_eq!(notices.len(), 1);
        assert_eq!(notices[0].kind, NoticeKind::ThreadReply);
        assert_eq!(notices[0].comment_id, reply.id);
        assert!(repository.forum_notices("Bob", 0, 10).unwrap().is_empty());
        assert_eq!(repository.forum_thread(post.id).unwrap().unwrap().comments, 2);
    }

    #[test]
    fn a_reply_to_a_comment_of_another_thread_or_to_a_missing_thread_is_refused() {
        let repository = repository();
        let post = thread(&repository, "Alice");
        let other = thread(&repository, "Bob");
        let elsewhere = comment(&repository, other.id, None, "Cara").unwrap();
        assert!(matches!(comment(&repository, post.id, Some(elsewhere.id), "Dan"), Err(Error::InvalidInput(_))));
        assert!(matches!(comment(&repository, 999, None, "Dan"), Err(Error::NotFound)));
    }

    #[test]
    fn notices_are_read_after_a_cursor_and_only_for_their_own_name() {
        let repository = repository();
        let post = thread(&repository, "Alice");
        let first = comment(&repository, post.id, None, "Bob").unwrap();
        let second = comment(&repository, post.id, None, "Cara").unwrap();
        let all: Vec<u64> = repository.forum_notices("ALICE", 0, 10).unwrap().iter().map(|notice| notice.comment_id).collect();
        assert_eq!(all, vec![first.id, second.id]);
        let later: Vec<u64> = repository.forum_notices("alice", first.id, 10).unwrap().iter().map(|notice| notice.comment_id).collect();
        assert_eq!(later, vec![second.id]);
        assert_eq!(repository.forum_notices("alice", 0, 1).unwrap().len(), 1);
        assert!(repository.forum_notices("Ali", 0, 10).unwrap().is_empty());
    }
}
