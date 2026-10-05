use std::sync::atomic::{AtomicU64, Ordering};

use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{Packet, PacketZcUseSkill};

use crate::server::Server;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::{CharacterStatusChange, GameEvent, ScriptWarp};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub(crate) const GD_BATTLEORDER: u32 = 10010;
pub(crate) const GD_REGENERATION: u32 = 10011;
pub(crate) const GD_RESTORE: u32 = 10012;
pub(crate) const GD_EMERGENCYCALL: u32 = 10013;
pub(crate) const GD_ITEMEMERGENCYCALL: u32 = 10015;

const AURAS: [(u32, StatusChangeKind); 4] = [
    (10006, StatusChangeKind::Leadership),
    (10007, StatusChangeKind::GloryWounds),
    (10008, StatusChangeKind::SoulCold),
    (10009, StatusChangeKind::HawkEyes),
];
const AURA_RANGE: u16 = 2;
const AURA_LIFETIME_MS: i32 = 3_000;
const AURA_REFRESH_BELOW_MS: u32 = 1_500;
const AURA_INTERVAL_MS: u64 = 1_000;
static NEXT_AURA_TICK: AtomicU64 = AtomicU64::new(0);
const CALL_OFFSETS: [(i32, i32); 9] = [(-1, 0), (1, 0), (0, 1), (0, -1), (-1, 1), (1, -1), (-1, -1), (1, 1), (0, 0)];

pub(crate) fn is_active_guild_skill(skill_id: u32) -> bool {
    matches!(skill_id, GD_BATTLEORDER | GD_REGENERATION | GD_RESTORE | GD_EMERGENCYCALL)
}

fn in_range(source: &Character, target: &Character, range: u16) -> bool {
    source.map_instance_key == target.map_instance_key && source.x.abs_diff(target.x).max(source.y.abs_diff(target.y)) <= range
}

impl Server {
    pub(crate) fn use_guild_skill(&self, state: &mut ServerState, caster: &mut Character, skill_id: u32, level: u8, tick: u128) -> Result<(), String> {
        let guild = self
            .repository
            .guild(caster.game_systems.guild_id)
            .map_err(|error| error.to_string())?
            .filter(|guild| guild.master_char_id == caster.char_id)
            .ok_or("Only the guild master can use guild skills")?;
        if level == 0 || level > guild.skill_level(skill_id) {
            return Err("Guild skill level is not learned".into());
        }
        let flags = state.map_flags(&caster.map_instance_key);
        if !flags.is_gvg() {
            return Err("Guild skills are restricted to Guild vs Guild maps".into());
        }
        if caster.script_skill_state.skill_blocked_until.get(&skill_id).is_some_and(|until| *until > tick) {
            return Err("Guild skill is still blocked".into());
        }
        let metadata = SkillMetadata::find(skill_id).ok_or("Guild skill metadata is unavailable")?;
        let sp_cost = metadata
            .requires
            .as_ref()
            .and_then(|requires| requires.get("SpCost"))
            .and_then(serde_json::Value::as_u64)
            .map_or(0, |cost| cost as u32);
        if caster.status.sp < sp_cost {
            return Err("Not enough SP for the guild skill".into());
        }
        caster.status.set_sp(caster.status.sp - sp_cost);
        let range = metadata.splash(level).unwrap_or(15).clamp(0, i32::from(u16::MAX)) as u16;
        let recipients: Vec<u32> = state
            .characters()
            .values()
            .filter(|member| member.game_systems.guild_id == guild.id && member.status.hp > 0 && in_range(caster, member, range))
            .map(|member| member.char_id)
            .collect();
        self.announce_guild_skill(caster, skill_id, level);
        match skill_id {
            GD_BATTLEORDER | GD_REGENERATION => {
                let kind = if skill_id == GD_BATTLEORDER { StatusChangeKind::BattleOrders } else { StatusChangeKind::Regeneration };
                let duration = metadata.duration(level, false).unwrap_or(0);
                let request = StatusChangeRequest::guaranteed(kind, duration, i32::from(level));
                StatusEffectService::start(self, caster, request.clone(), tick, &self.server_service().notification_sender())?;
                for char_id in recipients {
                    self.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange { char_id, request: request.clone() }));
                }
            }
            GD_RESTORE => {
                self.restore_guild_member(caster);
                for char_id in recipients {
                    if let Some(member) = state.characters_mut().get_mut(&char_id) {
                        self.restore_guild_member(member);
                    }
                }
            }
            GD_EMERGENCYCALL => self.call_guild_members(state, caster, guild.id, None),
            _ => return Err("Unsupported guild skill".into()),
        }
        let block = metadata.duration(level, true).unwrap_or(0).max(0) as u128;
        caster.script_skill_state.skill_blocked_until.insert(skill_id, tick + block);
        Ok(())
    }

    /// Scroll variant of Emergency Call: any guild member may read it, and it summons a level-bound number of members.
    pub(crate) fn use_item_emergency_call(&self, state: &mut ServerState, char_id: u32, level: u16) -> Result<(), String> {
        let calls = match level {
            1 => 7,
            2 => 12,
            3 => 20,
            _ => return Err("Invalid Emergency Call level".into()),
        };
        let mut caster = state.characters_mut().remove(&char_id).ok_or("Emergency Call source disconnected")?;
        let guild_id = caster.game_systems.guild_id;
        let result = if guild_id == 0 {
            Err("Emergency Call requires a guild".to_string())
        } else if !state.map_flags(&caster.map_instance_key).is_gvg() {
            Err("Guild skills are restricted to Guild vs Guild maps".to_string())
        } else {
            self.announce_guild_skill(&caster, GD_ITEMEMERGENCYCALL, level as u8);
            self.call_guild_members(state, &caster, guild_id, Some(calls));
            Ok(())
        };
        state.insert_character(caster);
        result
    }

    fn announce_guild_skill(&self, caster: &Character, skill_id: u32, level: u8) {
        let mut packet = PacketZcUseSkill::new(GlobalConfigService::instance().packetver());
        packet.set_src_aid(caster.char_id);
        packet.set_target_aid(caster.char_id);
        packet.set_skid(skill_id as u16);
        packet.set_level(i16::from(level));
        packet.set_result(true);
        packet.fill_raw();
        let notification = Notification::Area(AreaNotification::new(
            caster.current_map_name().clone(),
            caster.current_map_instance(),
            AreaNotificationRangeType::Fov { x: caster.x, y: caster.y, exclude_id: None },
            packet.raw,
        ));
        if self.server_service().notification_sender().try_send(notification).is_err() {
            log::warn!("Unable to announce guild skill {skill_id}");
        }
    }

    fn restore_guild_member(&self, member: &mut Character) {
        let snapshot = StatusService::instance().to_snapshot(&member.status);
        let hp = (member.status.hp + snapshot.max_hp() * 9 / 10).min(snapshot.max_hp());
        let sp = (member.status.sp + snapshot.max_sp() * 9 / 10).min(snapshot.max_sp());
        self.character_service().update_hp_sp(member, hp, sp);
    }

    /// Summons online guild members next to the caster; `limit` caps the number of members called.
    pub(crate) fn call_guild_members(&self, state: &mut ServerState, caster: &Character, guild_id: u32, limit: Option<usize>) {
        let destination_map = normalize_map(caster.current_map_name());
        let mut called = 0;
        let members: Vec<(u32, MapInstanceKey)> = state
            .characters()
            .values()
            .filter(|member| member.game_systems.guild_id == guild_id && member.char_id != caster.char_id)
            .filter(|member| member.status.hp > 0 && !member.is_dead() && member.game_systems.vending_store.is_none())
            .map(|member| (member.char_id, member.map_instance_key.clone()))
            .collect();
        for (index, (char_id, origin)) in members.into_iter().enumerate() {
            if limit.is_some_and(|limit| called >= limit) {
                break;
            }
            let origin_flags = state.map_flags(&origin);
            if origin_flags.enabled(MapFlag::NoWarp) && !origin_flags.is_gvg() {
                continue;
            }
            let (dx, dy) = CALL_OFFSETS[index % CALL_OFFSETS.len()];
            let x = (i32::from(caster.x) + dx).max(0) as u16;
            let y = (i32::from(caster.y) + dy).max(0) as u16;
            self.add_to_next_tick(GameEvent::ScriptWarp(ScriptWarp {
                char_id,
                map: destination_map.clone(),
                x,
                y,
                destination_instance: Some(caster.current_map_instance()),
            }));
            called += 1;
        }
    }

    /// Refreshes the Leadership, Glory Wounds, Soul Cold and Hawk Eyes auras projected by online guild masters.
    pub(crate) fn apply_guild_auras(&self, state: &mut ServerState, tick: u128) {
        if (tick as u64) < NEXT_AURA_TICK.load(Ordering::Relaxed) {
            return;
        }
        NEXT_AURA_TICK.store(tick as u64 + AURA_INTERVAL_MS, Ordering::Relaxed);
        let masters: Vec<(u32, u32)> = state
            .characters()
            .values()
            .filter(|character| character.game_systems.guild_id != 0 && character.status.hp > 0)
            .map(|character| (character.char_id, character.game_systems.guild_id))
            .collect();
        for (master_id, guild_id) in masters {
            let Ok(Some(guild)) = self.repository.guild(guild_id) else { continue };
            if guild.master_char_id != master_id {
                continue;
            }
            let auras: Vec<(StatusChangeKind, u8)> = AURAS
                .iter()
                .map(|(skill_id, kind)| (*kind, guild.skill_level(*skill_id)))
                .filter(|(_, level)| *level > 0)
                .collect();
            if auras.is_empty() {
                continue;
            }
            let Some(master) = state.get_character(master_id) else { continue };
            let targets: Vec<u32> = state
                .characters()
                .values()
                .filter(|member| member.game_systems.guild_id == guild_id && member.status.hp > 0 && in_range(master, member, AURA_RANGE))
                .map(|member| member.char_id)
                .collect();
            for char_id in targets {
                let Some(member) = state.characters_mut().get_mut(&char_id) else { continue };
                for (kind, level) in &auras {
                    let current = member.status.status_change(*kind).map(|change| change.remaining_ms(tick));
                    if current.is_some_and(|remaining| remaining >= AURA_REFRESH_BELOW_MS) {
                        continue;
                    }
                    let request = StatusChangeRequest::guaranteed(*kind, AURA_LIFETIME_MS, i32::from(*level));
                    if let Err(error) = StatusEffectService::start(self, member, request, tick, &self.server_service().notification_sender()) {
                        log::warn!("Guild aura failed for {char_id}: {error}");
                    }
                }
            }
        }
    }
}
