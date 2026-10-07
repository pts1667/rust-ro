//! Active class skills that need none of the shared pipelines: Cast Cancel and its relatives.

use models::enums::skill_enums::SkillEnum;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::{ScriptSkillService, metadata::SkillMetadata};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::Server;
use crate::server::state::character::Character;

impl ScriptSkillService {
    /// `skill_autospell`: Auto Spell lasts `Duration1` and casts `spell` up to the level the caster learned and the menu allows.
    pub fn start_auto_spell(&self, server: &Server, character: &mut Character, level: u8, spell: SkillEnum, learned: u8, tick: u128) -> Result<(), String> {
        let ceiling = match spell {
            SkillEnum::MgNapalmbeat => 3,
            SkillEnum::MgColdbolt | SkillEnum::MgFirebolt | SkillEnum::MgLightningbolt => match level {
                0..=1 => 0,
                2 => 1,
                3 => 2,
                _ => 3,
            },
            SkillEnum::MgSoulstrike => match level {
                5 => 1,
                6 => 2,
                _ => 3,
            },
            SkillEnum::MgFireball => if level == 8 { 1 } else { 2 },
            _ => 1,
        };
        let skill_id = SkillEnum::SaAutospell.id();
        let duration = SkillMetadata::find(skill_id).and_then(|metadata| metadata.duration(level, false)).unwrap_or(120_000);
        let request = StatusChangeRequest {
            kind: StatusChangeKind::AutoSpell,
            duration_ms: duration,
            values: [i32::from(level), spell.id() as i32, i32::from(learned.min(ceiling)), 5 + i32::from(level) * 2],
            rate: 10_000,
            flags: 0,
        };
        StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
        Ok(())
    }

    /// `SA_CASTCANCEL`: stops the cast in progress and charges a share of its SP, since the SP of a cast is only paid when it ends.
    pub fn cast_cancel(&self, server: &Server, character: &mut Character, skill_id: u32, level: u8, tick: u128) -> Result<(), String> {
        let casting = character
            .skill_in_use
            .as_ref()
            .filter(|cast| cast.used_at_tick.is_none())
            .map(|cast| (cast.skill.id(), cast.skill.level()))
            .or_else(|| (character.script_skill_state.casting_until > tick).then_some((character.script_skill_state.casting_skill_id, character.script_skill_state.casting_skill_level)));
        let (cancelled_id, cancelled_level) = casting.filter(|(id, _)| *id != skill_id && *id != 0).ok_or("There is no cast to cancel")?;
        let cancelled_cost = self.requirements_plan(character, cancelled_id, cancelled_level, tick).map_or(0, |plan| plan.sp);
        server.item_service().pay_skill_requirements(server, character, skill_id, level, tick, true, None)?;
        let share = 90_u32.saturating_sub(u32::from(level.saturating_sub(1)) * 20);
        let sp = character.status.sp.saturating_sub(cancelled_cost * share / 100);
        server.character_service().update_hp_sp(character, character.status.hp, sp);
        character.clear_skill_in_use();
        self.cancel_queued_cast(character);
        character.timing.set_canact_tick(tick);
        let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        self.notify_area(character, packet);
        Ok(())
    }
}
