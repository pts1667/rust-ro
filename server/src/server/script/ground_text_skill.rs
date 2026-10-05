use super::ScriptSkillService;
use super::ground::{GroundKind, GroundSkill};
use super::metadata::SkillMetadata;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub(crate) fn is_text_ground_skill(skill_id: u32) -> bool {
        SkillMetadata::find(skill_id).is_some_and(|skill| matches!(skill.name.as_str(), "HT_TALKIEBOX" | "RG_GRAFFITI"))
    }

    pub(super) fn tick_talkie_box(&self, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.kind != GroundKind::TalkieBox || ground.triggered || ground.expires_at <= tick {
            return;
        }
        if !state.characters().values().any(|player| {
            player.char_id != ground.source_id
                && player.loaded_from_client_side
                && player.status.hp > 0
                && player.current_map_name() == &ground.map
                && player.current_map_instance() == ground.instance
                && ground.covers(player.x, player.y)
        }) {
            return;
        }
        ground.triggered = true;
        ground.recovery_item = None;
        ground.expires_at = tick + 5000;
        let message_size = if self.configuration.packetver() >= 20190904 { 21 } else { 80 };
        for cell in ground.cells.iter().filter(|cell| cell.remaining_hits > 0) {
            let mut packet = 0x0191_u16.to_le_bytes().to_vec();
            packet.extend_from_slice(&cell.id.to_le_bytes());
            let length = ground.message.len().min(message_size - 1);
            packet.extend_from_slice(&ground.message[..length]);
            packet.resize(6 + message_size, 0);
            self.notify_ground_cell(ground, cell, packet);
        }
        self.change_trap_view(ground, 140);
    }

    pub(super) fn graffiti_entry_packet(unit_id: u32, source_id: u32, x: u16, y: u16, message: &[u8]) -> Vec<u8> {
        let mut packet = 0x01C9_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&unit_id.to_le_bytes());
        packet.extend_from_slice(&source_id.to_le_bytes());
        packet.extend_from_slice(&x.to_le_bytes());
        packet.extend_from_slice(&y.to_le_bytes());
        packet.extend_from_slice(&[GroundKind::Graffiti.view_id() as u8, 1, 1]);
        packet.extend_from_slice(&message[..message.len().min(79)]);
        packet.resize(97, 0);
        packet
    }

    pub(super) fn validate_graffiti_limit(&self, source: &super::GroundSkillSource, tick: u128) -> Result<(), String> {
        if *source.status.combat_actor_kind() != models::enums::actor::CombatActorKind::Player {
            return Ok(());
        }
        if self
            .ground_skills
            .lock()
            .map_err(|_| "Ground skill state is unavailable")?
            .iter()
            .any(|ground| {
                ground.kind == GroundKind::Graffiti
                    && ground.cast_verified
                    && ground.expires_at > tick
                    && ground.map == source.map
                    && ground.instance == source.instance
                    && ground.cells.iter().any(|cell| cell.remaining_hits > 0)
            })
        {
            return Err("Another Graffiti already exists on this map".into());
        }
        Ok(())
    }

    pub(super) fn erase_graffiti(&self, map: &str, instance: u8, x: u16, y: u16, radius: u16, tick: u128) -> Result<(), String> {
        let mut grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        for ground in grounds.iter_mut().filter(|ground| {
            ground.kind == GroundKind::Graffiti
                && ground.cast_verified
                && ground.expires_at > tick
                && ground.map == map
                && ground.instance == instance
                && ground
                    .cells
                    .iter()
                    .any(|cell| cell.remaining_hits > 0 && cell.x.abs_diff(x).max(cell.y.abs_diff(y)) <= radius)
        }) {
            ground.expires_at = tick;
        }
        Ok(())
    }

    pub(super) fn validate_graffiti_cleanup(
        &self,
        state: &ServerState,
        character: &crate::server::state::character::Character,
        map: &crate::server::model::map_instance::MapInstanceKey,
        x: u16,
        y: u16,
        skill_id: u32,
        level: u8,
    ) -> Result<(), String> {
        if character.map_instance_key != *map || character.status.hp == 0 || character.status.blocks_casting() {
            return Err("Graffiti removal cast was interrupted".into());
        }
        Self::validate_stealth_cast(state, character, skill_id)?;
        Self::validate_skill_map(state, character, skill_id, level, true)?;
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Graffiti removal map is unavailable")?;
        if x >= instance.x_size()
            || y >= instance.y_size()
            || instance
                .state()
                .cells()
                .get(usize::from(y) * usize::from(instance.x_size()) + usize::from(x))
                .is_none_or(|cell| cell & models::enums::EnumWithMaskValueU16::as_flag(&models::enums::cell::CellType::Shootable) == 0)
        {
            return Err("Graffiti removal target is outside usable terrain".into());
        }
        Ok(())
    }
}
