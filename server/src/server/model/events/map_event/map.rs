use super::{MapEventContext, MapEventHandler};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::map_item::MapItem;

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterDropItems {
    pub owner_id: u32,
    pub char_x: u16,
    pub char_y: u16,
    pub item_removal_info: Vec<(InventoryItemModel, CharacterRemoveItem)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetMapFlags {
    pub flags: crate::server::model::map_flags::MapFlags,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateActorVisibility {
    pub actors: Vec<(u32, crate::server::service::visibility_service::StealthState)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RemoveCharFromMap {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InsertCharToMap {
    pub map_item: MapItem,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RemoveDroppedItemFromMap {
    pub dropped_item_id: u32,
}

impl MapEventHandler for SetMapFlags {
    fn handle(self, ctx: &MapEventContext) {
        let SetMapFlags { flags } = self;
        ctx.map_instance.state_mut().flags = flags;
    }
}

impl MapEventHandler for UpdateActorVisibility {
    fn handle(self, ctx: &MapEventContext) {
        let UpdateActorVisibility { actors } = self;
        ctx.map_instance.state_mut().actor_visibility = actors.into_iter().collect();
    }
}

impl MapEventHandler for RemoveCharFromMap {
    fn handle(self, ctx: &MapEventContext) {
        let RemoveCharFromMap { char_id } = self;
        ctx.map_instance.state_mut().remove_item_with_id(char_id);
    }
}

impl MapEventHandler for InsertCharToMap {
    fn handle(self, ctx: &MapEventContext) {
        let InsertCharToMap { map_item } = self;
        ctx.map_instance.state_mut().insert_item(map_item);
    }
}

impl MapEventHandler for RemoveDroppedItemFromMap {
    fn handle(self, ctx: &MapEventContext) {
        let RemoveDroppedItemFromMap { dropped_item_id } = self;
        ctx.service
            .remove_dropped_item_from_map(ctx.map_instance.state_mut().as_mut(), dropped_item_id);
    }
}

impl MapEventHandler for CharacterDropItems {
    fn handle(self, ctx: &MapEventContext) {
        let character_drop_items = self;
        ctx.service
            .character_drop_items_and_send_packet(ctx.map_instance.state_mut().as_mut(), character_drop_items);
    }
}
