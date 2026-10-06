use models::enums::cell::CellType;
use models::enums::bonus::BonusType;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
use models::status::Status;
use models::status_change::{CloakingFlag, StatusChangeKind};

use super::{metadata::SkillMetadata, ScriptSkillService};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

impl ScriptSkillService {
    pub fn direction_to(x: u16, y: u16, target_x: u16, target_y: u16, fallback: u16) -> u16 {
        let (dx, dy) = (i32::from(target_x) - i32::from(x), i32::from(target_y) - i32::from(y));
        if dx == 0 && dy == 0 { return fallback % 8; }
        if dx.abs() >= 3 * dy.abs() { return if dx > 0 { 6 } else { 2 }; }
        if dy.abs() > 3 * dx.abs() { return if dy > 0 { 0 } else { 4 }; }
        match (dx > 0, dy > 0) { (true, true) => 7, (true, false) => 5, (false, false) => 3, (false, true) => 1 }
    }

    pub fn player_skill_source_allowed(status: &Status, skill_id: u32) -> bool {
        (!status.has_status_change(StatusChangeKind::Hiding)
            || SkillMetadata::find(skill_id).is_some_and(|metadata| metadata.flags.get("AllowWhenHidden").copied().unwrap_or(false)))
            && (!status.has_status_change(StatusChangeKind::ChaseWalk) || skill_id == SkillEnum::StChasewalk.id())
    }

    pub fn area_skill_target_allowed(source: &models::status::StatusSnapshot, target: &models::status::StatusSnapshot, skill_id: u32) -> bool {
        use crate::server::service::visibility_service::{can_target, StealthState, TargetingMode, VisibilityObserver};
        let can_hit_hidden = SkillMetadata::find(skill_id).is_some_and(|metadata| metadata.flags.get("TargetHidden").copied().unwrap_or(false));
        can_target(VisibilityObserver::player(source), StealthState::from_snapshot(target), TargetingMode::Area { can_hit_hidden })
    }

    pub fn adjacent_cloaking_wall(state: &ServerState, character: &Character) -> bool {
        let Some(instance) = state.get_map_instance_from_character(character) else { return false; };
        let map_state = instance.state();
        (-1..=1).any(|dy| (-1..=1).any(|dx| {
            if dx == 0 && dy == 0 { return false; }
            let (x, y) = (i32::from(character.x) + dx, i32::from(character.y) + dy);
            x < 0 || y < 0 || x >= i32::from(instance.x_size()) || y >= i32::from(instance.y_size())
                || map_state.cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() == 0
        }))
    }

    pub fn validate_stealth_cast(state: &ServerState, character: &Character, skill_id: u32) -> Result<(), String> {
        if !Self::player_skill_source_allowed(&character.status, skill_id) { return Err("This skill cannot be used while hidden".into()); }
        if skill_id == SkillEnum::AsCloaking.id() && !character.status.has_status_change(StatusChangeKind::Cloaking) {
            let snapshot = StatusService::instance().to_snapshot(&character.status);
            let learned = snapshot.known_skills().iter().find(|skill| skill.value == SkillEnum::AsCloaking).map_or(0, |skill| skill.level);
            if learned < 3 && !Self::adjacent_cloaking_wall(state, character) { return Err("Cloaking below learned level three requires an adjacent wall".into()); }
        }
        Ok(())
    }

    pub fn end_cloaking_on_skill(&self, server: &Server, character: &mut Character, skill_id: u32, tick: u128) {
        if skill_id != SkillEnum::AsCloaking.id() && character.status.status_change(StatusChangeKind::Cloaking).is_some_and(|change| change.values[3] as u32 & CloakingFlag::AllowSkills.as_flag() == 0) {
            StatusEffectService::end(server, character, Some(StatusChangeKind::Cloaking), tick, &self.client_notification_sender);
        }
    }

    pub fn admit_normal_attack(&self, server: &Server, character: &mut Character, tick: u128) -> bool {
        if character.status.hp == 0 || character.status.has_status_change(StatusChangeKind::Hiding) || character.status.has_status_change(StatusChangeKind::ChaseWalk) { return false; }
        if character.status.status_change(StatusChangeKind::Cloaking).is_some_and(|change| change.values[3] as u32 & CloakingFlag::AllowAttacks.as_flag() == 0) {
            StatusEffectService::end(server, character, Some(StatusChangeKind::Cloaking), tick, &self.client_notification_sender);
        }
        true
    }

    pub fn knockback_destination(x: u16, y: u16, source_x: u16, source_y: u16, cells: u16, mut walkable: impl FnMut(u16, u16) -> bool) -> (u16, u16) {
        let (dx, dy) = ((i32::from(x) - i32::from(source_x)).signum(), (i32::from(y) - i32::from(source_y)).signum());
        let (mut x, mut y) = (x, y);
        if dx == 0 && dy == 0 { return (x, y); }
        for _ in 0..cells.min(100) {
            let (Ok(next_x), Ok(next_y)) = (u16::try_from(i32::from(x) + dx), u16::try_from(i32::from(y) + dy)) else { break; };
            if !walkable(next_x, next_y) { break; }
            (x, y) = (next_x, next_y);
        }
        (x, y)
    }

    pub fn apply_knockback(&self, server: &Server, state: &ServerState, character: &mut Character, source_x: u16, source_y: u16, cells: u16, tick: u128) -> Result<(), String> {
        if character.status.hp == 0 || cells == 0 { return Ok(()); }
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        if snapshot.bonuses_raw().iter().any(|bonus| matches!(bonus, BonusType::EnableNoKnockback)) { return Ok(()); }
        let instance = state.get_map_instance_from_character(character).ok_or("Knockback map is unavailable")?;
        let map_state = instance.state();
        let (x, y) = Self::knockback_destination(character.x, character.y, source_x, source_y, cells, |x, y| x < instance.x_size() && y < instance.y_size() && map_state.cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() != 0);
        if (x, y) == (character.x, character.y) { return Ok(()); }
        server.character_service().cancel_movement(character, tick);
        character.clear_pending_skill(); character.clear_attack(); character.update_position(x, y); character.last_moved_at = tick;
        let mut packet = 0x0088_u16.to_le_bytes().to_vec(); packet.extend_from_slice(&character.char_id.to_le_bytes()); packet.extend_from_slice(&x.to_le_bytes()); packet.extend_from_slice(&y.to_le_bytes());
        self.notify_area(character, packet);
        Ok(())
    }

    pub fn tick_stealth(&self, server: &Server, state: &ServerState, character: &mut Character, tick: u128) {
        let Some(change) = character.status.status_change(StatusChangeKind::Cloaking) else { return; };
        let wall = Self::adjacent_cloaking_wall(state, character);
        if !wall && change.values[0] < 3 {
            StatusEffectService::end(server, character, Some(StatusChangeKind::Cloaking), tick, &self.client_notification_sender);
            return;
        }
        let was_wall = change.values[3] as u32 & CloakingFlag::AdjacentWall.as_flag() != 0;
        if wall != was_wall {
            let change = character.status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Cloaking).unwrap();
            if wall { change.values[3] |= CloakingFlag::AdjacentWall.as_flag() as i32; }
            else { change.values[3] &= !(CloakingFlag::AdjacentWall.as_flag() as i32); }
            server.character_service().reload_client_side_status(character);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::status_change::StatusChangeRequest;

    #[test]
    fn hiding_and_chase_walk_enforce_their_actual_skill_permissions() {
        let mut status = Status { hp: 100, ..Status::default() };
        StatusEffectService::apply_status(&mut status, StatusChangeRequest::guaranteed(StatusChangeKind::Hiding, 30000, 1), 0, 0).unwrap();
        assert!(ScriptSkillService::player_skill_source_allowed(&status, SkillEnum::TfHiding.id()));
        assert!(ScriptSkillService::player_skill_source_allowed(&status, SkillEnum::RgRaid.id()));
        assert!(!ScriptSkillService::player_skill_source_allowed(&status, SkillEnum::MgFirebolt.id()));
        StatusEffectService::end_status(&mut status, None);
        StatusEffectService::apply_status(&mut status, StatusChangeRequest::guaranteed(StatusChangeKind::ChaseWalk, 10000, 1), 0, 0).unwrap();
        assert!(ScriptSkillService::player_skill_source_allowed(&status, SkillEnum::StChasewalk.id()));
        assert!(!ScriptSkillService::player_skill_source_allowed(&status, SkillEnum::RgRaid.id()));
    }

    #[test]
    fn knockback_stops_at_obstacles_and_map_edges_without_crossing_the_source() {
        assert_eq!(ScriptSkillService::knockback_destination(5, 5, 4, 5, 10, |x, _| x < 8), (7, 5));
        assert_eq!(ScriptSkillService::knockback_destination(1, 1, 2, 2, 10, |_, _| true), (0, 0));
        assert_eq!(ScriptSkillService::knockback_destination(5, 5, 5, 5, 10, |_, _| true), (5, 5));
    }
}
