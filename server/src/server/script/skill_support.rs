use configuration::configuration::SkillConfig;
use models::enums::EnumWithMaskValueU32;
use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{GameEvent, CharacterDamage};
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub(super) fn validate_skill_map(
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        issued_skill: bool,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Skill metadata is unavailable")?;
        if metadata.name == "AL_TELEPORT" && level > 2 {
            return Ok(());
        }
        let flags = state.map_flags(&character.map_instance_key);
        if metadata.name == "MC_VENDING" && flags.enabled(MapFlag::NoVending) {
            return Err("Vending is disabled on this map".into());
        }
        if !issued_skill && flags.enabled(MapFlag::NoSkill) {
            return Err("Skills cannot be used on this map".into());
        }
        if !issued_skill && Self::forbidden_on_map(skill_id, &flags) {
            return Err("This skill is forbidden on this kind of map".into());
        }
        if metadata.name == "AL_TELEPORT" && flags.enabled(MapFlag::NoTeleport) {
            return Err(crate::server::script::skill::actor::TELEPORT_DISABLED.into());
        }
        if metadata.name == "AL_WARP" && flags.enabled(MapFlag::NoWarp) {
            return Err("Warp Portal is disabled on this map".into());
        }
        if metadata.name == "WZ_ICEWALL" && flags.enabled(MapFlag::NoIceWall) {
            return Err("Ice Wall is disabled on this map".into());
        }
        Ok(())
    }

    /// `skill_nocast_db`: skills banned in normal, PvP, GvG or Battleground maps and in restricted zones.
    pub(crate) fn forbidden_on_map(skill_id: u32, flags: &crate::server::model::map_flags::MapFlags) -> bool {
        const NORMAL: u32 = 1;
        const PVP: u32 = 2;
        const GVG: u32 = 4;
        const BATTLEGROUND: u32 = 8;
        const ZONES_FROM: u32 = 32;
        let Some(banned) = nocast_flags(skill_id) else { return false };
        let versus = flags.enabled(MapFlag::Pvp) || flags.is_gvg() || flags.enabled(MapFlag::Battleground);
        let zones = (flags.get(MapFlag::Restricted, None) as u32) & !(ZONES_FROM - 1);
        (!versus && banned & NORMAL != 0)
            || (flags.enabled(MapFlag::Pvp) && banned & PVP != 0)
            || (flags.is_gvg() && banned & GVG != 0)
            || (flags.enabled(MapFlag::Battleground) && banned & BATTLEGROUND != 0)
            || banned & zones != 0
    }

    pub(super) fn snatch_map_allowed(state: &ServerState, source: &Character) -> bool {
        let flags = state.map_flags(&source.map_instance_key);
        !flags.enabled(MapFlag::NoTeleport) && !flags.is_gvg() && !flags.enabled(MapFlag::Battleground)
    }

    pub(super) fn final_strike_slide_allowed(state: &ServerState, source: &Character) -> bool {
        let flags = state.map_flags(&source.map_instance_key);
        !flags.is_gvg() && !flags.enabled(MapFlag::Battleground)
    }

    pub(super) fn undead_target(target: &StatusSnapshot) -> bool {
        *target.race() == MobRace::RUndead || *target.element() == Element::Undead
    }

    pub(super) fn offensive_heal_damage(
        &self,
        server: &Server,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Damage {
        let context = crate::server::service::map_combat_service::MagicAttackContext::new(
            (effect.heal_value / 2).min(u32::from(u16::MAX)) as u16,
            1.0,
            Element::Holy,
            1,
            effect.skill_id,
        );
        let mut damage = Damage { notification: None,
            source_kind: *source.combat_actor_kind(),
            skill_damage_adjusted: false,
            target_id: effect.target_id,
            attacker_id: effect.source_char_id,
            damage: 0,
            healing: 0,
            right_hand_damage: None,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
            skill_id: effect.skill_id,
            skill_level: effect.level,
            landed: true,
            proc_depth: effect.proc_depth,
            credit_id: effect.source_char_id,
            defenses_applied: true,
            magic_context: Some(context),
        };
        damage.set_signed_damage(server.battle_service().magic_damage_from_context(source, target, context));
        damage
    }

    pub fn provoke_rate(source_level: u32, target_level: u32, level: u8) -> u16 {
        ((70 + 3 * i64::from(level) + i64::from(source_level) - i64::from(target_level)) * 100).clamp(0, i64::from(u16::MAX)) as u16
    }

    pub fn decrease_agi_rate(source_level: u32, source_int: u16, level: u8) -> u16 {
        ((50 + 3 * u64::from(level) + (u64::from(source_level) + u64::from(source_int)) / 5) * 100).min(u64::from(u16::MAX)) as u16
    }

    pub(super) fn endow_skill(name: &str) -> bool {
        matches!(
            name,
            "SA_FLAMELAUNCHER" | "SA_FROSTWEAPON" | "SA_LIGHTNINGLOADER" | "SA_SEISMICWEAPON"
        )
    }

    pub(super) fn support_status_request(
        skill: &SkillConfig,
        kind: StatusChangeKind,
        level: u8,
        source_level: u32,
        source_int: u16,
        target_level: u32,
    ) -> StatusChangeRequest {
        let mut request = Self::skill_status_request(skill, kind, level);
        request.rate = match skill.name().as_str() {
            "SM_PROVOKE" | "MS_PROVOKE" => Self::provoke_rate(source_level, target_level, level),
            "AL_DECAGI" => Self::decrease_agi_rate(source_level, source_int, level),
            "BA_PANGVOICE" => 7000,
            name if Self::endow_skill(name) => (6000 + 1000 * u16::from(level)).min(10000),
            _ => 10000,
        };
        request
    }

    pub(super) fn validate_support_target(
        &self,
        state: &ServerState,
        source: &Character,
        skill_id: u32,
        target_id: u32,
    ) -> Result<(), String> {
        let Some(metadata) = SkillMetadata::find(skill_id) else {
            return Ok(());
        };
        if Self::endow_skill(&metadata.name) {
            let target = if source.char_id == target_id {
                Some(source)
            } else {
                state.get_character(target_id)
            };
            if target.is_some_and(|target| target.status.right_hand_weapon().is_none()) {
                return Err("An elemental endow requires an equipped weapon".into());
            }
        }
        Ok(())
    }

    pub(super) fn delayed_support_request(effect: &ScriptSkillEffect, kind: StatusChangeKind) -> StatusChangeRequest {
        StatusChangeRequest {
            kind,
            duration_ms: SkillMetadata::find(effect.skill_id)
                .and_then(|metadata| metadata.duration(effect.level, true))
                .unwrap_or(0),
            values: [i32::from(effect.level), 0, 0, 0],
            rate: 10000,
            flags: 0,
        }
    }

    pub(super) fn queue_delayed_status(
        &self,
        server: &Server,
        source: &Character,
        effect: &ScriptSkillEffect,
        request: StatusChangeRequest,
    ) {
        let mut delayed = effect.clone();
        delayed.action = ScriptSkillAction::DelayedStatus {
            request,
            map: source.current_map_name().clone(),
            instance: source.current_map_instance(),
        };
        delayed.cast_generation = 0;
        delayed.deferred_requirements = None;
        delayed.source_index = None;
        delayed.source_item = None;
        delayed.skill_event_emitted = true;
        server.add_to_tick(
            GameEvent::CharacterScriptSkill(delayed),
            1000_usize.div_ceil(40).saturating_sub(1),
        );
    }

    pub fn cast_interruption_protected(snapshot: &StatusSnapshot, flags: &MapFlags) -> bool {
        snapshot
            .bonuses_raw()
            .iter()
            .any(|bonus| matches!(bonus, BonusType::EnableNoCancelCast2))
            || (!flags.is_gvg()
                && !flags.enabled(MapFlag::Battleground)
                && snapshot
                    .bonuses_raw()
                    .iter()
                    .any(|bonus| matches!(bonus, BonusType::EnableNoCancelCast)))
    }

    pub(super) fn cancel_provoked_cast(&self, state: &ServerState, character: &mut Character, tick: u128) {
        let casting = character
            .skill_in_use
            .as_ref()
            .filter(|cast| cast.used_at_tick.is_none())
            .map(|cast| cast.skill.id())
            .or_else(|| (character.script_skill_state.casting_until > tick).then_some(character.script_skill_state.casting_skill_id));
        if casting.is_none_or(|id| !self.cast_cancelable(id)) {
            return;
        }
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        if Self::cast_interruption_protected(&snapshot, &state.map_flags(&character.map_instance_key)) {
            return;
        }
        character.clear_skill_in_use();
        character.clear_pending_skill();
        self.cancel_queued_cast(character);
        character.timing.set_canact_tick(tick);
        let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        self.notify_area(character, packet);
    }

    pub(super) fn apply_player_support_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<bool, String> {
        let metadata = SkillMetadata::find(effect.skill_id).ok_or("Skill metadata is unavailable")?;
        let source = if effect.source_char_id == character.char_id {
            &*character
        } else {
            state.get_character(effect.source_char_id).ok_or("Caster disconnected")?
        };
        match metadata.name.as_str() {
            name if Self::party_support_skill(name) => {
                self.apply_party_support(server, state, character, effect, tick)?;
                return Ok(true);
            }
            "PR_LEXDIVINA" => {
                if character.status.has_status_change(StatusChangeKind::Silence) {
                    StatusEffectService::end(
                        server,
                        character,
                        Some(StatusChangeKind::Silence),
                        tick,
                        &self.client_notification_sender,
                    );
                } else {
                    self.queue_delayed_status(
                        server,
                        source,
                        effect,
                        Self::delayed_support_request(effect, StatusChangeKind::Silence),
                    );
                }
            }
            "AL_BLESSING" | "AL_INCAGI" if character.status.has_status_change(StatusChangeKind::ChangeUndead) => {
                if character.status.hp > 1 {
                    let damage = Damage { notification: None,
                        source_kind: models::enums::actor::CombatActorKind::Player,
                        skill_damage_adjusted: false,
                        target_id: effect.target_id,
                        attacker_id: effect.source_char_id,
                        damage: 1,
                        healing: 0,
                        right_hand_damage: None,
                        attacked_at: tick,
                        damage_motion: 0,
                        battle_flags: BattleFlag::Misc.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                        skill_id: effect.skill_id,
                        skill_level: effect.level,
                        landed: true,
                        proc_depth: effect.proc_depth,
                        credit_id: effect.source_char_id,
                        defenses_applied: true,
                        magic_context: None,
                    };
                    server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage }));
                }
            }
            "SM_PROVOKE" | "MS_PROVOKE" | "AL_DECAGI" | "SA_FLAMELAUNCHER" | "SA_FROSTWEAPON" | "SA_LIGHTNINGLOADER"
            | "SA_SEISMICWEAPON" => {
                let skill = self
                    .configuration
                    .find_skill_config(&script_sdk::Value::Number(effect.skill_id as i32))
                    .ok_or("Unknown skill")?;
                let kind = Self::status_for_skill(skill.name()).ok_or("Skill status is unavailable")?;
                let source_status = StatusService::instance().to_snapshot(&source.status);
                let target_status = StatusService::instance().to_snapshot(&character.status);
                if kind == StatusChangeKind::Provoke && Self::undead_target(&target_status) {
                    self.notify_support_skill_result(character, effect, false);
                    return Ok(true);
                }
                if Self::endow_skill(skill.name()) && character.status.right_hand_weapon().is_none() {
                    self.notify_support_skill_result(character, effect, false);
                    return Ok(true);
                }
                let request = Self::support_status_request(
                    skill,
                    kind,
                    effect.level,
                    source.status.base_level,
                    source_status.int(),
                    character.status.base_level,
                );
                let started = StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
                if started && kind == StatusChangeKind::Provoke {
                    self.cancel_provoked_cast(state, character, tick);
                    let rate = crate::server::service::combat_trigger_service::coma_chance(
                        &source_status,
                        &target_status,
                        BattleFlag::Misc.as_flag(),
                    );
                    if rate > 0 {
                        let request = StatusChangeRequest {
                            kind: StatusChangeKind::Coma,
                            values: [i32::from(effect.level), 0, effect.source_char_id as i32, 0],
                            duration_ms: 0,
                            rate,
                            flags: 0,
                        };
                        StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
                    }
                }
                if !started && Self::endow_skill(skill.name()) {
                    if let Some(weapon) = character.status.right_hand_weapon().copied() {
                        server.inventory_service().takeoff_equip_item(character, weapon.inventory_index);
                    }
                }
                self.notify_support_skill_result(character, effect, started);
                return Ok(true);
            }
            "PR_STRECOVERY" => {
                let target_status = StatusService::instance().to_snapshot(&character.status);
                if Self::undead_target(&target_status) {
                    self.queue_delayed_status(
                        server,
                        source,
                        effect,
                        Self::delayed_support_request(effect, StatusChangeKind::Blind),
                    );
                } else {
                    for kind in [
                        StatusChangeKind::Stone,
                        StatusChangeKind::StoneWait,
                        StatusChangeKind::Freeze,
                        StatusChangeKind::Stun,
                        StatusChangeKind::Sleep,
                    ] {
                        StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
                    }
                }
                StatusEffectService::end(
                    server,
                    character,
                    Some(StatusChangeKind::NoRecovery),
                    tick,
                    &self.client_notification_sender,
                );
            }
            _ => return Ok(false),
        }
        self.notify_support_skill(character, effect);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_chances_use_caster_level_and_current_intelligence() {
        assert_eq!(ScriptSkillService::provoke_rate(50, 60, 10), 9000);
        assert_eq!(ScriptSkillService::provoke_rate(1, 99, 1), 0);
        assert_eq!(ScriptSkillService::decrease_agi_rate(50, 70, 10), 10400);
    }

    #[test]
    fn cast_protection_distinguishes_gvg_and_battleground_from_pvp() {
        for (bonus, always) in [(BonusType::EnableNoCancelCast, false), (BonusType::EnableNoCancelCast2, true)] {
            let mut snapshot = StatusSnapshot::_from(&models::status::Status::default());
            snapshot.set_bonuses(vec![models::status_bonus::StatusBonus::new(bonus)]);
            for (map, protected) in [(MapFlag::Pvp, true), (MapFlag::Gvg, always), (MapFlag::Battleground, always)] {
                let mut flags = MapFlags::default();
                flags.set(map, true, if map == MapFlag::Battleground { &[1] } else { &[] }).unwrap();
                assert_eq!(ScriptSkillService::cast_interruption_protected(&snapshot, &flags), protected);
            }
        }
    }
}

fn nocast_flags(skill_id: u32) -> Option<u32> {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    static TABLE: OnceLock<HashMap<u32, u32>> = OnceLock::new();
    TABLE
        .get_or_init(|| {
            let mut table = HashMap::new();
            for line in include_str!("skill_nocast_db.txt").lines().filter(|line| !line.starts_with('#')) {
                if let Some((skill, flag)) = line.split_once(',').and_then(|(skill, flag)| Some((skill.parse::<u32>().ok()?, flag.parse::<u32>().ok()?))) {
                    *table.entry(skill).or_default() |= flag;
                }
            }
            table
        })
        .get(&skill_id)
        .copied()
}
