use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::look::LookType;
use models::enums::skill_enums::SkillEnum;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{EQUIPSLOTINFO, EquipmentitemExtrainfo301};

use crate::server::Server;
use crate::server::model::events::client_notification::{AreaNotification, CharNotification, Notification};
use crate::server::model::events::game_event::{ClientCommand, CharacterRemoveItem, CharacterRemoveItems};
use crate::server::model::permission_groups::Permission;
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::{Character, CharacterAction};
use crate::server::state::server::ServerState;

const EMOTION_COUNT: u8 = 88;
const EMOTION_CHAT_PROHIBIT: u8 = 34;
const MAX_DIRECTION: u8 = 7;
const MAX_HEAD_DIRECTION: u16 = 2;
const CONFIG_OPEN_EQUIPMENT_WINDOW: i32 = 0;
const CONFIG_CALL: i32 = 1;
const MSG_OPEN_EQUIPEDITEM_REFUSED: u16 = 1357;
const SUPER_NOVICE_PRAYER_STATUS_VALUE: i32 = 17;
const SUPER_NOVICE_PRAYER_SKILL_LEVEL: u8 = 5;
const TOKEN_OF_SIEGFRIED: [i32; 3] = [7621, 6293, 6316];

fn header(id: u16) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}

fn name_field(packet: &mut Vec<u8>, name: &str) {
    let mut field = [0u8; 24];
    let length = name.len().min(23);
    field[..length].copy_from_slice(&name.as_bytes()[..length]);
    packet.extend_from_slice(&field);
}

/// `ZC_ACK_TOUSESKILL` with the "no emotions" client message.
fn emotion_refused() -> Vec<u8> {
    let mut packet = header(0x0110);
    packet.extend_from_slice(&1u16.to_le_bytes());
    packet.extend_from_slice(&1u32.to_le_bytes());
    packet.extend_from_slice(&[0, 0]);
    packet
}

pub(crate) fn config_packet(kind: i32, enabled: bool) -> Vec<u8> {
    let mut packet = header(0x02D9);
    packet.extend_from_slice(&kind.to_le_bytes());
    packet.extend_from_slice(&i32::from(enabled).to_le_bytes());
    packet
}

pub(crate) fn equipment_window_packet(enabled: bool) -> Vec<u8> {
    let mut packet = header(0x02DA);
    packet.push(u8::from(enabled));
    packet
}

impl Server {
    pub(crate) fn send_raw(&self, char_id: u32, packet: Vec<u8>) {
        self.server_service()
            .notification_sender()
            .send(Notification::Char(CharNotification::new(char_id, packet)))
            .unwrap_or_else(|_| error!("Failed to send client command reply"));
    }

    pub(crate) fn send_area_raw(&self, character: &Character, packet: Vec<u8>, include_self: bool) {
        let notification = if include_self {
            AreaNotification::from_character(character, packet)
        } else {
            AreaNotification::from_character_exclude_self(character, packet)
        };
        self.server_service()
            .notification_sender()
            .send(Notification::Area(notification))
            .unwrap_or_else(|_| error!("Failed to send client command area notification"));
    }

    /// Settings the client restores on map entry (`clif_parse_LoadEndAck`).
    pub(crate) fn send_character_config(&self, character: &Character) {
        let systems = &character.game_systems;
        self.send_raw(character.char_id, equipment_window_packet(systems.show_equip));
        self.send_raw(character.char_id, config_packet(CONFIG_CALL, systems.disable_call));
    }

    pub(crate) fn run_client_command(
        &self,
        state: &mut ServerState,
        char_id: u32,
        command: ClientCommand,
        tick: u128,
    ) -> Result<(), String> {
        match command {
            ClientCommand::ChangeDirection { head_dir, dir } => self.change_direction(state, char_id, head_dir, dir),
            ClientCommand::Emotion(emotion) => self.emotion(state, char_id, emotion),
            ClientCommand::CharacterName(target_id) => self.reply_character_name(state, char_id, target_id),
            ClientCommand::PvpInfo => {
                let character = state.get_character(char_id).ok_or("Character is not in game")?;
                let mut packet = header(0x0210);
                packet.extend_from_slice(&character.char_id.to_le_bytes());
                packet.extend_from_slice(&character.account_id.to_le_bytes());
                packet.extend_from_slice(&character.pvp_won.to_le_bytes());
                packet.extend_from_slice(&character.pvp_lost.to_le_bytes());
                packet.extend_from_slice(&character.pvp_point.to_le_bytes());
                self.send_raw(char_id, packet);
            }
            ClientCommand::ViewEquipment { target_id } => self.view_equipment(state, char_id, target_id),
            ClientCommand::Config { kind, enabled } => {
                let character = state.characters_mut().get_mut(&char_id).ok_or("Character is not in game")?;
                match kind {
                    CONFIG_OPEN_EQUIPMENT_WINDOW => character.game_systems.show_equip = enabled,
                    CONFIG_CALL => character.game_systems.disable_call = enabled,
                    _ => return Err(format!("Unsupported configuration type {kind}")),
                }
                self.send_raw(char_id, config_packet(kind, enabled));
            }
            ClientCommand::LessEffect(enabled) => {
                let character = state.characters_mut().get_mut(&char_id).ok_or("Character is not in game")?;
                character.game_systems.less_effect = enabled;
            }
            ClientCommand::UserCount => {
                let mut packet = header(0x00C2);
                packet.extend_from_slice(&(state.characters().len() as u32).to_le_bytes());
                self.send_raw(char_id, packet);
            }
            ClientCommand::StopAttack => {
                let character = state.characters_mut().get_mut(&char_id).ok_or("Character is not in game")?;
                character.clear_attack();
            }
            ClientCommand::CloseStorage => {
                let character = state.characters_mut().get_mut(&char_id).ok_or("Character is not in game")?;
                self.script_world_service().close_storage(character)?;
            }
            ClientCommand::AutoRevive => self.revive_with_token(state, char_id),
            ClientCommand::ExplosionSpirits => self.pray_to_guardian_angel(state, char_id, tick)?,
            ClientCommand::AtCommand(text) => {
                crate::server::request_handler::atcommand::handle_atcommand(self, state, char_id, &format!("gm : {text}"));
            }
        }
        Ok(())
    }

    fn change_direction(&self, state: &mut ServerState, char_id: u32, head_dir: u16, dir: u8) {
        if dir > MAX_DIRECTION || head_dir > MAX_HEAD_DIRECTION {
            return;
        }
        let Some(character) = state.characters_mut().get_mut(&char_id) else {
            return;
        };
        character.dir = u16::from(dir);
        let mut packet = header(0x009C);
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&head_dir.to_le_bytes());
        packet.push(dir);
        self.send_area_raw(character, packet, false);
    }

    pub(crate) fn basic_skill_level(character: &Character) -> u8 {
        character
            .status
            .known_skills
            .iter()
            .find(|skill| skill.value == SkillEnum::NvBasic)
            .map_or(0, |skill| skill.level)
    }

    fn emotion(&self, state: &mut ServerState, char_id: u32, emotion: u8) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else {
            return;
        };
        if emotion >= EMOTION_COUNT {
            return;
        }
        let skilled = !self.configuration.game.basic_skill_check || Self::basic_skill_level(character) >= 2;
        let now = chrono::Utc::now().timestamp();
        let flooding = character.game_systems.last_emotion_secs + 1 >= now;
        if skilled && emotion != EMOTION_CHAT_PROHIBIT && !flooding {
            character.game_systems.last_emotion_secs = now;
            let mut packet = header(0x00C0);
            packet.extend_from_slice(&character.char_id.to_le_bytes());
            packet.push(emotion);
            self.send_area_raw(character, packet, true);
            return;
        }
        if skilled && emotion != EMOTION_CHAT_PROHIBIT {
            character.game_systems.last_emotion_secs = now;
        }
        self.send_raw(char_id, emotion_refused());
    }

    fn reply_character_name(&self, state: &ServerState, char_id: u32, target_id: u32) {
        let name = match state.get_character(target_id) {
            Some(target) => target.name.clone(),
            None => self
                .repository
                .char_find(target_id)
                .ok()
                .flatten()
                .map_or_else(|| "Unknown".to_string(), |record| record.name),
        };
        let mut packet = header(0x0194);
        packet.extend_from_slice(&target_id.to_le_bytes());
        name_field(&mut packet, &name);
        self.send_raw(char_id, packet);
    }

    fn view_equipment(&self, state: &ServerState, char_id: u32, target_id: u32) {
        let (Some(viewer), Some(target)) = (state.get_character(char_id), state.get_character(target_id)) else {
            return;
        };
        if viewer.map_instance_key != target.map_instance_key {
            return;
        }
        if !target.game_systems.show_equip && !state.has_permission(viewer.account_id, Permission::ViewEquipment) {
            let mut packet = header(0x0291);
            packet.extend_from_slice(&MSG_OPEN_EQUIPEDITEM_REFUSED.to_le_bytes());
            return self.send_raw(char_id, packet);
        }
        self.send_raw(char_id, self.equipment_window_reply(target));
    }

    fn equipment_window_reply(&self, target: &Character) -> Vec<u8> {
        let config = GlobalConfigService::instance();
        let packetver = config.packetver();
        let view = |item: Option<&models::item::WearGear>| item.and_then(|gear| config.get_item(gear.item_id).view).unwrap_or(0) as u16;
        let mut packet = header(0x0859);
        packet.extend_from_slice(&[0, 0]);
        name_field(&mut packet, &target.name);
        let words = [
            target.status.job as u16,
            target.get_look(LookType::Hair) as u16,
            view(target.status.head_low()),
            view(target.status.head_mid()),
            view(target.status.head_top()),
            target.get_look(LookType::Robe) as u16,
            target.get_look(LookType::HairColor) as u16,
            target.get_look(LookType::ClothesColor) as u16,
        ];
        for word in words {
            packet.extend_from_slice(&word.to_le_bytes());
        }
        packet.push(target.sex);
        for (index, item) in target.inventory_equip().into_iter().filter(|(_, item)| item.equip != 0) {
            let mut entry = EquipmentitemExtrainfo301::new(packetver);
            entry.set_itid(item.item_id as u16);
            entry.set_atype(item.item_type().value() as u8);
            entry.set_index(index as i16);
            entry.set_is_damaged(item.is_damaged);
            entry.set_is_identified(item.is_identified);
            entry.set_location(config.get_item(item.item_id).location as u16);
            entry.set_wear_state(item.equip as u16);
            entry.set_refining_level(item.refine as u8);
            let mut slots = EQUIPSLOTINFO::new(packetver);
            slots.set_card1(item.card0 as u16);
            slots.set_card2(item.card1 as u16);
            slots.set_card3(item.card2 as u16);
            slots.set_card4(item.card3 as u16);
            entry.set_slot(slots);
            entry.fill_raw();
            packet.extend_from_slice(&entry.raw);
        }
        let length = packet.len() as u16;
        packet[2..4].copy_from_slice(&length.to_le_bytes());
        packet
    }

    /// Token of Siegfried: stands the dead character up with full HP and SP.
    fn revive_with_token(&self, state: &mut ServerState, char_id: u32) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else {
            return;
        };
        if character.status.hp > 0 || character.status.has_status_change(StatusChangeKind::HellPower) {
            return;
        }
        let Some(index) = character
            .inventory
            .iter()
            .position(|slot| slot.as_ref().is_some_and(|item| TOKEN_OF_SIEGFRIED.contains(&item.item_id)))
        else {
            return;
        };
        let removal = CharacterRemoveItems {
            char_id,
            sell: false,
            items: vec![CharacterRemoveItem { char_id, index, amount: 1, price: 0 }],
            notify_client: true,
        };
        if self.inventory_service().remove_item_from_inventory(self.runtime.as_ref(), removal, character).is_err() {
            return;
        }
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        self.character_service().update_hp_sp(character, snapshot.max_hp(), snapshot.max_sp());
        character.action = CharacterAction::Idle;
        let mut packet = header(0x0148);
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&0u16.to_le_bytes());
        self.send_area_raw(character, packet, true);
    }

    /// Super Novice prayer: at exactly 10%, 20%, ... of the next base level the character gains Explosion Spirits.
    fn pray_to_guardian_angel(&self, state: &mut ServerState, char_id: u32, tick: u128) -> Result<(), String> {
        let mut character = state.characters_mut().remove(&char_id).ok_or("Character is not in game")?;
        let result = self.start_prayer_status(&mut character, tick);
        state.insert_character(character);
        result
    }

    fn start_prayer_status(&self, character: &mut Character, tick: u128) -> Result<(), String> {
        if !JobName::try_from_value(character.status.job as usize).is_ok_and(|job| job == JobName::SuperNovice) {
            return Ok(());
        }
        let next = self.character_service().next_base_level_required_exp(&character.status);
        if next == 0 || next == u32::MAX {
            return Ok(());
        }
        let permille = (f64::from(character.status.base_exp) / f64::from(next) * 1000.0) as u32;
        if permille == 0 || permille % 100 != 0 {
            return Ok(());
        }
        let duration = SkillMetadata::find(SkillEnum::MoExplosionspirits.id())
            .and_then(|metadata| metadata.duration(SUPER_NOVICE_PRAYER_SKILL_LEVEL, false))
            .ok_or("Explosion Spirits has no duration")?;
        StatusEffectService::start(
            self,
            character,
            StatusChangeRequest::guaranteed(StatusChangeKind::ExplosionSpirits, duration, SUPER_NOVICE_PRAYER_STATUS_VALUE),
            tick,
            &self.server_service().notification_sender(),
        )?;
        Ok(())
    }
}
