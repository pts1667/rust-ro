//! What the API does with a bot: the steps of an action, and the waiting that makes one call enough for a whole action.
use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::time::{Instant, sleep, timeout};

use super::command::{BotCommand, BotRequest, DialogInput};
use super::dialog::Prompt;
use super::interaction::{ChatAction, ItemAction, PartyAction, ProgressAction, SkillCast, SkillRef, Stat, TradeAction};
use super::lifecycle::{CreateError, NewBot, StoredBot, create_stored_bot, delete_stored_bot, find_stored_bot, load_bot_character};
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
/// Two ticks of the game loop: the event of a request runs on the next one.
const EFFECT_DELAY: Duration = Duration::from_millis(100);
/// A character leaves the game after the delay the server sets for logging out, and its last save.
const LOGOUT_WAIT: Duration = Duration::from_secs(20);
const SAVE_GRACE: Duration = Duration::from_secs(1);
const WARP_POLLS: u32 = 10;
const RECENT_MESSAGES: usize = 10;
/// A fight or a long walk started without `wait` has no caller to give up on it.
const BACKGROUND_LIMIT: Duration = Duration::from_secs(600);
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
    /// Spends status points on a stat: `str`, `agi`, `vit`, `int`, `dex` or `luk`.
    RaiseStat {
        stat: Stat,
        #[serde(default = "one_point")]
        amount: u16,
    },
    /// Spends a skill point on the next level of a skill.
    LearnSkill {
        skill: SkillRef,
    },
    UseItem {
        index: usize,
    },
    Equip {
        index: usize,
    },
    Unequip {
        index: usize,
    },
    DropItem {
        index: usize,
        #[serde(default = "one")]
        amount: i16,
    },
    /// Uses a skill on a target, a cell, or oneself, as its kind in the observation says.
    Skill {
        skill: SkillRef,
        level: Option<u8>,
        target: Option<u32>,
        x: Option<u16>,
        y: Option<u16>,
    },
    Say {
        text: String,
    },
    Whisper {
        to: String,
        text: String,
    },
    PartyChat {
        text: String,
    },
    PartyCreate {
        name: String,
    },
    PartyInvite {
        name: String,
    },
    PartyAccept,
    PartyDecline,
    PartyLeave,
    TradeRequest {
        target: u32,
    },
    TradeAccept,
    TradeDecline,
    TradeOffer {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    TradeZeny {
        amount: u32,
    },
    TradeLock,
    TradeConfirm,
    TradeCancel,
}

fn one_point() -> u16 {
    1
}

fn one() -> i16 {
    1
}

fn one_amount() -> u32 {
    1
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

    /// A bot of the game, for a request made on its behalf: that keeps it from being logged out for being idle.
    fn connected(&self, name: &str) -> Result<Arc<BotHandle>, BotError> {
        let bot = self.find(name)?;
        bot.touch();
        self.connected_bot(bot)
    }

    fn connected_bot(&self, bot: Arc<BotHandle>) -> Result<Arc<BotHandle>, BotError> {
        let name = &bot.name;
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
        bot.touch();
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

    /// Takes the character out of the game, and returns once it is gone and saved.
    pub async fn disconnect(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        self.leave_game(&bot).await;
        Ok(Self::summary(&bot))
    }

    async fn leave_game(&self, bot: &BotHandle) {
        // Ends the fights and walks the bot runs on its own
        bot.next_command();
        if bot.is_connected() {
            // The character may already be gone, which is what was asked for
            let _ = self.run(bot, BotRequest::Disconnect).await;
            bot.set_connected(false);
        }
        let deadline = Instant::now() + LOGOUT_WAIT;
        while self.server.directory().presence(bot.char_id).is_some() && Instant::now() < deadline {
            sleep(POLL_INTERVAL).await;
        }
    }

    /// Erases the character and the account of the bot for good.
    pub async fn delete(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.known_or_stored(name).await?;
        self.leave_game(&bot).await;
        if self.server.directory().presence(bot.char_id).is_some() {
            return Err(BotError::new(ErrorKind::Conflict, "The character did not leave the game yet, try again in a moment"));
        }
        // The last saves of the character are on the database thread
        sleep(SAVE_GRACE).await;
        let server = self.server.clone();
        let owned = bot.name.clone();
        let deleted = tokio::task::spawn_blocking(move || delete_stored_bot(&server, &owned))
            .await
            .map_err(|error| BotError::new(ErrorKind::Unavailable, error.to_string()))?
            .map_err(|message| BotError::new(ErrorKind::Unavailable, message))?;
        self.server.bots().remove(&bot);
        info!("Deleted bot {} (character {})", bot.name, bot.char_id);
        Ok(json!({ "name": bot.name, "char_id": bot.char_id, "deleted": deleted }))
    }

    /// Logs out the bots nobody asked anything of for `bots.idle_logout_secs`.
    pub async fn logout_idle_bots(&self) {
        let limit = Duration::from_secs(self.server.configuration.bots.idle_logout_secs);
        if limit.is_zero() {
            return;
        }
        for bot in self.server.bots().list() {
            if bot.is_connected() && bot.idle_for() >= limit {
                info!("Bot {} was idle for {}s, logging it out", bot.name, bot.idle_for().as_secs());
                self.leave_game(&bot).await;
            }
        }
    }

    pub async fn status(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        bot.touch();
        let mut summary = Self::summary(&bot);
        if bot.is_connected() {
            summary["status"] = self.run(&bot, BotRequest::Status).await?;
        }
        Ok(summary)
    }

    /// Everything the bot sees, plus what the NPC it talks to shows and the latest messages.
    pub async fn observe(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        self.view(&bot).await
    }

    /// The observation for a subscription: nobody asked for it, so it does not count as a request.
    pub(super) async fn observe_unprompted(&self, name: &str) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        let bot = self.connected_bot(bot)?;
        self.view(&bot).await
    }

    async fn view(&self, bot: &BotHandle) -> Result<Value, BotError> {
        let mut observation = self.run(bot, BotRequest::Observe).await?;
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
                let epoch = bot.next_command();
                let first = self.run(&bot, BotRequest::Move { x, y }).await?;
                let mut reply = json!({ "path_length": first["path_length"] });
                if wait {
                    let journey = self.follow_route(&bot, (x, y), first, epoch, self.action_timeout()).await?;
                    reply["journey"] = journey.name().into();
                } else if first.get("waypoint").is_some() {
                    let controller = self.clone();
                    tokio::spawn(async move {
                        let _ = controller.follow_route(&bot, (x, y), first, epoch, BACKGROUND_LIMIT).await;
                    });
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
                    let _ = controller.follow_fight(&bot, target, epoch, BACKGROUND_LIMIT).await;
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
            Action::RaiseStat { stat, amount } => self.send(&bot, BotRequest::Progress(ProgressAction::RaiseStat { stat, amount })).await,
            Action::LearnSkill { skill } => self.send(&bot, BotRequest::Progress(ProgressAction::LearnSkill { skill })).await,
            Action::UseItem { index } => self.send(&bot, BotRequest::Item(ItemAction::Use { index })).await,
            Action::Equip { index } => self.send(&bot, BotRequest::Item(ItemAction::Equip { index })).await,
            Action::Unequip { index } => self.send(&bot, BotRequest::Item(ItemAction::Unequip { index })).await,
            Action::DropItem { index, amount } => self.send(&bot, BotRequest::Item(ItemAction::Drop { index, amount })).await,
            Action::Skill { skill, level, target, x, y } => {
                let cell = x.zip(y);
                self.send(&bot, BotRequest::Skill(SkillCast { skill, level, target, cell })).await
            }
            Action::Say { text } => self.send(&bot, BotRequest::Chat(ChatAction::Say(text))).await,
            Action::Whisper { to, text } => self.send(&bot, BotRequest::Chat(ChatAction::Whisper { to, text })).await,
            Action::PartyChat { text } => self.send(&bot, BotRequest::Chat(ChatAction::Party(text))).await,
            Action::PartyCreate { name } => self.send(&bot, BotRequest::Party(PartyAction::Create { name })).await,
            Action::PartyInvite { name } => self.send(&bot, BotRequest::Party(PartyAction::Invite { name })).await,
            Action::PartyAccept => self.send(&bot, BotRequest::Party(PartyAction::Answer { accept: true })).await,
            Action::PartyDecline => self.send(&bot, BotRequest::Party(PartyAction::Answer { accept: false })).await,
            Action::PartyLeave => self.send(&bot, BotRequest::Party(PartyAction::Leave)).await,
            Action::TradeRequest { target } => self.send(&bot, BotRequest::Trade(TradeAction::Request { target })).await,
            Action::TradeAccept => self.send(&bot, BotRequest::Trade(TradeAction::Answer { accept: true })).await,
            Action::TradeDecline => self.send(&bot, BotRequest::Trade(TradeAction::Answer { accept: false })).await,
            Action::TradeOffer { index, amount } => self.send(&bot, BotRequest::Trade(TradeAction::OfferItem { index, amount })).await,
            Action::TradeZeny { amount } => self.send(&bot, BotRequest::Trade(TradeAction::OfferZeny { amount })).await,
            Action::TradeLock => self.send(&bot, BotRequest::Trade(TradeAction::Lock)).await,
            Action::TradeConfirm => self.send(&bot, BotRequest::Trade(TradeAction::Confirm)).await,
            Action::TradeCancel => self.send(&bot, BotRequest::Trade(TradeAction::Cancel)).await,
            Action::DialogClose => {
                let reply = self.run(&bot, BotRequest::Dialog(DialogInput::Close)).await?;
                bot.clear_dialog();
                Ok(reply)
            }
        }
    }

    /// Sends a request whose effect is in the state of the character, and gives the game loop the time to apply it.
    async fn send(&self, bot: &BotHandle, request: BotRequest) -> Result<Value, BotError> {
        let reply = self.run(bot, request).await?;
        sleep(EFFECT_DELAY).await;
        Ok(reply)
    }

    /// Walks the legs of a route that is too long for the pathfinder, `leg` being the answer to the request that started the first.
    async fn follow_route(&self, bot: &BotHandle, destination: (u16, u16), mut leg: Value, epoch: u64, limit: Duration) -> Result<Journey, BotError> {
        let deadline = Instant::now() + limit;
        loop {
            let waypoint = leg["waypoint"]["x"].as_u64().zip(leg["waypoint"]["y"].as_u64()).map(|(x, y)| (x as u16, y as u16));
            let journey = self.follow_walk(bot, Some(waypoint.unwrap_or(destination))).await?;
            if journey != Journey::Arrived || waypoint.is_none() {
                return Ok(journey);
            }
            if !bot.is_current_command(epoch) {
                return Ok(Journey::Stopped);
            }
            if Instant::now() > deadline {
                return Ok(Journey::Timeout);
            }
            leg = match self.run(bot, BotRequest::Move { x: destination.0, y: destination.1 }).await {
                Ok(next) => next,
                Err(error) if error.kind == ErrorKind::Invalid => return Ok(Journey::Stopped),
                Err(error) => return Err(error),
            };
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
        let start_map = self.probe(bot).await?.map;
        let deadline = Instant::now() + self.action_timeout();
        let mut journey = None;
        let mut approach;
        loop {
            approach = self.run(bot, BotRequest::Approach { target_id: target }).await?;
            if approach["in_range"] != json!(false) {
                break;
            }
            let destination = (approach["destination"]["x"].as_u64(), approach["destination"]["y"].as_u64());
            let destination = destination.0.zip(destination.1).map(|(x, y)| (x as u16, y as u16));
            let leg = self.follow_walk(bot, destination).await?;
            journey = Some(leg);
            if leg != Journey::Arrived {
                break;
            }
            if Instant::now() > deadline {
                journey = Some(Journey::Timeout);
                break;
            }
        }
        let kind = approach["kind"].as_str().unwrap_or_default().to_string();
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
