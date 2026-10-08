use std::sync::mpsc::SyncSender;

use models::enums::skill_enums::SkillEnum;
use models::status_change::StatusChangeKind;

use super::metadata::SkillMetadata;
use super::ScriptSkillService;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::map_item::MapItemType;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;

impl ScriptSkillService {
    /// Statuses the caster spends when a cast begins: Suffragium, a Memorize charge and the Mystical Amplification toggle.
    pub fn spend_cast_statuses(character: &mut Character, skill_id: u32, tick: u128, sender: &SyncSender<Notification>) {
        let Some(metadata) = SkillMetadata::find(skill_id) else { return };
        let mut spent = vec![];
        if character.status.has_status_change(StatusChangeKind::Suffragium) {
            spent.push(StatusChangeKind::Suffragium);
        }
        if metadata.cast_time.is_some() {
            if let Some(change) = character.status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Memorize) {
                change.values[1] -= 1;
                if change.values[1] <= 0 {
                    spent.push(StatusChangeKind::Memorize);
                }
            }
        }
        if skill_id != SkillEnum::HwMagicpower.id() && metadata.damage_type.as_deref() == Some("Magic") && metadata.damages() {
            if let Some(change) = character.status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::MagicPower) {
                if change.values[3] == 1 {
                    spent.push(StatusChangeKind::MagicPower);
                } else {
                    change.values[3] = 1;
                }
            }
        }
        for kind in spent {
            StatusEffectService::end_status_at(&mut character.status, Some(kind), tick);
            StatusEffectService::send_icon(character, kind, false, tick, sender);
        }
    }

    /// Each Martyr's Reckoning strike costs a charge and 9% of the caster's maximum HP, which can bring them to 1 HP but not kill.
    pub fn pay_martyrs_reckoning(server: &crate::server::Server, character: &mut Character, tick: u128, sender: &SyncSender<Notification>) {
        let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
        let cost = (snapshot.max_hp() * 9 / 100).min(character.status.hp.saturating_sub(1));
        server.character_service().update_hp_sp(character, character.status.hp - cost, character.status.sp);
        let spent = character.status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::Sacrifice).is_some_and(|change| {
            change.values[1] -= 1;
            change.values[1] <= 0
        });
        if spent {
            StatusEffectService::end_status_at(&mut character.status, Some(StatusChangeKind::Sacrifice), tick);
            StatusEffectService::send_icon(character, StatusChangeKind::Sacrifice, false, tick, sender);
        }
    }

    /// Each Envenom counter-cast uses one of Poison React's counters.
    pub fn spend_poison_react(character: &mut Character, tick: u128, sender: &SyncSender<Notification>) {
        let spent = character.status.active_statuses.iter_mut().find(|change| change.kind == StatusChangeKind::PoisonReact).is_some_and(|change| {
            change.values[1] -= 1;
            change.values[1] <= 0
        });
        if spent {
            StatusEffectService::end_status_at(&mut character.status, Some(StatusChangeKind::PoisonReact), tick);
            StatusEffectService::send_icon(character, StatusChangeKind::PoisonReact, false, tick, sender);
        }
    }

    /// Double Casting repeats a bolt: the damage lands a second time for a share of the casts.
    pub fn double_cast(character: &Character, skill_id: u32, mut damages: Vec<(MapItemType, Damage)>, roll: u8) -> Vec<(MapItemType, Damage)> {
        let bolt = [SkillEnum::MgColdbolt, SkillEnum::MgFirebolt, SkillEnum::MgLightningbolt].iter().any(|skill| skill.id() == skill_id);
        let repeats = bolt
            && character.status.status_change(StatusChangeKind::DoubleCast).is_some_and(|change| i32::from(roll) < change.values[1]);
        if repeats {
            let again = damages.clone();
            damages.extend(again);
        }
        damages
    }
}
