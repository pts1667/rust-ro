//! What the API does with a bot: the steps of an action, and the waiting that makes one call enough for a whole action.
use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::time::{Instant, sleep, timeout};

use super::command::{BotCommand, BotRequest, DialogInput};
use super::dialog::Prompt;
use super::lifecycle::{CreateError, NewBot, StoredBot, create_stored_bot, find_stored_bot, load_bot_character};
use super::registry::BotHandle;
use crate::server::Server;
use crate::server::model::character_lifecycle::SelectedCharacter;
use crate::server::model::events::game_event::GameEvent;

/// The game loop answers within a few ticks, a longer wait means it is stuck or gone.
const COMMAND_TIMEOUT: Duration = Duration::from_secs(5);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
const POLL_INTERVAL: Duration = Duration::from_millis(150);
/// Polls without movement after which a walk counts as stopped; the movement thread starts a walk within a few of its ticks.
const IDLE_POLLS_BEFORE_STOPPED: u32 = 4;
const WARP_POLLS: u32 = 10;
const RECENT_MESSAGES: usize = 10;
/// A fight started without `wait` has no caller to give up on it.
const BACKGROUND_FIGHT_LIMIT: Duration = Duration::from_secs(600);
const DIALOG_WAIT: Duration = Duration::from_secs(3);
const DIALOG_POLL: Duration = Duration::from_millis(40);
/// The packets of a script come one after the other, the dialogue is complete when none came for this long.
const DIALOG_QUIET: Duration = Duration::from_millis(150);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    Invalid,
    Conflict,
    Unavailable,
}

#[derive(Debug, Clone)]
pub struct BotError {
    pub kind: ErrorKind,
    pub message: String,
}

impl BotError {
    fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }

    fn invalid(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Invalid, message)
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Sex {
    #[serde(alias = "M", alias = "male")]
    M,
    #[serde(alias = "F", alias = "female")]
    F,
}

/// The body of a bot creation, over HTTP and over the WebSocket.
#[derive(Debug, Deserialize)]
pub struct CreateBot {
    pub name: String,
    #[serde(default = "default_sex")]
    pub sex: Sex,
    #[serde(default = "default_hair_style")]
    pub hair_style: i32,
    #[serde(default)]
    pub hair_color: i32,
    #[serde(default = "default_true")]
    pub connect: bool,
}

fn default_sex() -> Sex {
    Sex::M
}

fn default_hair_style() -> i32 {
    1
}

impl CreateBot {
    pub fn into_parts(self) -> (NewBot, bool) {
        let new_bot = NewBot { name: self.name, female: self.sex == Sex::F, hair_style: self.hair_style, hair_color: self.hair_color };
        (new_bot, self.connect)
    }
}

/// Everything a bot can be told to do, as the JSON of a request.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// Walks to a cell. With `wait` the call returns once the bot arrived, was stopped or changed map.
    Move {
        x: u16,
        y: u16,
        #[serde(default = "default_true")]
        wait: bool,
    },
    /// Walks to an NPC, a ground item or a warp and uses it: talks, picks up, or walks through.
    Use {
        target: u32,
    },
    /// Attacks a monster, the bot chases it. With `wait` the call returns once the fight is over.
    Attack {
        target: u32,
        #[serde(default)]
        wait: bool,
    },
    Stop,
    Respawn,
    DialogNext,
    DialogChoose {
        option: u8,
    },
    DialogNumber {
        value: i32,
    },
    DialogText {
        text: String,
    },
    DialogClose,
}

struct Probe {
    map: String,
    x: u16,
    y: u16,
    moving: bool,
    dead: bool,
    ready: bool,
}

impl Probe {
    fn parse(status: &Value) -> Option<Self> {
        let me = status.get("self")?;
        Some(Self {
            map: status.pointer("/map/name")?.as_str()?.to_string(),
            x: me.get("x")?.as_u64()? as u16,
            y: me.get("y")?.as_u64()? as u16,
            moving: me.get("moving")?.as_bool()?,
            dead: me.get("dead")?.as_bool()?,
            ready: status.get("ready")?.as_bool()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Journey {
    Arrived,
    MapChanged,
    Stopped,
    Timeout,
}

impl Journey {
    fn name(self) -> &'static str {
        match self {
            Journey::Arrived => "arrived",
            Journey::MapChanged => "map_changed",
            Journey::Stopped => "stopped",
            Journey::Timeout => "timeout",
        }
    }
}

#[derive(Clone)]
pub struct BotController {
    server: Arc<Server>,
}

impl BotController {
    pub fn new(server: Arc<Server>) -> Self {
        Self { server }
    }

    pub(super) fn find(&self, name: &str) -> Result<Arc<BotHandle>, BotError> {
        self.server
            .bots()
            .find(name)
            .ok_or_else(|| BotError::new(ErrorKind::NotFound, format!("No bot named {name}, create it first")))
    }

    fn connected(&self, name: &str) -> Result<Arc<BotHandle>, BotError> {
        let bot = self.find(name)?;
        if bot.is_connected() {
            Ok(bot)
        } else {
            Err(BotError::new(ErrorKind::Conflict, format!("{name} is not connected, connect it first")))
        }
    }

    fn action_timeout(&self) -> Duration {
        Duration::from_secs(self.server.configuration.bots.action_timeout_secs.max(1))
    }

    pub fn summary(bot: &BotHandle) -> Value {
        json!({ "name": bot.name, "char_id": bot.char_id, "connected": bot.is_connected() })
    }

    pub fn list(&self) -> Value {
        Value::Array(self.server.bots().list().iter().map(|bot| Self::summary(bot)).collect())
    }

    /// Sends a request to the game loop and waits for its answer.
    async fn run(&self, bot: &BotHandle, request: BotRequest) -> Result<Value, BotError> {
        let (command, answer) = BotCommand::new(bot.char_id, bot.account_id, request);
        self.server.add_to_next_tick(GameEvent::BotCommand(command));
        match timeout(COMMAND_TIMEOUT, answer).await {
            Ok(Ok(Ok(value))) => Ok(value),
            Ok(Ok(Err(message))) => Err(BotError::invalid(message)),
            Ok(Err(_)) => {
                bot.set_connected(false);
                Err(BotError::new(ErrorKind::Conflict, format!("{} is not in the game", bot.name)))
            }
            Err(_) => Err(BotError::new(ErrorKind::Unavailable, "The game loop did not answer in time")),
        }
    }

    async fn probe(&self, bot: &BotHandle) -> Result<Probe, BotError> {
        let status = self.run(bot, BotRequest::Status).await?;
        Probe::parse(&status).ok_or_else(|| BotError::new(ErrorKind::Unavailable, "Unreadable status of the character"))
    }

    /// Creates the account and character of a new bot, and brings it into the game unless `connect` is false.
    pub async fn create(&self, new_bot: NewBot, connect: bool) -> Result<Value, BotError> {
        let bots = self.server.bots();
        if bots.list().len() >= self.server.configuration.bots.max_bots {
            return Err(BotError::new(ErrorKind::Conflict, "The limit of bots was reached"));
        }
        let server = self.server.clone();
        let stored = tokio::task::spawn_blocking(move || create_stored_bot(&server, &new_bot))
            .await
            .map_err(|error| BotError::new(ErrorKind::Unavailable, error.to_string()))?
            .map_err(|error| match error {
                CreateError::AlreadyExists => BotError::new(ErrorKind::Conflict, "A bot with this name already exists, connect it instead"),
                CreateError::Refused(message) => BotError::invalid(message),
            })?;
        let bot = self.register(&stored);
        if connect {
            self.connect_bot(&bot).await?;
        }
        Ok(Self::summary(&bot))
    }

    fn register(&self, stored: &StoredBot) -> Arc<BotHandle> {
        let (name, char_id) = (&stored.character.name, stored.character.char_id as u32);
        match self.server.bots().find(name) {
            Some(bot) if bot.char_id == char_id => bot,
            _ => self.server.bots().register(BotHandle::new(name.clone(), stored.account_id, char_id)),
        }
    }

    /// The bot of this run, or the one an earlier run created.
    async fn known_or_stored(&self, name: &str) -> Result<Arc<BotHandle>, BotError> {
        if let Ok(bot) = self.find(name) {
            return Ok(bot);
        }
        let server = self.server.clone();
        let owned = name.to_string();
        let stored = tokio::task::spawn_blocking(move || find_stored_bot(&server, &owned))
            .await
            .map_err(|error| BotError::new(ErrorKind::Unavailable, error.to_string()))?
            .map_err(BotError::invalid)?;
        let stored = stored.ok_or_else(|| BotError::new(ErrorKind::NotFound, format!("No bot named {name}, create it first")))?;
        Ok(self.register(&stored))
    }

    pub async fn connect(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.known_or_stored(name).await?;
        self.connect_bot(&bot).await?;
        Ok(Self::summary(&bot))
    }

    async fn connect_bot(&self, bot: &Arc<BotHandle>) -> Result<(), BotError> {
        if bot.is_connected() {
            return Ok(());
        }
        let max = self.server.configuration.bots.max_bots;
        if self.server.bots().connected_count() >= max {
            return Err(BotError::new(ErrorKind::Conflict, format!("At most {max} bots can be connected")));
        }
        let server = self.server.clone();
        let name = bot.name.clone();
        let selected = tokio::task::spawn_blocking(move || {
            let stored = find_stored_bot(&server, &name)?.ok_or("The character of the bot no longer exists")?;
            load_bot_character(&server, &stored).map(SelectedCharacter::from_character)
        })
        .await
        .map_err(|error| BotError::new(ErrorKind::Unavailable, error.to_string()))?
        .map_err(BotError::invalid)?;
        let (command, answer) = BotCommand::new(
            bot.char_id,
            bot.account_id,
            BotRequest::Connect(Arc::new(std::sync::Mutex::new(Some(selected)))),
        );
        self.server.add_to_next_tick(GameEvent::BotCommand(command));
        match timeout(COMMAND_TIMEOUT, answer).await {
            Ok(Ok(Ok(_))) => {}
            Ok(Ok(Err(message))) => return Err(BotError::new(ErrorKind::Conflict, message)),
            _ => return Err(BotError::new(ErrorKind::Unavailable, "The game loop did not answer in time")),
        }
        bot.set_connected(true);
        let deadline = Instant::now() + CONNECT_TIMEOUT;
        while !self.probe(bot).await?.ready {
            if Instant::now() > deadline {
                return Err(BotError::new(ErrorKind::Unavailable, "The character did not finish entering its map"));
            }
            sleep(POLL_INTERVAL).await;
        }
        Ok(())
    }

    pub async fn disconnect(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        if bot.is_connected() {
            // The character may already be gone, which is what was asked for
            let _ = self.run(&bot, BotRequest::Disconnect).await;
            bot.set_connected(false);
        }
        Ok(Self::summary(&bot))
    }

    pub async fn status(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        let mut summary = Self::summary(&bot);
        if bot.is_connected() {
            summary["status"] = self.run(&bot, BotRequest::Status).await?;
        }
        Ok(summary)
    }

    /// Everything the bot sees, plus what the NPC it talks to shows and the latest messages.
    pub async fn observe(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        let mut observation = self.run(&bot, BotRequest::Observe).await?;
        observation["dialog"] = serde_json::to_value(bot.dialog()).unwrap_or(Value::Null);
        observation["messages"] = serde_json::to_value(bot.recent_messages(RECENT_MESSAGES)).unwrap_or(Value::Null);
        Ok(observation)
    }

    pub async fn map(&self, name: &str, center: Option<(u16, u16, u16)>) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        self.run(&bot, BotRequest::Map(center)).await
    }

    pub async fn act(&self, name: &str, action: Action) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        match action {
            Action::Move { x, y, wait } => {
                bot.next_command();
                let mut reply = self.run(&bot, BotRequest::Move { x, y }).await?;
                if wait {
                    reply["journey"] = self.follow_walk(&bot, Some((x, y))).await?.name().into();
                }
                Ok(reply)
            }
            Action::Use { target } => {
                bot.next_command();
                self.use_target(&bot, target).await
            }
            Action::Attack { target, wait } => {
                let epoch = bot.next_command();
                let first = self.run(&bot, BotRequest::Attack { target_id: target }).await?;
                if first["gone"] == json!(true) {
                    return Err(BotError::invalid(format!("No target {target} on this map")));
                }
                if wait {
                    let fight = self.follow_fight(&bot, target, epoch, self.action_timeout()).await?;
                    return Ok(json!({ "action": "attack", "fight": fight }));
                }
                let controller = self.clone();
                tokio::spawn(async move {
                    let _ = controller.follow_fight(&bot, target, epoch, BACKGROUND_FIGHT_LIMIT).await;
                });
                Ok(json!({ "action": "attack" }))
            }
            Action::Stop => {
                bot.next_command();
                self.run(&bot, BotRequest::Stop).await
            }
            Action::Respawn => self.run(&bot, BotRequest::Respawn).await,
            Action::DialogNext => self.answer(&bot, DialogInput::Next).await,
            Action::DialogChoose { option } => self.answer(&bot, DialogInput::Choose(option)).await,
            Action::DialogNumber { value } => self.answer(&bot, DialogInput::Number(value)).await,
            Action::DialogText { text } => self.answer(&bot, DialogInput::Text(text)).await,
            Action::DialogClose => {
                let reply = self.run(&bot, BotRequest::Dialog(DialogInput::Close)).await?;
                bot.clear_dialog();
                Ok(reply)
            }
        }
    }

    /// Follows the bot until it reaches `destination`, stops, or changes map.
    async fn follow_walk(&self, bot: &BotHandle, destination: Option<(u16, u16)>) -> Result<Journey, BotError> {
        let start = self.probe(bot).await?;
        let deadline = Instant::now() + self.action_timeout();
        let mut idle_polls = 0;
        loop {
            sleep(POLL_INTERVAL).await;
            let now = self.probe(bot).await?;
            if now.map != start.map {
                return Ok(Journey::MapChanged);
            }
            if destination == Some((now.x, now.y)) {
                return Ok(Journey::Arrived);
            }
            idle_polls = if now.moving { 0 } else { idle_polls + 1 };
            if idle_polls >= IDLE_POLLS_BEFORE_STOPPED {
                return Ok(Journey::Stopped);
            }
            if Instant::now() > deadline {
                return Ok(Journey::Timeout);
            }
        }
    }

    /// Plays the part of a client for a fight: walks to the monster, attacks, and walks again when it moved away.
    /// Ends with `over` (the monster is gone), `dead`, `stopped` (replaced by another command), `unreachable` or `timeout`.
    async fn follow_fight(&self, bot: &BotHandle, target: u32, epoch: u64, limit: Duration) -> Result<&'static str, BotError> {
        let deadline = Instant::now() + limit;
        loop {
            if !bot.is_current_command(epoch) {
                return Ok("stopped");
            }
            if Instant::now() > deadline {
                return Ok("timeout");
            }
            let now = self.probe(bot).await?;
            if now.dead {
                return Ok("dead");
            }
            if !now.moving {
                match self.run(bot, BotRequest::Attack { target_id: target }).await {
                    Ok(step) if step["gone"] == json!(true) => return Ok("over"),
                    Ok(_) => {}
                    Err(error) if error.kind == ErrorKind::Invalid => return Ok("unreachable"),
                    Err(error) => return Err(error),
                }
            }
            sleep(POLL_INTERVAL).await;
        }
    }

    /// The warp is applied by the game loop shortly after the bot stepped on it.
    async fn await_map_change(&self, bot: &BotHandle, start_map: &str) -> Result<Journey, BotError> {
        for _ in 0..WARP_POLLS {
            if self.probe(bot).await?.map != start_map {
                return Ok(Journey::MapChanged);
            }
            sleep(POLL_INTERVAL).await;
        }
        Ok(Journey::Arrived)
    }

    async fn use_target(&self, bot: &BotHandle, target: u32) -> Result<Value, BotError> {
        let approach = self.run(bot, BotRequest::Approach { target_id: target }).await?;
        let kind = approach["kind"].as_str().unwrap_or_default().to_string();
        let start_map = self.probe(bot).await?.map;
        let mut journey = None;
        if approach["in_range"] == json!(false) {
            let destination = (approach["destination"]["x"].as_u64(), approach["destination"]["y"].as_u64());
            let destination = destination.0.zip(destination.1).map(|(x, y)| (x as u16, y as u16));
            journey = Some(self.follow_walk(bot, destination).await?);
        }
        let mut reply = json!({ "target": target, "kind": kind });
        if let Some(journey) = journey {
            let journey = if kind == "warp" && journey == Journey::Arrived { self.await_map_change(bot, &start_map).await? } else { journey };
            reply["journey"] = journey.name().into();
            if journey != Journey::Arrived {
                return Ok(reply);
            }
        }
        if kind == "warp" {
            // Standing on the warp cell is the use, the walk above ended on the other map
            return Ok(reply);
        }
        bot.clear_dialog();
        let before = bot.dialog_version();
        let used = self.run(bot, BotRequest::Interact { target_id: target }).await?;
        reply["action"] = used["action"].clone();
        if used["action"] == json!("talk") {
            self.wait_for_dialog(bot, before).await;
            reply["dialog"] = serde_json::to_value(bot.dialog()).unwrap_or(Value::Null);
        }
        Ok(reply)
    }

    /// Sends the answer, after checking that the conversation asks for it: a wrong answer would end the script.
    async fn answer(&self, bot: &BotHandle, input: DialogInput) -> Result<Value, BotError> {
        let dialog = bot.dialog().ok_or_else(|| BotError::invalid("No conversation is open"))?;
        match (&input, &dialog.prompt) {
            (DialogInput::Next, Prompt::Next) | (DialogInput::Number(_), Prompt::Number) | (DialogInput::Text(_), Prompt::Text) => {}
            (DialogInput::Choose(option), Prompt::Menu { options }) if *option >= 1 && usize::from(*option) <= options.len() => {}
            (DialogInput::Choose(_), Prompt::Menu { options }) => {
                return Err(BotError::invalid(format!("Choose an option from 1 to {}", options.len())));
            }
            (_, prompt) => {
                return Err(BotError::invalid(format!("The conversation is waiting for {prompt:?}, not for this answer")));
            }
        }
        let before = bot.dialog_version();
        self.run(bot, BotRequest::Dialog(input)).await?;
        self.wait_for_dialog(bot, before).await;
        Ok(json!({ "dialog": bot.dialog() }))
    }

    /// Waits for the dialogue to change after `version`, then for it to stay quiet: a script says several lines in a row.
    async fn wait_for_dialog(&self, bot: &BotHandle, version: u64) {
        let deadline = Instant::now() + DIALOG_WAIT;
        while bot.dialog_version() == version {
            if Instant::now() > deadline {
                return;
            }
            sleep(DIALOG_POLL).await;
        }
        let mut seen = bot.dialog_version();
        let mut quiet_since = Instant::now();
        // A script that sleeps between pages leaves the prompt on Running, the next page is worth waiting for
        let still_running = || bot.dialog().is_some_and(|dialog| dialog.prompt == Prompt::Running);
        while (quiet_since.elapsed() < DIALOG_QUIET || still_running()) && Instant::now() < deadline {
            sleep(DIALOG_POLL).await;
            let now = bot.dialog_version();
            if now != seen {
                seen = now;
                quiet_since = Instant::now();
            }
        }
    }
}
