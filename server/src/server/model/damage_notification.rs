use models::enums::EnumWithNumberValue;
use models::enums::action::ActionType;
use packets::packets::{Packet, PacketZcNotifySkill2};

use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::map_flag_service::normalize_map;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DamageVisual {
    Action { action: ActionType, hands: (i32, i32) },
    Skill { skill_id: u16, level: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageNotification {
    map: [char; 16],
    instance: u8,
    x: u16,
    y: u16,
    tick: u32,
    attack_motion: u32,
    count: i16,
    visual: DamageVisual,
    pub target_override: Option<u32>,
}

impl DamageNotification {
    pub fn new(map: &str, instance: u8, x: u16, y: u16, tick: u128, attack_motion: u32, count: i16, visual: DamageVisual) -> Self {
        Self {
            map: MapInstanceKey::new(map.to_string(), instance).map_name_char(),
            instance,
            x,
            y,
            tick: tick as u32,
            attack_motion,
            count: count.max(1),
            visual,
            target_override: None,
        }
    }

    fn map_name(&self) -> String {
        self.map.iter().copied().take_while(|value| *value != '\0').collect()
    }

    pub(crate) fn matches(&self, key: &MapInstanceKey) -> bool {
        self.instance == key.map_instance() && normalize_map(&self.map_name()) == normalize_map(key.map_name())
    }

    pub(crate) fn relocated(mut self, map: &str, instance: u8, x: u16, y: u16, tick: u128) -> Self {
        self.map = MapInstanceKey::new(map.to_string(), instance).map_name_char();
        self.instance = instance;
        self.x = x;
        self.y = y;
        self.tick = tick as u32;
        self.target_override = None;
        self
    }

    pub(crate) fn notification(&self, damage: &Damage, admitted: i64, packetver: u32) -> Notification {
        let target = self.target_override.unwrap_or(damage.target_id);
        let raw = match self.visual {
            DamageVisual::Action { action, hands } => {
                let total = i64::from(hands.0) + i64::from(hands.1);
                let right = if total == 0 {
                    admitted
                } else {
                    admitted * i64::from(hands.0) / total
                };
                let left = admitted - right;
                let header = if packetver >= 20131223 {
                    0x08C8_u16
                } else if packetver >= 20071113 {
                    0x02E1_u16
                } else {
                    0x008A_u16
                };
                let mut packet = header.to_le_bytes().to_vec();
                packet.extend_from_slice(&damage.attacker_id.to_le_bytes());
                packet.extend_from_slice(&target.to_le_bytes());
                packet.extend_from_slice(&self.tick.to_le_bytes());
                packet.extend_from_slice(&(self.attack_motion.min(i32::MAX as u32) as i32).to_le_bytes());
                packet.extend_from_slice(&(damage.damage_motion.min(i32::MAX as u32) as i32).to_le_bytes());
                if packetver >= 20071113 {
                    packet.extend_from_slice(&(right.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32).to_le_bytes());
                } else {
                    packet.extend_from_slice(&(right.clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16).to_le_bytes());
                }
                if packetver >= 20131223 {
                    packet.push(0);
                }
                packet.extend_from_slice(&(self.count as u16).to_le_bytes());
                packet.push(action.value() as u8);
                if packetver >= 20071113 {
                    packet.extend_from_slice(&(left.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32).to_le_bytes());
                } else {
                    packet.extend_from_slice(&(left.clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16).to_le_bytes());
                }
                packet
            }
            DamageVisual::Skill { skill_id, level } => {
                let mut packet = PacketZcNotifySkill2::new(packetver);
                packet.set_aid(damage.attacker_id);
                packet.set_target_id(target);
                packet.set_skid(skill_id);
                packet.set_level(i16::from(level));
                packet.set_start_time(self.tick);
                packet.set_attack_mt(self.attack_motion.min(i32::MAX as u32) as i32);
                packet.set_attacked_mt(damage.damage_motion.min(i32::MAX as u32) as i32);
                packet.set_damage(admitted.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32);
                packet.set_count(self.count);
                packet.set_action(6);
                packet.fill_raw();
                packet.raw
            }
        };
        Notification::Area(AreaNotification::new(
            self.map_name(),
            self.instance,
            AreaNotificationRangeType::Fov {
                x: self.x,
                y: self.y,
                exclude_id: None,
            },
            raw,
        ))
    }
}

impl Damage {
    pub(crate) fn with_skill_notification(
        mut self,
        map: &str,
        instance: u8,
        x: u16,
        y: u16,
        tick: u128,
        count: i16,
        attack_motion: u32,
    ) -> Self {
        self.notification = Some(DamageNotification::new(
            map,
            instance,
            x,
            y,
            tick,
            attack_motion,
            count,
            DamageVisual::Skill {
                skill_id: self.skill_id as u16,
                level: self.skill_level,
            },
        ));
        self
    }

    pub(crate) fn with_action_notification(
        mut self,
        map: &str,
        instance: u8,
        x: u16,
        y: u16,
        tick: u128,
        count: i16,
        attack_motion: u32,
        action: ActionType,
        hands: (i32, i32),
    ) -> Self {
        self.notification = Some(DamageNotification::new(
            map,
            instance,
            x,
            y,
            tick,
            attack_motion,
            count,
            DamageVisual::Action { action, hands },
        ));
        self
    }

    pub(crate) fn notify_admitted(&self, sender: &std::sync::mpsc::SyncSender<Notification>, admitted: i64, packetver: u32) {
        if let Some(display) = self.notification {
            if sender.send(display.notification(self, admitted, packetver)).is_err() {
                warn!("Damage notification failed");
            }
        }
    }

    pub(crate) fn matches_notification_map(&self, key: &MapInstanceKey) -> bool {
        self.notification.is_none_or(|notification| notification.matches(key))
    }
}
