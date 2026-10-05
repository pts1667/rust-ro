use configuration::configuration::SkillConfig;
use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::enums::EnumWithMaskValueU16;
use models::enums::EnumWithNumberValue;
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};
use models::enums::EnumWithMaskValueU32;
use movement::position::Position;
use script_sdk::Value;
use packets::packets::{Packet, PacketZcUseskillAck2};

use super::{metadata::SkillMetadata, ScriptSkillAction, ScriptSkillEffect, ScriptSkillService, ScriptSkillState};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterMovement, CharacterUseSkill, GameEvent};
use crate::server::model::map_item::MapItemType;
use crate::server::model::movement::Movement;
use crate::server::model::path::PathNode;
use crate::server::service::battle_service::BattleService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

impl ScriptSkillState {
    pub fn add_sphere(&mut self, expires_at: u128, maximum: usize) {
        if maximum == 0 { return; }
        self.spirit_spheres.sort_unstable();
        while self.spirit_spheres.len() >= maximum { self.spirit_spheres.remove(0); }
        self.spirit_spheres.push(expires_at);
        self.spirit_spheres.sort_unstable();
    }

    pub fn expire_spheres(&mut self, tick: u128) -> bool {
        let previous = self.spirit_spheres.len();
        self.spirit_spheres.retain(|expiry| *expiry > tick);
        self.coins = self.coins.min(self.spirit_spheres.len() as u8);
        previous != self.spirit_spheres.len()
    }
}

impl ScriptSkillService {
    pub fn validate_pending_cast(&self, state: &ServerState, character: &Character, event: &CharacterUseSkill, tick: u128) -> Result<(), String> {
        let pending = character.pending_item_skill.as_ref().ok_or("No item skill is awaiting a target")?;
        if pending.expires_at <= tick { return Err("Item skill targeting expired".into()); }
        if event.char_id != character.char_id || event.skill_id != pending.skill_id || event.skill_level != pending.level { return Err("Targeting reply does not match the pending item skill".into()); }
        let skill = self.configuration.find_skill_config(&Value::Number(event.skill_id as i32)).ok_or("Unknown item skill")?;
        self.validate_skill(skill, event.skill_level as u32)?;
        if event.skill_id == models::enums::skill_enums::SkillEnum::SlSma.id()
            && !character.status.status_change(StatusChangeKind::Sma).is_some_and(|ready| !ready.expired(tick)) {
            return Err("Esma requires an active Estin or Estun readiness effect".into());
        }
        Self::validate_stealth_cast(state, character, event.skill_id)?;
        self.validate_damage_target(state, character, event.skill_id, event.target_id)?;
        self.validate_support_target(state, character, event.skill_id, event.target_id)?;
        if character.status.hp == 0 || character.status.blocks_casting() || character.is_using_skill() || character.script_skill_state.casting_until > tick || character.timing.get_canact_tick() > tick { return Err("Character cannot start a skill now".into()); }
        if let Some(target) = state.get_character(event.target_id) {
            if target.map_instance_key != character.map_instance_key { return Err("Item skill target is on another map".into()); }
        }
        let position = if matches!(skill.name().as_str(), "HT_REMOVETRAP" | "HT_SPRINGTRAP") {
            let (x, y) = self.validate_player_trap_control(state, character, event.target_id, skill.id, pending.level, tick, false)?;
            Position { x, y, dir: 0 }
        } else if event.target_id == character.char_id { Position { x: character.x, y: character.y, dir: character.dir } }
            else { state.map_item_snapshot(event.target_id, character.current_map_name(), character.current_map_instance()).ok_or("Item skill target is not on this map")?.position };
        if character.x.abs_diff(position.x).max(character.y.abs_diff(position.y)) > self.player_skill_range(&StatusService::instance().to_snapshot(&character.status), skill.id, pending.level).max(1) { return Err("Item skill target is out of range".into()); }
        let target = if event.target_id == character.char_id { Some(character) } else { state.get_character(event.target_id) };
        if skill.name() == "AS_SPLASHER" {
            let effect = ScriptSkillEffect { source_char_id: character.char_id, target_id: event.target_id, skill_id: event.skill_id, level: event.skill_level, heal_value: 0, proc_depth: 0, skill_event_emitted: false, cast_generation: 0, action: ScriptSkillAction::Cast, deferred_requirements: None, prepared_outcome: None, source_index: None, source_item: None };
            self.validate_splasher_effect(state, character, &effect)?;
        }
        if skill.name() == "CR_DEVOTION" { self.validate_devotion_target(state, character, target.ok_or("Devotion requires a player target")?, pending.level)?; }
        match skill.name().as_str() {
            "ALL_RESURRECTION" if target.is_none_or(|target| target.status.hp > 0 || target.status.has_status_change(StatusChangeKind::HellPower)) => return Err("Resurrection requires a dead target without Hell Power".into()),
            "AL_HEAL" if target.is_some_and(|target| target.status.hp == 0 || target.status.has_status_change(StatusChangeKind::NoRecovery)) => return Err("Target cannot be healed".into()),
            "WZ_ESTIMATION" if target.is_some() => return Err("Monster estimation requires a monster".into()),
            _ => {}
        }
        if pending.keep_requirements { self.requirements_plan(character, skill.id, pending.level, tick)?; }
        if pending.keep_requirements { self.validate_native_environment(state, character, skill.id, pending.level, tick)?; }
        Ok(())
    }

    pub fn validate_skill_requirements(&self, character: &Character, skill_id: u32, level: u8) -> Result<(), String> {
        let skill = skills::skill_enums::to_object(models::enums::skill_enums::SkillEnum::from_id(skill_id), level).ok_or("Skill requirements are unavailable")?;
        let status = StatusService::instance().to_snapshot(&character.status);
        if skill.validate_sp(&status).is_err() { return Err("Not enough SP".into()); }
        if skill.validate_hp(&status).is_err() { return Err("Not enough HP".into()); }
        if skill.validate_weapon(&status).is_err() { return Err("Skill requires a different weapon".into()); }
        if skill.validate_zeny(&status).is_err() { return Err("Not enough zeny".into()); }
        let ammo = character.status.ammo.map(|ammo| (ammo.ammo_type, character.get_item_from_inventory(ammo.inventory_index).map(|item| item.amount.max(0) as u32).unwrap_or(0)));
        if skill.validate_ammo(ammo).is_err() { return Err("Required ammunition is missing".into()); }
        let items = character.inventory_normal().iter().map(|(_, item)| item.to_normal_item()).collect::<Vec<_>>();
        if skill.validate_item(&items).is_err() { return Err("Required skill item is missing".into()); }
        Ok(())
    }

    pub fn queue_target_effect(&self, server: &Server, character: &mut Character, skill: &SkillConfig, effect: ScriptSkillEffect, tick: u128) {
        self.queue_target_effect_with_cast_time(server, character, skill, effect, tick, None);
    }

    pub(super) fn queue_target_effect_with_cast_time(&self, server: &Server, character: &mut Character, skill: &SkillConfig, mut effect: ScriptSkillEffect, tick: u128, cast_time: Option<u128>) {
        if !effect.skill_event_emitted { if let Some(payment) = character.script_skill_state.deferred_requirements.take().filter(|payment| payment.skill_id == effect.skill_id && payment.level == effect.level) { effect.deferred_requirements = Some(payment.requirements); effect.source_index = payment.source_index; effect.source_item = payment.source_item; } }
        let status = StatusService::instance().to_snapshot(&character.status);
        let delay = if effect.skill_event_emitted { 0 } else { cast_time.unwrap_or_else(|| SkillMetadata::find(skill.id).map_or(0, |metadata| metadata.cast_duration(effect.level, StatusService::skill_cast_modifier(&status, skill.id)))) };
        if let ScriptSkillAction::OpenWarpPortalMenu(ref cast) = effect.action {
            if !effect.skill_event_emitted { self.notify_portal_cast(character, &effect, cast, delay); }
        }
        if let ScriptSkillAction::MagicAttack { target_id, .. } = effect.action {
            if !effect.skill_event_emitted {
                character.clear_attack();
                character.movements.clear();
                let mut packet = PacketZcUseskillAck2::new(server.packetver());
                packet.set_target_id(target_id);
                packet.set_aid(character.char_id);
                packet.set_skid(effect.skill_id as u16);
                packet.set_property(12);
                packet.set_delay_time(delay.min(u128::from(u32::MAX)) as u32);
                packet.fill_raw();
                self.notify_area(character, packet.raw);
            }
        }
        if !effect.skill_event_emitted {
            character.script_skill_state.cast_generation = character.script_skill_state.cast_generation.wrapping_add(1);
            effect.cast_generation = character.script_skill_state.cast_generation;
            character.script_skill_state.casting_skill_id = skill.id;
            character.script_skill_state.casting_skill_level = effect.level;
        }
        if !effect.skill_event_emitted {
            character.script_skill_state.casting_until = tick + delay.div_ceil(40).max(1) * 40;
            let base_delay = SkillMetadata::find(skill.id).and_then(|metadata| metadata.after_cast_act_delay.as_ref()?.value(effect.level, "Time")).unwrap_or(0).max(0) as u32;
            let after_cast = StatusService::skill_after_cast_delay(&status, skill.id, base_delay) as u128;
            character.timing.set_canact_tick(tick + delay + after_cast);
            if character.status.has_status_change(StatusChangeKind::Suffragium) { StatusEffectService::end_status_at(&mut character.status, Some(StatusChangeKind::Suffragium), tick); StatusEffectService::send_icon(character, StatusChangeKind::Suffragium, false, tick, &self.client_notification_sender); }
        }
        server.add_to_tick(GameEvent::CharacterScriptSkill(effect), delay.div_ceil(40).max(1).saturating_sub(1).min(usize::MAX as u128) as usize);
    }

    pub fn apply_utility_skill(&self, server: &Server, state: &ServerState, character: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<bool, String> {
        let Some(metadata) = SkillMetadata::find(effect.skill_id) else { return Ok(false); };
        match metadata.name.as_str() {
            "BS_GREED" => self.collect_nearby_items(server, state, character)?,
            "MO_CALLSPIRITS" | "CH_SOULCOLLECT" | "GS_GLITTERING" => {
                character.script_skill_state.expire_spheres(tick);
                let expiry = tick + metadata.duration(effect.level, false).unwrap_or(600000).max(0) as u128;
                match metadata.name.as_str() {
                    "MO_CALLSPIRITS" => character.script_skill_state.add_sphere(expiry, effect.level as usize),
                    "CH_SOULCOLLECT" => for _ in 0..5 { character.script_skill_state.add_sphere(expiry, 5); },
                    _ if fastrand::u32(0..100) < 20 + 10 * effect.level as u32 => character.script_skill_state.add_sphere(expiry, 10),
                    _ => { if !character.script_skill_state.spirit_spheres.is_empty() { character.script_skill_state.spirit_spheres.remove(0); } }
                }
                if metadata.name == "GS_GLITTERING" { character.script_skill_state.coins = character.script_skill_state.spirit_spheres.len() as u8; }
                character.status.spirit_sphere_count = character.script_skill_state.spirit_spheres.len().min(u8::MAX as usize) as u8;
                self.notify_spheres(character);
            }
            "TF_BACKSLIDING" | "TK_HIGHJUMP" => self.execute_movement_skill(server, state, character, effect, tick)?,
            "TK_RUN" => self.toggle_run(server, state, character, effect, tick)?,
            "TK_MISSION" => {
                if !crate::server::service::script_character_service::begin_taekwon_mission(server, character)? { return Err("Taekwon Mission kept the current target".into()); }
            }
            "MC_VENDING" | "MC_PUSHCART" | "AM_CALLHOMUN" | "AM_REST" | "AM_RESURRECTHOMUN" | "WE_CALLPARTNER" | "WE_CALLBABY" | "WE_CALLPARENT" => {
                let request = match metadata.name.as_str() {
                    "MC_VENDING" => crate::server::model::game_systems::ScriptWorldRequest::PrepareVending { skill_level: effect.level },
                    "MC_PUSHCART" => crate::server::model::game_systems::ScriptWorldRequest::SetCart(1),
                    "AM_CALLHOMUN" => crate::server::model::game_systems::ScriptWorldRequest::CallHomunculus,
                    "WE_CALLPARTNER" => crate::server::model::game_systems::ScriptWorldRequest::CallPartner,
                    "WE_CALLBABY" => crate::server::model::game_systems::ScriptWorldRequest::CallBaby,
                    "WE_CALLPARENT" => crate::server::model::game_systems::ScriptWorldRequest::CallParents,
                    "AM_REST" => crate::server::model::game_systems::ScriptWorldRequest::RestHomunculus,
                    _ => crate::server::model::game_systems::ScriptWorldRequest::ResurrectHomunculus { skill_level: effect.level },
                };
                server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld { char_id: character.char_id, request }));
            }
            _ => return Ok(false)
        }
        self.notify_support_skill(character, effect);
        Ok(true)
    }

    pub fn tick_character_state(&self, character: &mut Character, tick: u128) {
        if character.script_skill_state.pending_warp_portal.as_ref().is_some_and(|menu| menu.expires_at <= tick || character.status.hp == 0) {
            self.cancel_warp_portal_menu(character, tick);
        }
        if character.script_skill_state.expire_spheres(tick) { self.notify_spheres(character); }
        character.status.spirit_sphere_count = character.script_skill_state.spirit_spheres.len().min(u8::MAX as usize) as u8;
        if character.script_skill_state.running && (character.status.hp == 0 || character.status.blocks_movement() || (tick > character.script_skill_state.run_started_at + 80 && character.movements.is_empty())) {
            character.script_skill_state.running = false;
            StatusEffectService::end_status(&mut character.status, Some(StatusChangeKind::Run));
        }
    }

    pub(super) fn notify_spheres(&self, character: &Character) {
        let mut packet = 0x01d0_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&(character.script_skill_state.spirit_spheres.len() as u16).to_le_bytes());
        self.notify_area(character, packet);
    }

    pub fn facing_vector(direction: u16) -> (i32, i32) { [(0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)][direction as usize % 8] }

    fn execute_movement_skill(&self, server: &Server, state: &ServerState, character: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<(), String> {
        let metadata = SkillMetadata::find(effect.skill_id).ok_or("Movement skill is unavailable")?;
        let instance = state.get_map_instance_from_character(character).ok_or("Map instance is unavailable")?;
        let (dx, dy) = Self::facing_vector(character.dir);
        let walkable = |x: i32, y: i32| x >= 0 && y >= 0 && x < instance.x_size() as i32 && y < instance.y_size() as i32 && instance.state().cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() != 0;
        let occupied = |x: i32, y: i32| state.characters().values().any(|target| target.char_id != character.char_id && target.map_instance_key == character.map_instance_key && target.x as i32 == x && target.y as i32 == y) || instance.state().mobs().values().any(|mob| mob.status.hp() > 0 && mob.x as i32 == x && mob.y as i32 == y) || instance.state().map_items().values().any(|item| instance.get_script(item.id()).is_some_and(|npc| npc.x() as i32 == x && npc.y() as i32 == y));
        let (mut x, mut y) = (character.x as i32, character.y as i32);
        if metadata.name == "TF_BACKSLIDING" {
            let distance = metadata.knockback.as_ref().and_then(|count| count.value(effect.level, "Amount")).unwrap_or(5);
            for _ in 0..distance { if !walkable(x - dx, y - dy) { break; } x -= dx; y -= dy; }
            if !character.status.has_status_change(StatusChangeKind::Endure) {
                let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Endure, 200, 0); request.flags |= StatusStartFlag::NoIcon.as_flag();
                StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
            }
        } else {
            let distance = if character.dir % 2 == 0 { effect.level as i32 * 2 } else { effect.level as i32 * 4 / 3 };
            let destination = (x + dx * distance, y + dy * distance);
            if !walkable(destination.0, destination.1) || !walkable(destination.0 + dx, destination.1 + dy) || occupied(destination.0, destination.1) || occupied(destination.0 + dx, destination.1 + dy) { return Ok(()); }
            (x, y) = destination;
        }
        character.movements.clear(); character.clear_attack(); character.update_position(x as u16, y as u16); character.last_moved_at = tick;
        let mut packet = 0x01ff_u16.to_le_bytes().to_vec(); packet.extend_from_slice(&character.char_id.to_le_bytes()); packet.extend_from_slice(&(x as u16).to_le_bytes()); packet.extend_from_slice(&(y as u16).to_le_bytes());
        self.notify_area(character, packet);
        Ok(())
    }

    fn toggle_run(&self, server: &Server, state: &ServerState, character: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<(), String> {
        if character.script_skill_state.running {
            character.script_skill_state.running = false;
            character.movements.clear();
            StatusEffectService::end(server, character, Some(StatusChangeKind::Run), tick, &self.client_notification_sender);
            if effect.level >= 7 && tick.saturating_sub(character.script_skill_state.run_started_at) <= 1000 && character.status.weapons.is_empty() {
                StatusEffectService::start(server, character, StatusChangeRequest::guaranteed(StatusChangeKind::Spurt, SkillMetadata::find(effect.skill_id).and_then(|skill| skill.duration(effect.level, true)).unwrap_or(150000), effect.level as i32), tick, &self.client_notification_sender)?;
            }
            return Ok(());
        }
        let instance = state.get_map_instance_from_character(character).ok_or("Map instance is unavailable")?;
        let (dx, dy) = Self::facing_vector(character.dir);
        let mut path = vec![];
        let (mut x, mut y) = (character.x as i32, character.y as i32);
        for _ in 0..instance.x_size().max(instance.y_size()) {
            x += dx; y += dy;
            if x < 0 || y < 0 || x >= instance.x_size() as i32 || y >= instance.y_size() as i32 || instance.state().cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() == 0 || instance.state().mobs().values().any(|mob| mob.status.hp() > 0 && mob.x as i32 == x && mob.y as i32 == y) { break; }
            path.push(PathNode { id: 0, parent_id: 0, x: x as u16, y: y as u16, g_cost: 0, f_cost: 0, is_open: false, is_diagonal: dx != 0 && dy != 0 });
        }
        let Some(destination) = path.last().map(|node| Position { x: node.x, y: node.y, dir: character.dir }) else { return Ok(()); };
        let movements = Movement::from_path(path, tick);
        StatusEffectService::start(server, character, StatusChangeRequest::guaranteed(StatusChangeKind::Run, -1, effect.level as i32), tick, &self.client_notification_sender)?;
        character.script_skill_state.running = true; character.script_skill_state.run_started_at = tick; character.script_skill_state.run_level = effect.level;
        server.add_to_next_movement_tick(GameEvent::CharacterMove(CharacterMovement { char_id: character.char_id, start_at: tick, destination, current_position: Position { x: character.x, y: character.y, dir: character.dir }, path: movements, cancel_attack: true }));
        Ok(())
    }

    pub fn show_monster_estimation(&self, _server: &Server, state: &ServerState, character: &Character, target_id: u32, level: u8) -> Result<(), String> {
        let instance = state.get_map_instance_from_character(character).ok_or("Map instance is unavailable")?;
        let map_state = instance.state();
        let mob = map_state.get_mob(target_id).ok_or("Monster estimation requires a monster")?;
        let status = &mob.status;
        let mut packet = 0x018c_u16.to_le_bytes().to_vec();
        for value in [mob.mob_id as u16, mob.status_effects.base_level.min(u16::MAX as u32) as u16, status.size().value() as u16] { packet.extend_from_slice(&value.to_le_bytes()); }
        packet.extend_from_slice(&status.hp().to_le_bytes()); packet.extend_from_slice(&status.def().to_le_bytes());
        let race = match status.race() { MobRace::Formless => 0, MobRace::RUndead => 1, MobRace::Brute => 2, MobRace::Plant => 3, MobRace::Insect => 4, MobRace::Fish => 5, MobRace::Demon => 6, MobRace::DemiHuman | MobRace::PlayerHuman => 7, MobRace::Angel => 8, MobRace::Dragon => 9, _ => 0 };
        packet.extend_from_slice(&(race as u16).to_le_bytes()); packet.extend_from_slice(&status.mdef().to_le_bytes()); packet.extend_from_slice(&(status.element().value() as u16).to_le_bytes());
        for element in [Element::Water, Element::Earth, Element::Fire, Element::Wind, Element::Poison, Element::Holy, Element::Dark, Element::Ghost, Element::Undead] { packet.push((BattleService::element_modifier(&element, status) * 100.0).clamp(0.0, 255.0) as u8); }
        self.client_notification_sender.try_send(Notification::Char(CharNotification::new(character.char_id, packet.clone()))).map_err(|error| error.to_string())?;
        if character.game_systems.party_id != 0 { for target in state.characters().values().filter(|target| target.char_id != character.char_id && target.map_instance_key == character.map_instance_key && target.game_systems.party_id == character.game_systems.party_id) { let _ = self.client_notification_sender.try_send(Notification::Char(CharNotification::new(target.char_id, packet.clone()))); } }
        self.notify_support_skill(character, &ScriptSkillEffect { source_char_id: character.char_id, target_id, skill_id: models::enums::skill_enums::SkillEnum::WzEstimation.id(), level, heal_value: 0, proc_depth: 0, skill_event_emitted: true, cast_generation: 0, action: super::ScriptSkillAction::Cast, deferred_requirements: None, prepared_outcome: None, source_index: None, source_item: None });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spirit_spheres_replace_the_oldest_timer_and_expire_independently() {
        let mut state = ScriptSkillState::default();
        state.add_sphere(100, 2); state.add_sphere(300, 2); state.add_sphere(200, 2);
        assert_eq!(state.spirit_spheres, vec![200, 300]);
        assert!(!state.expire_spheres(199));
        assert!(state.expire_spheres(200));
        assert_eq!(state.spirit_spheres, vec![300]);
    }

    #[test]
    fn skill_movement_uses_classic_client_direction_order() {
        assert_eq!(ScriptSkillService::facing_vector(0), (0, 1));
        assert_eq!(ScriptSkillService::facing_vector(2), (-1, 0));
        assert_eq!(ScriptSkillService::facing_vector(4), (0, -1));
        assert_eq!(ScriptSkillService::facing_vector(7), (1, 1));
    }
}
