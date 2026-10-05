use std::collections::{BTreeMap, BTreeSet};

use database::model::{CharacterRecord, InventoryRecord};
use database::{abort, next_id, read, tx_read, tx_required, tx_write};
use models::enums::EnumWithMaskValueU64;
use models::enums::item::{ItemFlag, ItemTradeFlag, ItemType};
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use super::model::item_model::{InventoryItemModel, ItemModel};
use super::{Error, SledRepository};
use crate::server::model::game_systems::{AccountGameSystems, BuyingSale, BuyingStore, CharacterGameSystems, GuildRecord, PetRecord};
use crate::server::service::script_world_service::{WorldEffectPlan, world_data};

#[path = "game_system_guild_storage_repository.rs"]
mod guild_storage;
#[path = "game_system_guild_management_repository.rs"]
mod guild_management;
#[path = "game_system_party_repository.rs"]
mod party;
#[path = "game_system_pet_loot_repository.rs"]
mod pet_loot;
#[path = "game_system_pet_custody_repository.rs"]
mod pet_custody;
pub use pet_loot::PetLootReturn;
#[path = "player_trade_repository.rs"]
mod player_trade;
pub use player_trade::{PlayerTradeCommit, PlayerTradeSide, PlayerTradeResult, restrictions_allow};
#[path = "game_system_trade_repository.rs"]
mod trade;
pub use trade::{ContainerTransfer, VendingTrade, StoreMove, StoreMoveResult};

use crate::server::model::game_systems::{ItemContainer, VendingStore};

#[derive(Debug, Clone)]
pub struct CommittedWorldEffects {
    pub systems: Option<CharacterGameSystems>,
    pub guild: Option<GuildRecord>,
    pub guild_storage: Option<(u32, Vec<InventoryRecord>)>,
    pub personal_storage: Option<Vec<InventoryRecord>>,
}

pub fn tx_commit_world_effects(
    systems: &TransactionalTree,
    metadata: &TransactionalTree,
    characters: &TransactionalTree,
    char_id: u32,
    plan: &WorldEffectPlan,
) -> ConflictableTransactionResult<CommittedWorldEffects, Error> {
    let current: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
    if current.revision != plan.expected_revision {
        return abort("World state changed while item effects were being prepared");
    }
    let guild_storage = guild_storage::commit_receipts(systems, char_id, plan)?;
    let personal_storage = if plan.open_personal_storage {
        let character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
        Some(tx_read::<Vec<InventoryRecord>>(systems, &key(b"storage/", character.account_id as u32))?.unwrap_or_default())
    } else {
        None
    };
    let saved = if let Some(staged) = &plan.systems {
        let mut staged = staged.clone();
        if staged.revision != current.revision {
            return abort("Invalid staged world revision");
        }
        if let Some(mercenary) = staged.mercenary.as_mut() {
            if mercenary.id == 0 {
                mercenary.id = next_id(metadata, b"companion_id", 0)? as u32;
            }
        }
        if let Some(homunculus) = staged.homunculus.as_mut() {
            if homunculus.id == 0 {
                homunculus.id = next_id(metadata, b"companion_id", 0)? as u32;
            }
        }
        write_state(systems, char_id, &mut staged)?;
        Some(staged)
    } else {
        None
    };
    let guild = if plan.guild_experience > 0 {
        if current.guild_id == 0 {
            return abort("Character no longer belongs to a guild");
        }
        let mut guild: GuildRecord = tx_required(systems, &key(b"guild/", current.guild_id))?;
        if !guild.members.contains(&char_id) {
            return abort("Guild membership is inconsistent");
        }
        guild.experience = guild
            .experience
            .checked_add(plan.guild_experience)
            .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Guild experience overflow".into())))?;
        let requirements = &world_data().guild_experience;
        while usize::from(guild.level) <= requirements.len() {
            let required = requirements[usize::from(guild.level - 1)];
            if required == 0 || guild.experience < required {
                break;
            }
            guild.experience -= required;
            guild.level += 1;
            guild.skill_points = guild.skill_points.saturating_add(1);
        }
        tx_write(systems, &key(b"guild/", guild.id), &guild)?;
        Some(guild)
    } else {
        None
    };
    Ok(CommittedWorldEffects {
        systems: saved,
        guild,
        guild_storage,
        personal_storage,
    })
}

pub struct BuyingTrade {
    pub seller_inventory: Vec<InventoryItemModel>,
    pub buyer_inventory: Vec<InventoryItemModel>,
    pub seller_zeny: u32,
    pub buyer_zeny: u32,
    pub store: Option<BuyingStore>,
}

pub struct StorageTransfer {
    pub storage: Vec<InventoryRecord>,
    pub inventory: Vec<InventoryItemModel>,
}

pub trait GameSystemRepository: Send + Sync {
    fn allocate_player_trade_session_id(&self) -> Result<u64, Error> {
        Err(Error::new("Player trade session persistence is unavailable".into()))
    }
    fn commit_player_trade(&self, _change: &PlayerTradeCommit) -> Result<PlayerTradeResult, Error> {
        Err(Error::new("Player trade persistence is unavailable".into()))
    }
    fn return_pet_loot(&self, _char_id: u32, _expected_revision: u64, _max_weight: u32, _max_slots: usize) -> Result<PetLootReturn, Error> {
        Err(Error::new("Pet cargo persistence is unavailable".into()))
    }
    fn create_party(
        &self,
        _leader: u32,
        _name: String,
        _item_pickup: bool,
        _item_share: bool,
    ) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        Err(Error::new("Party persistence is unavailable".into()))
    }
    fn join_party(&self, _char_id: u32, _party_id: u32, _inviter: u32) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        Err(Error::new("Party persistence is unavailable".into()))
    }
    fn leave_party(&self, _member: u32, _expeller: Option<u32>) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        Err(Error::new("Party persistence is unavailable".into()))
    }
    fn change_party_leader(
        &self,
        _actor: u32,
        _target: u32,
        _expected_revision: u64,
    ) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        Err(Error::new("Party persistence is unavailable".into()))
    }
    fn change_party_options(
        &self,
        _actor: u32,
        _expected_revision: u64,
        _exp_share: bool,
        _item_pickup: bool,
        _item_share: bool,
    ) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        Err(Error::new("Party persistence is unavailable".into()))
    }
    fn party(&self, _party_id: u32) -> Result<Option<crate::server::model::game_systems::PartyRecord>, Error> {
        Ok(None)
    }
    fn party_member_records(&self, _party_id: u32) -> Result<Vec<CharacterRecord>, Error> {
        Ok(Vec::new())
    }
    fn prepare_guild_storage_open(
        &self,
        _char_id: u32,
        _revision: u64,
        _original_personal_open: bool,
        _original_guild_open: Option<u32>,
        _personal_open_in_script: bool,
    ) -> Result<crate::server::model::game_systems::GuildStorageOpenReceipt, Error> {
        Err(Error::new("Guild storage planning is unavailable".into()))
    }
    fn reset_guild_storage_locks(&self) -> Result<(), Error> {
        Ok(())
    }
    fn open_guild_storage(&self, _char_id: u32) -> Result<crate::server::model::game_systems::GuildStorageAccess, Error> {
        Err(Error::new("Guild storage is unavailable".into()))
    }
    fn close_guild_storage(&self, _char_id: u32, _guild_id: u32) -> Result<(), Error> {
        Err(Error::new("Guild storage is unavailable".into()))
    }
    fn reset_item_group_pools(&self) -> Result<(), Error> {
        Ok(())
    }
    fn item_group_pool(&self, _group_id: i32, _subgroup_id: i32) -> Result<crate::server::script::game_data::ItemPoolState, Error> {
        Ok(crate::server::script::game_data::ItemPoolState::default())
    }
    fn commit_item_pool_draws(&self, _receipts: &[crate::server::script::game_data::PoolDrawReceipt]) -> Result<(), Error> {
        Err(Error::new("Shared reward pool persistence is unavailable".into()))
    }
    fn clear_character_stores(&self, _char_id: u32) -> Result<(), Error> {
        Ok(())
    }
    fn move_character_stores(&self, _change: &StoreMove) -> Result<StoreMoveResult, Error> {
        Err(Error::new("Store movement persistence is unavailable".into()))
    }
    fn commit_world_effects(&self, _char_id: u32, _plan: &WorldEffectPlan) -> Result<CommittedWorldEffects, Error> {
        Err(Error::new("World effect persistence is unavailable".into()))
    }
    fn save_game_systems_consuming_item(
        &self,
        _char_id: u32,
        _state: &CharacterGameSystems,
        _item_id: i32,
    ) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Atomic companion persistence is unavailable".into()))
    }
    fn save_game_systems_consuming_items(
        &self,
        _char_id: u32,
        _state: &CharacterGameSystems,
        _costs: &[(i32, u32)],
    ) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Atomic companion skill persistence is unavailable".into()))
    }
    fn character_cart(&self, _char_id: u32) -> Result<Vec<InventoryRecord>, Error> {
        Ok(Vec::new())
    }
    fn set_character_options(&self, _char_id: u32, _expected: u64, _options: u64) -> Result<(), Error> {
        Err(Error::new("Character option persistence is unavailable".into()))
    }
    fn container_transfer(
        &self,
        _char_id: u32,
        _inventory_id: i32,
        _amount: u32,
        _source: ItemContainer,
        _destination: ItemContainer,
        _max_weight: u32,
    ) -> Result<ContainerTransfer, Error> {
        Err(Error::new("Item container persistence is unavailable".into()))
    }
    fn create_vending_store(&self, _store: &VendingStore) -> Result<VendingStore, Error> {
        Err(Error::new("Vending persistence is unavailable".into()))
    }
    fn close_vending_store(&self, _char_id: u32, _store_id: u32) -> Result<(), Error> {
        Err(Error::new("Vending persistence is unavailable".into()))
    }
    fn vending_store_trade(&self, _buyer: u32, _store_id: u32, _purchases: &[(u16, u16)], _max_weight: u32) -> Result<VendingTrade, Error> {
        Err(Error::new("Vending persistence is unavailable".into()))
    }
    fn character_game_systems(&self, _char_id: u32) -> Result<CharacterGameSystems, Error> {
        Ok(CharacterGameSystems::default())
    }
    fn account_game_systems(&self, _account_id: u32) -> Result<AccountGameSystems, Error> {
        Ok(AccountGameSystems::default())
    }
    fn save_character_game_systems(&self, _char_id: u32, _state: &CharacterGameSystems) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Game system persistence is unavailable".into()))
    }
    fn set_vip_expiration(&self, _account_id: u32, _expires_at: u64) -> Result<(), Error> {
        Err(Error::new("Account game system persistence is unavailable".into()))
    }
    fn marry_characters(&self, _first: u32, _second: u32) -> Result<(), Error> {
        Err(Error::new("Marriage persistence is unavailable".into()))
    }
    fn divorce_character(&self, _char_id: u32) -> Result<u32, Error> {
        Err(Error::new("Marriage persistence is unavailable".into()))
    }
    fn adopt_character(&self, _father: u32, _mother: u32, _baby: u32) -> Result<(), Error> {
        Err(Error::new("Adoption persistence is unavailable".into()))
    }
    fn create_guild(&self, _master: u32, _name: String) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn join_guild(&self, _char_id: u32, _guild_id: u32) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild(&self, _guild_id: u32) -> Result<Option<GuildRecord>, Error> {
        Ok(None)
    }
    fn guild_member_records(&self, _guild_id: u32) -> Result<Vec<CharacterRecord>, Error> {
        Ok(Vec::new())
    }
    fn leave_guild(&self, _member: u32, _guild_id: u32, _expeller: Option<u32>) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild departure persistence is unavailable".into()))
    }
    fn disband_guild(&self, _master: u32, _name: &str) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild dissolution persistence is unavailable".into()))
    }
    fn guild_add_experience(&self, _char_id: u32, _amount: u64, _requirements: &[u64]) -> Result<Option<GuildRecord>, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_set_positions(&self, _actor: u32, _positions: Vec<crate::server::model::game_systems::GuildPosition>) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_set_member_position(&self, _actor: u32, _target: u32, _position: u8) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_upgrade_skill(&self, _actor: u32, _skill_id: u32) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_set_notice(&self, _actor: u32, _notice: crate::server::model::game_systems::GuildNotice) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_set_emblem(&self, _actor: u32, _emblem: Vec<u8>) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_invite_member(&self, _inviter: u32, _target: u32) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_expel_member(&self, _actor: u32, _target: u32) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_form_alliance(&self, _master_a: u32, _master_b: u32) -> Result<(GuildRecord, GuildRecord), Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_declare_opposition(&self, _master: u32, _target_guild: u32) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn guild_break_relation(&self, _master: u32, _other_guild: u32) -> Result<GuildRecord, Error> {
        Err(Error::new("Guild persistence is unavailable".into()))
    }
    fn castle_value(&self, _map: &str, _field: u8) -> Result<i32, Error> {
        Err(Error::new("Castle persistence is unavailable".into()))
    }
    fn set_castle_value(&self, _map: &str, _field: u8, _value: i32) -> Result<(), Error> {
        Err(Error::new("Castle persistence is unavailable".into()))
    }
    fn create_pet_egg(&self, _char_id: u32, _pet: &PetRecord, _max_weight: u32) -> Result<(PetRecord, InventoryItemModel), Error> {
        Err(Error::new("Pet persistence is unavailable".into()))
    }
    fn hatch_pet(&self, _char_id: u32, _inventory_id: i32, _now: u64) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Pet persistence is unavailable".into()))
    }
    fn feed_pet(&self, _char_id: u32, _food: i32, _hunger: i32, _intimacy: i32, _now: u64) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Pet persistence is unavailable".into()))
    }
    fn return_pet_to_egg(&self, _char_id: u32) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Pet persistence is unavailable".into()))
    }
    fn pet_accessory(&self, _char_id: u32, _inventory_id: Option<i32>, _accessory_item_id: i32) -> Result<CharacterGameSystems, Error> {
        Err(Error::new("Pet persistence is unavailable".into()))
    }
    fn create_buying_store(&self, _store: &BuyingStore) -> Result<BuyingStore, Error> {
        Err(Error::new("Buying store persistence is unavailable".into()))
    }
    fn buying_store(&self, _store_id: u32) -> Result<Option<BuyingStore>, Error> {
        Ok(None)
    }
    fn close_buying_store(&self, _char_id: u32, _store_id: u32) -> Result<(), Error> {
        Err(Error::new("Buying store persistence is unavailable".into()))
    }
    fn buying_store_trade(
        &self,
        _seller: u32,
        _store_id: u32,
        _sales: &[BuyingSale],
        _buyer_max_weight: u32,
    ) -> Result<BuyingTrade, Error> {
        Err(Error::new("Buying store persistence is unavailable".into()))
    }
    fn account_storage(&self, _account_id: u32) -> Result<Vec<InventoryRecord>, Error> {
        Ok(Vec::new())
    }
    fn storage_transfer(
        &self,
        _char_id: u32,
        _inventory_id: i32,
        _amount: u32,
        _deposit: bool,
        _max_weight: u32,
    ) -> Result<StorageTransfer, Error> {
        Err(Error::new("Storage persistence is unavailable".into()))
    }
}

pub fn character_key(char_id: u32) -> Vec<u8> {
    key(b"character/", char_id)
}
fn key(prefix: &[u8], id: u32) -> Vec<u8> {
    [prefix, &id.to_be_bytes()].concat()
}

fn write_state(tree: &TransactionalTree, char_id: u32, state: &mut CharacterGameSystems) -> ConflictableTransactionResult<(), Error> {
    state.revision = state
        .revision
        .checked_add(1)
        .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Game system revision overflow".into())))?;
    tx_write(tree, &character_key(char_id), state)?;
    if let Some(pet) = &state.pet {
        tx_write(tree, &key(b"pet/", pet.id), pet)?;
    }
    Ok(())
}

fn inventory_models(
    records: &[InventoryRecord],
    items: &TransactionalTree,
) -> ConflictableTransactionResult<Vec<InventoryItemModel>, Error> {
    records
        .iter()
        .map(|record| {
            let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
            Ok(InventoryItemModel::from_record(record, &item))
        })
        .collect()
}

fn inventory_weight(records: &[InventoryRecord], items: &TransactionalTree) -> ConflictableTransactionResult<u64, Error> {
    let mut weight = 0u64;
    for record in records {
        let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
        weight = weight.saturating_add(item.weight.max(0) as u64 * record.amount.max(0) as u64);
    }
    Ok(weight)
}

impl GameSystemRepository for SledRepository {
    fn allocate_player_trade_session_id(&self) -> Result<u64, Error> {
        player_trade::allocate_session(self)
    }
    fn commit_player_trade(&self, change: &PlayerTradeCommit) -> Result<PlayerTradeResult, Error> {
        player_trade::commit(self, change)
    }
    fn return_pet_loot(&self, char_id: u32, expected_revision: u64, max_weight: u32, max_slots: usize) -> Result<PetLootReturn, Error> {
        pet_loot::return_cargo(self, char_id, expected_revision, max_weight, max_slots)
    }
    fn create_party(
        &self,
        leader: u32,
        name: String,
        item_pickup: bool,
        item_share: bool,
    ) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        party::create(self, leader, name, item_pickup, item_share)
    }

    fn join_party(&self, char_id: u32, party_id: u32, inviter: u32) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        party::join(self, char_id, party_id, inviter)
    }

    fn leave_party(&self, member: u32, expeller: Option<u32>) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        party::leave(self, member, expeller)
    }

    fn change_party_leader(
        &self,
        actor: u32,
        target: u32,
        expected_revision: u64,
    ) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        party::leader(self, actor, target, expected_revision)
    }

    fn change_party_options(
        &self,
        actor: u32,
        expected_revision: u64,
        exp_share: bool,
        item_pickup: bool,
        item_share: bool,
    ) -> Result<crate::server::model::game_systems::PartyChange, Error> {
        party::options(self, actor, expected_revision, exp_share, item_pickup, item_share)
    }

    fn party(&self, party_id: u32) -> Result<Option<crate::server::model::game_systems::PartyRecord>, Error> {
        read(&self.database.game_systems, &key(b"party/", party_id))
    }

    fn party_member_records(&self, party_id: u32) -> Result<Vec<CharacterRecord>, Error> {
        party::records(self, party_id)
    }

    fn prepare_guild_storage_open(
        &self,
        char_id: u32,
        revision: u64,
        original_personal_open: bool,
        original_guild_open: Option<u32>,
        personal_open_in_script: bool,
    ) -> Result<crate::server::model::game_systems::GuildStorageOpenReceipt, Error> {
        guild_storage::prepare(
            self,
            char_id,
            revision,
            original_personal_open,
            original_guild_open,
            personal_open_in_script,
        )
    }

    fn reset_guild_storage_locks(&self) -> Result<(), Error> {
        guild_storage::reset_locks(self)
    }

    fn open_guild_storage(&self, char_id: u32) -> Result<crate::server::model::game_systems::GuildStorageAccess, Error> {
        guild_storage::open(self, char_id)
    }

    fn close_guild_storage(&self, char_id: u32, guild_id: u32) -> Result<(), Error> {
        guild_storage::close(self, char_id, guild_id)
    }

    fn reset_item_group_pools(&self) -> Result<(), Error> {
        let keys = self
            .database
            .game_systems
            .scan_prefix(b"item_pool/")
            .map(|entry| entry.map(|(key, _)| key))
            .collect::<Result<Vec<_>, _>>()?;
        self.database.game_systems.transaction(|tree| {
            for key in &keys {
                tree.remove(key.as_ref())?;
            }
            Ok::<_, sled::transaction::ConflictableTransactionError<Error>>(())
        })?;
        Ok(())
    }

    fn item_group_pool(&self, group_id: i32, subgroup_id: i32) -> Result<crate::server::script::game_data::ItemPoolState, Error> {
        Ok(read(
            &self.database.game_systems,
            &crate::server::script::game_data::item_pool_key(group_id, subgroup_id),
        )?
        .unwrap_or_default())
    }

    fn commit_item_pool_draws(&self, receipts: &[crate::server::script::game_data::PoolDrawReceipt]) -> Result<(), Error> {
        self.database
            .game_systems
            .transaction(|tree| crate::server::script::game_data::tx_commit_pool_draws(tree, receipts))?;
        Ok(())
    }

    fn clear_character_stores(&self, char_id: u32) -> Result<(), Error> {
        self.database.game_systems.transaction(|systems| {
            if let Some(id) = tx_read::<u32>(systems, &key(b"buyer/", char_id))? {
                if tx_read::<BuyingStore>(systems, &key(b"store/", id))?.is_some_and(|store| store.char_id != char_id) {
                    return abort("Buying store owner index is inconsistent");
                }
                systems.remove(key(b"store/", id))?;
                systems.remove(key(b"buyer/", char_id))?;
            }
            if let Some(id) = tx_read::<u32>(systems, &key(b"vender/", char_id))? {
                if tx_read::<VendingStore>(systems, &key(b"vending/", id))?.is_some_and(|store| store.char_id != char_id) {
                    return abort("Vending store owner index is inconsistent");
                }
                systems.remove(key(b"vending/", id))?;
                systems.remove(key(b"vender/", char_id))?;
            }
            Ok(())
        })?;
        Ok(())
    }
    fn move_character_stores(&self, change: &StoreMove) -> Result<StoreMoveResult, Error> {
        trade::move_character_stores(self, change)
    }

    fn commit_world_effects(&self, char_id: u32, plan: &WorldEffectPlan) -> Result<CommittedWorldEffects, Error> {
        Ok(
            (&self.database.game_systems, &self.database.characters, &self.database.metadata).transaction(
                |(systems, characters, metadata)| {
                    tx_required::<CharacterRecord>(characters, &(char_id as i32).to_be_bytes())?;
                    tx_commit_world_effects(systems, metadata, characters, char_id, plan)
                },
            )?,
        )
    }

    fn save_game_systems_consuming_item(
        &self,
        char_id: u32,
        state: &CharacterGameSystems,
        item_id: i32,
    ) -> Result<CharacterGameSystems, Error> {
        self.save_game_systems_consuming_items(char_id, state, &[(item_id, 1)])
    }

    fn save_game_systems_consuming_items(
        &self,
        char_id: u32,
        state: &CharacterGameSystems,
        costs: &[(i32, u32)],
    ) -> Result<CharacterGameSystems, Error> {
        let mut quantities = BTreeMap::<i32, u32>::new();
        for &(item_id, amount) in costs {
            if item_id <= 0 || amount == 0 {
                return Err(Error::new("Invalid companion skill item cost".into()));
            }
            let quantity = quantities.entry(item_id).or_default();
            *quantity = quantity
                .checked_add(amount)
                .ok_or_else(|| Error::new("Companion skill item cost overflow".into()))?;
        }
        let db = &self.database;
        Ok((
            &db.game_systems,
            &db.characters,
            &db.inventories,
            &db.inventory_owners,
            &db.metadata,
        )
            .transaction(|(systems, characters, inventories, owners, metadata)| {
                tx_required::<CharacterRecord>(characters, &(char_id as i32).to_be_bytes())?;
                let current: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                if current.revision != state.revision {
                    return abort("Companion state changed");
                }
                let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                for (&item_id, &amount) in &quantities {
                    let mut remaining = amount;
                    for record in inventory
                        .iter_mut()
                        .filter(|record| record.item_id == item_id && record.equip == 0 && record.amount > 0)
                    {
                        let consumed = remaining.min(record.amount as u32);
                        record.amount -= consumed as i16;
                        remaining -= consumed;
                        if record.amount == 0 {
                            owners.remove(record.id.to_be_bytes().to_vec())?;
                        }
                        if remaining == 0 {
                            break;
                        }
                    }
                    if remaining > 0 {
                        return abort("Required companion skill item is missing");
                    }
                }
                inventory.retain(|record| record.amount > 0);
                let mut state = state.clone();
                if let Some(homunculus) = state.homunculus.as_mut() {
                    if homunculus.id == 0 {
                        homunculus.id = next_id(metadata, b"companion_id", 0)? as u32;
                    }
                }
                write_state(systems, char_id, &mut state)?;
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &inventory)?;
                Ok(state)
            })?)
    }

    fn character_cart(&self, char_id: u32) -> Result<Vec<InventoryRecord>, Error> {
        trade::character_cart(self, char_id)
    }

    fn set_character_options(&self, char_id: u32, expected: u64, options: u64) -> Result<(), Error> {
        trade::set_character_options(self, char_id, expected, options)
    }

    fn container_transfer(
        &self,
        char_id: u32,
        record_id: i32,
        amount: u32,
        source: ItemContainer,
        destination: ItemContainer,
        max_weight: u32,
    ) -> Result<ContainerTransfer, Error> {
        trade::container_transfer(self, char_id, record_id, amount, source, destination, max_weight)
    }

    fn create_vending_store(&self, store: &VendingStore) -> Result<VendingStore, Error> {
        trade::create_vending_store(self, store)
    }

    fn close_vending_store(&self, char_id: u32, store_id: u32) -> Result<(), Error> {
        trade::close_vending_store(self, char_id, store_id)
    }

    fn vending_store_trade(&self, buyer: u32, store_id: u32, purchases: &[(u16, u16)], max_weight: u32) -> Result<VendingTrade, Error> {
        trade::vending_store_trade(self, buyer, store_id, purchases, max_weight)
    }

    fn character_game_systems(&self, char_id: u32) -> Result<CharacterGameSystems, Error> {
        Ok(read(&self.database.game_systems, &character_key(char_id))?.unwrap_or_default())
    }

    fn account_game_systems(&self, account_id: u32) -> Result<AccountGameSystems, Error> {
        Ok(read(&self.database.game_systems, &key(b"account/", account_id))?.unwrap_or_default())
    }

    fn save_character_game_systems(&self, char_id: u32, state: &CharacterGameSystems) -> Result<CharacterGameSystems, Error> {
        Ok(
            (&self.database.game_systems, &self.database.characters, &self.database.metadata).transaction(
                |(systems, characters, metadata)| {
                    tx_required::<CharacterRecord>(characters, &(char_id as i32).to_be_bytes())?;
                    let current: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                    if current.revision != state.revision {
                        return abort("Game system state changed before it could be persisted");
                    }
                    if let Some(pet) = &current.pet {
                        if state.pet.is_none() {
                            systems.remove(key(b"pet/", pet.id))?;
                        }
                    }
                    let mut state = state.clone();
                    if let Some(mercenary) = state.mercenary.as_mut() {
                        if mercenary.id == 0 {
                            mercenary.id = next_id(metadata, b"companion_id", 0)? as u32;
                        }
                    }
                    if let Some(homunculus) = state.homunculus.as_mut() {
                        if homunculus.id == 0 {
                            homunculus.id = next_id(metadata, b"companion_id", 0)? as u32;
                        }
                    }
                    write_state(systems, char_id, &mut state)?;
                    Ok(state)
                },
            )?,
        )
    }

    fn set_vip_expiration(&self, account_id: u32, expires_at: u64) -> Result<(), Error> {
        (&self.database.game_systems, &self.database.accounts).transaction(|(systems, accounts)| {
            if accounts.get(account_id.to_be_bytes())?.is_none() {
                return abort("Account does not exist");
            }
            tx_write(systems, &key(b"account/", account_id), &AccountGameSystems {
                vip_expires_at: expires_at,
            })
        })?;
        Ok(())
    }

    fn marry_characters(&self, first: u32, second: u32) -> Result<(), Error> {
        if first == second {
            return Err(Error::new("A character cannot marry itself".into()));
        }
        (&self.database.game_systems, &self.database.characters).transaction(|(systems, characters)| {
            tx_required::<CharacterRecord>(characters, &(first as i32).to_be_bytes())?;
            tx_required::<CharacterRecord>(characters, &(second as i32).to_be_bytes())?;
            let mut a: CharacterGameSystems = tx_read(systems, &character_key(first))?.unwrap_or_default();
            let mut b: CharacterGameSystems = tx_read(systems, &character_key(second))?.unwrap_or_default();
            if a.partner_id != 0 || b.partner_id != 0 {
                return abort("Character is already married");
            }
            a.partner_id = second;
            b.partner_id = first;
            write_state(systems, first, &mut a)?;
            write_state(systems, second, &mut b)
        })?;
        Ok(())
    }

    fn adopt_character(&self, father: u32, mother: u32, baby: u32) -> Result<(), Error> {
        (&self.database.game_systems, &self.database.characters).transaction(|(systems, characters)| {
            for id in [father, mother, baby] {
                tx_required::<CharacterRecord>(characters, &(id as i32).to_be_bytes())?;
            }
            let mut dad: CharacterGameSystems = tx_read(systems, &character_key(father))?.unwrap_or_default();
            let mut mom: CharacterGameSystems = tx_read(systems, &character_key(mother))?.unwrap_or_default();
            let mut kid: CharacterGameSystems = tx_read(systems, &character_key(baby))?.unwrap_or_default();
            if dad.partner_id != mother || mom.partner_id != father {
                return abort("Parents are not married");
            }
            if dad.child_id != 0 || mom.child_id != 0 || kid.father_id != 0 || kid.mother_id != 0 {
                return abort("Adoption is already in place");
            }
            dad.child_id = baby;
            mom.child_id = baby;
            kid.father_id = father;
            kid.mother_id = mother;
            write_state(systems, father, &mut dad)?;
            write_state(systems, mother, &mut mom)?;
            write_state(systems, baby, &mut kid)
        })?;
        Ok(())
    }

    fn divorce_character(&self, char_id: u32) -> Result<u32, Error> {
        Ok(self.database.game_systems.transaction(|systems| {
            let mut own: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
            let partner_id = own.partner_id;
            if partner_id == 0 {
                return abort("Character is not married");
            }
            let mut partner: CharacterGameSystems = tx_read(systems, &character_key(partner_id))?.unwrap_or_default();
            if partner.partner_id == char_id {
                partner.partner_id = 0;
                write_state(systems, partner_id, &mut partner)?;
            }
            own.partner_id = 0;
            write_state(systems, char_id, &mut own)?;
            Ok(partner_id)
        })?)
    }

    fn create_guild(&self, master: u32, name: String) -> Result<GuildRecord, Error> {
        let name = name.trim().to_string();
        if name.is_empty() || name.len() > 23 || name.chars().any(char::is_control) {
            return Err(Error::new("Invalid guild name".into()));
        }
        Ok((
            &self.database.game_systems,
            &self.database.characters,
            &self.database.metadata,
            &self.database.inventories,
            &self.database.inventory_owners,
        )
            .transaction(|(systems, characters, metadata, inventories, owners)| {
                tx_required::<CharacterRecord>(characters, &(master as i32).to_be_bytes())?;
                let mut member: CharacterGameSystems = tx_read(systems, &character_key(master))?.unwrap_or_default();
                if member.guild_id != 0 {
                    return abort("Character already belongs to a guild");
                }
                let name_key = [b"guild_name/".as_slice(), name.as_bytes()].concat();
                if systems.get(&name_key)?.is_some() {
                    return abort("Guild name already exists");
                }
                let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &(master as i32).to_be_bytes())?.unwrap_or_default();
                let index = inventory
                    .iter()
                    .position(|record| record.item_id == 714 && record.amount > 0 && record.equip == 0)
                    .ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::new("Guild creation requires an Emperium".into()))
                    })?;
                inventory[index].amount -= 1;
                if inventory[index].amount == 0 {
                    owners.remove(inventory[index].id.to_be_bytes().to_vec())?;
                    inventory.remove(index);
                }
                let id = next_id(metadata, b"guild_id", 0)? as u32;
                let guild = GuildRecord {
                    id,
                    name: name.clone(),
                    master_char_id: master,
                    members: vec![master],
                    level: 1,
                    experience: 0,
                    skill_points: 0,
                    positions: crate::server::model::game_systems::GuildPosition::defaults(),
                    member_positions: Default::default(),
                    notice: Default::default(),
                    emblem: Vec::new(),
                    emblem_version: 0,
                    allies: Vec::new(),
                    opposition: Vec::new(),
                    skills: Default::default(),
                };
                member.guild_id = id;
                write_state(systems, master, &mut member)?;
                tx_write(systems, &key(b"guild/", id), &guild)?;
                tx_write(systems, &name_key, &id)?;
                tx_write(inventories, &(master as i32).to_be_bytes(), &inventory)?;
                Ok(guild)
            })?)
    }

    fn join_guild(&self, char_id: u32, guild_id: u32) -> Result<GuildRecord, Error> {
        Ok(
            (&self.database.game_systems, &self.database.characters).transaction(|(systems, characters)| {
                tx_required::<CharacterRecord>(characters, &(char_id as i32).to_be_bytes())?;
                let mut member: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                let mut guild: GuildRecord = tx_required(systems, &key(b"guild/", guild_id))?;
                if member.guild_id != 0 || guild.members.len() >= guild.max_members() {
                    return abort("Character already joined a guild or this guild is full");
                }
                member.guild_id = guild_id;
                guild.members.push(char_id);
                write_state(systems, char_id, &mut member)?;
                tx_write(systems, &key(b"guild/", guild_id), &guild)?;
                Ok(guild)
            })?,
        )
    }

    fn guild(&self, guild_id: u32) -> Result<Option<GuildRecord>, Error> {
        read(&self.database.game_systems, &key(b"guild/", guild_id))
    }

    fn guild_member_records(&self, guild_id: u32) -> Result<Vec<CharacterRecord>, Error> {
        Ok(
            (&self.database.game_systems, &self.database.characters).transaction(|(systems, characters)| {
                let guild: GuildRecord = tx_required(systems, &key(b"guild/", guild_id))?;
                guild
                    .members
                    .iter()
                    .map(|member| tx_required(characters, &(*member as i32).to_be_bytes()))
                    .collect()
            })?,
        )
    }

    fn leave_guild(&self, member: u32, guild_id: u32, expeller: Option<u32>) -> Result<GuildRecord, Error> {
        Ok(self.database.game_systems.transaction(|systems| {
            let mut guild: GuildRecord = tx_required(systems, &key(b"guild/", guild_id))?;
            if guild.master_char_id == member || expeller.is_some_and(|expeller| expeller != guild.master_char_id) {
                return abort("Only the guild master can expel ordinary members; a master must dissolve the guild");
            }
            let mut member_state: CharacterGameSystems = tx_required(systems, &character_key(member))?;
            if member_state.guild_id != guild_id || !guild.members.contains(&member) {
                return abort("Character does not belong to this guild");
            }
            member_state.guild_id = 0;
            guild_storage::release_member_lock(systems, member, guild_id)?;
            guild.members.retain(|id| *id != member);
            guild.member_positions.remove(&member);
            write_state(systems, member, &mut member_state)?;
            tx_write(systems, &key(b"guild/", guild_id), &guild)?;
            Ok(guild)
        })?)
    }

    fn disband_guild(&self, master: u32, name: &str) -> Result<GuildRecord, Error> {
        Ok(self.database.game_systems.transaction(|systems| {
            let master_state: CharacterGameSystems = tx_required(systems, &character_key(master))?;
            let guild: GuildRecord = tx_required(systems, &key(b"guild/", master_state.guild_id))?;
            if guild.master_char_id != master || guild.name != name {
                return abort("Only the guild master can dissolve the named guild");
            }
            if guild.members.len() > 1 {
                return abort("Guild still has ordinary members");
            }
            for member in &guild.members {
                let mut state: CharacterGameSystems = tx_required(systems, &character_key(*member))?;
                if state.guild_id != guild.id {
                    return abort("Guild membership is inconsistent");
                }
                state.guild_id = 0;
                guild_storage::release_member_lock(systems, *member, guild.id)?;
                write_state(systems, *member, &mut state)?;
            }
            guild_management::clear_relations(systems, &guild)?;
            systems.remove(key(b"guild/", guild.id))?;
            systems.remove(key(b"guild_storage/", guild.id))?;
            systems.remove([b"guild_name/".as_slice(), guild.name.as_bytes()].concat())?;
            Ok(guild)
        })?)
    }

    fn guild_add_experience(&self, char_id: u32, amount: u64, requirements: &[u64]) -> Result<Option<GuildRecord>, Error> {
        if amount == 0 {
            return Err(Error::new("Guild experience must be positive".into()));
        }
        Ok(self.database.game_systems.transaction(|systems| {
            let member: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
            if member.guild_id == 0 {
                return Ok(None);
            }
            let mut guild: GuildRecord = tx_required(systems, &key(b"guild/", member.guild_id))?;
            if !guild.members.contains(&char_id) {
                return abort("Guild membership is inconsistent");
            }
            guild.experience = guild
                .experience
                .checked_add(amount)
                .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Guild experience overflow".into())))?;
            while usize::from(guild.level) <= requirements.len() {
                let required = requirements[usize::from(guild.level - 1)];
                if required == 0 || guild.experience < required {
                    break;
                }
                guild.experience -= required;
                guild.level += 1;
                guild.skill_points = guild.skill_points.saturating_add(1);
            }
            tx_write(systems, &key(b"guild/", guild.id), &guild)?;
            Ok(Some(guild))
        })?)
    }

    fn guild_set_positions(&self, actor: u32, positions: Vec<crate::server::model::game_systems::GuildPosition>) -> Result<GuildRecord, Error> {
        guild_management::set_positions(self, actor, positions)
    }
    fn guild_set_member_position(&self, actor: u32, target: u32, position: u8) -> Result<GuildRecord, Error> {
        guild_management::set_member_position(self, actor, target, position)
    }
    fn guild_upgrade_skill(&self, actor: u32, skill_id: u32) -> Result<GuildRecord, Error> {
        guild_management::upgrade_skill(self, actor, skill_id)
    }
    fn guild_set_notice(&self, actor: u32, notice: crate::server::model::game_systems::GuildNotice) -> Result<GuildRecord, Error> {
        guild_management::set_notice(self, actor, notice)
    }
    fn guild_set_emblem(&self, actor: u32, emblem: Vec<u8>) -> Result<GuildRecord, Error> {
        guild_management::set_emblem(self, actor, emblem)
    }
    fn guild_invite_member(&self, inviter: u32, target: u32) -> Result<GuildRecord, Error> {
        guild_management::invite_member(self, inviter, target)
    }
    fn guild_expel_member(&self, actor: u32, target: u32) -> Result<GuildRecord, Error> {
        guild_management::expel_member(self, actor, target)
    }
    fn guild_form_alliance(&self, master_a: u32, master_b: u32) -> Result<(GuildRecord, GuildRecord), Error> {
        guild_management::form_alliance(self, master_a, master_b)
    }
    fn guild_declare_opposition(&self, master: u32, target_guild: u32) -> Result<GuildRecord, Error> {
        guild_management::declare_opposition(self, master, target_guild)
    }
    fn guild_break_relation(&self, master: u32, other_guild: u32) -> Result<GuildRecord, Error> {
        guild_management::break_relation(self, master, other_guild)
    }

    fn castle_value(&self, map: &str, field: u8) -> Result<i32, Error> {
        guild_management::castle_value(self, map, field)
    }
    fn set_castle_value(&self, map: &str, field: u8, value: i32) -> Result<(), Error> {
        guild_management::set_castle_value(self, map, field, value)
    }

    fn create_pet_egg(&self, char_id: u32, pet: &PetRecord, max_weight: u32) -> Result<(PetRecord, InventoryItemModel), Error> {
        Ok((
            &self.database.game_systems,
            &self.database.characters,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.items,
            &self.database.metadata,
        )
            .transaction(|(systems, characters, inventories, owners, items, metadata)| {
                let character: CharacterRecord = tx_required(characters, &(char_id as i32).to_be_bytes())?;
                let mut records: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                let item: ItemModel = tx_required(items, &pet.egg_item_id.to_be_bytes())?;
                if records.len() >= character.inventory_slots as usize
                    || inventory_weight(&records, items)? + item.weight.max(0) as u64 > u64::from(max_weight)
                {
                    return abort("Cannot carry the pet egg");
                }
                let mut pet = pet.clone();
                pet.id = next_id(metadata, b"pet_id", 0)? as u32;
                pet.owner_char_id = char_id;
                pet.egg_inventory_id = next_id(metadata, b"inventory_id", 0)?;
                pet.incubating = true;
                let record = pet_egg(&pet);
                records.push(record.clone());
                tx_write(systems, &key(b"pet/", pet.id), &pet)?;
                tx_write(owners, &record.id.to_be_bytes(), &(char_id as i32))?;
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &records)?;
                Ok((pet, InventoryItemModel::from_record(&record, &item)))
            })?)
    }

    fn hatch_pet(&self, char_id: u32, inventory_id: i32, now: u64) -> Result<CharacterGameSystems, Error> {
        Ok((
            &self.database.game_systems,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.items,
        )
            .transaction(|(systems, inventories, owners, items)| {
                let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                if state.pet.is_some() {
                    return abort("A pet is already active");
                }
                let mut records: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                let index = records
                    .iter()
                    .position(|item| item.id == inventory_id && item.card0 == 256 && item.amount == 1 && item.equip == 0)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let item: ItemModel = tx_required(items, &records[index].item_id.to_be_bytes())?;
                let owner: i32 = tx_required(owners, &inventory_id.to_be_bytes())?;
                if item.item_type != ItemType::PetEgg || owner != char_id as i32 {
                    return abort("Invalid pet egg");
                }
                let mut pet = pet_custody::incubating_pet(systems, &records[index])?;
                pet.owner_char_id = char_id;
                pet.egg_inventory_id = inventory_id;
                pet.incubating = false;
                pet.next_hunger_at = now + 60_000;
                state.pet = Some(pet);
                records.remove(index);
                owners.remove(inventory_id.to_be_bytes().to_vec())?;
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &records)?;
                write_state(systems, char_id, &mut state)?;
                Ok(state)
            })?)
    }

    fn feed_pet(&self, char_id: u32, food: i32, hunger: i32, intimacy: i32, now: u64) -> Result<CharacterGameSystems, Error> {
        Ok((
            &self.database.game_systems,
            &self.database.inventories,
            &self.database.inventory_owners,
        )
            .transaction(|(systems, inventories, owners)| {
                let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                let pet = state
                    .pet
                    .as_mut()
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let mut records: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                let index = records
                    .iter()
                    .position(|item| item.item_id == food && item.amount > 0 && item.equip == 0)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                records[index].amount -= 1;
                if records[index].amount == 0 {
                    owners.remove(records[index].id.to_be_bytes().to_vec())?;
                    records.remove(index);
                }
                pet.hunger = (pet.hunger + hunger).clamp(0, 100);
                pet.intimacy = (pet.intimacy + intimacy).clamp(0, 1000);
                pet.next_hunger_at = now;
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &records)?;
                write_state(systems, char_id, &mut state)?;
                Ok(state)
            })?)
    }

    fn return_pet_to_egg(&self, char_id: u32) -> Result<CharacterGameSystems, Error> {
        Ok((
            &self.database.game_systems,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.characters,
        )
            .transaction(|(systems, inventories, owners, characters)| {
                let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                let mut pet = state
                    .pet
                    .take()
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let character: CharacterRecord = tx_required(characters, &(char_id as i32).to_be_bytes())?;
                let mut records: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                if records.len() >= character.inventory_slots as usize {
                    return abort("Inventory is full");
                }
                pet.incubating = true;
                records.push(pet_egg(&pet));
                tx_write(owners, &pet.egg_inventory_id.to_be_bytes(), &(char_id as i32))?;
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &records)?;
                tx_write(systems, &key(b"pet/", pet.id), &pet)?;
                write_state(systems, char_id, &mut state)?;
                Ok(state)
            })?)
    }

    fn pet_accessory(&self, char_id: u32, inventory_id: Option<i32>, accessory_item_id: i32) -> Result<CharacterGameSystems, Error> {
        Ok((
            &self.database.game_systems,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.characters,
            &self.database.items,
            &self.database.metadata,
        )
            .transaction(|(systems, inventories, owners, characters, items, metadata)| {
                let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                let pet = state
                    .pet
                    .as_mut()
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let mut records: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                if let Some(inventory_id) = inventory_id {
                    if pet.equipped_item != 0 {
                        return abort("Pet already has an accessory");
                    }
                    let index = records
                        .iter()
                        .position(|record| record.id == inventory_id && record.item_id == accessory_item_id && record.equip == 0)
                        .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                    let item: ItemModel = tx_required(items, &accessory_item_id.to_be_bytes())?;
                    if item.item_type != ItemType::PetArmor || records[index].amount != 1 {
                        return abort("Item is not a pet accessory");
                    }
                    pet.equipped_item = accessory_item_id;
                    owners.remove(records[index].id.to_be_bytes().to_vec())?;
                    records.remove(index);
                } else {
                    if pet.equipped_item == 0 {
                        return Ok(state);
                    }
                    let character: CharacterRecord = tx_required(characters, &(char_id as i32).to_be_bytes())?;
                    if records.len() >= character.inventory_slots as usize {
                        return abort("Inventory is full");
                    }
                    let id = next_id(metadata, b"inventory_id", 0)?;
                    let item = InventoryRecord {
                        id,
                        unique_id: i64::from(id),
                        item_id: pet.equipped_item,
                        amount: 1,
                        is_identified: true,
                        ..InventoryRecord::default()
                    };
                    pet.equipped_item = 0;
                    records.push(item);
                    tx_write(owners, &id.to_be_bytes(), &(char_id as i32))?;
                }
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &records)?;
                write_state(systems, char_id, &mut state)?;
                Ok(state)
            })?)
    }

    fn create_buying_store(&self, store: &BuyingStore) -> Result<BuyingStore, Error> {
        if store.title.is_empty() || store.title.len() > 79 || store.offers.is_empty() || store.offers.len() > 5 || store.zeny_limit == 0 {
            return Err(Error::new("Invalid buying store".into()));
        }
        Ok((
            &self.database.game_systems,
            &self.database.characters,
            &self.database.items,
            &self.database.inventories,
            &self.database.metadata,
        )
            .transaction(|(systems, characters, items, inventories, metadata)| {
                let character: CharacterRecord = tx_required(characters, &(store.char_id as i32).to_be_bytes())?;
                if character.account_id as u32 != store.account_id || character.zeny < 0 || store.zeny_limit > character.zeny as u32 {
                    return abort("Buying budget exceeds available zeny");
                }
                if let Some(old) = tx_read::<u32>(systems, &key(b"buyer/", store.char_id))? {
                    systems.remove(key(b"store/", old))?;
                }
                let records: Vec<InventoryRecord> = tx_read(inventories, &(store.char_id as i32).to_be_bytes())?.unwrap_or_default();
                let mut unique = BTreeSet::new();
                for offer in &store.offers {
                    let item: ItemModel = tx_required(items, &offer.item_id.to_be_bytes())?;
                    if offer.amount == 0
                        || offer.amount > 9999
                        || offer.price == 0
                        || offer.price > 99_990_000
                        || !unique.insert(offer.item_id)
                        || !item.item_type.is_stackable()
                        || item.flags & ItemFlag::BuyingStore.as_flag() == 0
                        || item.trade_flags & ItemTradeFlag::NoTrade.as_flag() != 0
                        || !records.iter().any(|record| record.item_id == offer.item_id && record.amount > 0)
                    {
                        return abort("Buying store contains an invalid offer");
                    }
                }
                let mut store = store.clone();
                store.id = next_id(metadata, b"buying_store_id", 0)? as u32;
                tx_write(systems, &key(b"store/", store.id), &store)?;
                tx_write(systems, &key(b"buyer/", store.char_id), &store.id)?;
                Ok(store)
            })?)
    }

    fn buying_store(&self, store_id: u32) -> Result<Option<BuyingStore>, Error> {
        read(&self.database.game_systems, &key(b"store/", store_id))
    }

    fn close_buying_store(&self, char_id: u32, store_id: u32) -> Result<(), Error> {
        self.database.game_systems.transaction(|systems| {
            if let Some(store) = tx_read::<BuyingStore>(systems, &key(b"store/", store_id))? {
                if store.char_id != char_id {
                    return abort("Buying store belongs to another character");
                }
                systems.remove(key(b"store/", store_id))?;
            }
            systems.remove(key(b"buyer/", char_id))?;
            Ok(())
        })?;
        Ok(())
    }

    fn buying_store_trade(&self, seller: u32, store_id: u32, sales: &[BuyingSale], buyer_max_weight: u32) -> Result<BuyingTrade, Error> {
        if sales.is_empty() || sales.len() > 100 {
            return Err(Error::new("Invalid buying store sale".into()));
        }
        Ok((
            &self.database.game_systems,
            &self.database.characters,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.items,
            &self.database.metadata,
        )
            .transaction(|(systems, characters, inventories, owners, items, metadata)| {
                let mut store: BuyingStore = tx_required(systems, &key(b"store/", store_id))?;
                if store.char_id == seller {
                    return abort("Cannot sell to your own buying store");
                }
                let mut buyer: CharacterRecord = tx_required(characters, &(store.char_id as i32).to_be_bytes())?;
                let mut seller_char: CharacterRecord = tx_required(characters, &(seller as i32).to_be_bytes())?;
                let mut buyer_records: Vec<InventoryRecord> =
                    tx_read(inventories, &(store.char_id as i32).to_be_bytes())?.unwrap_or_default();
                let mut seller_records: Vec<InventoryRecord> = tx_read(inventories, &(seller as i32).to_be_bytes())?.unwrap_or_default();
                let mut used = BTreeSet::new();
                let mut total = 0u64;
                for sale in sales {
                    if sale.amount == 0 || sale.amount > i16::MAX as u16 || !used.insert(sale.inventory_id) {
                        return abort("Invalid or duplicate sale item");
                    }
                    let index = seller_records
                        .iter()
                        .position(|item| item.id == sale.inventory_id && item.item_id == sale.item_id)
                        .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                    let source = &seller_records[index];
                    if source.equip != 0
                        || source.amount < sale.amount as i16
                        || source.unique_id != 0
                        || [source.card0, source.card1, source.card2, source.card3]
                            .iter()
                            .any(|card| *card != 0)
                    {
                        return abort("Cannot sell this inventory item");
                    }
                    let offer = store
                        .offers
                        .iter_mut()
                        .find(|offer| offer.item_id == sale.item_id)
                        .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                    if offer.amount < sale.amount {
                        return abort("Sale exceeds requested quantity");
                    }
                    total = total.checked_add(u64::from(offer.price) * u64::from(sale.amount)).ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::new("Buying store price overflow".into()))
                    })?;
                    offer.amount -= sale.amount;
                    let item: ItemModel = tx_required(items, &sale.item_id.to_be_bytes())?;
                    if !item.item_type.is_stackable()
                        || item.flags & ItemFlag::BuyingStore.as_flag() == 0
                        || item.trade_flags & ItemTradeFlag::NoTrade.as_flag() != 0
                    {
                        return abort("Item cannot be traded in buying stores");
                    }
                    if let Some(existing) = buyer_records
                        .iter_mut()
                        .find(|record| record.item_id == sale.item_id && record.unique_id == 0)
                    {
                        existing.amount = existing.amount.checked_add(sale.amount as i16).ok_or_else(|| {
                            sled::transaction::ConflictableTransactionError::Abort(Error::new("Inventory stack overflow".into()))
                        })?;
                    } else {
                        let id = next_id(metadata, b"inventory_id", 0)?;
                        buyer_records.push(InventoryRecord {
                            id,
                            item_id: sale.item_id,
                            amount: sale.amount as i16,
                            is_identified: true,
                            ..InventoryRecord::default()
                        });
                        tx_write(owners, &id.to_be_bytes(), &(store.char_id as i32))?;
                    }
                    seller_records[index].amount -= sale.amount as i16;
                    if seller_records[index].amount == 0 {
                        owners.remove(seller_records[index].id.to_be_bytes().to_vec())?;
                        seller_records.remove(index);
                    }
                }
                if total > u64::from(store.zeny_limit)
                    || buyer.zeny < 0
                    || total > buyer.zeny as u64
                    || total + seller_char.zeny.max(0) as u64 > i32::MAX as u64
                    || buyer_records.len() > buyer.inventory_slots as usize
                    || inventory_weight(&buyer_records, items)? > u64::from(buyer_max_weight)
                {
                    return abort("Buying store trade exceeds budget, zeny, slots or weight");
                }
                buyer.zeny -= total as i32;
                seller_char.zeny += total as i32;
                store.zeny_limit -= total as u32;
                tx_write(characters, &(store.char_id as i32).to_be_bytes(), &buyer)?;
                tx_write(characters, &(seller as i32).to_be_bytes(), &seller_char)?;
                tx_write(inventories, &(store.char_id as i32).to_be_bytes(), &buyer_records)?;
                tx_write(inventories, &(seller as i32).to_be_bytes(), &seller_records)?;
                let seller_inventory = inventory_models(&seller_records, items)?;
                let buyer_inventory = inventory_models(&buyer_records, items)?;
                let keep_open = store.offers.iter().any(|offer| offer.amount > 0 && offer.price <= store.zeny_limit);
                let store = if keep_open {
                    tx_write(systems, &key(b"store/", store.id), &store)?;
                    Some(store)
                } else {
                    systems.remove(key(b"store/", store.id))?;
                    systems.remove(key(b"buyer/", store.char_id))?;
                    None
                };
                Ok(BuyingTrade {
                    seller_inventory,
                    buyer_inventory,
                    seller_zeny: seller_char.zeny as u32,
                    buyer_zeny: buyer.zeny as u32,
                    store,
                })
            })?)
    }

    fn account_storage(&self, account_id: u32) -> Result<Vec<InventoryRecord>, Error> {
        Ok(read(&self.database.game_systems, &key(b"storage/", account_id))?.unwrap_or_default())
    }

    fn storage_transfer(
        &self,
        char_id: u32,
        inventory_id: i32,
        amount: u32,
        deposit: bool,
        max_weight: u32,
    ) -> Result<StorageTransfer, Error> {
        if amount == 0 || amount > i16::MAX as u32 {
            return Err(Error::new("Invalid storage transfer amount".into()));
        }
        Ok((
            &self.database.game_systems,
            &self.database.characters,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.items,
            &self.database.metadata,
        )
            .transaction(|(systems, characters, inventories, owners, items, metadata)| {
                let character: CharacterRecord = tx_required(characters, &(char_id as i32).to_be_bytes())?;
                let storage_key = key(b"storage/", character.account_id as u32);
                let mut storage: Vec<InventoryRecord> = tx_read(systems, &storage_key)?.unwrap_or_default();
                let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
                let (source, destination) = if deposit {
                    (&mut inventory, &mut storage)
                } else {
                    (&mut storage, &mut inventory)
                };
                let index = source
                    .iter()
                    .position(|record| record.id == inventory_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let record = source[index].clone();
                let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
                let masks = trade::trade_masks(items, &item, &record)?;
                if record.equip != 0
                    || record.amount < amount as i16
                    || (!item.item_type.is_stackable() && (record.amount != 1 || amount != 1))
                    || (deposit && masks.iter().any(|mask| mask & ItemTradeFlag::NoStorage.as_flag() != 0))
                {
                    return abort("Item cannot be moved to or from storage");
                }
                if deposit && tx_required::<i32>(owners, &record.id.to_be_bytes())? != char_id as i32 {
                    return abort("Storage item ownership changed");
                }
                let stack = item.item_type.is_stackable() && record.unique_id == 0;
                let existing = if stack {
                    destination.iter().position(|target| {
                        target.item_id == record.item_id
                            && target.unique_id == 0
                            && target.card0 == record.card0
                            && target.card1 == record.card1
                            && target.card2 == record.card2
                            && target.card3 == record.card3
                            && target.is_identified == record.is_identified
                            && target.refine == record.refine
                            && target.is_damaged == record.is_damaged
                    })
                } else {
                    None
                };
                let whole = record.amount == amount as i16;
                let moved_id;
                if let Some(existing) = existing {
                    destination[existing].amount = destination[existing].amount.checked_add(amount as i16).ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::new("Storage or inventory stack overflow".into()))
                    })?;
                    moved_id = destination[existing].id;
                } else {
                    if destination.iter().any(|target| target.id == record.id || (record.unique_id != 0 && target.unique_id == record.unique_id)) {
                        return abort("Storage item identity already exists at the destination");
                    }
                    let mut moved = record.clone();
                    moved.amount = amount as i16;
                    if !whole {
                        moved.id = next_id(metadata, b"inventory_id", 0)?;
                    }
                    moved_id = moved.id;
                    destination.push(moved);
                }
                if item.item_type == ItemType::PetEgg && record.card0 == 256 {
                    pet_custody::transfer_egg(systems, &record, moved_id, if deposit { 0 } else { char_id }, char_id)?;
                }
                if whole {
                    source.remove(index);
                    owners.remove(record.id.to_be_bytes().to_vec())?;
                } else {
                    source[index].amount -= amount as i16;
                }
                if !deposit {
                    tx_write(owners, &moved_id.to_be_bytes(), &(char_id as i32))?;
                }
                if inventory.len() > character.inventory_slots as usize
                    || storage.len() > 600
                    || (!deposit && inventory_weight(&inventory, items)? > u64::from(max_weight))
                {
                    return abort("Storage transfer exceeds slots or weight");
                }
                tx_write(systems, &storage_key, &storage)?;
                tx_write(inventories, &(char_id as i32).to_be_bytes(), &inventory)?;
                let inventory = inventory_models(&inventory, items)?;
                Ok(StorageTransfer { storage, inventory })
            })?)
    }
}

fn pet_egg(pet: &PetRecord) -> InventoryRecord {
    InventoryRecord {
        id: pet.egg_inventory_id,
        unique_id: i64::from(pet.id),
        item_id: pet.egg_item_id,
        amount: 1,
        is_identified: true,
        card0: 256,
        card1: pet.id as u16 as i16,
        card2: (pet.id >> 16) as u16 as i16,
        card3: i16::from(pet.renamed),
        ..InventoryRecord::default()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};

    use database::model::{AccountRecord, CharacterInventory, SeedData};
    use database::{required, tx_write};

    use super::*;

    fn repository() -> SledRepository {
        let repository = SledRepository::temporary().unwrap();
        repository
            .database
            .seed(
                &SeedData {
                    accounts: vec![AccountRecord {
                        account_id: 2_000_000,
                        username: "world-test".into(),
                        password: "password".into(),
                    }],
                    characters: vec![
                        CharacterRecord {
                            char_id: 150_000,
                            account_id: 2_000_000,
                            char_num: 0,
                            name: "Buyer".into(),
                            inventory_slots: 100,
                            zeny: 1000,
                            ..CharacterRecord::default()
                        },
                        CharacterRecord {
                            char_id: 150_001,
                            account_id: 2_000_000,
                            char_num: 1,
                            name: "Seller".into(),
                            inventory_slots: 100,
                            zeny: 0,
                            ..CharacterRecord::default()
                        },
                    ],
                    inventories: vec![
                        CharacterInventory {
                            char_id: 150_000,
                            items: vec![InventoryRecord {
                                id: 1,
                                item_id: 501,
                                amount: 1,
                                is_identified: true,
                                ..InventoryRecord::default()
                            }],
                        },
                        CharacterInventory {
                            char_id: 150_001,
                            items: vec![InventoryRecord {
                                id: 2,
                                item_id: 501,
                                amount: 5,
                                is_identified: true,
                                ..InventoryRecord::default()
                            }],
                        },
                    ],
                    ..SeedData::default()
                },
                false,
            )
            .unwrap();
        repository.database.items.transaction(|items| {
            for (id, item_type) in [(501i32, "Healing"), (9001i32, "PetEgg"), (10000i32, "PetArmor")] {
                let item: ItemModel = serde_json::from_value(serde_json::json!({"id": id, "name_aegis": format!("Item{id}"), "name_english": format!("Item {id}"),
                    "item_type": item_type, "weight": 10, "job_flags": 0, "class_flags": 0, "location": 0, "flags": ItemFlag::BuyingStore.as_flag(), "trade_flags": 0})).unwrap();
                tx_write(items, &id.to_be_bytes(), &item)?;
            }
            Ok(())
        }).unwrap();
        repository
    }

    fn store() -> BuyingStore {
        BuyingStore {
            id: 0,
            char_id: 150_000,
            account_id: 2_000_000,
            title: "Buying potions".into(),
            map: "prontera".into(),
            map_instance: 0,
            x: 100,
            y: 100,
            zeny_limit: 100,
            offers: vec![crate::server::model::game_systems::BuyingOffer {
                item_id: 501,
                amount: 4,
                price: 30,
            }],
        }
    }

    fn guild_storage_fixture() -> (SledRepository, GuildRecord) {
        let repository = repository();
        (&repository.database.inventories, &repository.database.inventory_owners)
            .transaction(|(inventories, owners)| {
                let mut inventory: Vec<InventoryRecord> = tx_required(inventories, &150_000_u32.to_be_bytes())?;
                inventory.push(InventoryRecord {
                    id: 50,
                    item_id: 714,
                    amount: 1,
                    is_identified: true,
                    ..InventoryRecord::default()
                });
                tx_write(inventories, &150_000_u32.to_be_bytes(), &inventory)?;
                tx_write(owners, &50_i32.to_be_bytes(), &150_000_i32)
            })
            .unwrap();
        let guild = repository.create_guild(150_000, "Storage Guild".into()).unwrap();
        (repository, guild)
    }

    fn guild_storage_consumption(
        repository: &SledRepository,
    ) -> (
        crate::server::state::character::Character,
        crate::repository::script_inventory_repository::ScriptInventoryTransaction,
    ) {
        use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemConsumption, ScriptItemGrant};
        let mut character = crate::server::state::character::Character::new(
            "Buyer".into(),
            150_000,
            2_000_000,
            models::status::Status::default(),
            1,
            1,
            0,
            "prontera".into(),
            1,
            vec![],
        );
        character.game_systems = repository.character_game_systems(150_000).unwrap();
        let change = ScriptInventoryTransaction {
            char_id: 150_000,
            account_id: 2_000_000,
            consumption: Some(ScriptItemConsumption {
                inventory_id: 1,
                item_id: 501,
                unique_id: 0,
                amount: 1,
            }),
            exact_removals: vec![],
            removals: vec![],
            identifications: vec![],
            grants: vec![ScriptItemGrant {
                item_id: 501,
                amount: 1,
                identified: true,
                refine: 0,
                cards: [0; 4],
                unique_id: None,
                damaged: false,
            }],
            variables: vec![],
            zeny: None,
            hp: None,
            sp: None,
            max_weight: 1000,
            max_slots: 100,
            world: None,
            reset_skills: None,
            fame: None,
            character_changes: vec![],
            pool_draws: vec![],
        };
        (character, change)
    }

    #[test]
    fn guild_storage_item_receipts_acquire_with_consumption_and_reject_stale_or_forged_results() {
        use crate::repository::script_inventory_repository::ScriptInventoryRepository;
        use crate::server::model::game_systems::GuildStorageAccess;
        use crate::server::service::script_world_service::{attach_guild_storage_receipts, plan_persistent_effects};
        let (repository, guild) = guild_storage_fixture();
        repository.join_guild(150_001, guild.id).unwrap();
        let (character, mut change) = guild_storage_consumption(&repository);
        let receipt = repository
            .prepare_guild_storage_open(character.char_id, character.game_systems.revision, false, None, false)
            .unwrap();
        assert_eq!(receipt.result_code, 0);
        let mut plan = plan_persistent_effects(&character, &[(script_sdk::Function::SetFont, vec![7.into()])], 1000).unwrap();
        attach_guild_storage_receipts(&character, &mut plan, &[receipt.clone()]).unwrap();
        change.world = Some(plan.clone());
        let before_id = read::<i32>(&repository.database.metadata, b"inventory_id").unwrap();
        repository.open_guild_storage(150_001).unwrap();
        assert!(repository.script_inventory_transaction(&change).is_err());
        assert_eq!(read::<i32>(&repository.database.metadata, b"inventory_id").unwrap(), before_id);
        assert_eq!(repository.character_game_systems(character.char_id).unwrap().font, 0);
        let unchanged: Vec<InventoryRecord> = required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!((unchanged.len(), unchanged[0].id, unchanged[0].amount), (1, 1, 1));
        repository.close_guild_storage(150_001, guild.id).unwrap();
        let mut forged = receipt.clone();
        forged.result_code = 5;
        attach_guild_storage_receipts(&character, &mut plan, &[forged]).unwrap();
        change.world = Some(plan.clone());
        assert!(repository.script_inventory_transaction(&change).is_err());
        assert!(
            repository
                .database
                .game_systems
                .get(key(b"guild_storage_locker/", guild.id))
                .unwrap()
                .is_none()
        );
        attach_guild_storage_receipts(&character, &mut plan, &[receipt]).unwrap();
        change.world = Some(plan);
        let committed = repository.script_inventory_transaction(&change).unwrap();
        assert!(committed.world.unwrap().guild_storage.is_some());
        assert_eq!(repository.character_game_systems(character.char_id).unwrap().font, 7);
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::Busy
        ));
    }

    #[test]
    fn guild_storage_item_host_predicts_codes_without_opening_and_overlays_personal_and_guild_windows() {
        use std::sync::Arc;

        use script_runtime::Host;
        use script_sdk::{Function, Request, Value};

        use crate::repository::script_inventory_repository::ScriptInventoryRepository;
        use crate::server::script::item_script_handler::{ItemEffect, ItemScriptHost};
        use crate::server::service::script_world_service::{attach_guild_storage_receipts, plan_persistent_effects};
        let (repository, guild) = guild_storage_fixture();
        let repository = Arc::new(repository);
        let (character, mut change) = guild_storage_consumption(&repository);
        let mut host = ItemScriptHost::consumable(models::status::Status::default(), 501)
            .with_pool_repository(repository.clone())
            .with_guild_storage_context(character.char_id, &character.game_systems);
        let mut invoke = |function| {
            futures::executor::block_on(host.invoke(Request::Call {
                function,
                arguments: vec![],
            }))
            .unwrap()
        };
        assert_eq!(invoke(Function::GuildOpenStorage), Value::Number(0));
        assert_eq!(invoke(Function::GuildOpenStorage), Value::Number(2));
        assert_eq!(invoke(Function::OpenStorage), Value::Number(1));
        assert_eq!(host.effects.len(), 1);
        assert!(
            repository
                .database
                .game_systems
                .get(key(b"guild_storage_locker/", guild.id))
                .unwrap()
                .is_none()
        );
        let mut personal = ItemScriptHost::consumable(models::status::Status::default(), 501)
            .with_pool_repository(repository.clone())
            .with_guild_storage_context(character.char_id, &character.game_systems);
        futures::executor::block_on(personal.invoke(Request::Call {
            function: Function::OpenStorage,
            arguments: vec![],
        }))
        .unwrap();
        assert_eq!(
            futures::executor::block_on(personal.invoke(Request::Call {
                function: Function::GuildOpenStorage,
                arguments: vec![]
            }))
            .unwrap(),
            Value::Number(1)
        );
        let receipt = match &personal.effects[1] {
            ItemEffect::GuildStorageOpen(receipt) => receipt,
            _ => panic!("Guild opener should stage its receipt"),
        };
        let mut plan = plan_persistent_effects(&character, &[(Function::OpenStorage, vec![])], 1000).unwrap();
        attach_guild_storage_receipts(&character, &mut plan, &[receipt.clone()]).unwrap();
        change.world = Some(plan);
        let saved = repository.script_inventory_transaction(&change).unwrap().world.unwrap();
        assert!(saved.personal_storage.is_some());
        assert!(saved.guild_storage.is_none());
        assert!(
            repository
                .database
                .game_systems
                .get(key(b"guild_storage_locker/", guild.id))
                .unwrap()
                .is_none()
        );
        let mut moved = character.game_systems.clone();
        moved.storage_open = true;
        let mut current = character;
        current.game_systems = moved;
        let mut original_plan = plan_persistent_effects(&current, &[], 1000).unwrap();
        assert!(attach_guild_storage_receipts(&current, &mut original_plan, &[receipt.clone()]).is_err());
    }

    #[test]
    fn guild_storage_checks_membership_and_exclusive_access_on_every_transfer() {
        use crate::server::model::game_systems::{GuildStorageAccess, ItemContainer};
        let (repository, guild) = guild_storage_fixture();
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::NoGuild
        ));
        assert!(
            matches!(repository.open_guild_storage(150_000).unwrap(), GuildStorageAccess::Open { guild_id, .. } if guild_id == guild.id)
        );
        repository.join_guild(150_001, guild.id).unwrap();
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::Busy
        ));
        assert!(repository.close_guild_storage(150_001, guild.id).is_err());
        assert!(
            repository
                .container_transfer(150_001, 2, 1, ItemContainer::Inventory, ItemContainer::GuildStorage, 1000)
                .is_err()
        );
        repository.close_guild_storage(150_000, guild.id).unwrap();
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::Open { .. }
        ));
        let moved = repository
            .container_transfer(150_001, 2, 2, ItemContainer::Inventory, ItemContainer::GuildStorage, 1000)
            .unwrap();
        assert_eq!((moved.guild_storage.len(), moved.guild_storage[0].amount), (1, 2));
        let stored_id = moved.guild_storage[0].id;
        repository.leave_guild(150_001, guild.id, Some(150_000)).unwrap();
        assert!(
            repository
                .container_transfer(
                    150_001,
                    stored_id,
                    1,
                    ItemContainer::GuildStorage,
                    ItemContainer::Inventory,
                    1000
                )
                .is_err()
        );
        assert!(matches!(
            repository.open_guild_storage(150_000).unwrap(),
            GuildStorageAccess::Open { .. }
        ));
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &150_001_u32.to_be_bytes()).unwrap();
        assert_eq!((inventory.len(), inventory[0].amount), (1, 3));
        let mut outsider = repository.character_game_systems(150_001).unwrap();
        outsider.guild_id = guild.id;
        repository.save_character_game_systems(150_001, &outsider).unwrap();
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::NoPermission
        ));
    }

    #[test]
    fn guild_storage_preserves_unique_items_and_rolls_back_capacity_and_trade_restrictions() {
        use database::read;

        use crate::server::model::game_systems::{GuildStorageAccess, ItemContainer};
        let (repository, guild) = guild_storage_fixture();
        repository.join_guild(150_001, guild.id).unwrap();
        let record = InventoryRecord {
            id: 99,
            item_id: 2229,
            amount: 1,
            unique_id: 987654,
            refine: 7,
            card0: 4001,
            card1: 4002,
            card2: 4003,
            card3: 4004,
            is_identified: true,
            is_damaged: true,
            ..InventoryRecord::default()
        };
        (&repository.database.items, &repository.database.inventories, &repository.database.inventory_owners).transaction(|(items, inventories, owners)| {
            let item: ItemModel = serde_json::from_value(serde_json::json!({"id":2229,"name_aegis":"StorageArmor","name_english":"Storage Armor","item_type":"Armor","weight":100,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":ItemTradeFlag::NoGuildStorage.as_flag()})).unwrap();
            tx_write(items, &2229_i32.to_be_bytes(), &item)?;
            let mut inventory: Vec<InventoryRecord> = tx_required(inventories, &150_001_u32.to_be_bytes())?;
            inventory.push(record.clone());
            tx_write(inventories, &150_001_u32.to_be_bytes(), &inventory)?;
            tx_write(owners, &record.id.to_be_bytes(), &150_001_i32)
        }).unwrap();
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::Open { .. }
        ));
        assert!(
            repository
                .container_transfer(150_001, 99, 1, ItemContainer::Inventory, ItemContainer::GuildStorage, 1000)
                .is_err()
        );
        repository
            .database
            .items
            .transaction(|items| {
                let mut item: ItemModel = tx_required(items, &2229_i32.to_be_bytes())?;
                item.trade_flags = 0;
                tx_write(items, &2229_i32.to_be_bytes(), &item)
            })
            .unwrap();
        let moved = repository
            .container_transfer(150_001, 99, 1, ItemContainer::Inventory, ItemContainer::GuildStorage, 1000)
            .unwrap();
        let saved = &moved.guild_storage[0];
        assert_eq!(
            (
                saved.id,
                saved.unique_id,
                saved.refine,
                saved.card0,
                saved.card1,
                saved.card2,
                saved.card3,
                saved.is_damaged
            ),
            (99, 987654, 7, 4001, 4002, 4003, 4004, true)
        );
        assert!(repository.database.inventory_owners.get(&99_i32.to_be_bytes()).unwrap().is_none());
        repository.close_guild_storage(150_001, guild.id).unwrap();
        repository.open_guild_storage(150_000).unwrap();
        let before_id = read::<i32>(&repository.database.metadata, b"inventory_id").unwrap();
        assert!(
            repository
                .container_transfer(150_000, 99, 1, ItemContainer::GuildStorage, ItemContainer::Inventory, 100)
                .is_err()
        );
        assert_eq!(read::<i32>(&repository.database.metadata, b"inventory_id").unwrap(), before_id);
        repository.close_guild_storage(150_000, guild.id).unwrap();
        match repository.open_guild_storage(150_000).unwrap() {
            GuildStorageAccess::Open { items, .. } => assert_eq!((items.len(), items[0].id, items[0].unique_id), (1, 99, 987654)),
            _ => panic!("Guild storage should open"),
        }
        repository
            .container_transfer(150_000, 99, 1, ItemContainer::GuildStorage, ItemContainer::Inventory, 1000)
            .unwrap();
        assert_eq!(
            read::<i32>(&repository.database.inventory_owners, &99_i32.to_be_bytes()).unwrap(),
            Some(150_000)
        );
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!(inventory.iter().find(|item| item.id == 99).unwrap().unique_id, 987654);
        repository.reset_guild_storage_locks().unwrap();
        assert!(matches!(
            repository.open_guild_storage(150_001).unwrap(),
            GuildStorageAccess::Open { .. }
        ));
    }

    #[test]
    fn cart_transfer_and_vending_purchase_commit_inventory_and_zeny_together() {
        use crate::server::model::game_systems::{PlayerOption, VendingOffer, VendingStore};
        let repository = repository();
        repository.set_character_options(150_001, 0, PlayerOption::Cart1.as_flag()).unwrap();
        let moved = repository
            .container_transfer(150_001, 2, 5, ItemContainer::Inventory, ItemContainer::Cart, 1000)
            .unwrap();
        assert_eq!(moved.cart[0].amount, 5);
        assert!(
            read::<Vec<InventoryRecord>>(&repository.database.inventories, &150_001i32.to_be_bytes())
                .unwrap()
                .unwrap()
                .is_empty()
        );
        assert!(
            read::<i32>(&repository.database.inventory_owners, &2i32.to_be_bytes())
                .unwrap()
                .is_none()
        );
        let vendor = repository
            .create_vending_store(&VendingStore {
                id: 0,
                char_id: 150_001,
                account_id: 2_000_000,
                title: "Potions".into(),
                map: "prontera".into(),
                map_instance: 0,
                x: 150,
                y: 150,
                offers: vec![VendingOffer {
                    inventory_id: 2,
                    index: 0,
                    amount: 5,
                    price: 50,
                }],
            })
            .unwrap();
        assert!(repository.vending_store_trade(150_000, vendor.id, &[(0, 2)], 29).is_err());
        assert_eq!(repository.character_cart(150_001).unwrap()[0].amount, 5);
        let buyer: CharacterRecord = read(&repository.database.characters, &150_000i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(buyer.zeny, 1000);
        assert!(repository.vending_store_trade(150_000, vendor.id, &[(0, 1), (0, 1)], 1000).is_err());
        let trade = repository.vending_store_trade(150_000, vendor.id, &[(0, 2)], 1000).unwrap();
        assert_eq!((trade.buyer_zeny, trade.seller_zeny), (900, 100));
        assert_eq!(trade.cart[0].amount, 3);
        assert_eq!(trade.store.as_ref().unwrap().offers[0].amount, 3);
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_000i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(inventory[0].amount, 3);
        assert_eq!(
            read::<i32>(&repository.database.inventory_owners, &inventory[0].id.to_be_bytes()).unwrap(),
            Some(150_000)
        );
        assert!(
            repository
                .container_transfer(150_001, 2, 1, ItemContainer::Cart, ItemContainer::Inventory, 1000)
                .is_err()
        );
        repository.close_vending_store(150_001, vendor.id).unwrap();
        let restored = repository
            .container_transfer(150_001, 2, 3, ItemContainer::Cart, ItemContainer::Inventory, 1000)
            .unwrap();
        assert!(restored.cart.is_empty());
        assert_eq!(
            read::<i32>(&repository.database.inventory_owners, &2i32.to_be_bytes()).unwrap(),
            Some(150_001)
        );
    }

    #[test]
    fn cart_limit_and_item_restrictions_roll_back_the_whole_transfer() {
        use crate::server::model::game_systems::PlayerOption;
        let repository = repository();
        assert!(
            repository
                .container_transfer(150_001, 2, 1, ItemContainer::Inventory, ItemContainer::Cart, 1000)
                .is_err()
        );
        repository.set_character_options(150_001, 0, PlayerOption::Cart1.as_flag()).unwrap();
        repository
            .database
            .items
            .transaction(|items| {
                let mut item: ItemModel = tx_required(items, &501i32.to_be_bytes())?;
                item.trade_flags = ItemTradeFlag::NoCart.as_flag();
                tx_write(items, &501i32.to_be_bytes(), &item)
            })
            .unwrap();
        assert!(
            repository
                .container_transfer(150_001, 2, 5, ItemContainer::Inventory, ItemContainer::Cart, 1000)
                .is_err()
        );
        assert!(repository.character_cart(150_001).unwrap().is_empty());
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!((inventory[0].id, inventory[0].amount), (2, 5));
        assert_eq!(
            read::<i32>(&repository.database.inventory_owners, &2i32.to_be_bytes()).unwrap(),
            Some(150_001)
        );
    }

    #[test]
    fn stale_companion_update_does_not_consume_its_food() {
        let repository = repository();
        let stale = repository.character_game_systems(150_001).unwrap();
        repository.save_character_game_systems(150_001, &stale).unwrap();
        assert!(repository.save_game_systems_consuming_item(150_001, &stale, 501).is_err());
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(inventory[0].amount, 5);
        let state = repository.character_game_systems(150_001).unwrap();
        let saved = repository.save_game_systems_consuming_item(150_001, &state, 501).unwrap();
        assert_eq!(saved.revision, state.revision + 1);
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(inventory[0].amount, 4);
    }

    #[test]
    fn companion_skill_item_costs_and_world_changes_commit_or_roll_back_together() {
        let repository = repository();
        let mut staged = repository.character_game_systems(150_001).unwrap();
        staged.font = 7;
        assert!(
            repository
                .save_game_systems_consuming_items(150_001, &staged, &[(501, 2), (545, 1)])
                .is_err()
        );
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(inventory[0].amount, 5);
        assert_eq!(repository.character_game_systems(150_001).unwrap().font, 0);
        let saved = repository
            .save_game_systems_consuming_items(150_001, &staged, &[(501, 2), (501, 1)])
            .unwrap();
        assert_eq!((saved.revision, saved.font), (1, 7));
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(inventory[0].amount, 2);
        let saved = repository.save_game_systems_consuming_items(150_001, &saved, &[(501, 2)]).unwrap();
        assert_eq!(saved.revision, 2);
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert!(inventory.is_empty());
        assert_eq!(
            read::<i32>(&repository.database.inventory_owners, &2i32.to_be_bytes()).unwrap(),
            None
        );
    }

    fn pet() -> PetRecord {
        PetRecord {
            id: 0,
            owner_char_id: 0,
            class_id: 1002,
            name: "Poring".into(),
            level: 1,
            egg_item_id: 9001,
            egg_inventory_id: 0,
            intimacy: 250,
            hunger: 70,
            equipped_item: 0,
            next_hunger_at: 0,
            incubating: true,
            renamed: false,
        }
    }

    #[test]
    fn game_system_revision_prevents_overwriting_another_mutation() {
        let repository = repository();
        let state = repository.character_game_systems(150_000).unwrap();
        let saved = repository.save_character_game_systems(150_000, &state).unwrap();
        assert_eq!(saved.revision, 1);
        assert!(repository.save_character_game_systems(150_000, &state).is_err());
        assert_eq!(repository.character_game_systems(150_000).unwrap().revision, 1);
    }

    #[test]
    fn marriage_is_reciprocal_and_failed_marriage_changes_neither_character() {
        let repository = repository();
        repository.marry_characters(150_000, 150_001).unwrap();
        assert_eq!(repository.character_game_systems(150_000).unwrap().partner_id, 150_001);
        assert_eq!(repository.character_game_systems(150_001).unwrap().partner_id, 150_000);
        assert!(repository.marry_characters(150_000, 150_001).is_err());
        assert_eq!(repository.character_game_systems(150_000).unwrap().revision, 1);
        assert!(repository.marry_characters(150_000, 150_000).is_err());
    }

    #[test]
    fn guild_experience_levels_the_guild_and_persists_unspent_experience() {
        let repository = repository();
        add_emperium(&repository, 150_000, 3, 1);
        let guild = repository.create_guild(150_000, "Classic Guild".into()).unwrap();
        repository.join_guild(150_001, guild.id).unwrap();
        let updated = repository.guild_add_experience(150_001, 17, &[5, 10, 20]).unwrap().unwrap();
        assert_eq!((updated.level, updated.skill_points, updated.experience), (3, 2, 2));
        assert_eq!(repository.guild(guild.id).unwrap().unwrap(), updated);
        assert!(repository.guild_add_experience(150_000, 0, &[5, 10]).is_err());
    }

    fn add_emperium(repository: &SledRepository, char_id: i32, id: i32, amount: i16) {
        (
            &repository.database.inventories,
            &repository.database.inventory_owners,
            &repository.database.metadata,
        )
            .transaction(|(inventories, owners, metadata)| {
                let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &char_id.to_be_bytes())?.unwrap_or_default();
                inventory.push(InventoryRecord {
                    id,
                    item_id: 714,
                    amount,
                    is_identified: true,
                    ..InventoryRecord::default()
                });
                tx_write(inventories, &char_id.to_be_bytes(), &inventory)?;
                tx_write(owners, &id.to_be_bytes(), &char_id)?;
                tx_write(metadata, b"inventory_id", &id)
            })
            .unwrap();
    }

    #[test]
    fn guild_creation_and_departure_enforce_cost_name_and_membership_atomically() {
        let repository = repository();
        assert!(repository.create_guild(150_000, "Classic Guild".into()).is_err());
        add_emperium(&repository, 150_000, 3, 2);
        add_emperium(&repository, 150_001, 4, 1);
        let guild = repository.create_guild(150_000, "Classic Guild".into()).unwrap();
        assert!(repository.create_guild(150_001, "Classic Guild".into()).is_err());
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_001i32.to_be_bytes()).unwrap().unwrap();
        assert_eq!(inventory.iter().find(|record| record.item_id == 714).unwrap().amount, 1);
        repository.join_guild(150_001, guild.id).unwrap();
        assert_eq!(repository.guild_member_records(guild.id).unwrap().len(), 2);
        assert!(repository.leave_guild(150_000, guild.id, None).is_err());
        assert!(repository.leave_guild(150_001, guild.id, Some(150_001)).is_err());
        assert!(repository.disband_guild(150_000, &guild.name).is_err());
        assert_eq!(repository.character_game_systems(150_001).unwrap().guild_id, guild.id);
        repository.leave_guild(150_001, guild.id, Some(150_000)).unwrap();
        assert_eq!(repository.character_game_systems(150_001).unwrap().guild_id, 0);
        assert!(repository.disband_guild(150_000, "Forged name").is_err());
        repository.disband_guild(150_000, &guild.name).unwrap();
        assert_eq!(repository.character_game_systems(150_000).unwrap().guild_id, 0);
        assert!(repository.guild(guild.id).unwrap().is_none());
        let recreated = repository.create_guild(150_000, guild.name).unwrap();
        assert_ne!(recreated.id, guild.id);
        let inventory: Vec<InventoryRecord> = read(&repository.database.inventories, &150_000i32.to_be_bytes()).unwrap().unwrap();
        assert!(inventory.iter().all(|record| record.item_id != 714));
        assert_eq!(
            read::<i32>(&repository.database.inventory_owners, &3i32.to_be_bytes()).unwrap(),
            None
        );
    }

    #[test]
    fn pet_hatching_and_returning_transfer_the_same_egg_atomically() {
        let repository = repository();
        let (pet, egg) = repository.create_pet_egg(150_000, &pet(), 100).unwrap();
        assert_eq!(egg.card0, 256);
        assert!(repository.hatch_pet(150_001, egg.id, 100).is_err());
        let state = repository.hatch_pet(150_000, egg.id, 100).unwrap();
        assert_eq!(state.pet.as_ref().unwrap().id, pet.id);
        assert!(!state.pet.as_ref().unwrap().incubating);
        let records: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000i32.to_be_bytes()).unwrap();
        assert!(!records.iter().any(|item| item.id == egg.id));
        let state = repository.return_pet_to_egg(150_000).unwrap();
        assert!(state.pet.is_none());
        let records: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000i32.to_be_bytes()).unwrap();
        assert!(records.iter().any(|item| item.id == egg.id && item.unique_id == i64::from(pet.id)));
    }

    #[test]
    fn pet_feeding_consumes_food_and_updates_pet_in_one_transaction() {
        let repository = repository();
        let (_, egg) = repository.create_pet_egg(150_000, &pet(), 100).unwrap();
        repository.hatch_pet(150_000, egg.id, 100).unwrap();
        assert!(repository.feed_pet(150_000, 9999, 20, 50, 1000).is_err());
        assert_eq!(repository.character_game_systems(150_000).unwrap().pet.unwrap().hunger, 70);
        let state = repository.feed_pet(150_000, 501, 20, 50, 1000).unwrap();
        let pet = state.pet.unwrap();
        assert_eq!((pet.hunger, pet.intimacy), (90, 300));
        let records: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000i32.to_be_bytes()).unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn buying_trade_rolls_back_all_trees_on_weight_failure_and_transfers_on_success() {
        let repository = repository();
        let store = repository.create_buying_store(&store()).unwrap();
        let sale = BuyingSale {
            inventory_id: 2,
            item_id: 501,
            amount: 2,
        };
        assert!(repository.buying_store_trade(150_001, store.id, &[sale.clone()], 15).is_err());
        let buyer: CharacterRecord = required(&repository.database.characters, &150_000i32.to_be_bytes()).unwrap();
        assert_eq!(buyer.zeny, 1000);
        assert_eq!(repository.buying_store(store.id).unwrap().unwrap().zeny_limit, 100);
        let trade = repository.buying_store_trade(150_001, store.id, &[sale], 100).unwrap();
        assert_eq!((trade.seller_zeny, trade.buyer_zeny), (60, 940));
        assert_eq!(trade.buyer_inventory[0].amount, 3);
        assert_eq!(trade.seller_inventory[0].amount, 3);
        assert_eq!(trade.store.unwrap().zeny_limit, 40);
    }

    #[test]
    fn competing_buying_trades_cannot_spend_the_same_budget_twice() {
        let repository = Arc::new(repository());
        let store = repository.create_buying_store(&store()).unwrap();
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let repository = repository.clone();
            let barrier = barrier.clone();
            let id = store.id;
            handles.push(std::thread::spawn(move || {
                barrier.wait();
                repository
                    .buying_store_trade(
                        150_001,
                        id,
                        &[BuyingSale {
                            inventory_id: 2,
                            item_id: 501,
                            amount: 2,
                        }],
                        100,
                    )
                    .is_ok()
            }));
        }
        barrier.wait();
        assert_eq!(
            handles
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .filter(|success| *success)
                .count(),
            1
        );
        let buyer: CharacterRecord = required(&repository.database.characters, &150_000i32.to_be_bytes()).unwrap();
        let seller: CharacterRecord = required(&repository.database.characters, &150_001i32.to_be_bytes()).unwrap();
        assert_eq!((buyer.zeny, seller.zeny), (940, 60));
    }

    #[test]
    fn account_storage_transfers_are_shared_between_characters_and_atomic_on_weight_failure() {
        let repository = repository();
        let transfer = repository.storage_transfer(150_001, 2, 2, true, u32::MAX).unwrap();
        assert_eq!(transfer.inventory[0].amount, 3);
        assert_eq!(transfer.storage[0].amount, 2);
        let stored_id = transfer.storage[0].id;
        assert!(repository.storage_transfer(150_000, stored_id, 1, false, 10).is_err());
        assert_eq!(repository.account_storage(2_000_000).unwrap()[0].amount, 2);
        let transferred = repository.storage_transfer(150_000, stored_id, 1, false, 100).unwrap();
        assert_eq!(transferred.inventory[0].amount, 2);
        assert_eq!(transferred.storage[0].amount, 1);
        assert!(repository.storage_transfer(150_001, stored_id, 2, false, 100).is_err());
        assert_eq!(repository.account_storage(2_000_000).unwrap()[0].amount, 1);
    }

    #[test]
    fn pet_accessory_equip_and_unequip_move_inventory_without_duplicating_it() {
        let repository = repository();
        let (_, egg) = repository.create_pet_egg(150_000, &pet(), 100).unwrap();
        repository.hatch_pet(150_000, egg.id, 100).unwrap();
        let accessory_id = (
            &repository.database.inventories,
            &repository.database.inventory_owners,
            &repository.database.metadata,
        )
            .transaction(|(inventories, owners, metadata)| {
                let id = next_id(metadata, b"inventory_id", 0)?;
                let mut records: Vec<InventoryRecord> = tx_required(inventories, &150_000i32.to_be_bytes())?;
                records.push(InventoryRecord {
                    id,
                    item_id: 10000,
                    unique_id: i64::from(id),
                    amount: 1,
                    is_identified: true,
                    ..InventoryRecord::default()
                });
                tx_write(inventories, &150_000i32.to_be_bytes(), &records)?;
                tx_write(owners, &id.to_be_bytes(), &150_000i32)?;
                Ok(id)
            })
            .unwrap();
        let equipped = repository.pet_accessory(150_000, Some(accessory_id), 10000).unwrap();
        assert_eq!(equipped.pet.unwrap().equipped_item, 10000);
        assert!(repository.pet_accessory(150_000, Some(accessory_id), 10000).is_err());
        let unequipped = repository.pet_accessory(150_000, None, 0).unwrap();
        assert_eq!(unequipped.pet.unwrap().equipped_item, 0);
        let records: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000i32.to_be_bytes()).unwrap();
        assert_eq!(records.iter().filter(|item| item.item_id == 10000).count(), 1);
    }
}
