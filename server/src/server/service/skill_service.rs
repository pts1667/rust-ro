use std::mem;
use std::sync::mpsc::SyncSender;

use models::enums::client_effect_icon::ClientEffectIcon;
use models::enums::skill::{SkillType, UseSkillFailure, UseSkillFailureClientSideType};
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::item::NormalInventoryItem;
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use packets::packets::{PacketZcAckTouseskill, PacketZcActionFailure, PacketZcMsgStateChange2, PacketZcUseSkill, PacketZcUseskillAck2};
use skills::Skill;

use crate::packets::packets::Packet;
use crate::server::model::action::{SkillCasted, SkillUsed};
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::map_item::{MapItemSnapshot, MapItemType};
use crate::server::service::battle_service::BattleService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;

#[allow(dead_code)]
pub struct SkillService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    configuration_service: &'static GlobalConfigService,
    battle_service: BattleService,
    status_service: &'static StatusService,
    force_no_delay: bool,
}

impl SkillService {
    pub fn new(
        client_notification_sender: SyncSender<Notification>,
        persistence_event_sender: SyncSender<PersistenceEvent>,
        battle_service: BattleService,
        status_service: &'static StatusService,
        configuration_service: &'static GlobalConfigService,
    ) -> SkillService {
        SkillService {
            client_notification_sender,
            persistence_event_sender,
            configuration_service,
            battle_service,
            status_service,
            force_no_delay: false,
        }
    }

    pub fn force_no_delay(mut self) -> Self {
        self.force_no_delay = true;
        self
    }

    pub fn start_use_skill(
        &self,
        character: &mut Character,
        target: Option<MapItemSnapshot>,
        source_status: &StatusSnapshot,
        target_status: Option<&StatusSnapshot>,
        skill_id: u32,
        skill_level: u8,
        tick: u128,
    ) -> SkillCasted {
        self.start_skill(
            character,
            target,
            source_status,
            target_status,
            skill_id,
            skill_level,
            tick,
            true,
        )
    }

    pub fn start_item_skill(
        &self,
        character: &mut Character,
        target: Option<MapItemSnapshot>,
        source_status: &StatusSnapshot,
        target_status: Option<&StatusSnapshot>,
        skill_id: u32,
        skill_level: u8,
        tick: u128,
        keep_requirements: bool,
    ) -> SkillCasted {
        self.start_skill(
            character,
            target,
            source_status,
            target_status,
            skill_id,
            skill_level,
            tick,
            keep_requirements,
        )
    }

    fn start_skill(
        &self,
        character: &mut Character,
        target: Option<MapItemSnapshot>,
        source_status: &StatusSnapshot,
        target_status: Option<&StatusSnapshot>,
        skill_id: u32,
        skill_level: u8,
        tick: u128,
        keep_requirements: bool,
    ) -> SkillCasted {
        if character.status.blocks_casting() {
            return SkillCasted::invalid();
        }
        if character.script_skill_state.skill_blocked_until.get(&skill_id).is_some_and(|until| *until > tick) {
            self.send_skill_fail_packet(character, UseSkillFailure::Skillinterval);
            return SkillCasted::invalid();
        }
        if target.is_none() || target_status.is_none() {
            return SkillCasted::invalid();
        }
        let target_snapshot = target.unwrap();
        let skill = SkillEnum::from_id(skill_id);
        let Some(mut skill) = skills::skill_enums::to_object(skill, skill_level) else {
            return SkillCasted::invalid();
        };

        if keep_requirements {
            let validate_sp = skill.validate_sp(source_status);
            if validate_sp.is_err() {
                self.send_skill_fail_packet(character, UseSkillFailure::SpInsufficient);
                return SkillCasted::invalid();
            }
            let validate_hp = skill.validate_hp(source_status);
            if validate_hp.is_err() {
                self.send_skill_fail_packet(character, UseSkillFailure::HpInsufficient);
                return SkillCasted::invalid();
            }
            let maybe_ammo = character.status.ammo.map(|ammo| {
                (
                    ammo.ammo_type,
                    character
                        .get_item_from_inventory(ammo.inventory_index)
                        .map(|ammo_in_inventory| ammo_in_inventory.amount as u32)
                        .unwrap_or(0),
                )
            });
            let validate_ammo = skill.validate_ammo(maybe_ammo);
            if validate_ammo.is_err() {
                let mut packet_zc_action_failure = PacketZcActionFailure::new(self.configuration_service.packetver());
                packet_zc_action_failure.fill_raw();
                self.client_notification_sender
                    .send(Notification::Char(CharNotification::new(
                        character.char_id,
                        mem::take(packet_zc_action_failure.raw_mut()),
                    )))
                    .unwrap_or_else(|_| error!("Failed to send notification packet_zc_action_failure to client"));
                return SkillCasted::invalid();
            }

            let validate_weapon = skill.validate_weapon(source_status);
            if validate_weapon.is_err() {
                self.send_skill_fail_packet(character, UseSkillFailure::ThisWeapon);
                return SkillCasted::invalid();
            }
            let validate_zeny = skill.validate_zeny(source_status);
            if validate_zeny.is_err() {
                self.send_skill_fail_packet(character, UseSkillFailure::Money);
                return SkillCasted::invalid();
            }

            let validate_items = skill.validate_item(
                &character
                    .inventory_normal()
                    .iter()
                    .map(|(_, i)| i.to_normal_item())
                    .collect::<Vec<NormalInventoryItem>>(),
            );
            if validate_items.is_err() {
                self.send_skill_fail_packet(character, validate_items.err().unwrap());
                return SkillCasted::invalid();
            }
        }

        // TODO use char stats
        skill.update_cast_time((skill.base_cast_time() as f32 * StatusService::skill_cast_modifier(source_status, skill_id)).ceil() as u32);
        skill.update_after_cast_act_delay(StatusService::skill_after_cast_delay(
            source_status,
            skill_id,
            skill.base_after_cast_act_delay(),
        ));
        skill.update_after_cast_walk_delay(skill.base_after_cast_walk_delay());
        crate::server::script::skill::ScriptSkillService::spend_cast_statuses(character, skill_id, tick, &self.client_notification_sender);
        let mut packet_zc_useskill_ack2 = PacketZcUseskillAck2::new(self.configuration_service.packetver());
        packet_zc_useskill_ack2.set_target_id(target_snapshot.map_item().id());
        packet_zc_useskill_ack2.set_skid(skill_id as u16);
        packet_zc_useskill_ack2.set_property(12); // element
        packet_zc_useskill_ack2.set_delay_time(skill.cast_time()); // cast time
        packet_zc_useskill_ack2.set_aid(character.char_id);
        packet_zc_useskill_ack2.fill_raw();
        self.client_notification_sender
            .send(Notification::Area(AreaNotification::new(
                character.current_map_name().clone(),
                character.current_map_instance(),
                AreaNotificationRangeType::Fov {
                    x: character.x,
                    y: character.y,
                    exclude_id: None,
                },
                mem::take(packet_zc_useskill_ack2.raw_mut()),
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_useskill_ack2 to client"));

        let no_delay = self.force_no_delay || skill.cast_time() == 0;
        character.set_skill_in_use(target.map(|target| target.map_item().id()), tick, skill);
        let mut skill_usage_response = SkillCasted::valid();
        if no_delay {
            skill_usage_response = SkillCasted::no_delay();
        }

        skill_usage_response
    }

    pub fn do_use_skill(
        &self,
        character: &mut Character,
        target: Option<MapItemSnapshot>,
        source_status: &StatusSnapshot,
        target_status: Option<&StatusSnapshot>,
        tick: u128,
    ) -> Option<SkillUsed> {
        if target.is_none() || target_status.is_none() {
            return None;
        }

        if !self.cast_is_ready(character, tick) {
            return None;
        }
        let skill = &character.skill_in_use().skill;
        let skill_type = skill.skill_type();
        let mut damage: i32 = 0;
        let mut magic_context = None;
        let mut landed = true;
        let mut packets: Vec<u8> = vec![];
        let mut attack_motion: i32 = 0;
        let mut target_id = character.char_id;
        let mut target_damage_motion: u32 = 0;
        let mut damage_notification = None;
        let mut bonuses = Default::default();
        let offensive = skill.as_offensive_skill();
        let battle_flags = (if offensive.is_some_and(BattleService::is_weapon_skill) {
            BattleFlag::Weapon
        } else if offensive.is_some_and(|skill| skill.is_magic()) {
            BattleFlag::Magic
        } else {
            BattleFlag::Misc
        })
        .as_flag()
            | (if offensive.is_some_and(|skill| skill.is_ranged() || skill.is_magic()) {
                BattleFlag::Long
            } else {
                BattleFlag::Short
            })
            .as_flag()
            | BattleFlag::Skill.as_flag()
            | if skill.id() == SkillEnum::TfThrowstone.id() {
                BattleFlag::Weapon.as_flag()
            } else {
                0
            };
        let skill_id = skill.id();
        let skill_level = skill.level();
        match skill.skill_type() {
            SkillType::Offensive => {
                let skill = skill.as_offensive_skill().unwrap();
                landed = skill.id() == SkillEnum::ChPalmstrike.id()
                    || !BattleService::is_weapon_skill(skill)
                    || self
                        .battle_service
                        .skill_hits(source_status, target_status.as_ref().unwrap(), skill.id(), skill.level());
                if landed && skill.id() != SkillEnum::WzWaterball.id() && skill.id() != SkillEnum::ChPalmstrike.id() {
                    (damage, magic_context) =
                        self.battle_service
                            .calculate_damage_with_context(source_status, target_status.as_ref().unwrap(), Some(skill));
                }
                let target = target.as_ref().unwrap();
                target_id = target.map_item().id();

                attack_motion = self.status_service.attack_motion(source_status) as i32;
                if matches!(target.map_item.object_type(), MapItemType::Mob) {
                    let mob = self.configuration_service.get_mob(target.map_item.client_item_class() as i32);
                    target_damage_motion = mob.damage_motion as u32;
                } else {
                    const PLAYER_DAMAGE_MOTION: u32 = 480;
                    target_damage_motion = PLAYER_DAMAGE_MOTION;
                }
                if skill.id() != SkillEnum::WzWaterball.id() && skill.id() != SkillEnum::ChPalmstrike.id() {
                    damage_notification = Some(crate::server::model::damage_notification::DamageNotification::new(
                        character.current_map_name(),
                        character.current_map_instance(),
                        character.x,
                        character.y,
                        tick,
                        attack_motion.max(0) as u32,
                        u32::from(skill.hit_count().unsigned_abs()).min(i16::MAX as u32) as i16,
                        crate::server::model::damage_notification::DamageVisual::Skill {
                            skill_id: skill.id() as u16,
                            level: skill.level(),
                        },
                    ));
                }
            }
            SkillType::Interactive => {}
            SkillType::Performance => {}
            SkillType::Support => {
                let skill = skill.as_supportive_skill().unwrap();
                bonuses = skill.bonuses_to_target(tick);
                let mut packet_zc_use_skill = PacketZcUseSkill::new(self.configuration_service.packetver());
                packet_zc_use_skill.set_src_aid(character.char_id);
                packet_zc_use_skill.set_target_aid(target_id);
                packet_zc_use_skill.set_skid(skill.id() as u16);
                packet_zc_use_skill.set_level(skill.level() as i16);
                packet_zc_use_skill.set_result(true);
                packet_zc_use_skill.fill_raw();
                packets = mem::take(packet_zc_use_skill.raw_mut());
            }
            SkillType::Passive => {}
        }
        character.update_skill_used_at_tick(tick);
        if !packets.is_empty() {
            self.client_notification_sender
                .send(Notification::Area(AreaNotification::new(
                    character.current_map_name().clone(),
                    character.current_map_instance(),
                    AreaNotificationRangeType::Fov {
                        x: character.x,
                        y: character.y,
                        exclude_id: None,
                    },
                    packets,
                )))
                .unwrap_or_else(|_| error!("Failed to send notification packet_zc_use_skill to client"));
        }

        let after_cast_delay = character.skill_in_use().skill.after_cast_act_delay();
        if after_cast_delay > 0 {
            let mut packet_zc_msg_state_change = PacketZcMsgStateChange2::new(self.configuration_service.packetver());
            packet_zc_msg_state_change.set_aid(character.char_id);
            packet_zc_msg_state_change.set_index(ClientEffectIcon::PostDelay as i16);
            packet_zc_msg_state_change.set_remain_ms(after_cast_delay);
            packet_zc_msg_state_change.set_state(true);
            packet_zc_msg_state_change.fill_raw();
            self.client_notification_sender
                .send(Notification::Char(CharNotification::new(
                    character.char_id,
                    mem::take(packet_zc_msg_state_change.raw_mut()),
                )))
                .unwrap_or_else(|_| error!("Failed to send EFST_POSTDELAY to client"));
        }

        Some(SkillUsed {
            notification: damage_notification,
            proc_depth: 0,
            credit_id: character.char_id,
            battle_flags,
            skill_id,
            skill_level,
            landed,
            skill_type,
            source_id: character.char_id,
            target_id,
            damage_to_target: damage,
            damage_to_self: 0,
            effects: vec![],
            bonuses,
            attacked_at: tick + attack_motion as u128,
            damage_motion: target_damage_motion,
            magic_context,
        })
    }

    pub fn after_skill_used(&self, character: &mut Character, tick: u128) {
        let used_at = character.skill_in_use().used_at_tick.unwrap();
        if tick < used_at + character.skill_in_use().skill.after_cast_act_delay() as u128 {
            return;
        }
        character.clear_skill_in_use();
    }

    pub(crate) fn send_skill_fail_packet(&self, character: &mut Character, cause: UseSkillFailure) {
        let mut packet_zc_ack_touseskill = PacketZcAckTouseskill::new(self.configuration_service.packetver());
        packet_zc_ack_touseskill.set_cause(cause.value() as u8);
        packet_zc_ack_touseskill.set_num(UseSkillFailureClientSideType::SkillFailed.value() as u32);
        packet_zc_ack_touseskill.set_result(false);
        packet_zc_ack_touseskill.fill_raw();
        self.client_notification_sender
            .send(Notification::Char(CharNotification::new(
                character.char_id,
                mem::take(packet_zc_ack_touseskill.raw_mut()),
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_ack_touseskill to client"));
    }

    pub fn cast_is_ready(&self, character: &Character, tick: u128) -> bool {
        character
            .skill_in_use
            .as_ref()
            .is_some_and(|cast| self.force_no_delay || tick >= cast.start_skill_tick + cast.skill.cast_time() as u128)
    }

    pub fn calculate_damage(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, skill: &dyn Skill) -> i32 {
        self.battle_service.calculate_damage(source_status, target_status, Some(skill))
    }
}
