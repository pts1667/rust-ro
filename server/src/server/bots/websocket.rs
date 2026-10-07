//! The WebSocket of the bot API: JSON commands in, answers and events out, over one connection for all bots.
//!
//! A command is `{"id": 1, "type": "move", "bot": "Name", ...}` and is answered by `{"id": 1, "ok": true, "result": ...}` or
//! `{"id": 1, "ok": false, "error": "..."}`, whenever the command is done. Commands run side by side, so `stop` can interrupt a walk.
//! Bots that were subscribed to are pushed as `{"event": "observation" | "dialog" | "message", "bot": ...}`.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::sync::mpsc;
use tokio::time::{Instant, sleep};

use super::controller::{Action, BotController, BotError, CreateBot, ErrorKind};

const OUTGOING_QUEUE: usize = 256;
const PUSH_INTERVAL: Duration = Duration::from_millis(100);
const DEFAULT_OBSERVATION_INTERVAL_MS: u64 = 1000;
const OBSERVATION_INTERVAL_MS: std::ops::RangeInclusive<u64> = 200..=10_000;

#[derive(Clone)]
struct Subscription {
    interval: Duration,
    last_observation: Option<Instant>,
    dialog_version: u64,
    last_message: u64,
}

type Subscriptions = Arc<Mutex<HashMap<String, Subscription>>>;
/// Names of the bots this connection put in the game, they leave it with the connection.
type Owned = Arc<Mutex<HashSet<String>>>;

fn invalid(message: impl Into<String>) -> BotError {
    BotError { kind: ErrorKind::Invalid, message: message.into() }
}

pub async fn serve(socket: WebSocket, controller: Arc<BotController>) {
    let (mut sink, mut stream) = socket.split();
    let (outgoing, mut queue) = mpsc::channel::<String>(OUTGOING_QUEUE);
    let writer = tokio::spawn(async move {
        while let Some(text) = queue.recv().await {
            if sink.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    });
    let subscriptions = Subscriptions::default();
    let owned = Owned::default();
    let pusher = tokio::spawn(push_events(controller.clone(), subscriptions.clone(), outgoing.clone()));
    while let Some(Ok(message)) = stream.next().await {
        match message {
            Message::Text(text) => {
                tokio::spawn(answer(controller.clone(), subscriptions.clone(), owned.clone(), outgoing.clone(), text.to_string()));
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
    pusher.abort();
    writer.abort();
    let leaving: Vec<String> = owned.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).drain().collect();
    for name in leaving {
        let _ = controller.disconnect(&name).await;
    }
}

async fn answer(controller: Arc<BotController>, subscriptions: Subscriptions, owned: Owned, outgoing: mpsc::Sender<String>, text: String) {
    let response = match serde_json::from_str::<Value>(&text) {
        Ok(request) => {
            let id = request.get("id").cloned().unwrap_or(Value::Null);
            match dispatch(&controller, &subscriptions, &owned, &request).await {
                Ok(result) => json!({ "id": id, "ok": true, "result": result }),
                Err(error) => json!({ "id": id, "ok": false, "error": error.message }),
            }
        }
        Err(error) => json!({ "id": null, "ok": false, "error": format!("Not a JSON message: {error}") }),
    };
    let _ = outgoing.send(response.to_string()).await;
}

async fn dispatch(controller: &BotController, subscriptions: &Subscriptions, owned: &Owned, request: &Value) -> Result<Value, BotError> {
    let own = |name: &str| owned.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(name.to_lowercase());
    let kind = request.get("type").and_then(Value::as_str).ok_or_else(|| invalid("Every command has a \"type\""))?;
    let bot = || {
        request
            .get("bot")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("This command needs the name of the bot in \"bot\""))
    };
    let number = |field: &str| request.get(field).and_then(Value::as_u64).and_then(|value| u16::try_from(value).ok());
    match kind {
        "list" => Ok(controller.list()),
        "create" => {
            let create: CreateBot = serde_json::from_value(request.clone()).map_err(|error| invalid(format!("Bad bot to create: {error}")))?;
            let (new_bot, connect) = create.into_parts();
            let name = new_bot.name.clone();
            let created = controller.create(new_bot, connect).await?;
            if connect {
                own(&name);
            }
            Ok(created)
        }
        "connect" => {
            let name = bot()?;
            let connected = controller.connect(name).await?;
            own(name);
            Ok(connected)
        }
        "disconnect" => {
            let name = bot()?;
            owned.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).remove(&name.to_lowercase());
            controller.disconnect(name).await
        }
        "delete" => {
            let name = bot()?;
            owned.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).remove(&name.to_lowercase());
            controller.delete(name).await
        }
        "status" => controller.status(bot()?).await,
        "observe" => controller.observe(bot()?).await,
        "map" => {
            let center = number("x").zip(number("y")).map(|(x, y)| (x, y, number("radius").unwrap_or(20)));
            controller.map(bot()?, center).await
        }
        "subscribe" => {
            let name = bot()?;
            let handle = controller.find(name)?;
            let interval_ms = request.get("interval_ms").and_then(Value::as_u64).unwrap_or(DEFAULT_OBSERVATION_INTERVAL_MS);
            let interval = Duration::from_millis(interval_ms.clamp(*OBSERVATION_INTERVAL_MS.start(), *OBSERVATION_INTERVAL_MS.end()));
            let last_message = handle.messages_after(0).last().map_or(0, |message| message.seq);
            subscriptions.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(
                handle.name.to_lowercase(),
                Subscription { interval, last_observation: None, dialog_version: handle.dialog_version(), last_message },
            );
            Ok(json!({ "subscribed": handle.name, "interval_ms": interval.as_millis() as u64 }))
        }
        "unsubscribe" => {
            let name = bot()?.to_lowercase();
            let removed = subscriptions.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).remove(&name).is_some();
            Ok(json!({ "unsubscribed": removed }))
        }
        _ => {
            let action: Action =
                serde_json::from_value(request.clone()).map_err(|error| invalid(format!("Unknown command or bad parameters: {error}")))?;
            controller.act(bot()?, action).await
        }
    }
}

async fn push_events(controller: Arc<BotController>, subscriptions: Subscriptions, outgoing: mpsc::Sender<String>) {
    loop {
        sleep(PUSH_INTERVAL).await;
        let due: Vec<(String, Subscription)> = subscriptions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .map(|(name, subscription)| (name.clone(), subscription.clone()))
            .collect();
        for (name, mut subscription) in due {
            let Ok(bot) = controller.find(&name) else { continue };
            let mut events = Vec::new();
            let version = bot.dialog_version();
            if version != subscription.dialog_version {
                subscription.dialog_version = version;
                events.push(json!({ "event": "dialog", "bot": bot.name, "dialog": bot.dialog() }));
            }
            for message in bot.messages_after(subscription.last_message) {
                subscription.last_message = message.seq;
                events.push(json!({ "event": "message", "bot": bot.name, "message": message }));
            }
            if bot.is_connected() && subscription.last_observation.is_none_or(|at| at.elapsed() >= subscription.interval) {
                subscription.last_observation = Some(Instant::now());
                if let Ok(observation) = controller.observe_unprompted(&name).await {
                    events.push(json!({ "event": "observation", "bot": bot.name, "observation": observation }));
                }
            }
            for event in events {
                if outgoing.send(event.to_string()).await.is_err() {
                    return;
                }
            }
            if let Some(current) = subscriptions.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).get_mut(&name) {
                *current = subscription;
            }
        }
    }
}
