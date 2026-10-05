use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::{Status, StatusSnapshot};
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};

use super::metadata::SkillMetadata;
use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::service::script_world_service::companion_status_snapshot;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const INHERITED_SHIELDS: [StatusChangeKind; 4] = [
    StatusChangeKind::AutoGuard,
    StatusChangeKind::Defender,
    StatusChangeKind::ReflectShield,
    StatusChangeKind::Endure,
];

impl ScriptSkillService {
    pub fn devotion_slot(
        source_id: u32,
        source_level: u32,
        target_id: u32,
        target: &Status,
        skill_level: u8,
        links: &[(u32, usize)],
    ) -> Result<usize, String> {
        if source_id == target_id || target.hp == 0 {
            return Err("Devotion requires another living player".into());
        }
        if source_level.abs_diff(target.base_level) > 10 {
            return Err("Devotion requires player levels within ten levels".into());
        }
        if JobName::try_from_value(target.job as usize)
            .is_ok_and(|job| matches!(job, JobName::Crusader | JobName::Paladin | JobName::BabyCrusader))
        {
            return Err("Crusaders cannot receive Devotion".into());
        }
        if target.has_status_change(StatusChangeKind::HellPower) {
            return Err("Hell Power prevents Devotion".into());
        }
        if target
            .status_change(StatusChangeKind::Devotion)
            .is_some_and(|link| link.values[0] as u32 != source_id)
        {
            return Err("Player already has another Devotion protector".into());
        }
        let count = usize::from(skill_level.min(5));
        if let Some((_, slot)) = links.iter().find(|(id, slot)| *id == target_id && *slot < count) {
            return Ok(*slot);
        }
        (0..count)
            .find(|slot| links.iter().all(|(_, occupied)| occupied != slot))
            .ok_or("Every Devotion slot is occupied".into())
    }

    pub fn validate_devotion_target(
        &self,
        state: &ServerState,
        source: &Character,
        target: &Character,
        level: u8,
    ) -> Result<usize, String> {
        if source.map_instance_key != target.map_instance_key {
            return Err("Devotion target is on another map".into());
        }
        let links = state
            .characters()
            .values()
            .chain(std::iter::once(target))
            .filter_map(|target| {
                let link = target.status.status_change(StatusChangeKind::Devotion)?;
                (link.values[0] as u32 == source.char_id).then_some((target.char_id, link.values[1].max(0) as usize))
            })
            .collect::<Vec<_>>();
        Self::devotion_slot(
            source.char_id,
            source.status.base_level,
            target.char_id,
            &target.status,
            level,
            &links,
        )
    }

    pub fn validate_effect_target(
        &self,
        state: &ServerState,
        source: &Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<(), String> {
        if let super::ScriptSkillAction::OpenWarpPortalMenu(ref cast) = effect.action {
            self.validate_portal_cast(state, source, cast, effect.level)?;
        }
        if let super::ScriptSkillAction::CleanGraffiti { ref map, x, y } = effect.action {
            self.validate_graffiti_cleanup(state, source, map, x, y, effect.skill_id, effect.level)?;
        }
        if let super::ScriptSkillAction::MagicAttack {
            target_id,
            ref map,
            issued_skill,
        } = effect.action
        {
            self.validate_magic_target(state, source, effect.skill_id, effect.level, target_id, map, issued_skill)?;
        }
        if let super::ScriptSkillAction::TrapControl {
            trap_id,
            ref map,
            ignore_range,
            ..
        } = effect.action
        {
            if source.map_instance_key != *map {
                return Err("Trap control source changed maps".into());
            }
            Self::validate_stealth_cast(state, source, effect.skill_id)?;
            Self::validate_skill_map(state, source, effect.skill_id, effect.level, true)?;
            self.validate_player_trap_control(state, source, trap_id, effect.skill_id, effect.level, tick, ignore_range)?;
        }
        if effect.action == super::ScriptSkillAction::OpenTeleportMenu {
            if source.status.blocks_casting() {
                return Err("Character cannot open Teleport now".into());
            }
            Self::validate_skill_map(state, source, effect.skill_id, effect.level, false)?;
        }
        if matches!(effect.action, super::ScriptSkillAction::Cast) {
            Self::validate_stealth_cast(state, source, effect.skill_id)?;
            self.validate_support_target(state, source, effect.skill_id, effect.target_id)?;
            Self::validate_skill_map(state, source, effect.skill_id, effect.level, true)?;
        }
        if let super::ScriptSkillAction::WaterBall { sequence, cell } = effect.action {
            self.validate_water_ball_shot(source, effect, sequence, cell, tick)?;
        }
        if effect.skill_id == SkillEnum::AsSplasher.id() {
            self.validate_splasher_effect(state, source, effect)?;
        }
        if let super::ScriptSkillAction::ActivateGround { skill_id, cast_generation } = effect.action {
            self.validate_ground_activation(state, source, skill_id, cast_generation, tick)?;
        }
        if effect.skill_id == SkillEnum::CrDevotion.id() {
            let target = state.get_character(effect.target_id).ok_or("Devotion requires a player target")?;
            self.validate_devotion_target(state, source, target, effect.level)?;
        }
        Ok(())
    }

    pub(super) fn start_devotion(
        &self,
        server: &Server,
        state: &ServerState,
        target: &mut Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<(), String> {
        let source = state
            .get_character(effect.source_char_id)
            .ok_or("Devotion protector disconnected")?;
        let slot = self.validate_devotion_target(state, source, target, effect.level)?;
        let metadata = SkillMetadata::find(effect.skill_id).ok_or("Devotion metadata is missing")?;
        let range = metadata.range(effect.level).unwrap_or(7).max(1);
        if source.x.abs_diff(target.x).max(source.y.abs_diff(target.y)) > range as u16 {
            return Err("Devotion target left skill range".into());
        }
        let request = StatusChangeRequest {
            kind: StatusChangeKind::Devotion,
            values: [source.char_id as i32, slot as i32, range, 0],
            duration_ms: metadata.duration(effect.level, true).unwrap_or(0),
            rate: 10000,
            flags: StatusStartFlag::NoAvoid.as_flag(),
        };
        StatusEffectService::start(server, target, request, tick, &self.client_notification_sender)?;
        self.inherit_devotion_shields(
            server,
            target,
            source.char_id,
            &StatusService::instance().to_snapshot(&source.status),
            tick,
        )?;
        self.notify_devotion_links(state, source, Some(target), effect.level);
        Ok(())
    }

    fn inherit_devotion_shields(
        &self,
        server: &Server,
        target: &mut Character,
        source_id: u32,
        source: &StatusSnapshot,
        tick: u128,
    ) -> Result<(), String> {
        for kind in INHERITED_SHIELDS {
            let existing = target.status.status_change(kind).cloned();
            let parent = source.status_change(kind).filter(|change| !change.expired(tick));
            let Some(parent) = parent else {
                if existing.is_some_and(|change| change.inherited_from.is_some_and(|(id, _)| id == source_id)) {
                    StatusEffectService::end(server, target, Some(kind), tick, &self.client_notification_sender);
                }
                continue;
            };
            if existing
                .as_ref()
                .is_some_and(|change| change.inherited_from == Some((source_id, parent.started_at)))
            {
                continue;
            }
            let skill = match kind {
                StatusChangeKind::AutoGuard => SkillEnum::CrAutoguard,
                StatusChangeKind::Defender => SkillEnum::CrDefender,
                StatusChangeKind::ReflectShield => SkillEnum::CrReflectshield,
                _ => SkillEnum::SmEndure,
            };
            let duration = SkillMetadata::find(skill.id())
                .and_then(|metadata| metadata.duration(parent.values[0].clamp(1, 255) as u8, false))
                .unwrap_or_else(|| parent.remaining_ms(tick).min(i32::MAX as u32) as i32);
            let mut request = StatusChangeRequest::guaranteed(kind, duration, parent.values[0]);
            if kind != StatusChangeKind::Defender {
                request.flags |= StatusStartFlag::NoIcon.as_flag();
            }
            if StatusEffectService::start(server, target, request, tick, &self.client_notification_sender)? {
                if let Some(change) = target.status.active_statuses.iter_mut().find(|change| change.kind == kind) {
                    change.values = parent.values;
                    if kind == StatusChangeKind::ReflectShield {
                        change.values[3] = 1;
                    }
                    change.inherited_from = Some((source_id, parent.started_at));
                }
            }
        }
        Ok(())
    }

    pub fn tick_devotion_links(&self, server: &Server, state: &ServerState, character: &mut Character, tick: u128) {
        let Some(link) = character.status.status_change(StatusChangeKind::Devotion).cloned() else {
            return;
        };
        let source_id = link.values[0] as u32;
        let source = state
            .get_character(source_id)
            .filter(|source| source.status.hp > 0 && source.map_instance_key == character.map_instance_key)
            .map(|source| (source.x, source.y, StatusService::instance().to_snapshot(&source.status)))
            .or_else(|| {
                let snapshot = companion_status_snapshot(character, source_id)?;
                if snapshot.hp() == 0 {
                    return None;
                }
                let position = character
                    .game_systems
                    .rendered_companions
                    .get(&source_id)
                    .map(|position| (position.x, position.y))
                    .unwrap_or((character.x, character.y));
                Some((position.0, position.1, snapshot))
            });
        let valid = source
            .as_ref()
            .is_some_and(|(x, y, _)| character.x.abs_diff(*x).max(character.y.abs_diff(*y)) <= link.values[2].max(1) as u16);
        if !valid {
            StatusEffectService::end(
                server,
                character,
                Some(StatusChangeKind::Devotion),
                tick,
                &self.client_notification_sender,
            );
            if let Some(source) = state.get_character(source_id) {
                self.notify_devotion_links(
                    state,
                    source,
                    Some(character),
                    source.script_skill_state.casting_skill_level.max(1),
                );
            }
            return;
        }
        if let Some((_, _, source)) = source {
            let _ = self.inherit_devotion_shields(server, character, source_id, &source, tick);
        }
    }

    fn notify_devotion_links(&self, state: &ServerState, source: &Character, updated: Option<&Character>, level: u8) {
        let mut targets = [0u32; 5];
        for target in state
            .characters()
            .values()
            .filter(|target| updated.is_none_or(|updated| target.char_id != updated.char_id))
            .chain(updated)
        {
            if let Some(link) = target
                .status
                .status_change(StatusChangeKind::Devotion)
                .filter(|link| link.values[0] as u32 == source.char_id)
            {
                if let Some(slot) = targets.get_mut(link.values[1].max(0) as usize) {
                    *slot = target.char_id;
                }
            }
        }
        let mut packet = 0x01CF_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&source.char_id.to_le_bytes());
        for target in targets {
            packet.extend_from_slice(&target.to_le_bytes());
        }
        packet.extend_from_slice(
            &(SkillMetadata::find(SkillEnum::CrDevotion.id())
                .and_then(|metadata| metadata.range(level))
                .unwrap_or(7)
                .max(1) as u16)
                .to_le_bytes(),
        );
        self.notify_area(source, packet);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn devotion_reuses_slots_and_rejects_class_level_and_other_protectors() {
        let mut target = Status {
            hp: 100,
            base_level: 50,
            ..Status::default()
        };
        assert_eq!(ScriptSkillService::devotion_slot(1, 60, 2, &target, 2, &[(3, 0)]).unwrap(), 1);
        assert_eq!(
            ScriptSkillService::devotion_slot(1, 50, 2, &target, 2, &[(2, 0), (3, 1)]).unwrap(),
            0
        );
        assert!(ScriptSkillService::devotion_slot(1, 50, 2, &target, 1, &[(3, 0)]).is_err());
        assert!(ScriptSkillService::devotion_slot(1, 61, 2, &target, 5, &[]).is_err());
        target.job = JobName::Paladin.value() as u32;
        assert!(ScriptSkillService::devotion_slot(1, 50, 2, &target, 5, &[]).is_err());
        target.job = 0;
        StatusEffectService::apply_status(
            &mut target,
            StatusChangeRequest::guaranteed(StatusChangeKind::Devotion, 30000, 9),
            0,
            0,
        )
        .unwrap();
        assert!(ScriptSkillService::devotion_slot(1, 50, 2, &target, 5, &[]).is_err());
    }
}
