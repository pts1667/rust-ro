use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::metadata::SkillMetadata;
use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;

const PERMANENT: i32 = -1;

impl ScriptSkillService {
    /// Stances and Tumbling last until they are cast again.
    fn taekwon_toggle(name: &str) -> Option<StatusChangeKind> {
        match name {
            "TK_READYSTORM" => Some(StatusChangeKind::ReadyStorm),
            "TK_READYDOWN" => Some(StatusChangeKind::ReadyDown),
            "TK_READYTURN" => Some(StatusChangeKind::ReadyTurn),
            "TK_READYCOUNTER" => Some(StatusChangeKind::ReadyCounter),
            "TK_DODGE" => Some(StatusChangeKind::Dodge),
            _ => None,
        }
    }

    pub(super) fn is_taekwon_skill(name: &str) -> bool {
        Self::taekwon_toggle(name).is_some() || name == "TK_SEVENWIND"
    }

    pub(super) fn apply_taekwon_skill(
        &self,
        server: &Server,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        name: &str,
        tick: u128,
    ) -> Result<(), String> {
        let level = i32::from(effect.level);
        let start = |character: &mut Character, kind: StatusChangeKind, duration_ms: i32| {
            let mut request = StatusChangeRequest::guaranteed(kind, duration_ms, level);
            request.flags = 0;
            StatusEffectService::start(server, character, request, tick, &self.client_notification_sender).map(|_| ())
        };
        if let Some(kind) = Self::taekwon_toggle(name) {
            if character.status.has_status_change(kind) {
                StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
                return Ok(());
            }
            return start(character, kind, PERMANENT);
        }
        let metadata = SkillMetadata::find(effect.skill_id).ok_or("Seven Wind metadata is unavailable")?;
        let weapon = match metadata.element(effect.level) {
            Some("Earth") => StatusChangeKind::EarthWeapon,
            Some("Wind") => StatusChangeKind::WindWeapon,
            Some("Water") => StatusChangeKind::WaterWeapon,
            Some("Fire") => StatusChangeKind::FireWeapon,
            Some("Ghost") => StatusChangeKind::GhostWeapon,
            Some("Dark") => StatusChangeKind::ShadowWeapon,
            Some("Holy") => StatusChangeKind::Aspersio,
            _ => return Err("Seven Wind has no element at this level".into()),
        };
        let duration_ms = metadata.duration(effect.level, false).unwrap_or(0);
        start(character, weapon, duration_ms)?;
        start(character, StatusChangeKind::SevenWind, duration_ms)
    }
}
