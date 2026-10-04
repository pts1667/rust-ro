use models::enums::element::Element;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{Packet, PacketZcNotifySkill2, PacketZcUseSkill};
use script_sdk::Value;

use super::ScriptSkillService;
use super::actor::{self, ScriptSkillActor};
use super::ground::GroundKind;
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterEndStatus, CharacterStatusChange, GameEvent, ScriptSkillCast};
use crate::server::model::events::map_event::{MapEvent, MobDispel, MobProvoke};
use crate::server::model::map_item::MapItemType;
use crate::server::service::battle_service::BattleService;
use crate::server::service::combat_trigger_service::ComaBonuses;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub(super) fn after_actor_skill_damage(
        &self,
        server: &Server,
        state: &mut ServerState,
        hit: super::ScriptSkillHit,
        tick: u128,
    ) -> Result<(), String> {
        let Some(source) = self.find_script_skill_actor(state, hit.source_id) else {
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
                | "NPC_DRAGONFEAR"
                | "AL_CRUCIS"
                | "NPC_WIDESOULDRAIN"
        ) {
            return self.cast_actor_area_status(server, state, source, metadata, level, source.x, source.y, tick);
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
                        instance.add_to_next_tick(MapEvent::MobLoseTarget { mob_id: request.target_id });
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
                    instance.add_to_next_tick(MapEvent::MobRandomWarp { mob_id: source.id });
                } else {
                    return Err("NPC teleport movement has no configured save point".into());
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
                MapEvent::MobStatusChange {
                    mob_id: target_id,
                    request,
                },
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
            instance.add_to_next_tick(MapEvent::MobEndStatus {
                mob_id: target_id,
                kind: Some(kind),
            });
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
            instance.add_to_next_tick(MapEvent::MobHeal { mob_id: target_id, hp, sp });
        }
        Ok(())
    }

    fn actor_damage(source: &ScriptSkillActor, request: &ScriptSkillCast, target_id: u32, tick: u128, flags: u32, landed: bool) -> Damage {
        Damage {
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
        state: &ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        target: &StatusSnapshot,
        player: bool,
        tick: u128,
    ) -> Result<(), String> {
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
            || server
                .battle_service()
                .skill_hits(&source.status, target, request.skill_id, request.level as u8);
        let mut damage = Self::actor_damage(source, request, request.target_id, tick, flags, landed);
        let amount = if !landed {
            0
        } else if request.skill_id == SkillEnum::TfThrowstone.id() {
            server
                .battle_service()
                .actor_misc_skill_damage(30, &source.status, target, &Element::Neutral, flags, request.skill_id)
                .min(i32::MAX as u32) as i32
        } else if weapon {
            server.battle_service().actor_physical_skill_damage_signed(
                source.raw_attack,
                &source.status,
                target,
                player,
                offensive.dmg_atk().unwrap_or(1.0),
                offensive.hit_count() as i16,
                &server.battle_service().attack_element(&source.status, Some(offensive)),
                flags,
                request.skill_id,
            )
        } else {
            let (amount, context) = server
                .battle_service()
                .calculate_damage_with_context(&source.status, target, Some(offensive));
            damage.magic_context = context;
            amount
        };
        damage.set_signed_damage(amount);
        self.queue_actor_damage(server, state, source, damage)
    }

    fn queue_actor_damage(&self, server: &Server, state: &ServerState, source: &ScriptSkillActor, damage: Damage) -> Result<(), String> {
        let mut packet = PacketZcNotifySkill2::new(self.configuration.packetver());
        packet.set_aid(source.id);
        packet.set_target_id(damage.target_id);
        packet.set_skid(damage.skill_id as u16);
        packet.set_level(i16::from(damage.skill_level));
        packet.set_damage(if damage.healing > 0 {
            -(damage.healing.min(i32::MAX as u32) as i32)
        } else {
            damage.damage.min(i32::MAX as u32) as i32
        });
        packet.set_count(
            SkillMetadata::find(damage.skill_id)
                .and_then(|metadata| metadata.hit_count.as_ref()?.value(damage.skill_level, "Count"))
                .unwrap_or(1)
                .unsigned_abs()
                .clamp(1, i16::MAX as u32) as i16,
        );
        packet.set_action(6);
        packet.fill_raw();
        actor::notify_actor(&self.client_notification_sender, source, packet.raw);
        let delay = u128::from(source.attack_motion);
        if state.get_character(damage.target_id).is_some()
            || state.companion_owner(damage.target_id, &source.map, source.instance).is_some()
        {
            server.add_to_delayed_tick(GameEvent::CharacterDamage(damage), delay);
        } else {
            state
                .get_map_instance(&source.map, source.instance)
                .ok_or("Unit skill target map is unavailable")?
                .add_to_delayed_tick(MapEvent::MobDamage(damage), delay);
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
