use std::collections::HashSet;

use models::enums::weapon::WeaponType;
use models::enums::EnumWithStringValue;
use models::enums::skill_enums::SkillEnum;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;

use super::{metadata::SkillMetadata, ScriptSkillEffect, ScriptSkillService};
use crate::server::model::events::game_event::{CharacterStatusChange, GameEvent};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

impl ScriptSkillService {
    pub(super) fn apply_party_support(&self, server: &Server, state: &ServerState, character: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<(), String> {
        let source = if effect.source_char_id == character.char_id { &*character } else { state.get_character(effect.source_char_id).ok_or("Party skill caster disconnected")? };
        let metadata = SkillMetadata::find(effect.skill_id).ok_or("Party skill metadata is unavailable")?;
        let skill = self.configuration.find_skill_config(&script_sdk::Value::Number(effect.skill_id as i32)).ok_or("Party skill configuration is unavailable")?;
        let kind = Self::skill_status(effect.skill_id).ok_or("Party skill status is unavailable")?;
        let radius = metadata.splash(effect.level).unwrap_or(-1);
        let radius = if radius < 0 { 14 } else { radius.min(i32::from(u16::MAX)) as u16 };
        let source_id = source.char_id;
        let party_id = source.game_systems.party_id;
        let source_map = source.map_instance_key.clone();
        let (source_x, source_y) = (source.x, source.y);
        let source_snapshot = StatusService::instance().to_snapshot(&source.status);
        let hilt_binding = source_snapshot.known_skills().iter().any(|skill| skill.value == SkillEnum::BsHiltbinding && skill.level > 0);
        let mut seen = HashSet::new();
        let recipients = state.characters().values().chain(std::iter::once(&*character)).filter(|target| {
            seen.insert(target.char_id) && target.status.hp > 0 && target.map_instance_key == source_map
                && if party_id == 0 { target.char_id == effect.target_id }
                else { target.game_systems.party_id == party_id && (radius == 0 || source_x.abs_diff(target.x).max(source_y.abs_diff(target.y)) <= radius) }
        }).filter(|target| {
            metadata.requires.as_ref().and_then(|requires| requires.get("Weapon")).and_then(|weapons| weapons.as_object()).is_none_or(|weapons| {
                let snapshot = StatusSnapshot::_from(&target.status);
                weapons.iter().any(|(weapon, enabled)| enabled.as_bool() == Some(true)
                    && WeaponType::try_from_string_ignore_case(weapon).is_ok_and(|weapon| weapon == *snapshot.right_hand_weapon_type()))
            })
        }).map(|target| {
            let mut request = Self::skill_status_request(skill, kind, effect.level);
            if matches!(kind, StatusChangeKind::WeaponPerfection | StatusChangeKind::Adrenaline | StatusChangeKind::Adrenaline2 | StatusChangeKind::Overthrust) {
                request.values[1] = i32::from(target.char_id == source_id);
                if hilt_binding { request.duration_ms = request.duration_ms.saturating_add(request.duration_ms / 10); }
            }
            (target.char_id, request)
        }).collect::<Vec<_>>();
        for (id, request) in recipients {
            if id == character.char_id {
                StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
            } else {
                server.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange { char_id: id, request }));
            }
            let mut notification = effect.clone();
            notification.source_char_id = id;
            notification.target_id = id;
            self.notify_support_skill(character, &notification);
        }
        Ok(())
    }
}
