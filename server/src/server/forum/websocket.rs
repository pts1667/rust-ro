//! The WebSocket of the forum: the replies to the character it was opened for, as `reply` events. It starts from the `after`
//! sequence given, so the replies that came while the socket was closed are sent first.
use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::mpsc;

use super::{Author, Forum};
use crate::repository::forum_repository::ForumNotice;

const OUTGOING_QUEUE: usize = 256;
const POLL_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Serialize)]
struct ReplyEvent<'a> {
    event: &'static str,
    #[serde(rename = "for")]
    watcher: &'a str,
    #[serde(flatten)]
    notice: &'a ForumNotice,
}

pub async fn serve(socket: WebSocket, forum: Arc<Forum>, author: Author, after: u64) {
    let (mut sink, mut stream) = socket.split();
    let (outgoing, mut queue) = mpsc::channel::<String>(OUTGOING_QUEUE);
    let writer = tokio::spawn(async move {
        while let Some(text) = queue.recv().await {
            if sink.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    });
    let poller = tokio::spawn(poll(forum, author, after, outgoing));
    while let Some(Ok(message)) = stream.next().await {
        if matches!(message, Message::Close(_)) {
            break;
        }
    }
    poller.abort();
    writer.abort();
}

/// Sends the notices of the character, then waits. It ends when the socket is gone.
async fn poll(forum: Arc<Forum>, author: Author, mut after: u64, outgoing: mpsc::Sender<String>) {
    loop {
        let reader = forum.clone();
        let who = author.clone();
        if let Ok(Ok(notices)) = tokio::task::spawn_blocking(move || reader.notices(&who, after)).await {
            for notice in notices {
                let event = ReplyEvent { event: "reply", watcher: &author.name, notice: &notice };
                let Ok(text) = serde_json::to_string(&event) else { continue };
                if outgoing.send(text).await.is_err() {
                    return;
                }
                after = after.max(notice.seq);
            }
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::SledRepository;
    use configuration::account_config::LoginConfig;
    use serde_json::Value;

    fn author(char_id: u32, name: &str) -> Author {
        Author { char_id, name: name.into() }
    }

    #[tokio::test]
    async fn each_reply_is_sent_once_from_the_cursor_it_starts_at() {
        let forum = Arc::new(Forum::new(Arc::new(SledRepository::temporary().unwrap()), LoginConfig::default()));
        let thread = forum.create_thread(&author(1, "Alice"), "Hello", "First post").unwrap();
        forum.comment(&author(2, "Bob"), thread.id, "One", None).unwrap();
        forum.comment(&author(3, "Cara"), thread.id, "Two", None).unwrap();
        let (outgoing, mut queue) = mpsc::channel(16);
        let poller = tokio::spawn(poll(forum.clone(), author(1, "Alice"), 0, outgoing));
        let mut bodies = Vec::new();
        for _ in 0..2 {
            let text = tokio::time::timeout(Duration::from_secs(5), queue.recv()).await.unwrap().unwrap();
            let event: Value = serde_json::from_str(&text).unwrap();
            assert_eq!(event["event"], "reply");
            assert_eq!(event["for"], "Alice");
            bodies.push(event["body"].as_str().unwrap().to_string());
        }
        assert_eq!(bodies, vec!["One", "Two"]);
        assert!(tokio::time::timeout(Duration::from_millis(1200), queue.recv()).await.is_err());
        poller.abort();
    }
}
