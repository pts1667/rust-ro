pub mod character_repository;
pub mod game_system_repository;
pub use game_system_repository::GameSystemRepository;
mod hotkey_repository;
pub mod inventory_repository;
pub mod item_repository;
mod login_repository;
pub mod mob_repository;
pub mod model;
pub mod script_inventory_repository;
pub mod script_variable_repository;
pub use script_inventory_repository::ScriptInventoryRepository;
pub mod script_character_repository;
pub use script_character_repository::ScriptCharacterRepository;
pub mod fame_repository;
pub use fame_repository::FameRepository;
#[cfg(test)]
mod tests;

use async_trait::async_trait;
use configuration::configuration::DatabaseConfig;
use database::model::{AccountRecord, IpBanRecord, LoginLogRecord};
use database::Database;
pub use database::DatabaseError as Error;
use models::status::{KnownSkill, Status, StatusSnapshot};

use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::repository::model::item_model::{GetItemModel, InventoryItemModel, ItemBuySellModel, ItemModel};
use crate::repository::model::mob_model::MobModel;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};
use crate::server::model::hotkey::Hotkey;
use crate::server::script::Value;

pub struct SledRepository {
    pub database: Database,
}

impl SledRepository {
    pub fn open(configuration: &DatabaseConfig) -> Result<Self, Error> {
        let repository = Self {
            database: Database::open(&configuration.path)?,
        };
        repository.seed_assets(configuration)?;
        repository.reset_item_group_pools()?;
        repository.reset_guild_storage_locks()?;
        Ok(repository)
    }

    #[cfg(test)]
    pub fn temporary() -> Result<Self, Error> {
        Ok(Self {
            database: Database::temporary()?,
        })
    }
}

pub trait Repository:
    Sync
    + Send
    + CharacterRepository
    + ItemRepository
    + InventoryRepository
    + MobRepository
    + ScriptVariableRepository
    + LoginRepository
    + HotKeyRepository
    + GameSystemRepository
    + ScriptInventoryRepository
    + ScriptCharacterRepository
    + FameRepository
{
}

impl Repository for SledRepository {}

pub trait LoginRepository {
    fn account_by_name(&self, _name: &str) -> Result<Option<AccountRecord>, Error> {
        Err(Error::InvalidInput("Account storage is unavailable".into()))
    }
    fn account_by_id(&self, _account_id: u32) -> Result<Option<AccountRecord>, Error> {
        Err(Error::InvalidInput("Account storage is unavailable".into()))
    }
    fn account_create(&self, _account: AccountRecord) -> Result<u32, Error> {
        Err(Error::InvalidInput("Account storage is unavailable".into()))
    }
    fn account_update(&self, _account_id: u32, _update: &dyn Fn(&mut AccountRecord)) -> Result<AccountRecord, Error> {
        Err(Error::InvalidInput("Account storage is unavailable".into()))
    }
    fn ip_ban_active(&self, _ip: std::net::Ipv4Addr, _now: i64) -> Result<bool, Error> {
        Ok(false)
    }
    fn ip_ban_add(&self, _ban: IpBanRecord) -> Result<(), Error> {
        Err(Error::InvalidInput("IP ban storage is unavailable".into()))
    }
    fn ip_ban_remove(&self, _list: &str) -> Result<bool, Error> {
        Err(Error::InvalidInput("IP ban storage is unavailable".into()))
    }
    fn ip_ban_cleanup(&self, _now: i64) -> Result<usize, Error> {
        Ok(0)
    }
    fn login_log_append(&self, _record: LoginLogRecord) -> Result<(), Error> {
        Ok(())
    }
    fn login_log_failed_attempts(&self, _ip: &str, _since: i64) -> Result<u32, Error> {
        Ok(0)
    }
    fn login_log_prune(&self, _before: i64) -> Result<usize, Error> {
        Ok(0)
    }
    fn login_log_entries(&self) -> Result<Vec<LoginLogRecord>, Error> {
        Ok(Vec::new())
    }
}

#[async_trait]
pub trait CharacterRepository: ScriptCharacterRepository {
    fn character_commit_skill_reset(
        &self,
        _char_id: u32,
        _account_id: u32,
        _plan: &crate::repository::script_character_repository::ScriptSkillResetPlan,
    ) -> Result<(), Error> {
        Err(Error::InvalidInput("Character skill transactions are unavailable".into()))
    }
    fn character_commit_skill_allocation(
        &self,
        _char_id: u32,
        _account_id: u32,
        _skill_id: u32,
        _expected_level: u8,
        _expected_points: u32,
        _max_level: u8,
    ) -> Result<u8, Error> {
        Err(Error::InvalidInput("Character skill transactions are unavailable".into()))
    }
    async fn characters_list_for_simulator(&self) -> Result<Vec<CharSelectModel>, Error> {
        todo!()
    }
    async fn character_insert(&self, _char_model: &CharInsertModel) -> Result<(), Error> {
        todo!()
    }
    async fn character_info(&self, _account_id: i32, _char_name: &str) -> Result<CharacterInfoNeoUnionWrapped, Error> {
        todo!()
    }
    async fn characters_info(&self, _account_id: u32) -> Vec<CharacterInfoNeoUnionWrapped> {
        todo!()
    }
    async fn character_delete_reserved(&self, _account_id: u32, _char_id: u32) -> Result<(), Error> {
        todo!()
    }
    async fn character_save_position(&self, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> {
        todo!()
    }
    async fn character_save_position_guarded(
        &self,
        position: crate::server::model::events::persistence_event::SavePositionUpdate,
    ) -> Result<(), Error> {
        self.character_save_position(position.char_id, position.map_name, position.x, position.y)
            .await
    }
    async fn character_update_status(&self, _char_id: u32, _field: String, _value: u32) -> Result<(), Error> {
        todo!()
    }
    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> {
        todo!()
    }
    async fn character_allocated_skill_points(&self, _char_id: u32) -> Result<i32, Error> {
        todo!()
    }
    async fn character_skills(&self, _char_id: u32) -> Result<Vec<KnownSkill>, Error> {
        todo!()
    }
    async fn character_fetch(&self, _account_id: u32, _char_num: u8) -> Result<CharSelectModel, Error> {
        todo!()
    }
    async fn character_with_id_fetch(&self, _char_id: u32) -> Result<CharSelectModel, Error> {
        todo!()
    }
    async fn character_reset_skills(&self, _char_id: i32, _skills: Vec<i32>) -> Result<(), Error> {
        todo!()
    }
    async fn character_allocate_skill_point(&self, _char_id: i32, _skill_id: i32, _increment: u8) -> Result<(), Error> {
        todo!()
    }
    async fn characters_update(
        &self,
        _statuses: Vec<&Status>,
        _snapshots: Vec<StatusSnapshot>,
        _char_ids: Vec<i32>,
        _x: Vec<i16>,
        _y: Vec<i16>,
        _maps: Vec<String>,
    ) -> Result<(), Error> {
        todo!()
    }
    async fn characters_update_with_positions(
        &self,
        statuses: Vec<&Status>,
        snapshots: Vec<StatusSnapshot>,
        positions: Vec<crate::server::model::events::persistence_event::SavePositionUpdate>,
    ) -> Result<(), Error> {
        self.characters_update(
            statuses,
            snapshots,
            positions.iter().map(|position| position.char_id as i32).collect(),
            positions.iter().map(|position| position.x as i16).collect(),
            positions.iter().map(|position| position.y as i16).collect(),
            positions.into_iter().map(|position| position.map_name).collect(),
        )
        .await
    }
    async fn character_save_temporary_bonus(
        &self,
        _char_id: u32,
        _account_id: u32,
        _temporary_bonuses: &models::status_bonus::TemporaryStatusBonuses,
    ) -> Result<(), Error> {
        todo!()
    }
    async fn character_load_temporary_bonus(
        &self,
        _char_id: u32,
        _account_id: u32,
    ) -> Result<models::status_bonus::TemporaryStatusBonuses, Error> {
        todo!()
    }
}

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn character_set_item_damaged(&self, _char_id: u32, _item: InventoryItemModel) -> Result<(), Error> {
        Err(Error::InvalidInput("Equipment damage persistence is unavailable".into()))
    }
    async fn character_identify_item(&self, _char_id: u32, _item: InventoryItemModel) -> Result<(), Error> {
        Err(Error::InvalidInput("Item identification is unavailable".into()))
    }
    async fn character_inventory_update_add(
        &self,
        _inventory_update_items: &[InventoryItemUpdate],
        _buy: bool,
    ) -> Result<Vec<InventoryItemModel>, Error> {
        todo!()
    }
    async fn character_inventory_update_remove(
        &self,
        _inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>,
        _sell: bool,
    ) -> Result<(), Error> {
        todo!()
    }
    async fn character_inventory_delete(&self, _delete_items: DeleteItems) -> Result<(), Error> {
        todo!()
    }
    async fn character_inventory_fetch(&self, _char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        todo!()
    }
    async fn character_inventory_wearable_item_update(&self, _items: Vec<InventoryItemModel>) -> Result<(), Error> {
        todo!()
    }
    async fn character_inventory_commit_equipment(&self, _char_id: u32, _items: Vec<InventoryItemModel>) -> Result<(), Error> {
        Err(Error::InvalidInput("Authenticated equipment persistence is unavailable".into()))
    }
    async fn character_slot_card(
        &self,
        _char_id: i32,
        _card_inventory_item: &InventoryItemModel,
        _equipment_inventory_item: &InventoryItemModel,
    ) -> Result<i32, Error> {
        todo!()
    }
}

#[async_trait]
pub trait HotKeyRepository {
    async fn save_hotkeys(&self, _char_id: u32, _hotkeys: &Vec<Hotkey>) -> Result<(), Error> {
        todo!()
    }
    async fn load_hotkeys(&self, _char_id: u32) -> Result<Vec<Hotkey>, Error> {
        todo!()
    }
}

#[async_trait]
pub trait ItemRepository: Sync + Send {
    async fn item_buy_sell_fetch_all_where_ids(&self, _ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> {
        todo!()
    }
    async fn get_items(&self, _ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error> {
        todo!()
    }
    async fn get_item_script(&self, _id: i32) -> Result<String, Error> {
        todo!()
    }
    async fn get_weight(&self, _ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error> {
        todo!()
    }
    async fn get_all_items(&self) -> Result<Vec<ItemModel>, Error> {
        todo!()
    }
}

#[async_trait]
pub trait MobRepository {
    async fn get_all_mobs(&self) -> Result<Vec<MobModel>, Error> {
        todo!()
    }
}

pub trait ScriptVariableRepository {
    fn script_variables_increment_batch(
        &self,
        _char_id: u32,
        _account_id: u32,
        _variables: &[script_sdk::Variable],
    ) -> Result<Vec<i32>, Error> {
        Err(Error::InvalidInput("Atomic script counters are unavailable".into()))
    }
    fn script_variables_save_batch(&self, _char_id: u32, _account_id: u32, _variables: &[script_sdk::Variable]) -> Result<(), Error> {
        Err(Error::InvalidInput("Batch script variables are unavailable".into()))
    }
    fn script_variable_char_num_save(&self, _char_id: u32, _key: String, _index: u32, _value: i32) {
        todo!()
    }
    fn script_variable_char_str_save(&self, _char_id: u32, _key: String, _index: u32, _value: String) {
        todo!()
    }
    fn script_variable_account_num_save(&self, _account_id: u32, _key: String, _index: u32, _value: i32) {
        todo!()
    }
    fn script_variable_account_str_save(&self, _account_id: u32, _key: String, _index: u32, _value: String) {
        todo!()
    }
    fn script_variable_server_num_save(&self, _varname: String, _index: u32, _value: i32) {
        todo!()
    }
    fn script_variable_server_str_save(&self, _varname: String, _index: u32, _value: String) {
        todo!()
    }
    fn script_variable_char_str_fetch_one(&self, _char_id: u32, _variable_name: String, _index: u32) -> String {
        todo!()
    }
    fn script_variable_char_num_fetch_one(&self, _char_id: u32, _variable_name: String, _index: u32) -> i32 {
        todo!()
    }
    fn script_variable_account_str_fetch_one(&self, _account_id: u32, _variable_name: String, _index: u32) -> String {
        todo!()
    }
    fn script_variable_account_num_fetch_one(&self, _account_id: u32, _variable_name: String, _index: u32) -> i32 {
        todo!()
    }
    fn script_variable_server_str_fetch_one(&self, _variable_name: String, _index: u32) -> String {
        todo!()
    }
    fn script_variable_server_num_fetch_one(&self, _variable_name: String, _index: u32) -> i32 {
        todo!()
    }
    fn script_variable_char_str_fetch_all(&self, _char_id: u32, _variable_name: String) -> Vec<(u32, String)> {
        todo!()
    }
    fn script_variable_char_num_fetch_all(&self, _char_id: u32, _variable_name: String) -> Vec<(u32, i32)> {
        todo!()
    }
    fn script_variable_account_str_fetch_all(&self, _account_id: u32, _variable_name: String) -> Vec<(u32, String)> {
        todo!()
    }
    fn script_variable_account_num_fetch_all(&self, _account_id: u32, _variable_name: String) -> Vec<(u32, i32)> {
        todo!()
    }
    fn script_variable_server_str_fetch_all(&self, _variable_name: String) -> Vec<(u32, String)> {
        todo!()
    }
    fn script_variable_server_num_fetch_all(&self, _variable_name: String) -> Vec<(u32, i32)> {
        todo!()
    }
}
