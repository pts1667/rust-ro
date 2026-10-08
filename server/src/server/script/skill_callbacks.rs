use models::enums::EnumWithMaskValueU32;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::metadata::SkillMetadata;
use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterPickUpItem, CharacterStatusChange, GameEvent};
use crate::server::model::events::map_event::{MapEvent, MobDamage, MobStatusAlternatives, MobStatusChange, ScriptMobCombat};
use crate::server::model::map_item::MapItemType;
use crate::server::service::map_combat_service::MagicAttackContext;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkillOperation {
    Damage,
    Status,
    AreaStatus,
    Ground,
    Recovery,
    Inventory,
    Movement,
    Spirit,
    Dispel,
    Tarot,
    Estimate,
}

impl ScriptSkillService {
    /// Script operation of a skill: the `Route` of its metadata, or `Status` for a plain buff that names no route.
    pub fn operation(name: &str) -> Option<SkillOperation> {
        let metadata = SkillMetadata::find_by_name(name)?;
        match metadata.route {
            Some(route) => route.operation(),
            None => Self::metadata_buff_status(name).map(|_| SkillOperation::Status),
        }
    }

    pub fn area_status_request(
        name: &str,
        skill_id: u32,
        level: u8,
        party_member: bool,
        dragon_choice: usize,
    ) -> Option<(StatusChangeRequest, u128)> {
        use StatusChangeKind::*;
        let metadata = SkillMetadata::find(skill_id)?;
        let (kind, chance, delay) = match name {
            "BA_FROSTJOKER" => (Freeze, 1500 + 500 * level as i32, 3000),
            "DC_SCREAM" => (Stun, 2500 + 500 * level as i32, 3000),
            "NPC_WIDEBLEEDING" => (Bleeding, 10_000, 0),
            "NPC_WIDECONFUSE" => (Confusion, 10_000, 0),
            "NPC_WIDECURSE" => (Curse, 10_000, 0),
            "NPC_WIDESILENCE" => (Silence, 10_000, 0),
            "NPC_WIDESLEEP" => (Sleep, 10_000, 0),
            "NPC_WIDESTONE" => (Stone, 10_000, 0),
            "NPC_WIDEFREEZE" => (Freeze, 10_000, 0),
            "NPC_WIDESTUN" => (Stun, 10_000, 0),
            "NPC_WIDEHELLDIGNITY" => (HellPower, 10_000, 0),
            "NPC_DRAGONFEAR" => ([Stun, Silence, Confusion, Bleeding][dragon_choice % 4], 10_000, 0),
            "AL_CRUCIS" => (SignumCrucis, 2500 + 400 * level as i32, 0),
            "BS_HAMMERFALL" => (Stun, (2000 + 1000 * level as i32).min(5000 + 500 * level as i32), 1000),
            "PR_BENEDICTIO" => (Benedictio, 10_000, 0),
            _ => return None,
        };
        let party_exception = party_member && matches!(name, "BA_FROSTJOKER" | "DC_SCREAM");
        let duration = if name == "AL_CRUCIS" {
            Some(-1)
        } else if party_exception || name == "PR_BENEDICTIO" {
            metadata.duration(level, false)
        } else if name == "NPC_DRAGONFEAR" {
            metadata.duration((dragon_choice % 4 + 1) as u8, true)
        } else {
            metadata.duration(level, true)
        }
        .unwrap_or(0);
        Some((
            StatusChangeRequest {
                kind,
                duration_ms: duration,
                values: [level as i32, 0, 0, 0],
                rate: (if party_exception { chance / 4 } else { chance }).clamp(0, 10_000) as u16,
                flags: 0,
            },
            delay,
        ))
    }

    pub fn wide_soul_drain(level: u8, current_sp: u32) -> u32 {
        if level == 0 || current_sp == 0 {
            return 0;
        }
        let percent = u64::from((level - 1) % 5 + 1) * 20;
        (u64::from(current_sp) * percent / 100).max(1).min(u64::from(current_sp)) as u32
    }

    pub fn signum_crucis_rate(level: u8, source_level: u32, target_level: u32) -> u16 {
        ((25 + 4 * i64::from(level) + i64::from(source_level) - i64::from(target_level)) * 100).clamp(0, i64::from(u16::MAX)) as u16
    }

    pub fn dragon_fear_requests(skill_id: u32, level: u8, source_id: u32, first_choice: usize) -> Vec<StatusChangeRequest> {
        (0..4)
            .filter_map(|offset| {
                let (mut request, _) = Self::area_status_request("NPC_DRAGONFEAR", skill_id, level, false, (first_choice + offset) % 4)?;
                request.values[1] = source_id as i32;
                Some(request)
            })
            .collect()
    }

    fn is_undead_or_demon(status: &StatusSnapshot) -> bool {
        *status.element() == Element::Undead || matches!(status.race(), MobRace::Demon | MobRace::RUndead)
    }

    /// Benedictio buffs non-undead, non-demon players in the area and hits undead or demon mobs with holy magic.
    fn cast_benedictio(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let radius = SkillMetadata::find(skill_id).and_then(|metadata| metadata.splash(level)).unwrap_or(1).max(0) as u16;
        let in_area = |target_x: u16, target_y: u16| target_x.abs_diff(x).max(target_y.abs_diff(y)) <= radius;
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        let (request, delay) = Self::area_status_request("PR_BENEDICTIO", skill_id, level, false, 0)
            .ok_or("Benedictio status is unavailable")?;
        let source = StatusService::instance().to_snapshot(&character.status);
        let lower = source.matk_min().min(source.matk_max());
        let upper = source.matk_max().max(lower);
        for mob in instance.state().mobs().values().filter(|mob| {
            mob.status.hp() > 0 && (!mob.summoned || mob.summon_ai == 0) && in_area(mob.x, mob.y) && Self::is_undead_or_demon(&mob.status)
        }) {
            let context = MagicAttackContext::new(fastrand::u16(lower..=upper), 1.0, Element::Holy, 1, skill_id);
            let mut damage = Damage {
                notification: None,
                source_kind: *source.combat_actor_kind(),
                skill_damage_adjusted: false,
                healing: 0,
                right_hand_damage: None,
                target_id: mob.id,
                attacker_id: character.char_id,
                damage: 0,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                skill_id,
                skill_level: level,
                landed: true,
                proc_depth: 0,
                credit_id: character.char_id,
                defenses_applied: true,
                magic_context: Some(context),
            };
            damage.set_signed_damage(server.battle_service().magic_damage_from_context(&source, &mob.status, context));
            let damage = damage.with_skill_notification(character.current_map_name(), character.current_map_instance(), character.x, character.y, tick, 1, 0);
            instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage }));
        }
        if in_area(character.x, character.y) && !Self::is_undead_or_demon(&source) {
            server.add_to_delayed_tick(
                GameEvent::CharacterStatusChange(CharacterStatusChange { char_id: character.char_id, request: request.clone() }),
                delay,
            );
        }
        for target in state.characters().values().filter(|target| {
            target.char_id != character.char_id && target.map_instance_key == character.map_instance_key && in_area(target.x, target.y)
        }) {
            if Self::is_undead_or_demon(&StatusService::instance().to_snapshot(&target.status)) {
                continue;
            }
            server.add_to_delayed_tick(
                GameEvent::CharacterStatusChange(CharacterStatusChange { char_id: target.char_id, request: request.clone() }),
                delay,
            );
        }
        let effect = ScriptSkillEffect {
            source_char_id: character.char_id,
            target_id: character.char_id,
            skill_id,
            level,
            heal_value: 0,
            proc_depth: 0,
            skill_event_emitted: true,
            cast_generation: 0,
            action: super::ScriptSkillAction::Cast,
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        };
        self.notify_support_skill(character, &effect);
        Ok(())
    }

    pub fn cast_area_status(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Area skill has no pre-renewal definition")?;
        if metadata.name == "PR_BENEDICTIO" {
            return self.cast_benedictio(server, state, character, skill_id, level, x, y, tick);
        }
        let radius = metadata.splash(level).unwrap_or(15);
        let radius = if radius < 0 { 15 } else { radius.min(u16::MAX as i32) as u16 };
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        for mob in instance.state().mobs().values().filter(|mob| {
            mob.status.hp() > 0 && (!mob.summoned || mob.summon_ai == 0) && mob.x.abs_diff(x).max(mob.y.abs_diff(y)) <= radius
        }) {
            if metadata.name == "AL_CRUCIS"
                && *mob.status.element() != Element::Undead
                && !matches!(mob.status.race(), MobRace::Demon | MobRace::RUndead)
            {
                continue;
            }
            if metadata.name == "NPC_WIDESOULDRAIN" {
                instance.add_to_next_tick(MapEvent::ScriptMobCombat(ScriptMobCombat {
                    source_id: character.char_id,
                    target_id: mob.id,
                    effect: crate::server::service::script_combat_service::MobCombatEffect::Vanish {
                        hp: 0,
                        sp: Self::wide_soul_drain(level, mob.status.sp()),
                    },
                }));
            } else if metadata.name == "NPC_DRAGONFEAR" {
                instance.add_to_next_tick(MapEvent::MobStatusAlternatives(MobStatusAlternatives {
                    mob_id: mob.id,
                    requests: Self::dragon_fear_requests(skill_id, level, character.char_id, fastrand::usize(0..4)),
                }));
            } else if let Some((mut request, delay)) =
                Self::area_status_request(&metadata.name, skill_id, level, false, fastrand::usize(0..4))
            {
                if metadata.name == "AL_CRUCIS" {
                    request.rate = Self::signum_crucis_rate(level, character.status.base_level, mob.status_effects.base_level);
                }
                if metadata.name.starts_with("NPC_") {
                    request.values[1] = character.char_id as i32;
                }
                instance.add_to_delayed_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: mob.id, request }), delay);
            }
        }
        for target in state.characters().values().filter(|target| {
            target.char_id != character.char_id
                && target.map_instance_key == character.map_instance_key
                && target.x.abs_diff(x).max(target.y.abs_diff(y)) <= radius
        }) {
            let same_party = character.game_systems.party_id != 0 && target.game_systems.party_id == character.game_systems.party_id;
            if same_party && matches!(metadata.name.as_str(), "BA_FROSTJOKER" | "DC_SCREAM") {
                if let Some((request, delay)) = Self::area_status_request(&metadata.name, skill_id, level, true, 0) {
                    server.add_to_delayed_tick(
                        GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange {
                            char_id: target.char_id,
                            request,
                        }),
                        delay,
                    );
                }
            }
        }
        let effect = ScriptSkillEffect {
            source_char_id: character.char_id,
            target_id: character.char_id,
            skill_id,
            level,
            heal_value: 0,
            proc_depth: 0,
            skill_event_emitted: true,
            cast_generation: 0,
            action: super::ScriptSkillAction::Cast,
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        };
        self.notify_support_skill(character, &effect);
        let _ = tick;
        Ok(())
    }

    pub fn collect_nearby_items(&self, server: &Server, state: &ServerState, character: &Character) -> Result<(), String> {
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        let ids = instance
            .state()
            .map_items()
            .iter()
            .filter_map(|(_, item)| {
                if matches!(item.object_type(), MapItemType::DroppedItem) {
                    instance
                        .state()
                        .get_dropped_item(item.id())
                        .filter(|item| item.location.x.abs_diff(character.x).max(item.location.y.abs_diff(character.y)) <= 2)
                        .map(|_| item.id())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for id in ids {
            server.add_to_next_tick(GameEvent::CharacterPickUpItem(CharacterPickUpItem {
                char_id: character.char_id,
                map_item_id: id,
            }));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signum_crucis_chance_uses_skill_level_and_the_actual_level_difference() {
        assert_eq!(ScriptSkillService::signum_crucis_rate(5, 80, 50), 7500);
        assert_eq!(ScriptSkillService::signum_crucis_rate(5, 50, 80), 1500);
        assert_eq!(ScriptSkillService::signum_crucis_rate(1, 1, 99), 0);
    }

    #[test]
    fn dragon_fear_orders_each_fallback_once_with_its_own_duration_and_caster() {
        let metadata = SkillMetadata::all().iter().find(|skill| skill.name == "NPC_DRAGONFEAR").unwrap();
        let requests = ScriptSkillService::dragon_fear_requests(metadata.id, 5, 123, 1);
        assert_eq!(requests.iter().map(|request| request.kind).collect::<Vec<_>>(), vec![
            StatusChangeKind::Silence,
            StatusChangeKind::Confusion,
            StatusChangeKind::Bleeding,
            StatusChangeKind::Stun
        ]);
        assert!(requests.iter().all(|request| request.values[1] == 123 && request.rate == 10000));
        assert_eq!(requests[0].duration_ms, metadata.duration(2, true).unwrap());
        assert_eq!(requests[3].duration_ms, metadata.duration(1, true).unwrap());
    }

    #[test]
    fn wide_soul_drain_wraps_every_five_levels_and_removes_at_least_one_sp() {
        assert_eq!(ScriptSkillService::wide_soul_drain(1, 23), 4);
        assert_eq!(ScriptSkillService::wide_soul_drain(1, 2), 1);
        assert_eq!(ScriptSkillService::wide_soul_drain(5, 23), 23);
        assert_eq!(ScriptSkillService::wide_soul_drain(6, 23), 4);
        assert_eq!(ScriptSkillService::wide_soul_drain(10, 23), 23);
        assert_eq!(ScriptSkillService::wide_soul_drain(10, 0), 0);
    }

    #[test]
    fn joker_and_scream_preserve_different_classic_probabilities() {
        let joker = SkillMetadata::all().iter().find(|skill| skill.name == "BA_FROSTJOKER").unwrap();
        let scream = SkillMetadata::all().iter().find(|skill| skill.name == "DC_SCREAM").unwrap();
        let (enemy_joker, delay) = ScriptSkillService::area_status_request(&joker.name, joker.id, 1, false, 0).unwrap();
        let (party_joker, _) = ScriptSkillService::area_status_request(&joker.name, joker.id, 1, true, 0).unwrap();
        let (enemy_scream, _) = ScriptSkillService::area_status_request(&scream.name, scream.id, 1, false, 0).unwrap();
        assert_eq!(enemy_joker.rate, 2000);
        assert_eq!(enemy_scream.rate, 3000);
        assert_eq!(party_joker.rate, 500);
        assert_eq!(enemy_joker.duration_ms, 12000);
        assert_eq!(party_joker.duration_ms, 15000);
        assert_eq!(delay, 3000);
    }
}
