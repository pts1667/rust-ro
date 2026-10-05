use models::enums::element::Element;
use models::enums::skill_enums::SkillEnum;
use models::enums::vanish::VanishType;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status_bonus::{BattleFlag, CombatTrigger};
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{Packet, PacketZcNotifyVanish, PacketZcUseSkill};

use super::MapInstanceService;
use super::unit_data::{walkable, warp_position};
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_item::MapItemType;
use crate::server::script::skill::actor::{self, NpcSkillState};
use crate::server::service::combat_trigger_service::{magic_reflection, physical_reflection};
use crate::server::service::map_combat_service::MagicReflectionRequest;
use crate::server::service::script_combat_service::{MobCombatEffect, ScriptCombatRequest};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::map_instance::MapInstanceState;

#[derive(Clone, Debug, PartialEq)]
pub struct MapNpcEffect {
    pub actor_id: u32,
    pub effect: NpcEffect,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NpcEffect {
    Status(StatusChangeRequest),
    StatusAlternatives(Vec<StatusChangeRequest>),
    EndStatus(Option<StatusChangeKind>),
    Dispel { clear_buffs: bool },
    Heal { hp: u32, sp: u32, raw: bool },
    Damage(Damage),
    Vanish { source_id: u32, hp: u32, sp: u32 },
    Direction(u16),
    Move { x: u16, y: u16 },
    RandomWarp,
    Knockback { source_x: u16, source_y: u16, cells: u16 },
}

impl MapInstanceService {
    pub fn handle_npc_map_event(&self, state: &mut MapInstanceState, event: &MapEvent, tick: u128) -> bool {
        let (actor_id, effect) = match event {
            MapEvent::MobDamage(damage) => (damage.target_id, NpcEffect::Damage(*damage)),
            MapEvent::MobStatusChange { mob_id, request } => (*mob_id, NpcEffect::Status(request.clone())),
            MapEvent::MobStatusAlternatives(request) => (request.mob_id, NpcEffect::StatusAlternatives(request.requests.clone())),
            MapEvent::MobEndStatus { mob_id, kind } => (*mob_id, NpcEffect::EndStatus(*kind)),
            MapEvent::MobDispel(request) => (request.mob_id, NpcEffect::Dispel { clear_buffs: false }),
            MapEvent::MobHeal { mob_id, hp, sp } => (*mob_id, NpcEffect::Heal {
                hp: *hp,
                sp: *sp,
                raw: false,
            }),
            MapEvent::MobProvoke(request) => {
                if !state.script_skill_state.npcs.contains_key(&request.mob_id) {
                    return false;
                }
                if state.get_map_item(request.source_id).is_none() {
                    return true;
                }
                (request.mob_id, NpcEffect::Status(request.request.clone()))
            }
            MapEvent::MobFace(request) => (request.mob_id, NpcEffect::Direction(request.dir)),
            MapEvent::MobWarpTo(request) => (request.mob_id, NpcEffect::Move {
                x: request.x,
                y: request.y,
            }),
            MapEvent::MobRandomWarp { mob_id } => (*mob_id, NpcEffect::RandomWarp),
            MapEvent::MobKnockback {
                mob_id,
                source_x,
                source_y,
                cells,
            } => (*mob_id, NpcEffect::Knockback {
                source_x: *source_x,
                source_y: *source_y,
                cells: *cells,
            }),
            MapEvent::ScriptMobCombat {
                source_id,
                target_id,
                effect: MobCombatEffect::Status(request),
            } => {
                if *source_id == 0 {
                    return false;
                }
                (*target_id, NpcEffect::Status(request.clone()))
            }
            MapEvent::ScriptMobCombat {
                source_id,
                target_id,
                effect: MobCombatEffect::Vanish { hp, sp },
            } => (*target_id, NpcEffect::Vanish {
                source_id: *source_id,
                hp: *hp,
                sp: *sp,
            }),
            MapEvent::MobLoseTarget { mob_id } if state.script_skill_state.npcs.contains_key(mob_id) => return true,
            _ => return false,
        };
        if !state.script_skill_state.npcs.contains_key(&actor_id) {
            return false;
        }
        self.apply_npc_effect(state, MapNpcEffect { actor_id, effect }, tick);
        true
    }

    pub fn apply_npc_effect(&self, state: &mut MapInstanceState, request: MapNpcEffect, tick: u128) {
        let Some(mut npc) = state.script_skill_state.npcs.get(&request.actor_id).cloned() else {
            return;
        };
        let old = npc.clone();
        let mut provoke = false;
        let result = match request.effect {
            NpcEffect::Status(request) => {
                let kind = request.kind;
                npc.start_status(request, tick, fastrand::u16(0..10000)).map(|outcome| {
                    provoke = outcome.started && kind == StatusChangeKind::Provoke;
                })
            }
            NpcEffect::StatusAlternatives(requests) => {
                let mut result = Ok(());
                for request in requests {
                    let kind = request.kind;
                    match npc.start_status(request, tick, fastrand::u16(0..10000)) {
                        Ok(outcome) if outcome.started => {
                            provoke = kind == StatusChangeKind::Provoke;
                            break;
                        }
                        Ok(_) => {}
                        Err(error) => {
                            result = Err(error);
                            break;
                        }
                    }
                }
                result
            }
            NpcEffect::EndStatus(kind) => {
                npc.end_status(kind, tick);
                Ok(())
            }
            NpcEffect::Dispel { clear_buffs } => {
                npc.dispel(clear_buffs);
                Ok(())
            }
            NpcEffect::Heal { hp, sp, raw } => {
                npc.heal(hp, sp, raw);
                Ok(())
            }
            NpcEffect::Damage(damage) => {
                self.damage_npc(state, &mut npc, damage, tick);
                Ok(())
            }
            NpcEffect::Vanish { source_id, hp, sp } => {
                npc.sp = npc.sp.saturating_sub(sp);
                if hp > 0 {
                    npc.damage(hp, 0, 0, state.flags.versus(false));
                }
                let _ = source_id;
                Ok(())
            }
            NpcEffect::Direction(direction) => {
                if direction < 8 {
                    npc.dir = direction;
                    Ok(())
                } else {
                    Err("Invalid NPC direction".into())
                }
            }
            NpcEffect::Move { x, y } => {
                if walkable(state, x, y) {
                    npc.x = x;
                    npc.y = y;
                    Ok(())
                } else {
                    Err("NPC movement destination is blocked".into())
                }
            }
            NpcEffect::RandomWarp => match warp_position(state) {
                Some((x, y)) => {
                    npc.x = x;
                    npc.y = y;
                    Ok(())
                }
                None => Err("NPC map has no walkable cell".into()),
            },
            NpcEffect::Knockback { source_x, source_y, cells } => {
                let (dx, dy) = (
                    ((i32::from(npc.x) - i32::from(source_x)).signum()),
                    ((i32::from(npc.y) - i32::from(source_y)).signum()),
                );
                for _ in 0..cells {
                    let (x, y) = (i32::from(npc.x) + dx, i32::from(npc.y) + dy);
                    if x < 0 || y < 0 || x > i32::from(u16::MAX) || y > i32::from(u16::MAX) || !walkable(state, x as u16, y as u16) {
                        break;
                    }
                    npc.x = x as u16;
                    npc.y = y as u16;
                }
                Ok(())
            }
        };
        if let Err(error) = result {
            debug!("NPC {} effect rejected: {}", npc.id, error);
        }
        let moved = (old.x, old.y) != (npc.x, npc.y);
        let dead = old.hp > 0 && npc.hp == 0;
        if dead {
            let mut status = npc.status();
            StatusEffectService::remove_on_death(&mut status);
            npc.active_statuses = status.active_statuses;
            npc.dead_sit = 1;
        }
        let blocked = npc.status().blocks_casting();
        state.script_skill_state.npcs.insert(npc.id, npc.clone());
        if moved || dead || blocked || provoke {
            self.cancel_npc_cast(state, npc.id);
        }
        if moved {
            self.vanish_npc(state, &old);
        }
        if old.active_statuses != npc.active_statuses {
            self.notify_area(
                state,
                npc.x,
                npc.y,
                StatusEffectService::visual_state_packet(npc.id, &npc.status()),
            );
        }
        if dead {
            self.notify_npc_death(state, &npc);
        } else if moved || old != npc {
            self.refresh_npc(state, &npc);
        }
    }

    fn damage_npc(&self, state: &mut MapInstanceState, npc: &mut NpcSkillState, mut damage: Damage, tick: u128) {
        if !damage.matches_notification_map(state.key()) {
            return;
        }
        if npc.hp == 0 || npc.damage_immune {
            damage.notify_admitted(&self.client_notification_sender, 0, self.configuration_service.packetver());
            return;
        }
        let target = npc.snapshot();
        if damage.healing > 0 {
            crate::server::service::map_flag_service::apply_map_skill_damage(&state.flags, &mut damage, &target);
        }
        if damage.healing > 0 {
            let before = npc.hp;
            npc.heal(damage.healing, 0, true);
            damage.notify_admitted(
                &self.client_notification_sender,
                -i64::from(npc.hp.saturating_sub(before)),
                self.configuration_service.packetver(),
            );
            return;
        }
        if damage.landed && damage.damage > 0 && damage.proc_depth < 8 {
            if let Some(kind) = magic_reflection(&target, damage.battle_flags, damage.skill_id, &mut fastrand::Rng::new()) {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ReflectMagic(MagicReflectionRequest {
                        damage,
                        reflector_id: npc.id,
                        reflector_credit_id: npc.id,
                        kind,
                        map_key: state.key().clone(),
                    }));
                damage.notify_admitted(&self.client_notification_sender, 0, self.configuration_service.packetver());
                return;
            }
        }
        if npc
            .absorb_magic_rod(damage.battle_flags, damage.skill_id, damage.skill_level)
            .is_some()
        {
            let mut packet = PacketZcUseSkill::new(self.configuration_service.packetver());
            damage.notify_admitted(&self.client_notification_sender, 0, self.configuration_service.packetver());
            packet.set_src_aid(npc.id);
            packet.set_target_aid(npc.id);
            packet.set_skid(SkillEnum::SaMagicrod.id() as u16);
            packet.set_level(i16::from(damage.skill_level));
            packet.set_result(true);
            packet.fill_raw();
            self.notify_area(state, npc.x, npc.y, packet.raw);
            return;
        }
        let amount = if damage.defenses_applied {
            damage.damage
        } else if let Some(source) = actor::map_actor(state, damage.attacker_id) {
            self.battle_service
                .apply_damage_reduction(
                    damage.damage as f32,
                    &source.status,
                    &target,
                    &Element::Neutral,
                    damage.battle_flags,
                )
                .max(0.0)
                .floor() as u32
        } else if damage.battle_flags & BattleFlag::Weapon.as_flag() != 0 {
            (damage.damage as f32 * (1.0 - f32::from(target.def().clamp(0, 100)) / 100.0) - f32::from(target.vit())).max(1.0) as u32
        } else if damage.battle_flags & BattleFlag::Magic.as_flag() != 0 {
            (damage.damage as f32 * (1.0 - f32::from(target.mdef().clamp(0, 100)) / 100.0) - f32::from(target.int())).max(1.0) as u32
        } else {
            damage.damage
        };
        let mut map_healing = 0;
        let applied = npc.admit_damage(
            amount,
            damage.battle_flags,
            damage.skill_id,
            state.flags.versus(false),
            |amount| {
                let amount = crate::server::service::map_flag_service::apply_map_combat_damage(
                    &state.flags,
                    &self.configuration_service.config().game,
                    amount,
                    damage.skill_id,
                    damage.battle_flags,
                );
                let mut adjusted = Damage { damage: amount, ..damage };
                crate::server::service::map_flag_service::apply_map_skill_damage(&state.flags, &mut adjusted, &target);
                map_healing = adjusted.healing;
                adjusted.damage
            },
        );
        let before_heal = npc.hp;
        if map_healing > 0 {
            npc.heal(map_healing, 0, true);
        }
        damage.notify_admitted(
            &self.client_notification_sender,
            if map_healing > 0 {
                -i64::from(npc.hp.saturating_sub(before_heal))
            } else {
                i64::from(applied)
            },
            self.configuration_service.packetver(),
        );
        if applied == 0 {
            return;
        }
        if damage.battle_flags != 0 {
            actor::interrupt_map_cast(state, npc.id, tick, &self.client_notification_sender);
        }
        let reflected = physical_reflection(&target, damage.battle_flags, damage.skill_id, applied);
        if reflected > 0 && damage.proc_depth < 8 {
            let reflected = Damage {
                notification: None,
                source_kind: models::enums::actor::CombatActorKind::Npc,
                skill_damage_adjusted: false,
                target_id: damage.attacker_id,
                attacker_id: npc.id,
                credit_id: npc.id,
                damage: reflected,
                healing: 0,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: 0,
                skill_id: 0,
                skill_level: 0,
                proc_depth: damage.proc_depth + 1,
                defenses_applied: true,
                landed: false,
                magic_context: None,
                right_hand_damage: None,
            }
            .with_action_notification(
                state.key().map_name(),
                state.key().map_instance(),
                npc.x,
                npc.y,
                tick,
                1,
                0,
                models::enums::action::ActionType::AttackNomotion,
                (reflected.min(i32::MAX as u32) as i32, 0),
            );
            let player = state.characters().iter().find(|item| item.map_item().id() == reflected.target_id);
            if player.is_some_and(|item| {
                matches!(
                    item.map_item().object_type(),
                    MapItemType::Character | MapItemType::Homunculus | MapItemType::Mercenary
                )
            }) {
                self.server_task_queue.add_to_first_index(GameEvent::CharacterDamage(reflected));
            } else {
                self.server_task_queue.add_to_first_index(GameEvent::ScriptMapDamage(
                    crate::server::model::events::game_event::ScriptMapDamage {
                        map: state.key().clone(),
                        damage: reflected,
                    },
                ));
            }
        }
        if damage.landed && damage.battle_flags != 0 && damage.proc_depth < 8 {
            let credited_id = if damage.credit_id == 0 {
                damage.attacker_id
            } else {
                damage.credit_id
            };
            if state
                .get_map_item(damage.attacker_id)
                .is_some_and(|item| *item.object_type() == MapItemType::Character)
            {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ScriptCombat(ScriptCombatRequest {
                        source_id: credited_id,
                        target_id: npc.id,
                        trigger: CombatTrigger::Attack,
                        battle_flags: damage.battle_flags,
                        skill_id: damage.skill_id,
                        damage: applied,
                        right_hand_damage: damage.admitted_right_hand_damage(applied),
                        other_mob_id: None,
                        depth: damage.proc_depth,
                        drop_position: None,
                        origin_map: Some(state.key().clone()),
                    }));
            }
            if damage.skill_id != 0 && damage.skill_level != 0 && damage.battle_flags & BattleFlag::Skill.as_flag() != 0 {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ScriptSkillHit(crate::server::script::skill::ScriptSkillHit {
                        source_map: Some(state.key().map_name().clone()),
                        source_instance: Some(state.key().map_instance()),
                        source_id: damage.attacker_id,
                        target_id: npc.id,
                        skill_id: damage.skill_id,
                        skill_level: damage.skill_level,
                        damage: applied,
                        depth: damage.proc_depth,
                    }));
            }
        }
    }

    fn cancel_npc_cast(&self, state: &mut MapInstanceState, id: u32) {
        if state.script_skill_state.casts.remove(&id).is_some() {
            let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
            packet.extend_from_slice(&id.to_le_bytes());
            if let Some(npc) = state.script_skill_state.npcs.get(&id) {
                self.notify_area(state, npc.x, npc.y, packet);
            }
        }
        state.script_skill_state.generations.remove(&id);
    }

    fn notify_npc_death(&self, state: &MapInstanceState, npc: &NpcSkillState) {
        let mut packet = PacketZcNotifyVanish::new(self.configuration_service.packetver());
        packet.set_gid(npc.id);
        packet.set_atype(VanishType::Die.value() as u8);
        packet.fill_raw();
        self.notify_area(state, npc.x, npc.y, packet.raw);
    }

    pub fn tick_npc_statuses(&self, state: &mut MapInstanceState, tick: u128) {
        let mut changed = Vec::new();
        for npc in state
            .script_skill_state
            .npcs
            .values_mut()
            .filter(|npc| npc.max_hp > 0 && npc.hp > 0)
        {
            let old_hp = npc.hp;
            let old_sp = npc.sp;
            let statuses = npc.active_statuses.clone();
            npc.tick(tick);
            if old_hp != npc.hp || old_sp != npc.sp || statuses != npc.active_statuses {
                changed.push((npc.clone(), old_hp > 0 && npc.hp == 0));
            }
        }
        for (mut npc, dead) in changed {
            if dead {
                let mut status = npc.status();
                StatusEffectService::remove_on_death(&mut status);
                npc.active_statuses = status.active_statuses;
                npc.dead_sit = 1;
                state.script_skill_state.npcs.insert(npc.id, npc.clone());
                self.cancel_npc_cast(state, npc.id);
                self.notify_npc_death(state, &npc);
            } else {
                self.refresh_npc(state, &npc);
            }
            self.notify_area(
                state,
                npc.x,
                npc.y,
                StatusEffectService::visual_state_packet(npc.id, &npc.status()),
            );
        }
    }
}
