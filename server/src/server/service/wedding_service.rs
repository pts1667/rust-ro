use models::status_change::{StatusChangeKind, StatusChangeRequest};

use crate::repository::model::item_model::InventoryItemModel;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterAddItems, CharacterRemoveItem, CharacterRemoveItems};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;

const BRIDEGROOM_RING: i32 = 2634;
const BRIDE_RING: i32 = 2635;
const WEDDING_STATUS_MS: i32 = 3_600_000;
const MALE: u8 = 1;

fn ring_for(character: &Character) -> i32 {
    if character.sex == MALE { BRIDEGROOM_RING } else { BRIDE_RING }
}

impl Server {
    /// Gives the spouse's ring and the wedding outfit status that follow a completed ceremony.
    pub(crate) fn start_wedding(&self, character: &mut Character) -> Result<(), String> {
        let ring = ring_for(character);
        let item = GlobalConfigService::instance().get_item(ring);
        self.inventory_service().try_add_items_in_inventory(
            self.runtime(),
            CharacterAddItems {
                char_id: character.char_id,
                should_perform_check: false,
                buy: false,
                items: vec![InventoryItemModel::from_item_model(item, 1, true)],
            },
            character,
        )?;
        let request = StatusChangeRequest::guaranteed(StatusChangeKind::Wedding, WEDDING_STATUS_MS, 0);
        StatusEffectService::start(self, character, request, crate::util::tick::get_tick(), &self.server_service().notification_sender()).map(|_| ())
    }

    /// Removes the wedding ring (equipped or not) when a marriage ends.
    pub(crate) fn drop_wedding_ring(&self, character: &mut Character) {
        let ring = ring_for(character);
        let Some((index, equipped)) = character
            .inventory_iter()
            .find(|(_, item)| item.item_id == ring)
            .map(|(index, item)| (index, item.equip != 0))
        else {
            return;
        };
        if equipped && self.inventory_service().takeoff_equip_item(character, index).is_none() {
            return;
        }
        let removal = CharacterRemoveItems {
            char_id: character.char_id,
            sell: false,
            items: vec![CharacterRemoveItem { char_id: character.char_id, index, amount: 1, price: 0 }],
            notify_client: true,
        };
        if let Err(error) = self.inventory_service().remove_item_from_inventory(self.runtime(), removal, character) {
            warn!("Wedding ring removal failed for {}: {error}", character.char_id);
        }
    }
}
