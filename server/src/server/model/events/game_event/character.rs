use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::look::LookType;
use models::enums::skill_enums::SkillEnum;

use super::*;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::hotkey::Hotkey;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterLook {
    pub char_id: u32,
    pub look_type: LookType,
    pub look_value: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterZeny {
    pub char_id: u32,
    pub zeny: Option<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterAddItems {
    pub char_id: u32,
    pub should_perform_check: bool, // indicate if we should perform checks before adding items to user
    pub buy: bool,                  // indicate zeny should be used to buy item (zeny will be updated)
    pub items: Vec<InventoryItemModel>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRemoveItems {
    pub char_id: u32,
    pub sell: bool, // indicate zeny should be given to character after item sell (zeny will be updated)
    pub items: Vec<CharacterRemoveItem>,
    pub notify_client: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseItem {
    pub char_id: u32,
    pub target_char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterEquipItem {
    pub char_id: u32,
    pub index: usize,
    pub requested_location: Option<u64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterTakeoffEquipItem {
    pub char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CharacterRemoveItem {
    pub char_id: u32,
    pub index: usize,
    pub amount: i16,
    pub price: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterAttack {
    pub char_id: u32,
    pub target_id: u32,
    pub repeat: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeJobLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeJob {
    pub char_id: u32,
    pub job: JobName,
    pub should_reset_skills: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterKillMonster {
    pub attacker_id: u32,
    pub char_id: u32,
    pub mob_id: i16,
    pub mob_x: u16,
    pub mob_y: u16,
    pub map_instance_key: MapInstanceKey,
    pub mob_base_exp: u32,
    pub mob_job_exp: u32,
    pub mob_max_hp: u32,
    pub contributions: Vec<crate::server::model::action::DamageContribution>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterPickUpItem {
    pub char_id: u32,
    pub map_item_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUpdateStat {
    pub char_id: u32,
    pub stat_id: u16,
    pub change_amount: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSkillUpgrade {
    pub char_id: u32,
    pub skill_id: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRequestCardCompositionList {
    pub char_id: u32,
    pub card_index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSlotCard {
    pub char_id: u32,
    pub card_index: usize,
    pub equip_index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUpdateWeight {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterInitInventory {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSit {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterStand {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUpdateClientSideStats {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterHotkeyAdd {
    pub char_id: u32,
    pub hotkey: Hotkey,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterHotkeyRemove {
    pub char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterResetSkills {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterResetStats {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUpdateSpeed {
    pub char_id: u32,
    pub speed: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRestoreAllHpAndSP {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterDamage {
    pub damage: Damage,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapNotifyItemRemoved {
    pub map_item_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReleaseScriptCapture {
    pub id: u32,
}

impl GameEventHandler for CharacterLook {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_look = self;
        let character = state.characters_mut().get_mut(&character_look.char_id).unwrap();
        server.character_service().change_look(character_look, character);
        Ok(())
    }
}

impl GameEventHandler for CharacterZeny {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let zeny_update = self;
        let character = state.characters_mut().get_mut(&zeny_update.char_id).unwrap();
        server
            .character_service()
            .update_zeny(server.runtime.as_ref(), zeny_update, character);
        Ok(())
    }
}

impl GameEventHandler for CharacterUpdateWeight {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterUpdateWeight { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().notify_weight(character);
        Ok(())
    }
}

impl GameEventHandler for CharacterInitInventory {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterInitInventory { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server
            .inventory_service()
            .reload_inventory(server.runtime.as_ref(), char_id, character);
        server.inventory_service().reload_equipped_item_sprites(character);
        server.refresh_forged_rank(character);
        server.refresh_taekwon_rank(character);
        server.character_service().reload_client_side_status(character);
        server.character_service().reload_client_side_hotkeys(character);
        if let Err(error) = server.script_world_service().initialize_cart(character) {
            warn!("Cart initialization failed: {error}");
        }
        if let Err(error) = server.script_world_service().initialize_guild(server, character) {
            warn!("Guild initialization failed: {error}");
        }
        if let Err(error) = server.script_world_service().initialize_party(server, character) {
            warn!("Party initialization failed: {error}");
        }
        character.refresh_script_context();
        Ok(())
    }
}

impl GameEventHandler for CharacterAddItems {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let add_items = self;
        let character = state.characters_mut().get_mut(&add_items.char_id).unwrap();
        server
            .inventory_service()
            .add_items_in_inventory(server.runtime.as_ref(), add_items, character);
        Ok(())
    }
}

impl GameEventHandler for CharacterRemoveItems {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_remove_items = self;
        let character = state.characters_mut().get_mut(&character_remove_items.char_id).unwrap();
        server
            .inventory_service()
            .character_sell_items(server.runtime.as_ref(), character, character_remove_items);
        Ok(())
    }
}

impl GameEventHandler for CharacterUseItem {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_use_item = self;
        if let Some(mut character) = state.characters_mut().remove(&character_use_item.char_id) {
            server
                .item_service()
                .use_item_in_state(server, &*state, server.runtime.as_ref(), character_use_item, &mut character);
            state.insert_character(character);
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterEquipItem {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_equip_item = self;
        let character = state.characters_mut().get_mut(&character_equip_item.char_id).unwrap();
        if character
            .get_item_from_inventory(character_equip_item.index)
            .is_some_and(|item| item.item_type() == models::enums::item::ItemType::PetArmor)
        {
            server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                char_id: character.char_id,
                request: crate::server::model::game_systems::ScriptWorldRequest::Pet(crate::server::model::game_systems::PetRequest::EquipPetAccessory(character_equip_item.index as u16)),
            }));
            return Ok(());
        }
        let equipped_item = server.inventory_service().equip_item(character, character_equip_item);
        equipped_item.map(|item| {
            server
                .inventory_service()
                .sprite_change_packet_for_item(character, &item, false)
                .map(|packet| {
                    server
                        .character_service()
                        .send_area_notification_around_characters(character, packet)
                })
        });
        Ok(())
    }
}

impl GameEventHandler for CharacterTakeoffEquipItem {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_takeoff_equip_item = self;
        let character = state.characters_mut().get_mut(&character_takeoff_equip_item.char_id).unwrap();
        let index = character_takeoff_equip_item.index;
        server.inventory_service().takeoff_equip_item(character, index).map(|item| {
            server
                .inventory_service()
                .sprite_change_packet_for_item(character, &item, true)
                .map(|packet| {
                    server
                        .character_service()
                        .send_area_notification_around_characters(character, packet)
                })
        });
        Ok(())
    }
}

impl GameEventHandler for CharacterAttack {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_attack = self;
        let Some(source) = state.characters().get(&character_attack.char_id) else {
            return Ok(());
        };
        if !server.player_target_allowed(
            &*state,
            source,
            character_attack.target_id,
            crate::server::service::visibility_service::TargetingMode::Direct,
        ) {
            return Ok(());
        }
        let character = state.characters_mut().get_mut(&character_attack.char_id).unwrap();
        if character.is_dead()
            || character.status.blocks_attack()
            || character.game_systems.is_trading()
            || character.game_systems.buying_store.is_some()
            || character.game_systems.vending_store.is_some()
        {
            return Ok(());
        }
        if !character.is_attacking() {
            // last_attack_tick = 0 allows first attack to happen immediately
            // canmove_tick is set in basic_attack() when attack animation starts
            character.set_attack(character_attack.target_id, character_attack.repeat, 0);
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterSit {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterSit { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().sit(character);
        Ok(())
    }
}

impl GameEventHandler for CharacterStand {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterStand { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().stand(character);
        Ok(())
    }
}

impl GameEventHandler for CharacterUpdateClientSideStats {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterUpdateClientSideStats { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.refresh_forged_rank(character);
        server.refresh_taekwon_rank(character);
        server.character_service().reload_client_side_status(character);
        Ok(())
    }
}

impl GameEventHandler for CharacterChangeLevel {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_change_level = self;
        let character = state.characters_mut().get_mut(&character_change_level.char_id).unwrap();
        let delta =
            server
                .character_service()
                .update_base_level(character, character_change_level.set_level, character_change_level.add_level);
        if delta < 0 {
            // TODO ensure equip required min level
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterChangeJobLevel {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_change_level = self;
        let character = state.characters_mut().get_mut(&character_change_level.char_id).unwrap();
        let delta =
            server
                .character_service()
                .update_job_level(character, character_change_level.set_level, character_change_level.add_level);
        if delta < 0 {
            // TODO ensure equip required min level
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterChangeJob {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_change_job = self;
        let character = state.characters_mut().get_mut(&character_change_job.char_id).unwrap();
        match server.repository.character_change_fame_class(
            character.char_id,
            character.account_id,
            character_change_job.job.value() as u32,
        ) {
            Ok(updates) => {
                for update in updates {
                    server.add_to_next_tick(GameEvent::FameChanged(crate::server::model::events::game_event::FameChanged {
                        category: update.category,
                        ranked_creators: update.rankings.iter().map(|entry| entry.char_id).collect(),
                    }));
                }
            }
            Err(error) => {
                warn!("Job transaction failed: {error}");
                return Ok(());
            }
        }
        server
            .character_service()
            .change_job(character, character_change_job.job, character_change_job.should_reset_skills);
        server.refresh_taekwon_rank(character);
        server.character_service().reload_client_side_status(character);
        // TODO ensure equip required class
        Ok(())
    }
}

impl GameEventHandler for CharacterKillMonster {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let kill = self;
        if let Err(error) = server.reward_monster_kill(state, kill, tick) {
            warn!("Monster reward failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterPickUpItem {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_pickup_item = self;
        if let Some(mut character) = state.characters_mut().remove(&character_pickup_item.char_id) {
            if let Some(map_instance) = state.get_map_instance_from_character(&character) {
                if let Err(error) = server.server_service.character_pickup_item(
                    server,
                    state,
                    &mut character,
                    character_pickup_item.map_item_id,
                    map_instance.as_ref(),
                ) {
                    warn!("Floor item transfer failed: {error}");
                }
            }
            state.insert_character(character);
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterUpdateStat {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_update_stat = self;
        let character = state.characters_mut().get_mut(&character_update_stat.char_id).unwrap();
        server.character_service().character_increase_stat(character, character_update_stat);
        Ok(())
    }
}

impl GameEventHandler for CharacterSkillUpgrade {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_skill_upgrade = self;
        let character = state.characters_mut().get_mut(&character_skill_upgrade.char_id).unwrap();
        if (8001..=8016).contains(&(character_skill_upgrade.skill_id as u32)) {
            if let Err(error) = server
                .script_world_service()
                .learn_homunculus_skill(character, character_skill_upgrade.skill_id as u32)
            {
                warn!("Homunculus skill learning failed: {error}");
            }
            return Ok(());
        }
        if (10000..=10015).contains(&u32::from(character_skill_upgrade.skill_id)) {
            server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                char_id: character.char_id,
                request: crate::server::model::game_systems::ScriptWorldRequest::Guild(crate::server::model::game_systems::GuildRequest::GuildSkillUp(u32::from(character_skill_upgrade.skill_id))),
            }));
            return Ok(());
        }
        server
            .character_service()
            .allocate_skill_point(character, SkillEnum::from_id(character_skill_upgrade.skill_id as u32));
        Ok(())
    }
}

impl GameEventHandler for CharacterHotkeyAdd {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterHotkeyAdd { char_id, hotkey } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().hotkey_add(character, hotkey);
        Ok(())
    }
}

impl GameEventHandler for CharacterHotkeyRemove {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterHotkeyRemove {
            char_id,
            index: hotkey_index,
        } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().hotkey_remove(character, hotkey_index);
        Ok(())
    }
}

impl GameEventHandler for CharacterRemoveItem {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_drop_item = self;
        if let Err(error) = server.server_service().character_drop_item(server, state, character_drop_item) {
            warn!("Item drop failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterResetSkills {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterResetSkills { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().reset_skills(character, true);
        Ok(())
    }
}

impl GameEventHandler for CharacterResetStats {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterResetStats { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().reset_stats(character);
        Ok(())
    }
}

impl GameEventHandler for CharacterUpdateSpeed {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterUpdateSpeed { char_id, speed } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        character.status.set_speed(speed);
        server.character_service().reload_client_side_status(character);
        Ok(())
    }
}

impl GameEventHandler for CharacterRestoreAllHpAndSP {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterRestoreAllHpAndSP { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        let status = StatusService::instance().to_snapshot(&character.status);
        server.character_service().update_hp_sp(character, status.max_hp(), status.max_sp());
        Ok(())
    }
}

impl GameEventHandler for CharacterDamage {
    fn required_character(&self) -> Option<u32> {
        Some(self.damage.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let CharacterDamage { damage } = self;
        if let Err(error) = server.admit_character_damage(state, damage, tick) {
            warn!("Character damage failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterRequestCardCompositionList {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_request_card_composition_list = self;
        let character = state
            .characters_mut()
            .get_mut(&character_request_card_composition_list.char_id)
            .unwrap();
        server
            .inventory_service()
            .send_card_composition_list(character, character_request_card_composition_list);
        Ok(())
    }
}

impl GameEventHandler for CharacterSlotCard {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_slot_card = self;
        let character = state.characters_mut().get_mut(&character_slot_card.char_id).unwrap();
        server
            .inventory_service()
            .slot_card(server.runtime(), character, character_slot_card);
        Ok(())
    }
}

impl GameEventHandler for MapNotifyItemRemoved {
    fn required_character(&self) -> Option<u32> {
        Some(self.map_item_id)
    }

    fn handle(self, _server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let MapNotifyItemRemoved { map_item_id } = self;
        state.remove_locked_map_item(map_item_id);
        Ok(())
    }
}

impl GameEventHandler for ReleaseScriptCapture {
    fn handle(self, _server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let ReleaseScriptCapture { id } = self;
        state.remove_locked_map_item(id);
        Ok(())
    }
}
