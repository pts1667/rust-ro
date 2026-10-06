use async_trait::async_trait;

use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::item_model::InventoryItemModel;
use crate::repository::{
    CharacterRepository, Error, HotKeyRepository, InventoryRepository, ItemRepository, LoginRepository, MobRepository, Repository,
    ScriptVariableRepository,
};
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};

#[derive(Default)]
pub struct MockedRepository;

impl Repository for MockedRepository {}

impl crate::repository::GameSystemRepository for MockedRepository {}
impl crate::repository::ScriptInventoryRepository for MockedRepository {}
impl crate::repository::ScriptCharacterRepository for MockedRepository {
    fn character_adjust_zeny(&self, _char_id: u32, _account_id: u32, expected: u32, delta: i64) -> Result<u32, Error> {
        Ok((i64::from(expected) + delta).clamp(0, i64::from(i32::MAX)) as u32)
    }
}
impl crate::repository::FameRepository for MockedRepository {}

impl HotKeyRepository for MockedRepository {}

impl ItemRepository for MockedRepository {}

impl MobRepository for MockedRepository {}

impl ScriptVariableRepository for MockedRepository {}

impl LoginRepository for MockedRepository {}

impl crate::repository::CharServerRepository for MockedRepository {}

#[async_trait]
impl InventoryRepository for MockedRepository {
    async fn character_inventory_update_add(
        &self,
        inventory_update_items: &[InventoryItemUpdate],
        _buy: bool,
    ) -> Result<Vec<InventoryItemModel>, Error> {
        Ok(added_items(inventory_update_items))
    }

    async fn character_inventory_update_remove(
        &self,
        _inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>,
        _sell: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn character_inventory_delete(&self, _delete_items: DeleteItems) -> Result<(), Error> {
        Ok(Default::default())
    }

    async fn character_inventory_fetch(&self, _char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        Ok(Default::default())
    }

    async fn character_inventory_wearable_item_update(&self, _items: Vec<InventoryItemModel>) -> Result<(), Error> {
        Ok(Default::default())
    }

    async fn character_inventory_commit_equipment(&self, _char_id: u32, _items: Vec<InventoryItemModel>) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
impl CharacterRepository for MockedRepository {
    fn character_commit_skill_reset(&self, _char_id: u32, _account_id: u32, _plan: &crate::repository::script_character_repository::ScriptSkillResetPlan) -> Result<(), Error> { Ok(()) }
    fn character_commit_skill_allocation(&self, _char_id: u32, _account_id: u32, _skill_id: u32, expected_level: u8, expected_points: u32, max_level: u8) -> Result<u8, Error> {
        expected_level.checked_add(1).filter(|level| *level <= max_level && expected_points > 0)
            .ok_or_else(|| Error::InvalidInput("Invalid mocked skill allocation".into()))
    }
    async fn character_save_position(&self, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> {
        Ok(())
    }

    async fn character_update_status(&self, _char_id: u32, _field: String, _value: u32) -> Result<(), Error> {
        Ok(())
    }

    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> {
        Ok(Default::default())
    }

    async fn character_fetch(&self, _account_id: u32, _char_num: u8) -> Result<CharSelectModel, Error> {
        Ok(Default::default())
    }
}

pub fn added_items(updates: &[InventoryItemUpdate]) -> Vec<InventoryItemModel> {
    updates
        .iter()
        .map(|update| {
            let item = crate::server::service::global_config_service::GlobalConfigService::instance().get_item(update.item_id);
            let mut model = InventoryItemModel::from_item_model(item, update.amount, update.identified);
            model.unique_id = update.unique_id;
            model.refine = update.refine;
            model.is_damaged = update.damaged;
            model.card0 = update.cards[0];
            model.card1 = update.cards[1];
            model.card2 = update.cards[2];
            model.card3 = update.cards[3];
            model
        })
        .collect()
}
