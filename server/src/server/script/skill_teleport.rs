use models::enums::skill_enums::SkillEnum;

use super::requirements::DeferredSkillPayment;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::ScriptTeleportSelection;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Clone, Debug)]
pub struct PendingTeleportMenu {
    pub skill_id: u32,
    pub level: u8,
    pub origin_map: MapInstanceKey,
    pub save_map: String,
}

impl ScriptSkillService {
    pub fn start_native_teleport_menu(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        level: u8,
        tick: u128,
    ) -> Result<(), String> {
        let skill_id = SkillEnum::AlTeleport.id();
        let skill = self
            .configuration
            .find_skill_config(&script_sdk::Value::Number(skill_id as i32))
            .ok_or("Teleport skill is unavailable")?;
        if !(1..=2).contains(&level) {
            return Err("Learned Teleport level must be one or two".into());
        }
        if character.status.hp == 0
            || character.status.blocks_casting()
            || character.is_using_skill()
            || character.script_skill_state.casting_until > tick
            || character.timing.get_canact_tick() > tick
        {
            return Err("Character cannot cast Teleport now".into());
        }
        Self::validate_stealth_cast(state, character, skill_id)?;
        Self::validate_skill_map(state, character, skill_id, level, false)?;
        let requirements = self.requirements_plan(character, skill_id, level, tick)?;
        character.script_skill_state.pending_teleport = None;
        character.script_skill_state.deferred_requirements = Some(DeferredSkillPayment {
            skill_id,
            level,
            keep_requirements: true,
            requirements,
            source_index: None,
            source_item: None,
        });
        self.end_cloaking_on_skill(server, character, skill_id, tick);
        self.queue_target_effect(
            server,
            character,
            skill,
            ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: 0,
                skill_event_emitted: false,
                cast_generation: 0,
                action: ScriptSkillAction::OpenTeleportMenu,
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            },
            tick,
        );
        Ok(())
    }

    pub(super) fn open_teleport_menu(&self, character: &mut Character, effect: &ScriptSkillEffect, _tick: u128) -> Result<(), String> {
        if effect.source_char_id != character.char_id
            || effect.target_id != character.char_id
            || effect.skill_id != SkillEnum::AlTeleport.id()
            || !(1..=2).contains(&effect.level)
        {
            return Err("Teleport menu does not match its caster".into());
        }
        let save_map = Map::name_without_ext(&character.save_map);
        character.script_skill_state.pending_teleport = Some(PendingTeleportMenu {
            skill_id: effect.skill_id,
            level: effect.level,
            origin_map: character.map_instance_key.clone(),
            save_map: save_map.clone(),
        });
        let mut maps = vec!["Random".to_owned()];
        if effect.level == 2 {
            maps.push(save_map);
        }
        self.notify_support_skill(character, effect);
        self.queue_notification(Notification::Char(CharNotification::new(
            character.char_id,
            Self::teleport_menu_packet(self.configuration.packetver(), effect.skill_id, &maps),
        )));
        Ok(())
    }

    pub fn teleport_menu_packet(packetver: u32, skill_id: u32, maps: &[String]) -> Vec<u8> {
        let modern = packetver >= 20170502;
        let count = if modern { maps.len().min(4) } else { 4 };
        let mut packet = (if modern { 0x0ABE_u16 } else { 0x011C_u16 }).to_le_bytes().to_vec();
        if modern {
            packet.extend_from_slice(&((6 + count * 16) as u16).to_le_bytes());
        }
        packet.extend_from_slice(&(skill_id as u16).to_le_bytes());
        for index in 0..count {
            let mut name = [0u8; 16];
            if let Some(map) = maps.get(index) {
                let map = format!("{}.gat", Map::name_without_ext(map));
                let length = map.len().min(15);
                name[..length].copy_from_slice(&map.as_bytes()[..length]);
            }
            packet.extend_from_slice(&name);
        }
        packet
    }

    pub fn finish_teleport_menu(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        selection: &ScriptTeleportSelection,
        _tick: u128,
    ) -> Result<(), String> {
        let pending = character
            .script_skill_state
            .pending_teleport
            .as_ref()
            .ok_or("No Teleport menu is awaiting a choice")?;
        if selection.char_id != character.char_id || selection.skill_id != pending.skill_id {
            return Err("Teleport selection does not match the menu".into());
        }
        let pending = character.script_skill_state.pending_teleport.take().unwrap();
        if character.map_instance_key != pending.origin_map {
            return Err("Teleport caster changed maps".into());
        }
        if character.status.hp == 0 || character.status.blocks_casting() {
            return Err("Character cannot finish Teleport now".into());
        }
        let map = Map::name_without_ext(&selection.map);
        character.clear_attack();
        if map == "cancel" {
            return Ok(());
        }
        Self::validate_skill_map(state, character, pending.skill_id, pending.level, true)?;
        if map == "Random" {
            server.server_service.schedule_warp_to_walkable_cell_by_character_in_instance(
                character.current_map_name(),
                RANDOM_CELL.0,
                RANDOM_CELL.1,
                character.char_id,
                character.current_map_instance(),
            );
        } else if pending.level == 2 && map == pending.save_map {
            let save_map = Map::name_without_ext(&character.save_map);
            server.server_service.schedule_warp_to_walkable_cell_by_character(
                &save_map,
                character.save_x,
                character.save_y,
                character.char_id,
            );
        } else {
            return Err("Destination was not offered by the Teleport menu".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teleport_menu_packets_use_primary_versioned_headers_and_map_extensions() {
        let maps = vec!["Random".into(), "prontera".into()];
        let legacy = ScriptSkillService::teleport_menu_packet(20150513, 26, &maps);
        assert_eq!(legacy.len(), 68);
        assert_eq!(&legacy[..4], &[28, 1, 26, 0]);
        assert_eq!(&legacy[4..14], b"Random.gat");
        assert_eq!(&legacy[20..33], b"prontera.gat\0");
        let modern = ScriptSkillService::teleport_menu_packet(20180620, 26, &maps);
        assert_eq!(modern.len(), 38);
        assert_eq!(&modern[..6], &[190, 10, 38, 0, 26, 0]);
        assert_eq!(&modern[6..16], b"Random.gat");
    }
}
