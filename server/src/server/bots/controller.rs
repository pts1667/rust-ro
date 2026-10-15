//! What the API does with a bot: the steps of an action, and the waiting that makes one call enough for a whole action.
use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::time::{Instant, sleep, timeout};

use super::combat::DuelRequest;
use super::command::{BotCommand, BotRequest, Deal, DialogInput, PathQuery};
use super::commerce::{BuyingLine, CartMove, StoreAction, VendingLine};
use super::companions::{CompanionAction, HomunculusAction, MercenaryAction, Order, PetAction};
use super::dialog::Prompt;
use super::events::{BotEvent, BotEventKind};
use super::interaction::{ChatAction, ItemAction, PartyAction, ProgressAction, SkillCast, SkillRef, Stat, StorageAction, TradeAction};
use super::menu::{CraftAction, MenuAnswer, SkillMenuAction};
use super::party_window::PartyWindow;
use super::lifecycle::{CreateError, NewBot, StoredBot, create_stored_bot, delete_stored_bot, find_stored_bot, load_bot_character};
use super::registry::BotHandle;
use super::social::{EmoteRef, SocialRequest};
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
/// The map thread settles a capture within a few ticks.
const CAPTURE_WAIT: Duration = Duration::from_secs(3);
/// A hit keeps a character from walking for its damage motion, a few hundred milliseconds.
const RESUME_DELAY: Duration = Duration::from_millis(300);
/// Walks `keep_walking` starts again after hits, at most, in one action.
const MAX_RESUMES: u32 = 30;
/// The experience of a kill comes a few ticks after the monster is gone.
const KILL_WAIT: Duration = Duration::from_secs(1);

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

/// Members of a chat room nobody limited.
fn room_limit() -> u16 {
    20
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

#[derive(Debug, Clone, Deserialize)]
pub struct BuyLine {
    item_id: u32,
    #[serde(default = "one")]
    amount: i16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SellLine {
    index: usize,
    #[serde(default = "one")]
    amount: i16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VendingItem {
    cart_index: u16,
    #[serde(default = "one_unit")]
    amount: u16,
    price: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VendorLine {
    index: u16,
    #[serde(default = "one_unit")]
    amount: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WantedItem {
    item_id: i32,
    #[serde(default = "one_unit")]
    amount: u16,
    price: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuyerLine {
    index: usize,
    #[serde(default = "one_unit")]
    amount: u16,
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
        /// Walks on after a hit stopped the bot, instead of answering `stop_reason: "hit"`.
        #[serde(default)]
        keep_walking: bool,
    },
    /// Walks to an NPC, a ground item or a warp and uses it: talks, picks up, or walks through.
    Use {
        target: u32,
        #[serde(default)]
        keep_walking: bool,
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
    /// Answers the buy or sell choice of a shop.
    DialogShop {
        deal: Deal,
    },
    /// Buys from the list a shop showed.
    DialogBuy {
        items: Vec<BuyLine>,
    },
    /// Sells items of the inventory to a shop.
    DialogSell {
        items: Vec<SellLine>,
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
    /// Creates a party. `item_pickup` lets members pick up each other's loot, `item_share` spreads the loot picked up.
    PartyCreate {
        name: String,
        #[serde(default)]
        item_pickup: bool,
        #[serde(default)]
        item_share: bool,
    },
    PartyInvite {
        name: String,
    },
    PartyAccept,
    PartyDecline,
    PartyLeave,
    /// Changes the sharing of the party (leader only); what is left out stays as it is.
    PartyOptions {
        exp_share: Option<bool>,
        item_pickup: Option<bool>,
        item_share: Option<bool>,
    },
    /// Hands the leadership to a member on the same map (leader only).
    PartyLeader {
        name: String,
    },
    /// Expels a member, online or not (leader only).
    PartyKick {
        name: String,
    },
    Sit,
    Stand,
    /// Shows an emote over the bot: its name (`thx`, `/thx`) or its number.
    Emote {
        emotion: EmoteRef,
    },
    /// Creates a guild, which takes an Emperium.
    GuildCreate {
        name: String,
    },
    GuildInvite {
        name: String,
    },
    GuildAccept,
    GuildDecline,
    GuildLeave,
    GuildChat {
        text: String,
    },
    /// Asks a player in the game to be friends.
    FriendAdd {
        name: String,
    },
    FriendAccept,
    FriendDecline,
    FriendRemove {
        name: String,
    },
    /// Opens a chat room, its title shows above the bot.
    RoomCreate {
        title: String,
        #[serde(default = "room_limit")]
        limit: u16,
        #[serde(default = "default_true")]
        public: bool,
        #[serde(default)]
        password: String,
    },
    /// Joins the chat room of a player on the same map, or the waiting room of an NPC (`npc`, its id).
    RoomJoin {
        #[serde(default)]
        owner: String,
        #[serde(default)]
        npc: Option<u32>,
        #[serde(default)]
        password: String,
    },
    RoomLeave,
    /// Puts up a party booking ad: the level (the bot's by default), a map number of the client's list, the jobs
    /// looked for (none for anyone).
    BookingRegister {
        level: Option<u32>,
        #[serde(default)]
        map_id: i16,
        #[serde(default)]
        jobs: Vec<String>,
    },
    /// Searches the ads: by level (ads up to 15 levels under it), and by map or by job.
    BookingSearch {
        level: Option<u32>,
        #[serde(default)]
        map_id: i16,
        job: Option<String>,
    },
    BookingDelete,
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
    /// Puts an inventory item in the open storage.
    StorageDeposit {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    /// Takes an item of the open storage back, by the `index` of the observation's `storage`.
    StorageWithdraw {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    StorageClose,
    /// Puts an inventory item in the cart.
    CartPut {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    /// Takes an item of the cart (`cart` in the observation) back to the inventory.
    CartGet {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    /// Moves a cart item to the open storage.
    CartToStorage {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    /// Moves an item of the open storage to the cart.
    CartFromStorage {
        index: usize,
        #[serde(default = "one_amount")]
        amount: u32,
    },
    /// Opens a vending store with items of the cart.
    VendingOpen {
        title: String,
        items: Vec<VendingItem>,
    },
    VendingClose,
    /// Buys from the store of a vendor of the observation.
    VendingBuy {
        vendor: u32,
        items: Vec<VendorLine>,
    },
    /// Opens a buying store, after a store license (a usable item) prepared it.
    BuyingOpen {
        title: String,
        zeny_limit: u32,
        items: Vec<WantedItem>,
    },
    BuyingClose,
    /// Sells inventory items to the buying store of a buyer of the observation.
    BuyingSell {
        buyer: u32,
        items: Vec<BuyerLine>,
    },
    /// Answers the window a skill opened (`skill_menu` in the observation) with one of its choices.
    SkillChoose {
        choice: MenuAnswer,
    },
    SkillCancel,
    /// Makes an item of the crafting window (`crafting` in the observation).
    Craft {
        item_id: i32,
        #[serde(default)]
        additives: Vec<i32>,
    },
    DuelCreate {
        #[serde(default)]
        limit: usize,
    },
    DuelInvite {
        name: String,
    },
    DuelAccept,
    DuelDecline,
    DuelLeave,
    /// Hatches the pet egg at an inventory index, in the window that using the egg opened (`pet_eggs` in the observation).
    PetHatch {
        index: usize,
    },
    PetFeed,
    PetPerform,
    PetReturnToEgg,
    PetUnequip,
    PetRename {
        name: String,
    },
    PetEmote {
        emotion: i32,
    },
    /// Puts the pet accessory at an inventory index on the pet.
    PetEquip {
        index: usize,
    },
    /// Throws the taming item that was just used (`pet_capture` in the observation) at a monster. Answers with the outcome.
    PetCapture {
        target: u32,
    },
    HomunculusFeed,
    HomunculusRename {
        name: String,
    },
    HomunculusDelete,
    HomunculusMove {
        x: u16,
        y: u16,
    },
    HomunculusFollow,
    HomunculusAttack {
        target: u32,
        #[serde(default = "default_true")]
        repeat: bool,
    },
    MercenaryMove {
        x: u16,
        y: u16,
    },
    MercenaryFollow,
    MercenaryAttack {
        target: u32,
        #[serde(default = "default_true")]
        repeat: bool,
    },
    MercenaryDismiss,
    /// Runs an `@` command the permission group of the bots allows (`bots.command_group`), and answers with what it said.
    Command {
        text: String,
    },
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

fn one_unit() -> u16 {
    1
}

/// `(listed, amount)` of every line of a purchase or a sale: a line the shop did not list ends the script with an error.
/// The party as the client shows it (`PartyWindow`, built from the packets the server sent), with the party id and the
/// character id of the members in the game. Until the first packets arrive, the server's list of members stands in.
fn party_view(server: Value, window: Option<PartyWindow>) -> Value {
    if server.is_null() {
        return Value::Null;
    }
    let members = server["members"].as_array().cloned().unwrap_or_default();
    let listed = |name: &str| {
        members.iter().find(|member| !name.is_empty() && member["name"].as_str().is_some_and(|known| known.eq_ignore_ascii_case(name)))
    };
    let field_of = |name: &str, field: &str| listed(name).map_or(Value::Null, |member| member[field].clone());
    let Some(window) = window else {
        let leader = server["leader"].clone();
        let members: Vec<Value> = members
            .iter()
            .map(|member| {
                let name = member["name"].as_str().unwrap_or_default();
                json!({
                    "name": name,
                    "char_id": if name.is_empty() { Value::Null } else { member["char_id"].clone() },
                    "map": "",
                    "leader": member["char_id"] == leader,
                    "online": !name.is_empty(),
                    "job": member["job"],
                    "base_level": member["base_level"],
                    "x": null, "y": null, "hp": null, "max_hp": null,
                })
            })
            .collect();
        let leader_name = members.iter().find(|member| member["leader"] == json!(true)).map_or(Value::Null, |member| member["name"].clone());
        return json!({
            "id": server["id"],
            "name": server["name"],
            "leader": leader_name,
            "exp_share": server["exp_share"],
            "exp_share_denied": false,
            "item_pickup": server["item_pickup"],
            "item_share": server["item_share"],
            "members": members,
        });
    };
    let leader = window.members.iter().find(|member| member.leader).map(|member| member.name.clone());
    let mut view = serde_json::to_value(&window).unwrap_or(Value::Null);
    if let Some(listed) = view["members"].as_array_mut() {
        for member in listed {
            let name = member["name"].as_str().unwrap_or_default().to_owned();
            for field in ["char_id", "job", "base_level"] {
                member[field] = field_of(&name, field);
            }
        }
    }
    view["id"] = server["id"].clone();
    view["leader"] = json!(leader);
    view
}

fn check_lines(lines: impl Iterator<Item = (bool, i16)>, what: &str) -> Result<(), BotError> {
    let mut count = 0;
    for (listed, amount) in lines {
        count += 1;
        if !listed {
            return Err(BotError::invalid(format!("A line names {what} the shop does not list")));
        }
        if amount < 1 {
            return Err(BotError::invalid("Every amount is at least 1"));
        }
    }
    if count == 0 {
        return Err(BotError::invalid("Give at least one line in items, or close the dialogue"));
    }
    Ok(())
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
    /// The bot stood on a warp and stayed on its map: the warp needs something the bot does not have, or is off.
    NoWarp,
}

impl Journey {
    fn name(self) -> &'static str {
        match self {
            Journey::Arrived => "arrived",
            Journey::MapChanged => "map_changed",
            Journey::Stopped => "stopped",
            Journey::Timeout => "timeout",
            Journey::NoWarp => "no_warp",
        }
    }
}

/// How a walk of several legs ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Walked {
    journey: Journey,
    /// For a `stopped` walk: `cancelled` (another command took over), `dead`, `hit` (an attack stops a walking character) or
    /// `blocked` (no way on from there).
    stop_reason: Option<&'static str>,
    /// Walks started again after a hit, with `keep_walking`.
    resumed: u32,
}

impl Walked {
    fn write(&self, reply: &mut Value) {
        reply["journey"] = self.journey.name().into();
        if let Some(reason) = self.stop_reason {
            reply["stop_reason"] = reason.into();
        }
        if self.resumed > 0 {
            reply["resumed"] = self.resumed.into();
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
        let max = self.server.configuration.bots.max_registered_bots;
        if self.server.bots().list().len() >= max {
            return Err(BotError::new(ErrorKind::Conflict, format!("At most {max} bots can be registered")));
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
        observation["party"] = party_view(observation["party"].take(), bot.party_window());
        let social = bot.social_window();
        observation["friend_request"] = serde_json::to_value(social.friend_request).unwrap_or(Value::Null);
        observation["booking_results"] = serde_json::to_value(social.booking_results).unwrap_or(Value::Null);
        observation["messages"] = serde_json::to_value(bot.recent_messages(RECENT_MESSAGES)).unwrap_or(Value::Null);
        Ok(observation)
    }

    /// The events of the bot after `after`, also once it left the game, and the number of the last one.
    pub fn events(&self, name: &str, after: u64) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        bot.touch();
        Ok(json!({ "events": bot.events_after(after), "last": bot.last_event() }))
    }

    /// The messages of the bot after `after` (the last 50 are kept).
    pub fn messages(&self, name: &str, after: u64) -> Result<Value, BotError> {
        let bot = self.find(name)?;
        bot.touch();
        Ok(json!({ "messages": bot.messages_after(after) }))
    }

    /// How far a cell or a target is on foot, see `BotRequest::Path`.
    pub async fn path(&self, name: &str, query: PathQuery) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        self.run(&bot, BotRequest::Path(query)).await
    }

    /// The observation with only the parts named in `fields` (`self`, `mobs`, `inventory`...), and `ready` and `map`.
    pub async fn observe_fields(&self, name: &str, fields: Option<&str>) -> Result<Value, BotError> {
        let observation = self.observe(name).await?;
        let Some(fields) = fields else { return Ok(observation) };
        let wanted: Vec<&str> = fields.split(',').map(str::trim).filter(|field| !field.is_empty()).collect();
        let Value::Object(all) = observation else { return Ok(observation) };
        if let Some(unknown) = wanted.iter().find(|field| !all.contains_key(**field)) {
            let known: Vec<&str> = all.keys().map(String::as_str).collect();
            return Err(BotError::invalid(format!("No field {unknown} in an observation, they are: {}", known.join(", "))));
        }
        Ok(Value::Object(all.into_iter().filter(|(key, _)| key == "ready" || key == "map" || wanted.contains(&key.as_str())).collect()))
    }

    pub async fn map(&self, name: &str, center: Option<(u16, u16, u16)>) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        self.run(&bot, BotRequest::Map(center)).await
    }

    pub async fn act(&self, name: &str, action: Action) -> Result<Value, BotError> {
        let bot = self.connected(name)?;
        match action {
            Action::Move { x, y, wait, keep_walking } => {
                let epoch = bot.next_command();
                let first = self.run(&bot, BotRequest::Move { x, y }).await?;
                let mut reply = json!({ "path_length": first["path_length"] });
                if wait {
                    let walked = self.follow_route(&bot, (x, y), first, epoch, self.action_timeout(), keep_walking).await?;
                    walked.write(&mut reply);
                } else if first.get("waypoint").is_some() || keep_walking {
                    let controller = self.clone();
                    tokio::spawn(async move {
                        let _ = controller.follow_route(&bot, (x, y), first, epoch, BACKGROUND_LIMIT, keep_walking).await;
                    });
                }
                Ok(reply)
            }
            Action::Use { target, keep_walking } => {
                let epoch = bot.next_command();
                self.use_target(&bot, target, epoch, keep_walking).await
            }
            Action::Attack { target, wait } => {
                let epoch = bot.next_command();
                let events_before = bot.last_event();
                let first = self.run(&bot, BotRequest::Attack { target_id: target }).await?;
                if first["gone"] == json!(true) {
                    return Err(BotError::invalid(format!("No target {target} on this map")));
                }
                if wait {
                    let fight = self.follow_fight(&bot, target, epoch, self.action_timeout()).await?;
                    let mut reply = json!({ "action": "attack", "fight": fight });
                    self.fight_summary(&bot, fight, events_before, &mut reply).await?;
                    return Ok(reply);
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
            Action::DialogShop { deal } => self.answer(&bot, DialogInput::Deal(deal)).await,
            Action::DialogBuy { items } => {
                let lines = items.into_iter().map(|line| (line.item_id, line.amount)).collect();
                self.answer(&bot, DialogInput::Buy(lines)).await
            }
            Action::DialogSell { items } => {
                let lines = items.into_iter().map(|line| (line.index, line.amount)).collect();
                self.answer(&bot, DialogInput::Sell(lines)).await
            }
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
            Action::PartyCreate { name, item_pickup, item_share } => {
                self.send(&bot, BotRequest::Party(PartyAction::Create { name, item_pickup, item_share })).await
            }
            Action::PartyInvite { name } => self.send(&bot, BotRequest::Party(PartyAction::Invite { name })).await,
            Action::PartyAccept => self.send(&bot, BotRequest::Party(PartyAction::Answer { accept: true })).await,
            Action::PartyDecline => self.send(&bot, BotRequest::Party(PartyAction::Answer { accept: false })).await,
            Action::PartyLeave => self.send(&bot, BotRequest::Party(PartyAction::Leave)).await,
            Action::PartyOptions { exp_share, item_pickup, item_share } => {
                self.send(&bot, BotRequest::Party(PartyAction::Options { exp_share, item_pickup, item_share })).await
            }
            Action::PartyLeader { name } => self.send(&bot, BotRequest::Party(PartyAction::Leader { name })).await,
            Action::PartyKick { name } => self.send(&bot, BotRequest::Party(PartyAction::Kick { name })).await,
            Action::Sit => self.send(&bot, BotRequest::Social(SocialRequest::Sit)).await,
            Action::Stand => self.send(&bot, BotRequest::Social(SocialRequest::Stand)).await,
            Action::Emote { emotion } => self.send(&bot, BotRequest::Social(SocialRequest::Emote(emotion))).await,
            Action::GuildCreate { name } => self.send(&bot, BotRequest::Social(SocialRequest::GuildCreate { name })).await,
            Action::GuildInvite { name } => self.send(&bot, BotRequest::Social(SocialRequest::GuildInvite { name })).await,
            Action::GuildAccept => self.send(&bot, BotRequest::Social(SocialRequest::GuildAnswer { accept: true })).await,
            Action::GuildDecline => self.send(&bot, BotRequest::Social(SocialRequest::GuildAnswer { accept: false })).await,
            Action::GuildLeave => self.send(&bot, BotRequest::Social(SocialRequest::GuildLeave)).await,
            Action::GuildChat { text } => self.send(&bot, BotRequest::Social(SocialRequest::GuildChat(text))).await,
            Action::FriendAdd { name } => self.send(&bot, BotRequest::Social(SocialRequest::FriendAdd { name })).await,
            Action::FriendAccept => self.answer_friend(&bot, true).await,
            Action::FriendDecline => self.answer_friend(&bot, false).await,
            Action::FriendRemove { name } => self.send(&bot, BotRequest::Social(SocialRequest::FriendRemove { name })).await,
            Action::RoomCreate { title, limit, public, password } => {
                self.send(&bot, BotRequest::Social(SocialRequest::RoomCreate { title, limit, public, password })).await
            }
            Action::RoomJoin { owner, npc, password } => {
                self.send(&bot, BotRequest::Social(SocialRequest::RoomJoin { owner, npc, password })).await
            }
            Action::RoomLeave => self.send(&bot, BotRequest::Social(SocialRequest::RoomLeave)).await,
            Action::BookingRegister { level, map_id, jobs } => {
                self.send(&bot, BotRequest::Social(SocialRequest::BookingRegister { level, map_id, jobs })).await
            }
            Action::BookingSearch { level, map_id, job } => {
                self.send(&bot, BotRequest::Social(SocialRequest::BookingSearch { level, map_id, job })).await
            }
            Action::BookingDelete => self.send(&bot, BotRequest::Social(SocialRequest::BookingDelete)).await,
            Action::TradeRequest { target } => self.send(&bot, BotRequest::Trade(TradeAction::Request { target })).await,
            Action::TradeAccept => self.send(&bot, BotRequest::Trade(TradeAction::Answer { accept: true })).await,
            Action::TradeDecline => self.send(&bot, BotRequest::Trade(TradeAction::Answer { accept: false })).await,
            Action::TradeOffer { index, amount } => self.send(&bot, BotRequest::Trade(TradeAction::OfferItem { index, amount })).await,
            Action::TradeZeny { amount } => self.send(&bot, BotRequest::Trade(TradeAction::OfferZeny { amount })).await,
            Action::TradeLock => self.send(&bot, BotRequest::Trade(TradeAction::Lock)).await,
            Action::TradeConfirm => self.send(&bot, BotRequest::Trade(TradeAction::Confirm)).await,
            Action::TradeCancel => self.send(&bot, BotRequest::Trade(TradeAction::Cancel)).await,
            Action::StorageDeposit { index, amount } => self.send(&bot, BotRequest::Storage(StorageAction::Deposit { index, amount })).await,
            Action::StorageWithdraw { index, amount } => self.send(&bot, BotRequest::Storage(StorageAction::Withdraw { index, amount })).await,
            Action::StorageClose => self.send(&bot, BotRequest::Storage(StorageAction::Close)).await,
            Action::CartPut { index, amount } => self.store(&bot, StoreAction::CartMove { how: CartMove::Put, index, amount }).await,
            Action::CartGet { index, amount } => self.store(&bot, StoreAction::CartMove { how: CartMove::Get, index, amount }).await,
            Action::CartToStorage { index, amount } => self.store(&bot, StoreAction::CartMove { how: CartMove::ToStorage, index, amount }).await,
            Action::CartFromStorage { index, amount } => self.store(&bot, StoreAction::CartMove { how: CartMove::FromStorage, index, amount }).await,
            Action::VendingOpen { title, items } => {
                let lines = items.into_iter().map(|item| VendingLine { cart_index: item.cart_index, amount: item.amount, price: item.price }).collect();
                self.store(&bot, StoreAction::OpenVending { title, lines }).await
            }
            Action::VendingClose => self.store(&bot, StoreAction::CloseVending).await,
            Action::VendingBuy { vendor, items } => {
                let lines = items.into_iter().map(|line| (line.index, line.amount)).collect();
                self.store(&bot, StoreAction::BuyFromVendor { vendor, lines }).await
            }
            Action::BuyingOpen { title, zeny_limit, items } => {
                let lines = items.into_iter().map(|item| BuyingLine { item_id: item.item_id, amount: item.amount, price: item.price }).collect();
                self.store(&bot, StoreAction::OpenBuying { title, zeny_limit, lines }).await
            }
            Action::BuyingClose => self.store(&bot, StoreAction::CloseBuying).await,
            Action::BuyingSell { buyer, items } => {
                let lines = items.into_iter().map(|line| (line.index, line.amount)).collect();
                self.store(&bot, StoreAction::SellToBuyer { buyer, lines }).await
            }
            Action::SkillChoose { choice } => self.send(&bot, BotRequest::SkillMenu(SkillMenuAction::Choose(choice))).await,
            Action::SkillCancel => self.send(&bot, BotRequest::SkillMenu(SkillMenuAction::Cancel)).await,
            Action::Craft { item_id, additives } => self.send(&bot, BotRequest::Craft(CraftAction { item_id, additives })).await,
            Action::DuelCreate { limit } => self.send(&bot, BotRequest::Duel(DuelRequest::Create { limit })).await,
            Action::DuelInvite { name } => self.send(&bot, BotRequest::Duel(DuelRequest::Invite { name })).await,
            Action::DuelAccept => self.send(&bot, BotRequest::Duel(DuelRequest::Accept)).await,
            Action::DuelDecline => self.send(&bot, BotRequest::Duel(DuelRequest::Decline)).await,
            Action::DuelLeave => self.send(&bot, BotRequest::Duel(DuelRequest::Leave)).await,
            Action::PetHatch { index } => self.companion(&bot, PetAction::Hatch { index }.into()).await,
            Action::PetFeed => self.companion(&bot, PetAction::Feed.into()).await,
            Action::PetPerform => self.companion(&bot, PetAction::Perform.into()).await,
            Action::PetReturnToEgg => self.companion(&bot, PetAction::ReturnToEgg.into()).await,
            Action::PetUnequip => self.companion(&bot, PetAction::Unequip.into()).await,
            Action::PetRename { name } => self.companion(&bot, PetAction::Rename(name).into()).await,
            Action::PetEmote { emotion } => self.companion(&bot, PetAction::Emote(emotion).into()).await,
            Action::PetEquip { index } => self.companion(&bot, PetAction::Equip { index }.into()).await,
            Action::PetCapture { target } => self.capture_pet(&bot, target).await,
            Action::HomunculusFeed => self.companion(&bot, HomunculusAction::Feed.into()).await,
            Action::HomunculusRename { name } => self.companion(&bot, HomunculusAction::Rename(name).into()).await,
            Action::HomunculusDelete => self.companion(&bot, HomunculusAction::Delete.into()).await,
            Action::HomunculusMove { x, y } => self.companion(&bot, HomunculusAction::Order(Order::Move { x, y }).into()).await,
            Action::HomunculusFollow => self.companion(&bot, HomunculusAction::Order(Order::Follow).into()).await,
            Action::HomunculusAttack { target, repeat } => self.companion(&bot, HomunculusAction::Order(Order::Attack { target, repeat }).into()).await,
            Action::MercenaryMove { x, y } => self.companion(&bot, MercenaryAction::Order(Order::Move { x, y }).into()).await,
            Action::MercenaryFollow => self.companion(&bot, MercenaryAction::Order(Order::Follow).into()).await,
            Action::MercenaryAttack { target, repeat } => self.companion(&bot, MercenaryAction::Order(Order::Attack { target, repeat }).into()).await,
            Action::MercenaryDismiss => self.companion(&bot, MercenaryAction::Dismiss.into()).await,
            Action::Command { text } => self.send(&bot, BotRequest::Command(text)).await,
            Action::DialogClose => {
                let reply = self.run(&bot, BotRequest::Dialog(DialogInput::Close)).await?;
                bot.clear_dialog();
                Ok(reply)
            }
        }
    }

    async fn store(&self, bot: &BotHandle, action: StoreAction) -> Result<Value, BotError> {
        self.send(bot, BotRequest::Store(action)).await
    }

    async fn companion(&self, bot: &BotHandle, action: CompanionAction) -> Result<Value, BotError> {
        self.send(bot, BotRequest::Companion(action)).await
    }

    /// The capture is settled by the map thread, so the call waits for the outcome the game loop records.
    async fn capture_pet(&self, bot: &BotHandle, target: u32) -> Result<Value, BotError> {
        let started = self.run(bot, BotRequest::Companion(PetAction::Capture { target }.into())).await?;
        let claim = started["claim"].as_u64().unwrap_or_default();
        let deadline = Instant::now() + CAPTURE_WAIT;
        loop {
            sleep(POLL_INTERVAL).await;
            let mut outcome = self.run(bot, BotRequest::Companion(PetAction::CaptureOutcome { claim }.into())).await?;
            if outcome["capture"] != json!("pending") || Instant::now() > deadline {
                outcome["target"] = target.into();
                return Ok(outcome);
            }
        }
    }

    /// Sends a request whose effect is in the state of the character, and gives the game loop the time to apply it.
    /// Answers the friend request the window of the bot shows; the window closes either way.
    async fn answer_friend(&self, bot: &BotHandle, accept: bool) -> Result<Value, BotError> {
        let from = bot.social_window().friend_request.map(|request| (request.account_id, request.char_id));
        let reply = self.send(bot, BotRequest::Social(SocialRequest::FriendAnswer { accept, from })).await;
        bot.clear_friend_request();
        reply
    }

    async fn send(&self, bot: &BotHandle, request: BotRequest) -> Result<Value, BotError> {
        let reply = self.run(bot, request).await?;
        sleep(EFFECT_DELAY).await;
        Ok(reply)
    }

    /// Walks the legs of a route that is too long for the pathfinder, `leg` being the answer to the request that started the first.
    /// With `keep_walking`, a walk a hit stopped starts again once the bot can move.
    async fn follow_route(
        &self,
        bot: &BotHandle,
        destination: (u16, u16),
        mut leg: Value,
        epoch: u64,
        limit: Duration,
        keep_walking: bool,
    ) -> Result<Walked, BotError> {
        let deadline = Instant::now() + limit;
        let mut resumed = 0;
        let ended = |journey, stop_reason, resumed| Ok(Walked { journey, stop_reason, resumed });
        loop {
            let events_before = bot.last_event();
            let waypoint = leg["waypoint"]["x"].as_u64().zip(leg["waypoint"]["y"].as_u64()).map(|(x, y)| (x as u16, y as u16));
            let journey = self.follow_walk(bot, Some(waypoint.unwrap_or(destination))).await?;
            if journey == Journey::Stopped {
                let reason = self.stop_reason(bot, epoch, events_before).await?;
                if reason != "hit" || !keep_walking || resumed >= MAX_RESUMES || Instant::now() > deadline {
                    return ended(journey, Some(reason), resumed);
                }
                resumed += 1;
                sleep(RESUME_DELAY).await;
            } else if journey != Journey::Arrived || waypoint.is_none() {
                return ended(journey, None, resumed);
            }
            if !bot.is_current_command(epoch) {
                return ended(Journey::Stopped, Some("cancelled"), resumed);
            }
            if Instant::now() > deadline {
                return ended(Journey::Timeout, None, resumed);
            }
            leg = match self.run(bot, BotRequest::Move { x: destination.0, y: destination.1 }).await {
                Ok(next) => next,
                Err(error) if error.kind == ErrorKind::Invalid => return ended(Journey::Stopped, Some("blocked"), resumed),
                Err(error) => return Err(error),
            };
            if leg["path_length"] == json!(0) {
                return ended(Journey::Arrived, None, resumed);
            }
        }
    }

    /// Why a walk stopped short, see `Walked::stop_reason`. `events_before` is the last event when the walk started.
    async fn stop_reason(&self, bot: &BotHandle, epoch: u64, events_before: u64) -> Result<&'static str, BotError> {
        if !bot.is_current_command(epoch) {
            return Ok("cancelled");
        }
        if self.probe(bot).await?.dead {
            return Ok("dead");
        }
        let hit = bot.events_after(events_before).iter().any(|event| matches!(event.kind, BotEventKind::Damaged { .. }));
        Ok(if hit { "hit" } else { "blocked" })
    }

    /// Adds to the answer of a fight what it cost and gave: `damage_taken`, `killed` and the experience of the kill, and `hp`.
    async fn fight_summary(&self, bot: &BotHandle, fight: &str, events_before: u64, reply: &mut Value) -> Result<(), BotError> {
        let killed = |events: &[BotEvent]| events.iter().any(|event| matches!(event.kind, BotEventKind::Killed { .. }));
        let mut events = bot.events_after(events_before);
        if fight == "over" && !killed(&events) {
            let deadline = Instant::now() + KILL_WAIT;
            while !killed(&events) && Instant::now() < deadline {
                sleep(POLL_INTERVAL).await;
                events = bot.events_after(events_before);
            }
        }
        let (mut damage_taken, mut base_exp, mut job_exp) = (0_u64, 0_u64, 0_u64);
        for event in &events {
            match &event.kind {
                BotEventKind::Damaged { damage, .. } => damage_taken += u64::from(*damage),
                BotEventKind::Killed { base_exp: base, job_exp: job, .. } => {
                    base_exp += u64::from(*base);
                    job_exp += u64::from(*job);
                }
                _ => {}
            }
        }
        reply["damage_taken"] = damage_taken.into();
        reply["killed"] = killed(&events).into();
        reply["base_exp"] = base_exp.into();
        reply["job_exp"] = job_exp.into();
        let status = self.run(bot, BotRequest::Status).await?;
        reply["hp"] = status["self"]["hp"].clone();
        Ok(())
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
                    // A bot that died since the probe can not attack either
                    Err(error) if error.kind == ErrorKind::Invalid => {
                        return Ok(if self.probe(bot).await?.dead { "dead" } else { "unreachable" });
                    }
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

    async fn use_target(&self, bot: &BotHandle, target: u32, epoch: u64, keep_walking: bool) -> Result<Value, BotError> {
        let start_map = self.probe(bot).await?.map;
        let deadline = Instant::now() + self.action_timeout();
        let mut journey = None;
        let mut stop_reason = None;
        let mut resumed = 0;
        let mut kind = String::new();
        loop {
            let approach = match self.run(bot, BotRequest::Approach { target_id: target }).await {
                Ok(approach) => approach,
                // The last leg ended on (or crossed) a warp, which the game loop applied after the walk
                Err(_) if journey.is_some() && self.probe(bot).await?.map != start_map => {
                    journey = Some(Journey::MapChanged);
                    stop_reason = None;
                    break;
                }
                Err(error) => return Err(error),
            };
            kind = approach["kind"].as_str().unwrap_or_default().to_string();
            if approach["in_range"] != json!(false) {
                break;
            }
            let destination = (approach["destination"]["x"].as_u64(), approach["destination"]["y"].as_u64());
            let destination = destination.0.zip(destination.1).map(|(x, y)| (x as u16, y as u16));
            let events_before = bot.last_event();
            let leg = self.follow_walk(bot, destination).await?;
            journey = Some(leg);
            if leg == Journey::Stopped {
                let reason = self.stop_reason(bot, epoch, events_before).await?;
                stop_reason = Some(reason);
                if reason == "hit" && keep_walking && resumed < MAX_RESUMES && Instant::now() < deadline {
                    resumed += 1;
                    sleep(RESUME_DELAY).await;
                    continue;
                }
            }
            if leg != Journey::Arrived {
                break;
            }
            stop_reason = None;
            if Instant::now() > deadline {
                journey = Some(Journey::Timeout);
                break;
            }
        }
        let mut reply = json!({ "target": target, "kind": kind });
        if kind == "warp" && journey.is_none_or(|journey| journey == Journey::Arrived) {
            // Standing on the warp cell is the use; a bot that stays on its map stood on a warp that did nothing
            journey = Some(match self.await_map_change(bot, &start_map).await? {
                Journey::Arrived => Journey::NoWarp,
                changed => changed,
            });
        }
        if let Some(journey) = journey {
            Walked { journey, stop_reason, resumed }.write(&mut reply);
            if journey != Journey::Arrived {
                return Ok(reply);
            }
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
            (DialogInput::Deal(_), Prompt::Shop) => {}
            (DialogInput::Buy(lines), Prompt::Buy { items }) => {
                check_lines(lines.iter().map(|(item_id, amount)| (items.iter().any(|offer| offer.item_id == *item_id), *amount)), "an item_id")?
            }
            (DialogInput::Sell(lines), Prompt::Sell { items }) => {
                check_lines(lines.iter().map(|(index, amount)| (items.iter().any(|offer| offer.index == *index), *amount)), "an index")?
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(json: &str) -> Action {
        serde_json::from_str(json).unwrap_or_else(|error| panic!("{json}: {error}"))
    }

    #[test]
    fn the_cart_and_store_actions_read_the_json_of_the_documentation() {
        assert!(matches!(parse(r#"{"type":"cart_put","index":2}"#), Action::CartPut { index: 2, amount: 1 }));
        assert!(matches!(parse(r#"{"type":"cart_to_storage","index":0,"amount":5}"#), Action::CartToStorage { index: 0, amount: 5 }));
        let Action::VendingOpen { title, items } = parse(r#"{"type":"vending_open","title":"Cheap","items":[{"cart_index":0,"amount":20,"price":8},{"cart_index":1,"price":3}]}"#) else {
            panic!("not a vending_open")
        };
        assert_eq!((title.as_str(), items.len(), items[1].amount, items[1].price), ("Cheap", 2, 1, 3));
        assert!(matches!(parse(r#"{"type":"vending_buy","vendor":150001,"items":[{"index":0}]}"#), Action::VendingBuy { vendor: 150001, .. }));
        let Action::BuyingOpen { zeny_limit, items, .. } = parse(r#"{"type":"buying_open","title":"Wanted","zeny_limit":300,"items":[{"item_id":909,"amount":100,"price":5}]}"#) else {
            panic!("not a buying_open")
        };
        assert_eq!((zeny_limit, items[0].item_id), (300, 909));
        assert!(matches!(parse(r#"{"type":"buying_sell","buyer":150002,"items":[{"index":4,"amount":10}]}"#), Action::BuyingSell { buyer: 150002, .. }));
        assert!(matches!(parse(r#"{"type":"vending_close"}"#), Action::VendingClose));
    }

    #[test]
    fn the_menu_and_duel_actions_read_the_json_of_the_documentation() {
        assert!(matches!(parse(r#"{"type":"skill_choose","choice":"Random"}"#), Action::SkillChoose { choice: MenuAnswer::Text(_) }));
        assert!(matches!(parse(r#"{"type":"skill_choose","choice":716}"#), Action::SkillChoose { choice: MenuAnswer::Number(716) }));
        assert!(matches!(parse(r#"{"type":"skill_cancel"}"#), Action::SkillCancel));
        let Action::Craft { item_id, additives } = parse(r#"{"type":"craft","item_id":501}"#) else { panic!("not a craft") };
        assert_eq!((item_id, additives.len()), (501, 0));
        assert!(matches!(parse(r#"{"type":"duel_create"}"#), Action::DuelCreate { limit: 0 }));
        assert!(matches!(parse(r#"{"type":"duel_invite","name":"Other"}"#), Action::DuelInvite { .. }));
        assert!(serde_json::from_str::<Action>(r#"{"type":"skill_choose","choice":-1}"#).is_err());
    }

    #[test]
    fn the_companion_and_command_actions_read_the_json_of_the_documentation() {
        assert!(matches!(parse(r#"{"type":"pet_hatch","index":3}"#), Action::PetHatch { index: 3 }));
        assert!(matches!(parse(r#"{"type":"pet_capture","target":1234}"#), Action::PetCapture { target: 1234 }));
        assert!(matches!(parse(r#"{"type":"pet_return_to_egg"}"#), Action::PetReturnToEgg));
        assert!(matches!(parse(r#"{"type":"homunculus_move","x":10,"y":12}"#), Action::HomunculusMove { x: 10, y: 12 }));
        assert!(matches!(parse(r#"{"type":"homunculus_attack","target":9}"#), Action::HomunculusAttack { target: 9, repeat: true }));
        assert!(matches!(parse(r#"{"type":"mercenary_attack","target":9,"repeat":false}"#), Action::MercenaryAttack { target: 9, repeat: false }));
        assert!(matches!(parse(r#"{"type":"mercenary_dismiss"}"#), Action::MercenaryDismiss));
        let Action::Command { text } = parse(r#"{"type":"command","text":"@go prontera"}"#) else { panic!("not a command") };
        assert_eq!(text, "@go prontera");
    }

    #[test]
    fn the_party_actions_read_the_json_of_the_documentation() {
        assert!(matches!(parse(r#"{"type":"party_create","name":"Wanderers"}"#), Action::PartyCreate { item_pickup: false, item_share: false, .. }));
        assert!(matches!(
            parse(r#"{"type":"party_create","name":"Wanderers","item_pickup":true,"item_share":true}"#),
            Action::PartyCreate { item_pickup: true, item_share: true, .. }
        ));
        assert!(matches!(
            parse(r#"{"type":"party_options","exp_share":true}"#),
            Action::PartyOptions { exp_share: Some(true), item_pickup: None, item_share: None }
        ));
        assert!(matches!(parse(r#"{"type":"party_leader","name":"Beta"}"#), Action::PartyLeader { .. }));
        assert!(matches!(parse(r#"{"type":"party_kick","name":"Beta"}"#), Action::PartyKick { .. }));
    }

    #[test]
    fn the_social_actions_read_the_json_of_the_documentation() {
        assert!(matches!(parse(r#"{"type":"sit"}"#), Action::Sit));
        assert!(matches!(parse(r#"{"type":"emote","emotion":"thx"}"#), Action::Emote { emotion: EmoteRef::Name(_) }));
        assert!(matches!(parse(r#"{"type":"emote","emotion":15}"#), Action::Emote { emotion: EmoteRef::Number(15) }));
        assert!(matches!(parse(r#"{"type":"guild_invite","name":"Beta"}"#), Action::GuildInvite { .. }));
        assert!(matches!(parse(r#"{"type":"friend_accept"}"#), Action::FriendAccept));
        assert!(matches!(
            parse(r#"{"type":"room_create","title":"LF party"}"#),
            Action::RoomCreate { limit: 20, public: true, .. }
        ));
        assert!(matches!(parse(r#"{"type":"room_join","owner":"Beta"}"#), Action::RoomJoin { .. }));
        assert!(matches!(parse(r#"{"type":"room_join","npc":110042}"#), Action::RoomJoin { npc: Some(110042), .. }));
        let Action::BookingRegister { level, map_id, jobs } = parse(r#"{"type":"booking_register","jobs":["Acolyte"]}"#) else {
            panic!("not a booking")
        };
        assert_eq!((level, map_id, jobs), (None, 0, vec!["Acolyte".to_string()]));
        assert!(matches!(parse(r#"{"type":"booking_search","level":12}"#), Action::BookingSearch { level: Some(12), map_id: 0, job: None }));
    }

    #[test]
    fn a_purchase_names_listed_items_with_a_positive_amount() {
        assert!(check_lines([(true, 1), (true, 30)].into_iter(), "an item_id").is_ok());
        assert!(check_lines([(true, 1), (false, 1)].into_iter(), "an item_id").is_err());
        assert!(check_lines([(true, 0)].into_iter(), "an item_id").is_err());
        assert!(check_lines(std::iter::empty(), "an item_id").is_err());
    }
}
