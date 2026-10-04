use std::collections::BTreeMap;

use models::enums::{EnumWithMaskValueU8, EnumWithMaskValueU32, EnumWithMaskValueU64, WithMaskValueU32, WithMaskValueU64};
use serde::{Deserialize, Serialize};

#[path = "pet_support.rs"]
mod pet_support;
pub use pet_support::{PetAttackSkill, PetRecovery, PetSupportCast, PetSupportRuntime, PetSupportSkill, PetTimedBonus,
    PetLootRuntime, PetLootCargo, PetLootDropReceipt, PetLootDropReservation, PendingPetLootClaim};
#[path = "player_trade.rs"]
mod player_trade;
pub use player_trade::{PlayerTrade, PlayerTradeItem, PlayerTradePhase, PlayerTradeReceipt};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CharacterGameSystems {
    pub revision: u64,
    pub font: u16,
    pub permanent_skill_grants: BTreeMap<u32, u8>,
    pub partner_id: u32,
    pub guild_id: u32,
    pub party_id: u32,
    pub party_leader_id: u32,
    pub party_members: Vec<u32>,
    pub mounting: bool,
    pub party_name: String,
    pub party_invite_disabled: bool,
    pub pet: Option<PetRecord>,
    pub pet_loot: Option<PetLootCargo>,
    pub last_pet_loot_drop: Option<PetLootDropReceipt>,
    pub last_trade_receipt: Option<PlayerTradeReceipt>,
    #[serde(skip)]
    pub trade: Option<PlayerTrade>,
    pub homunculus: Option<HomunculusRecord>,
    pub mercenary: Option<MercenaryRecord>,
    pub mercenary_faith: BTreeMap<u8, u16>,
    pub mercenary_calls: BTreeMap<u8, u32>,
    #[serde(skip)]
    pub pet_capture: Option<PetCapture>,
    #[serde(skip)]
    pub pending_pet_capture: Option<PendingPetCapture>,
    #[serde(skip)]
    pub completed_pet_capture: Option<PendingPetCapture>,
    #[serde(skip)]
    pub pet_support: Option<PetSupportRuntime>,
    #[serde(skip)]
    pub pending_pet_loot: Option<PendingPetLootClaim>,
    #[serde(skip)]
    pub completed_pet_loot: Option<PendingPetLootClaim>,
    #[serde(skip)]
    pub pet_loot_retry_at: u64,
    #[serde(skip)]
    pub pet_hatching: bool,
    #[serde(skip)]
    pub buying_slots: u8,
    #[serde(skip)]
    pub buying_store: Option<BuyingStore>,
    #[serde(skip)]
    pub cart_items: Vec<database::model::InventoryRecord>,
    #[serde(skip)]
    pub vending_slots: u8,
    #[serde(skip)]
    pub vending_store: Option<VendingStore>,
    #[serde(skip)]
    pub opened_vending_store: Option<u32>,
    #[serde(skip)]
    pub remote_vending_store: Option<u32>,
    #[serde(skip)]
    pub store_search: Option<StoreSearch>,
    #[serde(skip)]
    pub rendered_companions: BTreeMap<u32, CompanionPosition>,
    #[serde(skip)]
    pub companion_commands: BTreeMap<u32, CompanionCommand>,
    #[serde(skip)]
    pub last_companion_tick: u64,
    #[serde(skip)]
    pub last_companion_map: String,
    #[serde(skip)]
    pub storage_open: bool,
    #[serde(skip)]
    pub storage_items: Vec<database::model::InventoryRecord>,
    #[serde(skip)]
    pub guild_storage_open: Option<u32>,
    #[serde(skip)]
    pub guild_storage_items: Vec<database::model::InventoryRecord>,
    #[serde(skip)]
    pub remote_store: Option<u32>,
    #[serde(skip)]
    pub pet_owner_dead: bool,
    #[serde(skip)]
    pub potion_success_counter: u8,
    #[serde(skip)]
    pub guild_invitation: Option<GuildInvitation>,
    #[serde(skip)]
    pub party_invitation: Option<PartyInvitation>,
    #[serde(skip)]
    pub party: Option<PartyRecord>,
    #[serde(skip)]
    pub last_party_update: u64,
    #[serde(skip)]
    pub party_position: Option<(String, u8, u16, u16, u32)>,
    #[serde(skip)]
    pub party_health: Option<(u32, u32)>,
}

impl CharacterGameSystems {
    pub fn is_trading(&self) -> bool { self.trade.as_ref().is_some_and(|trade| trade.phase.blocks_actions()) }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AccountGameSystems {
    pub vip_expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PetRecord {
    pub id: u32,
    pub owner_char_id: u32,
    pub class_id: u16,
    pub name: String,
    pub level: u16,
    pub egg_item_id: i32,
    pub egg_inventory_id: i32,
    pub intimacy: i32,
    pub hunger: i32,
    pub equipped_item: i32,
    pub next_hunger_at: u64,
    pub incubating: bool,
    pub renamed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PendingPetCapture {
    pub claim_id: u64,
    pub target_id: u32,
    pub map_key: crate::server::model::map_instance::MapInstanceKey,
    pub expires_at: u64,
    pub committed: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct HomunculusRecord {
    pub id: u32,
    pub class_id: u16,
    pub name: String,
    pub level: u16,
    pub intimacy: u32,
    pub hunger: u16,
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    #[serde(default)]
    pub base_max_hp: u32,
    #[serde(default)]
    pub base_max_sp: u32,
    #[serde(default)]
    pub statuses: Vec<models::status_change::StatusChange>,
    pub stats: [u16; 6],
    pub active: bool,
    pub evolved: bool,
    pub skill_points: u16,
    pub skills: BTreeMap<u32, u8>,
    #[serde(default)]
    pub experience: u64,
    #[serde(default)]
    pub renamed: bool,
    #[serde(default)]
    pub next_hunger_at: u64,
    #[serde(default)]
    pub skill_cooldowns: BTreeMap<u32, u64>,
    #[serde(default)]
    pub cooldown_pause_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MercenaryRecord {
    pub id: u32,
    pub class_id: u16,
    pub name: String,
    pub level: u16,
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub expires_at: u64,
    #[serde(default)]
    pub contract_remaining: Option<u64>,
    #[serde(default)]
    pub skill_cooldowns: BTreeMap<u32, u64>,
    #[serde(default)]
    pub cooldown_pause_at: Option<u64>,
    pub guild: u8,
    pub kill_count: u32,
    pub statuses: Vec<models::status_change::StatusChange>,
    #[serde(skip)]
    pub last_attack_at: u64,
    #[serde(skip)]
    pub last_regen_hp_at: u64,
    #[serde(skip)]
    pub last_regen_sp_at: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CompanionPosition {
    pub x: u16,
    pub y: u16,
    pub map_instance: u8,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompanionCommand {
    pub target: Option<u32>,
    pub repeat: bool,
    pub destination: Option<(u16, u16)>,
    pub stay: bool,
    pub next_move_at: u64,
    pub last_attack_at: u64,
    pub last_regen_hp_at: u64,
    pub last_regen_sp_at: u64,
    pub can_act_at: u64,
    pub cooldowns: BTreeMap<u32, u64>,
    pub cast: Option<CompanionCast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompanionCast {
    pub skill_id: u32,
    pub level: u8,
    pub target_id: u32,
    pub ground: Option<(u16, u16)>,
    pub scripted: bool,
    pub ignore_range: bool,
    pub cast_cancel: Option<bool>,
    pub completes_at: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetCapture {
    pub item_id: i32,
    pub flag: u8,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildRecord {
    pub id: u32,
    pub name: String,
    pub master_char_id: u32,
    pub members: Vec<u32>,
    pub level: u16,
    pub experience: u64,
    pub skill_points: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuildInvitation {
    pub guild_id: u32,
    pub inviter_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartyRecord {
    pub id: u32,
    pub revision: u64,
    pub name: String,
    pub leader_char_id: u32,
    pub members: Vec<u32>,
    pub exp_share: bool,
    pub item_pickup: bool,
    pub item_share: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartyInvitation {
    pub party_id: u32,
    pub inviter_id: u32,
}

#[derive(Debug, Clone)]
pub struct PartyChange {
    pub before: Option<PartyRecord>,
    pub party: Option<PartyRecord>,
    pub member_states: BTreeMap<u32, CharacterGameSystems>,
    pub member_records: Vec<database::model::CharacterRecord>,
    pub removed_members: Vec<u32>,
}

#[derive(Debug, Clone, Copy, WithMaskValueU32)]
pub enum GuildPermission {
    #[mask_value = 1]
    Invite,
    #[mask_value = 16]
    Expel,
}

#[derive(Debug, Clone, Copy, WithMaskValueU32)]
pub enum GuildMenu {
    #[mask_value = 1]
    Members,
    Positions,
    Skills,
    #[mask_value = 16]
    Expulsions,
    #[mask_value = 128]
    Notice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuyingOffer {
    pub item_id: i32,
    pub amount: u16,
    pub price: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuyingStore {
    pub id: u32,
    pub char_id: u32,
    pub account_id: u32,
    pub title: String,
    pub map: String,
    pub map_instance: u8,
    pub x: u16,
    pub y: u16,
    pub zeny_limit: u32,
    pub offers: Vec<BuyingOffer>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoreSearch {
    pub remaining_uses: u8,
    pub next_query_at: u64,
    pub remote: bool,
    pub map: Option<String>,
    pub results: Vec<StoreSearchResult>,
    pub next_page: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoreSearchResult {
    pub store: StoreLocation,
    pub kind: u8,
    pub item_id: i32,
    pub amount: u16,
    pub price: u32,
    pub item_type: u8,
    pub refine: u8,
    pub cards: [u16; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoreLocation {
    pub id: u32,
    pub char_id: u32,
    pub account_id: u32,
    pub title: String,
    pub map: String,
    pub map_instance: u8,
    pub x: u16,
    pub y: u16,
}

impl From<&BuyingStore> for StoreLocation {
    fn from(store: &BuyingStore) -> Self {
        Self {
            id: store.id,
            char_id: store.char_id,
            account_id: store.account_id,
            title: store.title.clone(),
            map: store.map.clone(),
            map_instance: store.map_instance,
            x: store.x,
            y: store.y,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendingOffer {
    pub inventory_id: i32,
    pub index: u16,
    pub amount: u16,
    pub price: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendingStore {
    pub id: u32,
    pub char_id: u32,
    pub account_id: u32,
    pub title: String,
    pub map: String,
    pub map_instance: u8,
    pub x: u16,
    pub y: u16,
    pub offers: Vec<VendingOffer>,
}

impl From<&VendingStore> for StoreLocation {
    fn from(store: &VendingStore) -> Self {
        Self {
            id: store.id,
            char_id: store.char_id,
            account_id: store.account_id,
            title: store.title.clone(),
            map: store.map.clone(),
            map_instance: store.map_instance,
            x: store.x,
            y: store.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemContainer {
    Inventory,
    Cart,
    Storage,
    GuildStorage,
}

#[derive(Debug, Clone)]
pub enum GuildStorageAccess {
    Open {
        guild_id: u32,
        items: Vec<database::model::InventoryRecord>,
    },
    NoGuild,
    NoPermission,
    Busy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildStorageOpenReceipt {
    pub char_id: u32,
    pub character_revision: u64,
    pub guild_id: u32,
    pub original_personal_open: bool,
    pub original_guild_open: Option<u32>,
    pub personal_open_in_script: bool,
    pub expected_locker: Option<u32>,
    pub result_code: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyingSale {
    pub inventory_id: i32,
    pub item_id: i32,
    pub amount: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptWorldRequest {
    PetLootTarget(u32),
    CreateParty {
        name: String,
        item_pickup: bool,
        item_share: bool,
    },
    InviteParty(u32),
    InvitePartyByName(String),
    AnswerPartyInvite {
        party_id: u32,
        accept: bool,
    },
    LeaveParty,
    ExpelParty {
        account_id: u32,
        name: String,
    },
    ChangePartyLeader(u32),
    ChangePartyOptions {
        exp_share: bool,
        item_rules: Option<(bool, bool)>,
    },
    DisablePartyInvites(bool),
    PartyMessage(String),
    RefreshParty,
    CreateGuild(String),
    InviteGuild(u32),
    AnswerGuildInvite {
        guild_id: u32,
        accept: bool,
    },
    GuildMenu,
    GuildInformation(u32),
    LeaveGuild {
        guild_id: u32,
        reason: String,
    },
    ExpelGuild {
        guild_id: u32,
        member_id: u32,
        reason: String,
    },
    DisbandGuild(String),
    CapturePet(u32),
    HatchPet(u16),
    PetMenu(u8),
    PetRename(String),
    PetEmotion(i32),
    EquipPetAccessory(u16),
    CreateBuyingStore {
        title: String,
        zeny_limit: u32,
        offers: Vec<BuyingOffer>,
    },
    CloseBuyingStore,
    OpenBuyingStore(u32),
    TradeBuyingStore {
        account_id: u32,
        store_id: u32,
        items: Vec<(u16, i32, u16)>,
    },
    SearchStores {
        kind: u8,
        min_price: u32,
        max_price: u32,
        items: Vec<i32>,
        cards: Vec<u16>,
    },
    NextSearchPage,
    CloseStoreSearch,
    LocateStore {
        account_id: u32,
        store_id: u32,
        item_id: i32,
    },
    DismissMercenary(u8),
    StorageDeposit {
        index: u16,
        amount: u32,
    },
    StorageWithdraw {
        index: u16,
        amount: u32,
    },
    CloseStorage,
    SetCart(u8),
    ChangeCart(u8),
    RemoveOption,
    ContainerTransfer {
        source: ItemContainer,
        destination: ItemContainer,
        index: u16,
        amount: u32,
    },
    PrepareVending {
        skill_level: u8,
    },
    CreateVendingStore {
        title: String,
        offers: Vec<(u16, u16, u32)>,
    },
    CloseVendingStore,
    OpenVendingStore(u32),
    PurchaseVendingStore {
        account_id: u32,
        store_id: Option<u32>,
        items: Vec<(u16, u16)>,
    },
    CallHomunculus,
    RestHomunculus,
    ResurrectHomunculus {
        skill_level: u8,
    },
    HomunculusMenu(u8),
    HomunculusRename(String),
    CompanionMove {
        id: u32,
        x: u16,
        y: u16,
    },
    CompanionMoveToOwner(u32),
    CompanionAttack {
        id: u32,
        target: u32,
        repeat: bool,
    },
    UseCompanionSkill {
        skill_id: u32,
        skill_level: u8,
        target_id: u32,
    },
    UseCompanionGroundSkill {
        skill_id: u32,
        skill_level: u8,
        x: u16,
        y: u16,
    },
    FinishCompanionSkill(u32),
    FinishPetSupport(u64),
    PetCombatTarget {
        target_id: u32,
        retaliation: bool,
    },
    CompanionSelfDestruct(u32),
    HealByCompanion {
        source_id: u32,
        hp: u32,
        sp: u32,
    },
    HealCompanion {
        target_id: u32,
        hp: u32,
        sp: u32,
    },
    CompanionAttackLanded {
        id: u32,
        damage: u32,
    },
}

#[derive(WithMaskValueU64)]
pub enum PlayerOption {
    #[mask_value = 8]
    Cart1,
    Falcon,
    Riding,
    #[mask_value = 128]
    Cart2,
    Cart3,
    Cart4,
    Cart5,
    #[mask_value = 4194304]
    Madogear,
}

#[derive(models::enums::WithMaskValueU8)]
pub enum HomunculusInfoFlag {
    #[mask_value = 1]
    Renamed,
    Resting,
    Alive,
}

#[derive(models::enums::WithMaskValueU8)]
pub enum StorageItemFlag {
    #[mask_value = 1]
    Identified,
    Damaged,
}
