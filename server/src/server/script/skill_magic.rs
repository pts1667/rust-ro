use models::enums::skill::{UseSkillFailure, UseSkillFailureClientSideType};
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};
use packets::packets::{Packet, PacketZcAckTouseskill};

use super::metadata::SkillMetadata;
use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptMapDamage};
use crate::server::model::events::map_event::{MapEvent, MobDamage};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::{MapItemSnapshot, MapItemType, ToMapItemSnapshot};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub(crate) fn uses_metadata_magic(name: &str) -> bool {
        matches!(name, "SL_STIN" | "SL_STUN" | "SL_SMA") || super::monster::MonsterSkill::of_name(name) == Some(super::monster::MonsterSkill::MagicalAttack)
    }

    pub(super) fn validate_magic_target(
        &self,
        state: &ServerState,
        source: &Character,
        skill_id: u32,
        level: u8,
        target_id: u32,
        map: &MapInstanceKey,
        issued_skill: bool,
    ) -> Result<(MapItemSnapshot, StatusSnapshot), String> {
        if source.map_instance_key != *map {
            return Err("Magic caster changed maps".into());
        }
        if source.status.hp == 0 || source.status.blocks_casting() {
            return Err("Magic caster cannot complete the cast".into());
        }
        Self::validate_stealth_cast(state, source, skill_id)?;
        Self::validate_skill_map(state, source, skill_id, level, issued_skill)?;
        let (target, status) = if target_id == source.char_id {
            (
                source.to_map_item_snapshot(),
                StatusService::instance().to_snapshot(&source.status),
            )
        } else {
            let target = state
                .map_item_snapshot(target_id, map.map_name(), map.map_instance())
                .ok_or("Magic target left the map")?;
            let status = state
                .get_character(target_id)
                .filter(|character| character.map_instance_key == *map)
                .map(|character| StatusService::instance().to_snapshot(&character.status))
                .or_else(|| state.map_item_mob_status(&target.map_item(), map.map_name(), map.map_instance()))
                .ok_or("Magic target has no battle status")?;
            (target, status)
        };
        if status.hp() == 0 {
            return Err("Magic target died before completion".into());
        }
        Ok((target, status))
    }

    pub(super) fn execute_player_magic(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        target_id: u32,
        map: &MapInstanceKey,
        issued_skill: bool,
        tick: u128,
    ) -> Result<(), String> {
        let (target, target_status) =
            self.validate_magic_target(state, character, effect.skill_id, effect.level, target_id, map, issued_skill)?;
        if !server.player_skill_target_allowed(state, character, target_id, effect.skill_id, true) {
            return Err("Magic target is hidden or unavailable".into());
        }
        let metadata = SkillMetadata::find(effect.skill_id).ok_or("Magic skill metadata is unavailable")?;
        if metadata.name == "SL_SMA" {
            StatusEffectService::end(
                server,
                character,
                Some(StatusChangeKind::Sma),
                tick,
                &self.client_notification_sender,
            );
        }
        if matches!(metadata.name.as_str(), "SL_STIN" | "SL_STUN" | "SL_SMA")
            && *target.map_item().object_type() != MapItemType::Mob
            && !self.configuration.config().game.allow_es_magic_players
        {
            let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 500, i32::from(effect.level));
            request.flags = StatusStartFlag::NoRateReduction.as_flag() | StatusStartFlag::NoDurationReduction.as_flag();
            StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
            let mut packet = PacketZcAckTouseskill::new(server.packetver());
            packet.set_skid(effect.skill_id as u16);
            packet.set_cause(UseSkillFailure::Fail.value() as u8);
            packet.set_num(UseSkillFailureClientSideType::SkillFailed.value() as u32);
            packet.set_result(false);
            packet.fill_raw();
            self.client_notification_sender
                .send(Notification::Char(CharNotification::new(character.char_id, packet.raw)))
                .unwrap_or_else(|error| log::warn!("Es magic failure notification failed: {error}"));
            return Err("Es magic cannot target this actor".into());
        }
        let source = StatusService::instance().to_snapshot(&character.status);
        let (amount, context) = server
            .battle_service()
            .metadata_magic_damage(&source, &target_status, metadata, effect.level)?;
        let motion = StatusService::instance().attack_motion(&source);
        let mut damage = Damage {
            notification: None,
            source_kind: *source.combat_actor_kind(),
            skill_damage_adjusted: false,
            target_id,
            attacker_id: character.char_id,
            damage: 0,
            healing: 0,
            right_hand_damage: None,
            attacked_at: tick.saturating_add(u128::from(motion)),
            damage_motion: if *target.map_item().object_type() == MapItemType::Mob {
                self.configuration
                    .get_mob_safe(target.map_item().client_item_class() as i32)
                    .map_or(480, |mob| mob.damage_motion as u32)
            } else {
                480
            },
            battle_flags: metadata.battle_flags(true),
            skill_id: effect.skill_id,
            skill_level: effect.level,
            landed: true,
            proc_depth: effect.proc_depth,
            credit_id: character.char_id,
            defenses_applied: true,
            magic_context: Some(context),
        };
        damage.set_signed_damage(amount);
        damage = damage.with_skill_notification(
            map.map_name(),
            map.map_instance(),
            character.x,
            character.y,
            tick,
            context.hits.min(i16::MAX as u16) as i16,
            motion,
        );
        if *target.map_item().object_type() == MapItemType::Mob {
            state
                .get_map_instance(map.map_name(), map.map_instance())
                .ok_or("Magic map is unavailable")?
                .add_to_delayed_tick(MapEvent::MobDamage(MobDamage { damage }), u128::from(motion));
        } else {
            server.add_to_delayed_tick(
                GameEvent::ScriptMapDamage(ScriptMapDamage { map: map.clone(), damage }),
                u128::from(motion),
            );
        }
        if metadata.monster_skill() == Some(super::monster::MonsterSkill::MagicalAttack) {
            StatusEffectService::start(
                server,
                character,
                StatusChangeRequest::guaranteed(
                    StatusChangeKind::MagicalAttack,
                    metadata.duration(effect.level, false).unwrap_or(0),
                    i32::from(effect.level),
                ),
                tick,
                &self.client_notification_sender,
            )?;
        } else if matches!(metadata.name.as_str(), "SL_STIN" | "SL_STUN")
            && effect.level >= 7
            && !character
                .status
                .status_change(StatusChangeKind::Sma)
                .is_some_and(|ready| !ready.expired(tick))
        {
            let duration = SkillMetadata::find(SkillEnum::SlSma.id())
                .and_then(|metadata| metadata.duration(effect.level, false))
                .unwrap_or(3000);
            StatusEffectService::start(
                server,
                character,
                StatusChangeRequest::guaranteed(StatusChangeKind::Sma, duration, i32::from(effect.level)),
                tick,
                &self.client_notification_sender,
            )?;
        }
        Ok(())
    }
}
