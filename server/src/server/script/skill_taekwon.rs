use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::metadata::SkillMetadata;
use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;

const PERMANENT: i32 = -1;

impl ScriptSkillService {
    /// Stances and Tumbling last until they are cast again.
    pub(super) fn apply_stance(&self, server: &Server, character: &mut Character, effect: &ScriptSkillEffect, kind: StatusChangeKind, tick: u128) -> Result<(), String> {
        if character.status.has_status_change(kind) {
            StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
            return Ok(());
        }
        self.start_taekwon_status(server, character, effect, kind, PERMANENT, tick)
    }

    pub(super) fn apply_seven_wind(&self, server: &Server, character: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<(), String> {
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
        self.start_taekwon_status(server, character, effect, weapon, duration_ms, tick)?;
        self.start_taekwon_status(server, character, effect, StatusChangeKind::SevenWind, duration_ms, tick)
    }

    fn start_taekwon_status(&self, server: &Server, character: &mut Character, effect: &ScriptSkillEffect, kind: StatusChangeKind, duration_ms: i32, tick: u128) -> Result<(), String> {
        let mut request = StatusChangeRequest::guaranteed(kind, duration_ms, i32::from(effect.level));
        request.flags = 0;
        StatusEffectService::start(server, character, request, tick, &self.client_notification_sender).map(|_| ())
    }
}
