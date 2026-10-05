use models::enums::element::Element;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{Packet, PacketZcUseSkill};
use script_sdk::Value;

use super::ScriptSkillService;
use super::ground_unit_effects::GANBANTEIN_SUCCESS_PERCENT;
use super::actor::{self, ScriptSkillActor};
use super::ground::GroundKind;
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterEndStatus, CharacterStatusChange, GameEvent, ScriptSkillCast};
use crate::server::model::events::map_event::{MapEvent, MobDispel, MobProvoke, MobDamage, MobEndStatus, MobHeal, MobKnockback, MobLoseTarget, MobRandomWarp, MobSlide, MobStatusChange};
use crate::server::model::map_item::MapItemType;
use crate::server::service::battle_service::BattleService;
use crate::server::service::combat_trigger_service::ComaBonuses;
use crate::server::service::script_combat_service::{BreakSlot, break_equipment};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

const PALM_STRIKE_DELAY: u128 = 1000;
const INTIMIDATE_WARP_DELAY_MS: u128 = 800;
const RANDOM_CELL_ATTEMPTS: usize = 64;

impl ScriptSkillService {
    pub(super) fn after_actor_skill_damage(
        &self,
        server: &Server,
        state: &mut ServerState,
        hit: super::ScriptSkillHit,
        tick: u128,
    ) -> Result<(), String> {
        let Some(source) = self.find_script_skill_actor_in(state, hit.source_id, hit.source_map.as_deref(), hit.source_instance)? else {
            return Ok(());
        };
        if !matches!(source.object_type, MapItemType::Mob | MapItemType::Npc) {
            return Ok(());
        }
        let metadata = SkillMetadata::find(hit.skill_id).ok_or("Actor skill callback metadata is unavailable")?;
        let (_, target) = self.actor_target_status(state, &source, hit.target_id)?;
        if target.hp() == 0 {
            return Ok(());
        }
        let weapon_break_rate = source
            .status
            .status_change(StatusChangeKind::WeaponBreaker)
            .map_or(0, |change| change.values[1]);
        if weapon_break_rate > 0
            && metadata.damage_type.as_deref() == Some("Weapon")
            && state.get_character(hit.target_id).is_some()
            && fastrand::i32(0..10_000) < weapon_break_rate
        {
            break_equipment(server, state, hit.target_id, BreakSlot::Weapon)?;
        }
        if metadata.name == "RG_INTIMIDATE" && source.object_type == MapItemType::Mob && hit.damage > 0 {
            self.actor_intimidate_warp(server, state, &source, &hit, &target, tick);
        }
        if metadata.name == "TF_THROWSTONE" {
            let requests = Self::stone_fling_status_requests(false, hit.skill_level);
            if state.get_character(hit.target_id).is_some() || state.companion_owner(hit.target_id, &source.map, source.instance).is_some()
            {
                server.add_to_next_tick(GameEvent::CharacterStatusAlternatives(
                    crate::server::model::events::game_event::CharacterStatusAlternatives {
                        char_id: hit.target_id,
                        requests,
                    },
                ));
            } else {
                state
                    .get_map_instance(&source.map, source.instance)
                    .ok_or("Actor callback map is unavailable")?
                    .add_to_next_tick(MapEvent::MobStatusAlternatives(
                        crate::server::model::events::map_event::MobStatusAlternatives {
                            mob_id: hit.target_id,
                            requests,
                        },
                    ));
            }
        }
        let joints = [
            models::status_change::JointBreak::Ankle,
            models::status_change::JointBreak::Wrist,
            models::status_change::JointBreak::Knee,
            models::status_change::JointBreak::Shoulder,
            models::status_change::JointBreak::Waist,
            models::status_change::JointBreak::Neck,
        ];
        for request in Self::secondary_status_requests(
            metadata,
            hit.skill_level,
            source.status.base_level(),
            false,
            false,
            &target,
            0,
            joints[fastrand::usize(0..6)],
        ) {
            self.start_actor_target_status(server, state, &source, hit.target_id, request, 0, tick)?;
        }
        Ok(())
    }

    fn actor_intimidate_warp(
        &self,
        server: &Server,
        state: &ServerState,
        source: &ScriptSkillActor,
        hit: &super::ScriptSkillHit,
        target: &StatusSnapshot,
        tick: u128,
    ) {
        let _ = tick;
        if target.mob_capabilities() & models::enums::mob::MobCapability::StatusImmune.as_flag() != 0
            || state
                .map_flags_for(&source.map, source.instance)
                .enabled(crate::server::model::map_flags::MapFlag::NoTeleport)
        {
            return;
        }
        let rate = 50 + 5 * i32::from(hit.skill_level) + source.status.base_level() as i32 - target.base_level() as i32;
        if fastrand::i32(0..100) >= rate {
            return;
        }
        let Some(instance) = state.get_map_instance(&source.map, source.instance) else {
            return;
        };
        let Some((x, y)) = Self::random_walkable_cell(instance.x_size(), instance.y_size(), instance.state().cells()) else {
            return;
        };
        instance.add_to_delayed_tick(
            MapEvent::MobWarpTo(crate::server::model::events::map_event::MobWarpTo { mob_id: source.id, x, y }),
            INTIMIDATE_WARP_DELAY_MS,
        );
        if state.get_character(hit.target_id).is_some() {
            server.add_to_delayed_tick(
                GameEvent::ScriptWarp(crate::server::model::events::game_event::ScriptWarp {
                    char_id: hit.target_id,
                    map: source.map.clone(),
                    x: x.saturating_add(1),
                    y,
                    destination_instance: Some(source.instance),
                }),
                INTIMIDATE_WARP_DELAY_MS,
            );
        } else {
            instance.add_to_delayed_tick(
                MapEvent::MobWarpTo(crate::server::model::events::map_event::MobWarpTo {
                    mob_id: hit.target_id,
                    x: x.saturating_add(1),
                    y,
                }),
                INTIMIDATE_WARP_DELAY_MS,
            );
        }
    }

    fn random_walkable_cell(x_size: u16, y_size: u16, cells: &[u16]) -> Option<(u16, u16)> {
        use models::enums::cell::CellType;
        use models::enums::EnumWithMaskValueU16;
        (0..RANDOM_CELL_ATTEMPTS).find_map(|_| {
            let (x, y) = (fastrand::u16(0..x_size), fastrand::u16(0..y_size));
            (cells[y as usize * x_size as usize + x as usize] & CellType::Walkable.as_flag() != 0).then_some((x, y))
        })
    }

    pub(super) fn execute_actor_skill(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        tick: u128,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(request.skill_id).ok_or("Unit skill has no pre-renewal definition")?;
        let level = request.level as u8;
        if matches!(metadata.name.as_str(), "HW_GANBANTEIN" | "MO_BODYRELOCATION") {
            let (x, y) = request.ground.unwrap_or((source.x, source.y));
            if metadata.name == "HW_GANBANTEIN" {
                if fastrand::u8(0..100) < GANBANTEIN_SUCCESS_PERCENT {
                    self.clear_ground_units_at(&source.map, source.instance, x, y);
                }
            } else if source.object_type == MapItemType::Mob {
                state
                    .get_map_instance(&source.map, source.instance)
                    .ok_or("Unit skill map is unavailable")?
                    .add_to_next_tick(MapEvent::MobWarpTo(crate::server::model::events::map_event::MobWarpTo { mob_id: source.id, x, y }));
            } else {
                return Err("Only monsters can use Body Relocation as actors".into());
            }
            self.notify_actor_support(source, request, true);
            return Ok(());
        }
        if request.ground.is_some() || GroundKind::from_name(&metadata.name).is_some() {
            let (x, y) = if let Some(ground) = request.ground {
                ground
            } else if metadata.target_type.as_deref() == Some("Self") {
                (source.x, source.y)
            } else {
                let (target, _) = self.actor_target_status(state, source, request.target_id)?;
                (target.x(), target.y())
            };
            if metadata.name == "BS_HAMMERFALL" {
                return self.cast_actor_area_status(server, state, source, metadata, level, x, y, tick);
            }
            if metadata.name == "RG_CLEANER" {
                self.erase_graffiti(
                    &source.map,
                    source.instance,
                    x,
                    y,
                    metadata.splash(level).unwrap_or(5).max(0) as u16,
                    tick,
                )?;
                self.notify_actor_support(source, request, true);
                return Ok(());
            }
            self.place_script_actor_ground(server, state, source, request, x, y, tick)?;
            self.notify_actor_support(source, request, true);
            return Ok(());
        }
        if matches!(
            metadata.name.as_str(),
            "BA_FROSTJOKER"
                | "DC_SCREAM"
                | "NPC_WIDEBLEEDING"
                | "NPC_WIDECONFUSE"
                | "NPC_WIDECURSE"
                | "NPC_WIDESILENCE"
                | "NPC_WIDESLEEP"
                | "NPC_WIDESTONE"
                | "NPC_WIDEFREEZE"
                | "NPC_WIDESTUN"
                | "NPC_WIDEHELLDIGNITY"
                | "NPC_DRAGONFEAR"
                | "AL_CRUCIS"
                | "NPC_WIDESOULDRAIN"
        ) {
            return self.cast_actor_area_status(server, state, source, metadata, level, source.x, source.y, tick);
        }
        if metadata.name == "NPC_SELFDESTRUCTION" {
            return self.execute_actor_self_destruct(server, state, source, request, metadata, tick);
        }
        let (target, status) = self.actor_target_status(state, source, request.target_id)?;
        if status.hp() == 0 && metadata.name != "ALL_RESURRECTION" {
            return Err("Unit skill target died".into());
        }
        let player = *target.map_item().object_type() == MapItemType::Character;
        let instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Unit skill map is unavailable")?;
        match metadata.name.as_str() {
            "AL_HEAL" => {
                let amount = Self::heal_amount(&source.status, source.status.base_level(), level);
                if Self::undead_target(&status) {
                    let context = crate::server::service::map_combat_service::MagicAttackContext::new(
                        (amount / 2).min(u32::from(u16::MAX)) as u16,
                        1.0,
                        Element::Holy,
                        1,
                        request.skill_id,
                    );
                    let mut damage = Self::actor_damage(
                        source,
                        request,
                        target.map_item().id(),
                        tick,
                        BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                        true,
                    );
                    damage.magic_context = Some(context);
                    damage.set_signed_damage(server.battle_service().magic_damage_from_context(&source.status, &status, context));
                    self.queue_actor_damage(server, state, source, damage)?;
                } else {
                    self.heal_actor_target(
                        server,
                        state,
                        source,
                        request.target_id,
                        Self::target_heal_amount(&status, amount),
                        0,
                        tick,
                    )?;
                }
            }
            "NV_FIRSTAID" => self.heal_actor_target(server, state, source, request.target_id, 5, 0, tick)?,
            "ALL_RESURRECTION" => {
                if !player || status.hp() > 0 || status.has_status_change(StatusChangeKind::HellPower) {
                    return Err("Resurrection requires a dead player without Hell Power".into());
                }
                let character = state
                    .characters_mut()
                    .get_mut(&request.target_id)
                    .ok_or("Resurrection target disconnected")?;
                let (hp, sp) = Self::resurrection_resources(&status, level)?;
                server.character_service().update_hp_sp(character, hp, sp);
                character.action = crate::server::state::character::CharacterAction::Idle;
                let mut packet = 0x0148_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes());
                packet.extend_from_slice(&0_u16.to_le_bytes());
                actor::notify_actor(&self.client_notification_sender, source, packet);
            }
            "TF_DETOXIFY" | "AL_CURE" | "PR_STRECOVERY" => {
                let kinds: &[StatusChangeKind] = match metadata.name.as_str() {
                    "TF_DETOXIFY" => &[StatusChangeKind::Poison, StatusChangeKind::DeadlyPoison],
                    "AL_CURE" => &[StatusChangeKind::Silence, StatusChangeKind::Blind, StatusChangeKind::Confusion],
                    _ => &[
                        StatusChangeKind::Stone,
                        StatusChangeKind::StoneWait,
                        StatusChangeKind::Freeze,
                        StatusChangeKind::Stun,
                        StatusChangeKind::Sleep,
                        StatusChangeKind::NoRecovery,
                    ],
                };
                for kind in kinds {
                    self.end_actor_target_status(server, state, source, request.target_id, *kind);
                }
                if metadata.name == "PR_STRECOVERY" && !player {
                    if Self::undead_target(&status) {
                        self.start_actor_target_status(
                            server,
                            state,
                            source,
                            request.target_id,
                            StatusChangeRequest::guaranteed(
                                StatusChangeKind::Blind,
                                metadata.duration(level, true).unwrap_or(0),
                                i32::from(level),
                            ),
                            1000,
                            tick,
                        )?;
                    } else {
                        instance.add_to_next_tick(MapEvent::MobLoseTarget(MobLoseTarget { mob_id: request.target_id }));
                    }
                }
            }
            "SA_DISPELL" => {
                if fastrand::u8(0..100) < 50 + 10 * level {
                    if let Some(character) = state.characters_mut().get_mut(&request.target_id) {
                        if character.status.job != models::enums::class::JobName::SoulLinker.value() as u32 {
                            for kind in StatusEffectService::dispel_statuses(&mut character.status, false) {
                                StatusEffectService::send_icon(character, kind, false, tick, &self.client_notification_sender);
                            }
                            server.character_service().reload_client_side_status(character);
                            StatusEffectService::send_visual_status(character, &self.client_notification_sender);
                        }
                    } else {
                        instance.add_to_next_tick(MapEvent::MobDispel(MobDispel { mob_id: request.target_id }));
                    }
                }
            }
            "PR_LEXDIVINA" => {
                if status.has_status_change(StatusChangeKind::Silence) {
                    self.end_actor_target_status(server, state, source, request.target_id, StatusChangeKind::Silence);
                } else {
                    self.start_actor_target_status(
                        server,
                        state,
                        source,
                        request.target_id,
                        StatusChangeRequest::guaranteed(
                            StatusChangeKind::Silence,
                            metadata.duration(level, true).unwrap_or(0),
                            i32::from(level),
                        ),
                        1000,
                        tick,
                    )?;
                }
            }
            "MG_STONECURSE" => self.start_actor_target_status(
                server,
                state,
                source,
                request.target_id,
                Self::stone_curse_request(level, source.id),
                0,
                tick,
            )?,
            "RG_STRIPWEAPON" | "RG_STRIPSHIELD" | "RG_STRIPARMOR" | "RG_STRIPHELM" | "ST_FULLSTRIP" => {
                for effect in Self::strip_requests(
                    &metadata.name,
                    request.skill_id,
                    level,
                    source.status.dex(),
                    status.dex(),
                    player,
                    fastrand::u16(0..1000),
                ) {
                    self.start_actor_target_status(server, state, source, request.target_id, effect, 0, tick)?;
                }
            }
            "AL_TELEPORT" => {
                if state
                    .map_flags_for(&source.map, source.instance)
                    .enabled(crate::server::model::map_flags::MapFlag::NoTeleport)
                {
                    return Err("Teleport is disabled on this map".into());
                }
                if source.object_type == MapItemType::Mob {
                    instance.add_to_next_tick(MapEvent::MobRandomWarp(MobRandomWarp { mob_id: source.id }));
                } else {
                    instance.add_to_next_tick(MapEvent::NpcEffect(crate::server::service::map_npc_effect::MapNpcEffect {
                        actor_id: source.id,
                        effect: crate::server::service::map_npc_effect::NpcEffect::RandomWarp,
                    }));
                }
            }
            "TF_BACKSLIDING" => {
                if source.object_type != MapItemType::Mob {
                    return Err("Only monsters can back slide as actors".into());
                }
                let (dx, dy) = Self::facing_vector(source.dir);
                let cells = metadata.knockback.as_ref().and_then(|value| value.value(level, "Amount")).unwrap_or(5).clamp(0, i32::from(u16::MAX)) as u16;
                instance.add_to_next_tick(MapEvent::MobSlide(MobSlide {
                    mob_id: source.id,
                    source_x: (i32::from(source.x) + dx).clamp(0, i32::from(u16::MAX)) as u16,
                    source_y: (i32::from(source.y) + dy).clamp(0, i32::from(u16::MAX)) as u16,
                    cells,
                }));
            }
            "NPC_EXPULSION" => {
                if state
                    .map_flags_for(&source.map, source.instance)
                    .enabled(crate::server::model::map_flags::MapFlag::NoTeleport)
                {
                    self.notify_actor_support(source, request, false);
                    return Ok(());
                }
                if player {
                    server.server_service.schedule_warp_to_walkable_cell_by_character(
                        &source.map,
                        crate::server::model::map::RANDOM_CELL.0,
                        crate::server::model::map::RANDOM_CELL.1,
                        request.target_id,
                    );
                } else {
                    instance.add_to_next_tick(MapEvent::MobRandomWarp(MobRandomWarp { mob_id: request.target_id }));
                }
            }
            name if Self::status_for_skill(name).is_some()
                || metadata.damage_flags.get("NoDamage").copied().unwrap_or(false) && metadata.status.is_some() =>
            {
                let kind = Self::status_for_skill(name)
                    .or_else(|| metadata.status.as_deref().and_then(StatusChangeKind::from_name))
                    .ok_or("Unit skill status is unavailable")?;
                if kind == StatusChangeKind::Provoke && Self::undead_target(&status) {
                    self.notify_actor_support(source, request, false);
                    return Ok(());
                }
                let skill = self.configuration.find_skill_config(&Value::Number(request.skill_id as i32));
                let mut effect = if let Some(skill) = skill {
                    Self::support_status_request(
                        skill,
                        kind,
                        level,
                        source.status.base_level(),
                        source.status.int(),
                        status.base_level(),
                    )
                } else {
                    StatusChangeRequest::guaranteed(kind, metadata.duration(level, false).unwrap_or(0), i32::from(level))
                };
                if kind == StatusChangeKind::ElementalChange {
                    effect.values[0] = if level <= 1 { fastrand::i32(1..=4) } else { i32::from(level).min(4) };
                    effect.values[1] = Self::npc_element_change(name);
                }
                if kind == StatusChangeKind::Provoke && level == 10 {
                    effect.values[1] = 0;
                    effect.values[2] = 100;
                }
                if metadata.flags.get("Toggleable").copied().unwrap_or(false) && status.has_status_change(kind) {
                    self.end_actor_target_status(server, state, source, request.target_id, kind);
                } else if kind == StatusChangeKind::Provoke && !player {
                    instance.add_to_next_tick(MapEvent::MobProvoke(MobProvoke {
                        mob_id: request.target_id,
                        source_id: source.id,
                        request: effect,
                        coma: ComaBonuses::from_bonuses(source.status.bonuses()),
                    }));
                } else {
                    self.start_actor_target_status(server, state, source, request.target_id, effect, 0, tick)?;
                }
            }
            _ => {
                self.execute_actor_damage(server, state, source, request, &status, player, tick)?;
                return Ok(());
            }
        }
        self.notify_actor_support(source, request, true);
        Ok(())
    }

    fn npc_element_change(skill_name: &str) -> i32 {
        use models::enums::element::Element;
        use models::enums::EnumWithNumberValue;
        let element = match skill_name {
            "NPC_CHANGEWATER" => Element::Water,
            "NPC_CHANGEGROUND" => Element::Earth,
            "NPC_CHANGEFIRE" => Element::Fire,
            "NPC_CHANGEWIND" => Element::Wind,
            "NPC_CHANGEPOISON" => Element::Poison,
            "NPC_CHANGEHOLY" => Element::Holy,
            "NPC_CHANGEDARKNESS" => Element::Dark,
            "NPC_CHANGETELEKINESIS" => Element::Ghost,
            _ => return fastrand::i32(0..10),
        };
        element.value() as i32
    }

    pub(super) fn start_actor_target_status(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        target_id: u32,
        mut request: StatusChangeRequest,
        delay: u128,
        tick: u128,
    ) -> Result<(), String> {
        let (_, target) = self.actor_target_status(state, source, target_id)?;
        let player = state.get_character(target_id).is_some();
        request = StatusEffectService::normalize_request_for_target(request, &target, player);
        if delay == 0 {
            if let Some(mut character) = state.characters_mut().remove(&target_id) {
                let kind = request.kind;
                let result = StatusEffectService::start(server, &mut character, request, tick, &self.client_notification_sender);
                if kind == StatusChangeKind::Provoke && result.as_ref().is_ok_and(|started| *started) {
                    self.cancel_provoked_cast(state, &mut character, tick);
                }
                state.insert_character(character);
                result?;
                return Ok(());
            }
        }
        if player || state.companion_owner(target_id, &source.map, source.instance).is_some() {
            server.add_to_delayed_tick(
                GameEvent::CharacterStatusChange(CharacterStatusChange {
                    char_id: target_id,
                    request,
                }),
                delay,
            );
        } else if let Some(instance) = state.get_map_instance(&source.map, source.instance) {
            instance.add_to_delayed_tick(
                MapEvent::MobStatusChange(MobStatusChange {
                    mob_id: target_id,
                    request,
                }),
                delay,
            );
        }
        Ok(())
    }

    fn end_actor_target_status(
        &self,
        server: &Server,
        state: &ServerState,
        source: &ScriptSkillActor,
        target_id: u32,
        kind: StatusChangeKind,
    ) {
        if state.get_character(target_id).is_some() || state.companion_owner(target_id, &source.map, source.instance).is_some() {
            server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus {
                char_id: target_id,
                kind: Some(kind),
            }));
        } else if let Some(instance) = state.get_map_instance(&source.map, source.instance) {
            instance.add_to_next_tick(MapEvent::MobEndStatus(MobEndStatus {
                mob_id: target_id,
                kind: Some(kind),
            }));
        }
    }

    fn heal_actor_target(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        target_id: u32,
        hp: u32,
        sp: u32,
        tick: u128,
    ) -> Result<(), String> {
        if let Some(character) = state.characters_mut().get_mut(&target_id) {
            if character.status.hp > 0
                && !character.status.has_status_change(StatusChangeKind::NoRecovery)
                && !character.status.has_status_change(StatusChangeKind::Berserk)
            {
                let status = StatusService::instance().to_snapshot(&character.status);
                server.character_service().update_hp_sp(
                    character,
                    character.status.hp.saturating_add(hp).min(status.max_hp()),
                    character.status.sp.saturating_add(sp).min(status.max_sp()),
                );
            }
        } else if let Some(owner_id) = state
            .companion_owner(target_id, &source.map, source.instance)
            .map(|owner| owner.char_id)
        {
            let character = state.characters_mut().get_mut(&owner_id).ok_or("Companion owner disconnected")?;
            server
                .script_world_service()
                .heal_companion(server, character, target_id, hp, sp, tick as u64)?;
        } else if let Some(instance) = state.get_map_instance(&source.map, source.instance) {
            instance.add_to_next_tick(MapEvent::MobHeal(MobHeal { mob_id: target_id, hp, sp }));
        }
        Ok(())
    }

    fn actor_damage(source: &ScriptSkillActor, request: &ScriptSkillCast, target_id: u32, tick: u128, flags: u32, landed: bool) -> Damage {
        Damage {
            notification: None,
            source_kind: *source.status.combat_actor_kind(),
            skill_damage_adjusted: false,
            target_id,
            attacker_id: source.id,
            credit_id: source.credit_id,
            damage: 0,
            healing: 0,
            right_hand_damage: None,
            attacked_at: tick + u128::from(source.attack_motion),
            damage_motion: 0,
            battle_flags: flags,
            skill_id: request.skill_id,
            skill_level: request.level as u8,
            proc_depth: 0,
            defenses_applied: true,
            magic_context: None,
            landed,
        }
    }

    fn execute_actor_damage(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        target: &StatusSnapshot,
        player: bool,
        tick: u128,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(request.skill_id).ok_or("Unit skill metadata is unavailable")?;
        if metadata.name == "NPC_DARKBREATH" {
            let level = u32::from(request.level).max(1);
            let percent = if level <= 5 { 100 / (2 * (6 - level)) } else { 50 };
            let raw = (u64::from(target.hp()) * u64::from(percent) / 100).min(u64::from(u32::MAX)) as u32;
            let flags = metadata.battle_flags(true);
            let amount = server
                .battle_service()
                .actor_misc_skill_damage_signed(raw, &source.status, target, &Element::Dark, flags, request.skill_id);
            let mut damage = Self::actor_damage(source, request, request.target_id, tick, flags, true);
            damage.set_signed_damage(amount);
            return self.queue_actor_damage(server, state, source, damage);
        }
        if Self::uses_metadata_magic(&metadata.name) || Self::actor_npc_magic(metadata) {
            let level = request.level as u8;
            if metadata.name == "SL_SMA" {
                self.end_actor_target_status(server, state, source, source.id, StatusChangeKind::Sma);
            }
            let (amount, context) = server
                .battle_service()
                .metadata_magic_damage(&source.status, target, metadata, level)?;
            let mut damage = Self::actor_damage(source, request, request.target_id, tick, metadata.battle_flags(true), true);
            damage.magic_context = Some(context);
            damage.set_signed_damage(amount);
            self.queue_actor_damage(server, state, source, damage)?;
            if metadata.name == "NPC_MAGICALATTACK" {
                self.start_actor_target_status(
                    server,
                    state,
                    source,
                    source.id,
                    StatusChangeRequest::guaranteed(
                        StatusChangeKind::MagicalAttack,
                        metadata.duration(level, false).unwrap_or(0),
                        i32::from(level),
                    ),
                    0,
                    tick,
                )?;
            } else if matches!(metadata.name.as_str(), "SL_STIN" | "SL_STUN")
                && level >= 7
                && !source
                    .status
                    .status_change(StatusChangeKind::Sma)
                    .is_some_and(|ready| !ready.expired(tick))
            {
                let duration = SkillMetadata::find(SkillEnum::SlSma.id())
                    .and_then(|metadata| metadata.duration(level, false))
                    .unwrap_or(3000);
                self.start_actor_target_status(
                    server,
                    state,
                    source,
                    source.id,
                    StatusChangeRequest::guaranteed(StatusChangeKind::Sma, duration, i32::from(level)),
                    0,
                    tick,
                )?;
            }
            return Ok(());
        }
        if let Some(radius) = Self::actor_splash_radius(metadata, request.level as u8) {
            return self.execute_actor_splash(server, state, source, request, metadata, radius, tick);
        }
        let mut damage = self.build_actor_offensive_damage(server, source, request, request.target_id, target, player, tick, None)?;
        if request.skill_id == SkillEnum::ChPalmstrike.id() {
            damage.attacked_at += PALM_STRIKE_DELAY;
            return self.queue_actor_damage_after(server, state, source, damage, PALM_STRIKE_DELAY);
        }
        let landed = damage.landed;
        self.queue_actor_damage(server, state, source, damage)?;
        let slot = match metadata.name.as_str() {
            "NPC_ARMORBRAKE" => Some(BreakSlot::Armor),
            "NPC_HELMBRAKE" => Some(BreakSlot::Helm),
            "NPC_SHIELDBRAKE" => Some(BreakSlot::Shield),
            _ => None,
        };
        if let Some(slot) = slot.filter(|_| landed && player && fastrand::i32(0..10_000) < 150 * i32::from(request.level)) {
            break_equipment(server, state, request.target_id, slot)?;
        }
        Ok(())
    }

    pub(super) fn actor_npc_weapon(metadata: &SkillMetadata) -> bool {
        metadata.name.starts_with("NPC_")
            && metadata.damage_type.as_deref() == Some("Weapon")
            && matches!(metadata.target_type.as_deref(), Some("Attack" | "Self"))
    }

    fn execute_actor_self_destruct(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        metadata: &SkillMetadata,
        tick: u128,
    ) -> Result<(), String> {
        let radius = metadata.splash(request.level as u8).unwrap_or(5).clamp(0, i32::from(u16::MAX)) as u16;
        let hp = source.status.hp().min(i32::MAX as u32) as i32;
        let flags = BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        let knockback = metadata
            .knockback
            .as_ref()
            .and_then(|value| value.value(request.level as u8, "Amount"))
            .map_or(0, |cells| cells.clamp(0, i32::from(u16::MAX)) as u16);
        for (id, status, player) in self.actor_area_targets(server, state, source, request.skill_id, source.x, source.y, radius) {
            let mut damage = Self::actor_damage(source, request, id, tick, flags, true);
            damage.set_signed_damage((hp as f32 * BattleService::element_modifier(&Element::Fire, &status)).floor() as i32);
            self.queue_actor_damage(server, state, source, damage)?;
            if knockback > 0 {
                self.knock_back_actor_target(server, state, source, id, player, knockback);
            }
        }
        let mut suicide = Self::actor_damage(source, request, source.id, tick, flags, true);
        suicide.set_signed_damage(hp);
        self.queue_actor_damage(server, state, source, suicide)
    }

    pub(super) fn actor_metadata_status(metadata: &SkillMetadata) -> bool {
        metadata.damage_flags.get("NoDamage").copied().unwrap_or(false)
            && (metadata.name.starts_with("NPC_") && !metadata.name.starts_with("NPC_WIDE") || metadata.name == "SA_REVERSEORCISH")
            && metadata.status.as_deref().and_then(StatusChangeKind::from_name).is_some()
    }

    pub(super) fn actor_npc_magic(metadata: &SkillMetadata) -> bool {
        metadata.name.starts_with("NPC_")
            && metadata.damage_type.as_deref() == Some("Magic")
            && metadata.target_type.as_deref() == Some("Attack")
            && !matches!(metadata.name.as_str(), "NPC_DARKBREATH" | "NPC_GRANDDARKNESS" | "NPC_EARTHQUAKE")
    }

    fn actor_splash_radius(metadata: &SkillMetadata, level: u8) -> Option<u16> {
        if !matches!(metadata.name.as_str(), "MG_FIREBALL" | "WZ_FROSTNOVA" | "SM_MAGNUM") && !Self::actor_npc_weapon(metadata) {
            return None;
        }
        metadata
            .splash(level)
            .filter(|radius| *radius > 0)
            .map(|radius| radius.min(i32::from(u16::MAX)) as u16)
    }

    fn execute_actor_splash(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        metadata: &SkillMetadata,
        radius: u16,
        tick: u128,
    ) -> Result<(), String> {
        let self_centered = metadata.target_type.as_deref() == Some("Self");
        let (x, y) = if self_centered {
            (source.x, source.y)
        } else {
            let (target, _) = self.actor_target_status(state, source, request.target_id)?;
            (target.x(), target.y())
        };
        let level = request.level as u8;
        if metadata.name == "SM_MAGNUM" {
            self.start_actor_target_status(
                server,
                state,
                source,
                source.id,
                StatusChangeRequest::guaranteed(
                    StatusChangeKind::WeaponAttackElement,
                    metadata.duration(level, true).unwrap_or(10000),
                    Element::Fire.value() as i32,
                ),
                0,
                tick,
            )?;
        }
        let mut recipients = self.actor_area_targets(server, state, source, request.skill_id, x, y, radius);
        if !self_centered && !recipients.iter().any(|(id, ..)| *id == request.target_id) {
            let (_, status) = self.actor_target_status(state, source, request.target_id)?;
            let player = state.characters().contains_key(&request.target_id);
            recipients.push((request.target_id, status, player));
        }
        for (id, status, player) in recipients {
            let position = state.characters().get(&id).map(|character| (character.x, character.y)).or_else(|| {
                state
                    .get_map_instance(&source.map, source.instance)?
                    .state()
                    .get_mob(id)
                    .map(|mob| (mob.x, mob.y))
            });
            let distance = position.map_or(0, |(tx, ty)| tx.abs_diff(x).max(ty.abs_diff(y)));
            let damage = self.build_actor_offensive_damage(server, source, request, id, &status, player, tick, Some(distance))?;
            let landed = damage.landed;
            let dealt = damage.damage;
            self.queue_actor_damage(server, state, source, damage)?;
            if metadata.name == "NPC_VAMPIRE_GIFT" && landed && dealt > 0 {
                self.heal_actor_target(server, state, source, source.id, dealt as u32, 0, tick)?;
            }
            if metadata.name == "SM_MAGNUM" && landed {
                self.knock_back_actor_target(server, state, source, id, player, 2);
            }
        }
        self.notify_actor_support(source, request, true);
        Ok(())
    }

    fn knock_back_actor_target(&self, server: &Server, state: &ServerState, source: &ScriptSkillActor, target_id: u32, player: bool, cells: u16) {
        if !player {
            if let Some(instance) = state.get_map_instance(&source.map, source.instance) {
                instance.add_to_next_tick(MapEvent::MobKnockback(MobKnockback {
                    mob_id: target_id,
                    source_x: source.x,
                    source_y: source.y,
                    cells,
                }));
            }
            return;
        }
        server.add_to_next_tick(GameEvent::GroundTrapEffect(super::trap::GroundTrapEffect {
            map: crate::server::model::map_instance::MapInstanceKey::new(source.map.clone(), source.instance),
            target_id,
            kind: super::trap::GroundTrapEffectKind::Knockback {
                source_x: source.x,
                source_y: source.y,
                cells,
            },
        }));
    }

    fn build_actor_offensive_damage(
        &self,
        server: &Server,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        target_id: u32,
        target: &StatusSnapshot,
        player: bool,
        tick: u128,
        distance: Option<u16>,
    ) -> Result<Damage, String> {
        if let Some(metadata) = SkillMetadata::find(request.skill_id).filter(|metadata| Self::actor_npc_weapon(metadata)) {
            return Ok(self.build_actor_npc_weapon_damage(server, source, request, metadata, target_id, target, player, tick));
        }
        let skill_enum = SkillEnum::try_from_value(request.skill_id).map_err(|_| "Unknown classic offensive skill")?;
        let object = skills::skill_enums::to_object(skill_enum, request.level as u8)
            .ok_or("Unit skill has no executable offensive implementation")?;
        let offensive = object
            .as_offensive_skill()
            .ok_or("Unit skill requires an additional actor-specific handler")?;
        let weapon = BattleService::is_weapon_skill(offensive);
        let flags = (if weapon {
            BattleFlag::Weapon
        } else if offensive.is_magic() {
            BattleFlag::Magic
        } else {
            BattleFlag::Misc
        })
        .as_flag()
            | if request.skill_id == SkillEnum::TfThrowstone.id() {
                BattleFlag::Weapon.as_flag()
            } else {
                0
            }
            | (if offensive.is_magic() || offensive.is_ranged() {
                BattleFlag::Long
            } else {
                BattleFlag::Short
            })
            .as_flag()
            | BattleFlag::Skill.as_flag();
        let landed = !weapon
            || request.skill_id == SkillEnum::ChPalmstrike.id()
            || server
                .battle_service()
                .skill_hits(&source.status, target, request.skill_id, request.level as u8);
        let mut damage = Self::actor_damage(source, request, target_id, tick, flags, landed);
        let amount = if !landed {
            0
        } else if request.skill_id == SkillEnum::TfThrowstone.id() {
            server
                .battle_service()
                .actor_misc_skill_damage(30, &source.status, target, &Element::Neutral, flags, request.skill_id)
                .min(i32::MAX as u32) as i32
        } else if weapon {
            let magnum = request.skill_id == SkillEnum::SmMagnum.id();
            let ratio = match (magnum, distance) {
                (true, Some(distance)) => 1.0 + f32::from(request.level) * if distance <= 1 { 0.2 } else { 0.1 },
                _ => offensive.dmg_atk().unwrap_or(1.0),
            };
            server.battle_service().actor_physical_skill_damage_signed(
                source.raw_attack,
                &source.status,
                target,
                player,
                ratio,
                if magnum { 1 } else { offensive.hit_count() as i16 },
                &if magnum {
                    Element::Fire
                } else {
                    server.battle_service().attack_element(&source.status, Some(offensive))
                },
                flags,
                request.skill_id,
            )
        } else {
            let (amount, context) = server
                .battle_service()
                .calculate_damage_with_context(&source.status, target, Some(offensive));
            damage.magic_context = context;
            match damage.magic_context {
                Some(mut context) if request.skill_id == SkillEnum::MgFireball.id() && distance == Some(2) => {
                    context.modifier *= 0.75;
                    damage.magic_context = Some(context);
                    server.battle_service().magic_damage_from_context(&source.status, target, context)
                }
                _ => amount,
            }
        };
        damage.set_signed_damage(amount);
        Ok(damage)
    }

    fn npc_weapon_skill_ratio(name: &str, level: u8) -> f32 {
        match name {
            "NPC_VAMPIRE_GIFT" => 1.0 + ((level.max(1) - 1) % 5 + 1) as f32,
            _ => 1.0,
        }
    }

    fn build_actor_npc_weapon_damage(
        &self,
        server: &Server,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        metadata: &SkillMetadata,
        target_id: u32,
        target: &StatusSnapshot,
        player: bool,
        tick: u128,
    ) -> Damage {
        let level = request.level as u8;
        let long_range = metadata.range(level).is_some_and(|range| range > 3);
        let flags = BattleFlag::Weapon.as_flag()
            | (if long_range { BattleFlag::Long } else { BattleFlag::Short }).as_flag()
            | BattleFlag::Skill.as_flag();
        let landed = metadata.name == "NPC_CRITICALSLASH"
            || server.battle_service().skill_hits(&source.status, target, request.skill_id, level);
        let mut damage = Self::actor_damage(source, request, target_id, tick, flags, landed);
        let hits = metadata
            .hit_count
            .as_ref()
            .and_then(|count| count.value(level, "Count"))
            .unwrap_or(1)
            .unsigned_abs()
            .clamp(1, i16::MAX as u32) as i16;
        let element = metadata.element(level).and_then(|name| <Element as models::enums::EnumWithStringValue>::try_from_string(name).ok());
        let amount = if landed {
            server.battle_service().actor_physical_skill_damage_signed(
                source.raw_attack,
                &source.status,
                target,
                player,
                Self::npc_weapon_skill_ratio(&metadata.name, level),
                hits,
                &server.battle_service().metadata_weapon_element(&source.status, element, request.skill_id),
                flags,
                request.skill_id,
            )
        } else {
            0
        };
        damage.set_signed_damage(amount);
        damage
    }

    fn queue_actor_damage(&self, server: &Server, state: &ServerState, source: &ScriptSkillActor, damage: Damage) -> Result<(), String> {
        self.queue_actor_damage_after(server, state, source, damage, 0)
    }

    fn queue_actor_damage_after(
        &self,
        server: &Server,
        state: &ServerState,
        source: &ScriptSkillActor,
        damage: Damage,
        extra_delay: u128,
    ) -> Result<(), String> {
        let count = SkillMetadata::find(damage.skill_id)
            .and_then(|metadata| metadata.hit_count.as_ref()?.value(damage.skill_level, "Count"))
            .unwrap_or(1)
            .unsigned_abs()
            .clamp(1, i16::MAX as u32) as i16;
        let damage = damage.with_skill_notification(
            &source.map,
            source.instance,
            source.x,
            source.y,
            damage.attacked_at,
            count,
            source.attack_motion,
        );
        let delay = u128::from(source.attack_motion) + extra_delay;
        if state.get_character(damage.target_id).is_some()
            || state.ground_unit(damage.target_id, &source.map, source.instance).is_some()
            || state.companion_owner(damage.target_id, &source.map, source.instance).is_some()
        {
            server.add_to_delayed_tick(
                GameEvent::ScriptMapDamage(crate::server::model::events::game_event::ScriptMapDamage {
                    map: crate::server::model::map_instance::MapInstanceKey::new(source.map.clone(), source.instance),
                    damage,
                }),
                delay,
            );
        } else {
            state
                .get_map_instance(&source.map, source.instance)
                .ok_or("Unit skill target map is unavailable")?
                .add_to_delayed_tick(MapEvent::MobDamage(MobDamage { damage }), delay);
        }
        Ok(())
    }

    pub(super) fn notify_actor_support(&self, source: &ScriptSkillActor, request: &ScriptSkillCast, succeeded: bool) {
        let mut packet = PacketZcUseSkill::new(self.configuration.packetver());
        packet.set_src_aid(source.id);
        packet.set_target_aid(request.target_id);
        packet.set_skid(request.skill_id as u16);
        packet.set_level(request.level as i16);
        packet.set_result(succeeded);
        packet.fill_raw();
        actor::notify_actor(&self.client_notification_sender, source, packet.raw);
    }
}
