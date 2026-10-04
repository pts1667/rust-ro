use std::sync::Arc;
use std::sync::atomic::Ordering;

use models::enums::script::BroadcastFlag;
use models::enums::EnumWithMaskValueU32;
use script_sdk::{Value, Variable, VariableScope};

use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptEvent, ScriptWarp};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::script::Script;
use crate::server::model::movement::Movable;
use crate::server::script::NpcScriptHost;
use crate::server::service::{map_combat_service, script_combat_service};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::CharacterUseSkill;
use crate::server::script::skill::{PendingItemSkill, ScriptSkillAction, ScriptSkillService};
use models::status_bonus::{BattleFlag, CombatTrigger};
use models::enums::bonus::BonusType;
use crate::server::state::server::ServerState;
use crate::server::Server;

fn devotion_protector(state: &ServerState, protected: &crate::server::state::character::Character, status: &models::status_change::StatusChange, tick: u128) -> Option<u32> {
    if status.expired(tick) || !(0..5).contains(&status.values[1]) { return None; }
    let id = u32::try_from(status.values[0]).ok().filter(|id| *id != protected.char_id)?;
    let range = u16::try_from(status.values[2]).ok()?;
    if let Some(protector) = state.characters().get(&id) {
        return (!protector.is_dead() && protector.status.hp > 0 && protector.map_instance_key == protected.map_instance_key
            && protected.x.abs_diff(protector.x).max(protected.y.abs_diff(protector.y)) <= range).then_some(id);
    }
    let mercenary = protected.game_systems.mercenary.as_ref().filter(|mercenary| super::script_world_service::mercenary_world_id(mercenary) == id && mercenary.hp > 0)?;
    let snapshot = super::script_world_service::companion_snapshots(protected).into_iter().find(|snapshot| snapshot.map_item().id() == super::script_world_service::mercenary_world_id(mercenary))?;
    (protected.x.abs_diff(snapshot.x()).max(protected.y.abs_diff(snapshot.y())) <= range).then_some(id)
}

impl Server {
    pub(crate) fn player_target_allowed(&self, state: &ServerState, source: &crate::server::state::character::Character, target_id: u32,
        mode: super::visibility_service::TargetingMode) -> bool {
        use super::visibility_service::{StealthState, VisibilityObserver, can_target};
        if target_id == source.char_id { return true; }
        let stealth = if let Some(target) = state.characters().get(&target_id).filter(|target| target.map_instance_key == source.map_instance_key) {
            Some(StealthState::from_status_options(&target.status, target.options))
        } else if let Some(snapshot) = super::script_world_service::companion_status_snapshot(source, target_id) {
            Some(StealthState::from_snapshot(&snapshot))
        } else if let Some(instance) = state.get_map_instance_from_character(source) {
            instance.state().get_mob(target_id).map(|mob| StealthState::from_status(&mob.status_effects))
                .or_else(|| state.characters().values().filter(|owner| owner.map_instance_key == source.map_instance_key)
                    .find_map(|owner| super::script_world_service::companion_status_snapshot(owner, target_id).map(|snapshot| StealthState::from_snapshot(&snapshot))))
        } else { None };
        stealth.is_some_and(|stealth| can_target(VisibilityObserver::player(&StatusService::instance().to_snapshot(&source.status)), stealth, mode))
    }

    pub(crate) fn player_skill_target_allowed(&self, state: &ServerState, source: &crate::server::state::character::Character, target_id: u32, skill_id: u32, completed: bool) -> bool {
        use super::visibility_service::TargetingMode;
        use models::enums::skill_enums::SkillEnum;
        if skill_id == SkillEnum::SaDispell.id() {
            let own_party = state.characters().get(&target_id).is_some_and(|target| source.game_systems.party_id > 0 && source.game_systems.party_id == target.game_systems.party_id && source.map_instance_key == target.map_instance_key);
            if target_id != source.char_id && !own_party && !self.player_combat_target_allowed(state, source, target_id) { return false; }
        } else if Self::player_skill_requires_hostile_target(skill_id) && !self.player_combat_target_allowed(state, source, target_id) { return false; }
        if [SkillEnum::AlHeal.id(), SkillEnum::AllResurrection.id(), SkillEnum::PrAspersio.id()].contains(&skill_id) {
            let target = self.server_service().get_target_status(state, source, Some(target_id), 0)
                .or_else(|| super::script_world_service::companion_status_snapshot(source, target_id));
            let undead = target.is_some_and(|target| *target.race() == models::enums::mob::MobRace::RUndead || *target.element() == models::enums::element::Element::Undead);
            if undead && !self.player_combat_target_allowed(state, source, target_id) { return false; }
        }
        let hidden = crate::server::script::skill::metadata::SkillMetadata::find(skill_id).is_some_and(|skill| skill.flags.get("TargetHidden").copied().unwrap_or(false));
        let mode = if completed { TargetingMode::SkillCompletion { can_hit_hidden: hidden } } else if hidden { TargetingMode::DirectHiddenSkill } else { TargetingMode::Direct };
        self.player_target_allowed(state, source, target_id, mode)
    }

    pub(crate) fn refresh_forged_rank(&self, character: &mut crate::server::state::character::Character) {
        if let Ok(rankings) = self.repository.fame_rankings(crate::repository::fame_repository::FameCategory::Blacksmith) {
            let creators: Vec<_> = rankings.iter().map(|entry| entry.char_id).collect();
            crate::repository::fame_repository::refresh_forged_rank(&mut character.status, &creators);
        }
    }

    pub(crate) fn refresh_taekwon_rank(&self, character: &mut crate::server::state::character::Character) {
        if let Ok(rankings) = self.repository.fame_rankings(crate::repository::fame_repository::FameCategory::Taekwon) {
            let ranked: Vec<_> = rankings.iter().map(|entry| entry.char_id).collect();
            match self.update_taekwon_rank(character, &ranked) {
                Ok(true) => self.skill_tree_service().send_skill_tree(character),
                Ok(false) => {},
                Err(error) => warn!("Taekwon rank refresh failed: {error}"),
            }
        }
    }

    fn update_taekwon_rank(&self, character: &mut crate::server::state::character::Character, ranked: &[u32]) -> Result<bool, String> {
        let previous = character.status.taekwon_ranked;
        if !super::script_character_service::refresh_rank_status(character, ranked) { return Ok(false); }
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let (max_hp, max_sp) = (snapshot.max_hp(), snapshot.max_sp());
        let (hp, sp) = (character.status.hp.min(max_hp), character.status.sp.min(max_sp));
        let change = crate::repository::script_inventory_repository::ScriptInventoryTransaction {
            char_id: character.char_id, account_id: character.account_id, consumption: None,
            exact_removals: vec![], removals: vec![], identifications: vec![], grants: vec![], variables: vec![],
            zeny: None, hp: None, sp: None, max_weight: self.character_service().max_weight(character), max_slots: 100,
            world: None, reset_skills: None, fame: None, pool_draws: vec![],
            character_changes: vec![crate::repository::script_inventory_repository::ScriptCharacterChange::ResourcePools { hp, sp, max_hp, max_sp }],
        };
        if let Err(error) = self.repository.script_inventory_transaction(&change) {
            character.status.taekwon_ranked = previous;
            return Err(error.to_string());
        }
        character.status.max_hp = max_hp;
        character.status.max_sp = max_sp;
        if character.status.hp != hp || character.status.sp != sp { self.character_service().update_hp_sp(character, hp, sp); }
        Ok(true)
    }

    pub(crate) fn handle_script_event(&self, state: &mut ServerState, event: GameEvent, tick: u128) -> Result<(), String> {
        match event {
            GameEvent::ScriptCombat(request) => script_combat_service::handle(self, state, request, tick),
            GameEvent::NpcContact(contact) => self.start_npc_conversation(state, contact)?,
            GameEvent::ItemScriptComplete(completion) => self.item_service().complete_item_dialog(self, state, completion)?,
            GameEvent::ScriptTeleportSelection(selection) => {
                let Some(mut character)=state.characters_mut().remove(&selection.char_id) else {return Ok(());};
                let result=self.script_skill_service().finish_teleport_menu(self,state,&mut character,&selection,tick);
                state.insert_character(character);
                result?;
            },
            GameEvent::ScriptReveal(actor) => self.script_skill_service().reveal_from_actor(self, state, &actor, tick)?,
            GameEvent::CharacterKnockback(knockback) => {
                if let Some(mut character) = state.characters_mut().remove(&knockback.char_id) {
                    let result = self.script_skill_service().apply_knockback(self, state, &mut character, knockback.source_x, knockback.source_y, knockback.cells, tick);
                    state.insert_character(character);
                    result?;
                }
            }
            GameEvent::FameChanged(update) => {
                for character in state.characters_mut().values_mut() {
                    let changed = match update.category {
                        crate::repository::fame_repository::FameCategory::Blacksmith => crate::repository::fame_repository::refresh_forged_rank(&mut character.status, &update.ranked_creators),
                        crate::repository::fame_repository::FameCategory::Taekwon => {
                            let changed = match self.update_taekwon_rank(character, &update.ranked_creators) {
                                Ok(changed) => changed,
                                Err(error) => { warn!("Taekwon rank event failed: {error}"); continue; }
                            };
                            if changed { self.skill_tree_service().send_skill_tree(character); }
                            changed
                        }
                        crate::repository::fame_repository::FameCategory::Alchemist => false,
                    };
                    if changed {
                        self.character_service().reload_client_side_status(character);
                        character.refresh_script_context();
                    }
                }
            }
            GameEvent::TaekwonMissionKill(credit) => {
                if let Some(mut character) = state.characters_mut().remove(&credit.char_id) {
                    let result = super::script_character_service::record_mission_kill(self, &mut character, credit.mob_id);
                    state.insert_character(character);
                    result?;
                }
            }
            GameEvent::ScriptSkillHit(hit) => self.script_skill_service().after_skill_damage(self, state, hit, tick)?,
            GameEvent::MobAttack(request) => map_combat_service::handle(self, state, request, tick),
            GameEvent::ReflectMagic(request) => map_combat_service::reflect_magic(self, state, request, tick)?,
            GameEvent::ScriptUnitSkill(mut request) => {
                super::script_unit_skill_service::normalize_unit_skill_actor_ids(state, &mut request);
                self.script_skill_service().cast_script_unit_skill(self, state, request, tick)?;
            }
            GameEvent::ScriptActorSkillComplete(completion) => self.script_skill_service().finish_script_actor_skill(self, state, completion, tick)?,
            GameEvent::ScriptSpawned(crate::server::model::events::game_event::ScriptSpawned { char_id, mob_ids }) => {
                let variables: Vec<_> = mob_ids.into_iter().enumerate().map(|(index, id)| Variable { scope: VariableScope::ServerTemporary, name: "mobid".into(), index: index as u32, value: (id as i32).into() }).collect();
                self.script_service().install_temporary_variables(char_id, &variables);
            }
            GameEvent::ScriptEvent(event) => self.run_compiled_event(state, event)?,
            GameEvent::ScriptSpawn(event) => {
                let source = state.characters().get(&event.char_id).ok_or("Monster source disconnected")?;
                let map = GlobalConfigService::instance().find_map(&event.map).ok_or("Monster map is unavailable")?;
                let instance_id = if Map::name_without_ext(source.current_map_name()) == event.map { source.current_map_instance() } else { 0 };
                let instance = state.get_map_instance(&event.map, instance_id).unwrap_or_else(|| self.server_service().create_map_instance(state, map, instance_id));
                instance.add_to_next_tick(MapEvent::ScriptSpawn(event.request));
            }
            GameEvent::ScriptPartyWarp(event) => {
                for warp in self.plan_party_warp(state, &event)? { self.add_to_next_tick(GameEvent::ScriptWarp(warp)); }
            }
            GameEvent::ScriptBroadcast(event) => {
                let source = state.characters().get(&event.char_id).ok_or("Announcement source disconnected")?;
                let scope = event.flags & (BroadcastFlag::Map.as_flag() | BroadcastFlag::Area.as_flag() | BroadcastFlag::ReservedTarget.as_flag());
                let recipients: Vec<_> = state.characters().values().filter(|character| match scope {
                    0 => true,
                    1 => character.map_instance_key == source.map_instance_key,
                    2 => character.map_instance_key == source.map_instance_key && character.x().abs_diff(source.x()) <= crate::server::PLAYER_FOV && character.y().abs_diff(source.y()) <= crate::server::PLAYER_FOV,
                    3 => character.char_id == source.char_id,
                    _ => false,
                }).map(|character| character.char_id).collect();
                let sender = self.server_service().notification_sender();
                for char_id in recipients { sender.send(Notification::Char(CharNotification::new(char_id, event.packet.clone()))).map_err(|error| error.to_string())?; }
            }
            GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange { char_id, request }) => {
                if self.script_world_service().handle_companion_status_change(self, state, char_id, request.clone(), tick)? { return Ok(()); }
                if let Some(mut character) = state.characters_mut().remove(&char_id) {
                    let result = StatusEffectService::start(self, &mut character, request, tick, &self.server_service().notification_sender());
                    state.insert_character(character); result?;
                } else {
                    let instance = state.map_instances().values().flatten().find(|instance| instance.state().get_mob(char_id).is_some()).ok_or("Status target is no longer on a map")?;
                    instance.add_to_next_tick(MapEvent::MobStatusChange { mob_id: char_id, request });
                }
            }
            GameEvent::CharacterStatusAlternatives(crate::server::model::events::game_event::CharacterStatusAlternatives {char_id,requests}) => {
                if self.script_world_service().handle_companion_status_alternatives(self,state,char_id,requests.clone(),tick)? {return Ok(());}
                if let Some(mut character)=state.characters_mut().remove(&char_id) {
                    let result=StatusEffectService::start_alternatives(self,&mut character,requests,tick,&self.server_service().notification_sender());
                    state.insert_character(character);
                    result?;
                } else {
                    let instance=state.map_instances().values().flatten().find(|instance|instance.state().get_mob(char_id).is_some()).ok_or("Status target is no longer on a map")?;
                    instance.add_to_next_tick(MapEvent::MobStatusAlternatives(crate::server::model::events::map_event::MobStatusAlternatives {mob_id:char_id,requests}));
                }
            },
            GameEvent::CharacterEndStatus(crate::server::model::events::game_event::CharacterEndStatus { char_id, kind }) => {
                if self.script_world_service().handle_companion_end_status(self, state, char_id, kind, tick)? { return Ok(()); }
                if let Some(mut character) = state.characters_mut().remove(&char_id) {
                    StatusEffectService::end(self, &mut character, kind, tick, &self.server_service().notification_sender()); state.insert_character(character);
                } else {
                    let instance = state.map_instances().values().flatten().find(|instance| instance.state().get_mob(char_id).is_some()).ok_or("Status target is no longer on a map")?;
                    instance.add_to_next_tick(MapEvent::MobEndStatus { mob_id: char_id, kind });
                }
            }
            GameEvent::CharacterScriptSkill(mut effect) => {
                let source = state.characters().get(&effect.source_char_id).ok_or("Skill caster disconnected")?;
                self.script_skill_service().validate_effect_cast(source, &effect)?;
                self.script_skill_service().validate_effect_target(state, source, &effect, tick)?;
                let internal_action = matches!(effect.action, ScriptSkillAction::OpenTeleportMenu | ScriptSkillAction::SetResources { .. } | ScriptSkillAction::ActivateGround { .. }
                    | ScriptSkillAction::ExplodeSplasher | ScriptSkillAction::WaterBall { .. } | ScriptSkillAction::AreaStatus { .. }
                    | ScriptSkillAction::Face { .. } | ScriptSkillAction::FinalStrike { .. } | ScriptSkillAction::DelayedWeaponHit { .. } | ScriptSkillAction::SnatchWarp { .. });
                if !internal_action && !self.player_skill_target_allowed(state, source, effect.target_id, effect.skill_id, true) { return Err("Skill target is hidden or unavailable".into()); }
                let (target_status, snapshot, immune) = if let Some(target) = state.characters().get(&effect.target_id) {
                    (target.status.clone(), StatusService::instance().to_snapshot(&target.status), ScriptSkillService::conditional_player_immunity(target, &effect))
                } else {
                    let instance = state.get_map_instance_from_character(source).ok_or("Skill map is unavailable")?;
                    let instance_state = instance.state();
                    let target = instance_state.get_mob(effect.target_id).ok_or("Skill target is unavailable")?;
                    (target.status_effects.clone(), target.status.clone(), ScriptSkillService::conditional_mob_immunity(target, &effect))
                };
                let (mut cost, conditional) = ScriptSkillService::partition_skill_requirements(effect.skill_id, effect.level, effect.deferred_requirements.take().unwrap_or_default());
                effect.deferred_requirements = Some(conditional);
                let completion = self.script_skill_service().prepare_conditional_completion(&effect, &target_status, &snapshot, immune, tick)?;
                cost.hp = cost.hp.checked_add(completion.cost.hp).ok_or("Skill HP cost overflow")?;
                cost.sp = cost.sp.checked_add(completion.cost.sp).ok_or("Skill SP cost overflow")?;
                cost.zeny = cost.zeny.checked_add(completion.cost.zeny).ok_or("Skill zeny cost overflow")?;
                cost.spirit_spheres = cost.spirit_spheres.checked_add(completion.cost.spirit_spheres).ok_or("Skill sphere cost overflow")?;
                cost.removals.extend(completion.cost.removals);
                effect = completion.effect;
                let mut caster = state.characters_mut().remove(&effect.source_char_id).ok_or("Skill caster disconnected")?;
                let paid = (|| {
                    if let Some(identity) = effect.source_item {
                        let item = effect.source_index.and_then(|index| caster.get_item_from_inventory(index)).ok_or("Delayed source is unavailable")?;
                        if (item.id, item.item_id, item.unique_id) != identity { return Err("Delayed consumable identity changed".into()); }
                    }
                    self.item_service().pay_requirement_plan_in_state(self, state, &mut caster, &cost, effect.source_index, tick)
                })();
                if !effect.skill_event_emitted {
                    caster.script_skill_state.casting_until = 0;
                    caster.script_skill_state.casting_skill_id = 0;
                }
                state.insert_character(caster);
                paid?;
                if !completion.succeeded { return Ok(()); }
                if let Some(mut character) = state.characters_mut().remove(&effect.target_id) {
                    let result = self.script_skill_service().apply_target_effect(self, state, &mut character, &effect, tick);
                    state.insert_character(character); result?;
                } else {
                    let mut caster = state.characters_mut().remove(&effect.source_char_id).ok_or("Skill caster disconnected")?;
                    let result = self.script_skill_service().apply_mob_target_effect(self, state, &mut caster, &effect, tick);
                    state.insert_character(caster); result?;
                }
                if !effect.skill_event_emitted {
                    let battle_flags = crate::server::script::skill::metadata::SkillMetadata::find(effect.skill_id).map_or(
                        BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
                        |metadata| metadata.battle_flags(metadata.damage_type.as_deref() == Some("Magic") || metadata.range(effect.level).is_some_and(|range| range > 3)));
                    self.add_to_next_tick(GameEvent::ScriptCombat(script_combat_service::ScriptCombatRequest { source_id: effect.source_char_id, target_id: effect.target_id,
                        trigger: CombatTrigger::Skill, battle_flags,
                        skill_id: effect.skill_id, damage: 0, other_mob_id: None, depth: effect.proc_depth, drop_position: None, right_hand_damage: None,
                        origin_map: state.characters().get(&effect.source_char_id).map(|source| source.map_instance_key.clone()) }));
                }
            }
            GameEvent::ScriptCraft(selection) => {
                let mut character = state.characters_mut().remove(&selection.char_id).ok_or("Crafter disconnected")?;
                let result = self.item_service().make_item(self, &mut character, selection, tick);
                state.insert_character(character); result?;
            }
            GameEvent::ScriptIdentify(selection) => {
                let mut character = state.characters_mut().remove(&selection.char_id).ok_or("Identification source disconnected")?;
                let result = self.item_service().identify_item_in_state(self, state, &mut character, selection.index, tick);
                state.insert_character(character); result?;
            }
            GameEvent::CharacterUseGroundSkill(event) => {
                if state.get_character(event.char_id).is_some_and(|character| character.game_systems.is_trading()) { return Err("Skills cannot be used while trading".into()); }
                if (8001..=8016).contains(&event.skill_id) || (8201..=8240).contains(&event.skill_id) {
                    self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld { char_id: event.char_id, request: crate::server::model::game_systems::ScriptWorldRequest::UseCompanionGroundSkill {
                        skill_id: event.skill_id, skill_level: event.skill_level, x: event.x, y: event.y } }));
                    return Ok(());
                }
                let mut character = state.characters_mut().remove(&event.char_id).ok_or("Ground skill source disconnected")?;
                let result: Result<(), String> = (|| {
                    let pending = character.pending_item_skill.clone();
                    if let Some(pending) = pending {
                        self.script_skill_service().validate_pending_ground(state, &character, event.skill_id, event.skill_level, event.x, event.y, tick)?;
                        self.item_service().defer_skill_requirements_in_state(self, state, &mut character, event.skill_id, event.skill_level, tick, pending.keep_requirements, pending.item_index)?;
                        if let Some(pending) = character.pending_item_skill.as_mut() { pending.keep_requirements = false; pending.item_index = None; }
                        self.script_skill_service().cast_pending_ground(self, state, &mut character, event.skill_id, event.skill_level, event.x, event.y, tick)?;
                    } else {
                        let learned = StatusService::instance().to_snapshot(&character.status).known_skills().iter().find(|skill| skill.value.id() == event.skill_id).map_or(0, |skill| skill.level);
                        if event.skill_level == 0 || event.skill_level > learned { return Err("Ground skill level is not learned".into()); }
                        self.script_skill_service().validate_ground_target(state, &character, event.skill_id, event.skill_level, event.x, event.y, tick)?;
                        self.item_service().defer_skill_requirements_in_state(self, state, &mut character, event.skill_id, event.skill_level, tick, true, None)?;
                        self.script_skill_service().place_ground_skill(self, state, &mut character, event.skill_id, event.skill_level, event.x, event.y, tick)?;
                    }
                    Ok(())
                })();
                state.insert_character(character); result?;
            }
            _ => return Err("Unexpected script event".into()),
        }
        Ok(())
    }

    fn run_compiled_event(&self, state: &ServerState, event: ScriptEvent) -> Result<(), String> {
        let character = state.characters().get(&event.char_id).ok_or("Event player disconnected")?;
        let session = state.find_session(character.account_id).ok_or("Event session expired")?;
        let server = self.shared().ok_or("Server runtime is not bound")?;
        let (_, inputs) = tokio::sync::mpsc::channel(1);
        let host = NpcScriptHost { server, generation: session.script_generation.load(Ordering::Acquire), background: true, session,
            script: Arc::new(Script { id: 0, entry_id: event.entry_id, map_name: character.current_map_name().clone(), name: "CompiledEvent".into(), sprite: 0,
                x: character.x(), y: character.y(), dir: 0, x_size: 0, y_size: 0, constructor_args: event.args }), inputs,
            notifications: self.server_service().notification_sender(), map_instance: character.current_map_instance(), error: None };
        let vm = self.script_service().vm.clone();
        self.runtime().spawn(async move {
            let (host, result) = vm.execute(host, "run_event", event.entry_id).await;
            if let Err(error) = result { warn!("Compiled event failed: {}", host.error.unwrap_or(error)); }
        });
        Ok(())
    }

    pub(crate) fn handle_character_skill(&self, state: &mut ServerState, event: CharacterUseSkill, tick: u128) -> Result<(), String> {
        let mut character = state.characters_mut().remove(&event.char_id).ok_or("Skill source disconnected")?;
        let previous_pending = character.pending_item_skill.clone();
        let result = (|| {
            if character.is_dead() || character.status.blocks_casting() || character.game_systems.is_trading() || character.game_systems.buying_store.is_some() || character.game_systems.vending_store.is_some() { return Err("Character cannot use skills now".into()); }
            if (8001..=8016).contains(&event.skill_id) || (8201..=8240).contains(&event.skill_id) {
                self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld { char_id: character.char_id,
                    request: crate::server::model::game_systems::ScriptWorldRequest::UseCompanionSkill { skill_id: event.skill_id, skill_level: event.skill_level, target_id: event.target_id } }));
                return Ok(());
            }
            if let Some(pending) = character.pending_item_skill.clone() {
                if !self.player_skill_target_allowed(state, &character, event.target_id, event.skill_id, false) { return Err("Skill target is hidden or unavailable".into()); }
                self.script_skill_service().validate_pending_cast(state, &character, &event, tick)?;
                self.item_service().defer_skill_requirements_in_state(self, state, &mut character, event.skill_id, event.skill_level, tick, pending.keep_requirements, pending.item_index)?;
                if let Some(pending) = character.pending_item_skill.as_mut() { pending.keep_requirements = false; pending.item_index = None; }
                self.script_skill_service().cast_pending(self, state, &mut character, &event, tick)?;
                return Ok(());
            }
            let learned = StatusService::instance().to_snapshot(&character.status).known_skills().iter().find(|skill| skill.value.id() == event.skill_id).map_or(0, |skill| skill.level);
            if event.skill_level == 0 || event.skill_level > learned { return Err("Requested skill level is not learned".into()); }
            let configuration = GlobalConfigService::instance();
            let skill = configuration.find_skill_config(&(event.skill_id as i32).into()).ok_or("Unknown skill")?;
            if skill.name() == "MC_VENDING" || skill.name() == "MC_PUSHCART" {
                self.item_service().pay_skill_requirements(self, &mut character, event.skill_id, event.skill_level, tick, true, None)?;
                let request = if skill.name() == "MC_VENDING" { crate::server::model::game_systems::ScriptWorldRequest::PrepareVending { skill_level: event.skill_level } }
                    else { crate::server::model::game_systems::ScriptWorldRequest::SetCart(1) };
                self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld { char_id: character.char_id, request }));
                return Ok(());
            }
            self.script_skill_service().validate_skill(skill, u32::from(event.skill_level))?;
            if !self.player_skill_target_allowed(state, &character, event.target_id, event.skill_id, false) { return Err("Skill target is hidden or unavailable".into()); }
            if ScriptSkillService::operation(skill.name()).is_some_and(|operation| operation != crate::server::script::skill::callbacks::SkillOperation::Damage) {
                if skill.name() == "MC_IDENTIFY" {
                    self.item_service().defer_skill_requirements_in_state(self, state, &mut character, event.skill_id, event.skill_level, tick, true, None)?;
                    return self.script_skill_service().handle_skill(self, &mut character, skill, u32::from(event.skill_level), false);
                }
                if skill.name() == "AL_TELEPORT" {
                    return self.script_skill_service().start_native_teleport_menu(self,state,&mut character,event.skill_level,tick);
                }
                character.pending_item_skill = Some(PendingItemSkill { skill_id: event.skill_id, level: event.skill_level, keep_requirements: true, item_index: None, source_item: None, expires_at: tick + 120_000 });
                self.script_skill_service().validate_pending_cast(state, &character, &event, tick)?;
                self.item_service().defer_skill_requirements_in_state(self, state, &mut character, event.skill_id, event.skill_level, tick, true, None)?;
                if let Some(pending) = character.pending_item_skill.as_mut() { pending.keep_requirements = false; }
                self.script_skill_service().cast_pending(self, state, &mut character, &event, tick)?;
            } else {
                self.server_service().character_start_use_skill(self, state, &mut character, event, tick);
            }
            Ok(())
        })();
        if result.is_err() && previous_pending.is_none() { character.pending_item_skill = None; character.script_skill_state.deferred_requirements = None; }
        state.insert_character(character);
        result
    }

    pub(crate) fn admit_character_damage(&self, state: &mut ServerState, mut damage: Damage, tick: u128) -> Result<(), String> {
        if damage.landed && damage.attacker_id != damage.target_id {
            let owner = state.characters().get(&damage.attacker_id).or_else(|| state.characters().values().find(|owner| super::script_world_service::companion_status_snapshot(owner, damage.attacker_id).is_some()));
            if owner.is_some_and(|owner| !self.player_combat_target_allowed(state, owner, damage.target_id)) { return Ok(()); }
        }
        let original_damage = damage;
        if self.script_world_service().handle_companion_damage(self, state, damage, tick)? { return Ok(()); }
        let Some(target) = state.characters().get(&damage.target_id) else { return Ok(()); };
        if target.is_dead() || target.status.hp == 0 { return Ok(()); }
        if damage.healing > 0 {
            let hp = target.status.hp.saturating_add(damage.healing).min(StatusService::instance().to_snapshot(&target.status).max_hp());
            let sp = target.status.sp;
            self.repository.script_inventory_transaction(&crate::repository::script_inventory_repository::ScriptInventoryTransaction {
                char_id: target.char_id, account_id: target.account_id, consumption: None, exact_removals: vec![],
                removals: vec![], identifications: vec![], grants: vec![], variables: vec![], zeny: None,
                hp: Some(hp), sp: None, max_weight: self.character_service().max_weight(target),
                max_slots: usize::from(GlobalConfigService::instance().config().game.max_inventory),
                world: None, reset_skills: None, fame: None, character_changes: vec![], pool_draws: vec![],
            }).map_err(|error| error.to_string())?;
            let target = state.characters_mut().get_mut(&damage.target_id).unwrap();
            self.character_service().update_hp_sp(target, hp, sp);
            return Ok(());
        }
        if damage.damage == 0 { return Ok(()); }
        let mut target_snapshot = StatusService::instance().to_snapshot(&target.status);
        if damage.landed && damage.proc_depth < 8 {
            if let Some(kind) = super::combat_trigger_service::magic_reflection(&target_snapshot, damage.battle_flags, damage.skill_id, &mut fastrand::Rng::new()) {
                let request = map_combat_service::MagicReflectionRequest { damage, reflector_id: target.char_id, reflector_credit_id: target.char_id, kind, map_key: target.map_instance_key.clone() };
                return map_combat_service::reflect_magic(self, state, request, tick);
            }
        }
        let magic_rod = target.status.status_change(models::status_change::StatusChangeKind::MagicRod).is_some_and(|change| !change.expired(tick));
        if magic_rod {
            let mut character = state.characters_mut().remove(&damage.target_id).unwrap();
            let absorbed = StatusEffectService::absorb_magic_rod(&mut character.status, damage.battle_flags, damage.skill_id, damage.skill_level).is_some();
            if absorbed {
                let (hp, sp) = (character.status.hp, character.status.sp);
                self.character_service().update_hp_sp(&mut character, hp, sp);
                self.script_skill_service().notify_magic_rod_absorption(&character, damage.skill_level);
            }
            state.insert_character(character);
            if absorbed { return Ok(()); }
        }
        let target = state.characters().get(&damage.target_id).unwrap();
        let source_player = state.characters().contains_key(&damage.attacker_id);
        let other_mob_id = state.get_map_instance_from_character(target).and_then(|instance| instance.state().get_mob(damage.attacker_id).map(|mob| mob.mob_id as u32));
        let devotion = if damage.battle_flags != 0 && damage.skill_id != models::enums::skill_enums::SkillEnum::PaPressure.id()
            && damage.skill_id != models::enums::skill_enums::SkillEnum::CrReflectshield.id()
            && damage.skill_id != models::enums::skill_enums::SkillEnum::PaPressure.id() {
            target.status.status_change(models::status_change::StatusChangeKind::Devotion).map(|change| devotion_protector(state, target, change, tick))
        } else { None };
        let mut character = state.characters_mut().remove(&damage.target_id).unwrap();
        if devotion == Some(None) {
            StatusEffectService::end(self, &mut character, Some(models::status_change::StatusChangeKind::Devotion), tick, &self.server_service().notification_sender());
            target_snapshot = StatusService::instance().to_snapshot(&character.status);
        }
        if !damage.defenses_applied {
            damage.damage = if damage.battle_flags & BattleFlag::Weapon.as_flag() != 0 {
                (damage.damage as f32 * (1.0 - target_snapshot.def().clamp(0, 100) as f32 / 100.0) - f32::from(target_snapshot.vit())).max(1.0) as u32
            } else if damage.battle_flags & BattleFlag::Magic.as_flag() != 0 {
                (damage.damage as f32 * (1.0 - target_snapshot.mdef().clamp(0, 100) as f32 / 100.0) - f32::from(target_snapshot.int())).max(1.0) as u32
            } else { damage.damage };
        }
        let statuses: Vec<_> = character.status.active_statuses.iter().map(|change| change.kind).collect();
        let map_flags=state.map_flags(&character.map_instance_key);
        damage.damage = StatusEffectService::apply_incoming_skill_damage_flags(&mut character.status, damage.damage, damage.battle_flags, map_flags.versus(state.siege_active), damage.skill_id);
        damage.damage = super::map_flag_service::apply_map_combat_damage(&map_flags, &GlobalConfigService::instance().config().game,
            damage.damage, damage.skill_id, damage.battle_flags);
        let sender = self.server_service().notification_sender();
        for kind in statuses { if !character.status.has_status_change(kind) { StatusEffectService::send_icon(&character, kind, false, tick, &sender); } }
        let mut redirected = None;
        if damage.damage > 0 {
            let reflected = super::combat_trigger_service::physical_reflection(&target_snapshot, damage.battle_flags, damage.skill_id, damage.damage);
            if reflected > 0 && damage.proc_depth < 8 {
                let reflected = Damage { target_id: damage.attacker_id, attacker_id: character.char_id, credit_id: character.char_id, damage: reflected, healing: 0,
                    attacked_at: tick, damage_motion: 0, battle_flags: 0, skill_id: 0, skill_level: 0, proc_depth: damage.proc_depth + 1,
                    defenses_applied: true, landed: false, magic_context: None, right_hand_damage: None };
                let player_actor = state.characters().contains_key(&reflected.target_id) || (1_000_000_000..1_300_000_000).contains(&reflected.target_id);
                if player_actor { self.add_to_next_tick(GameEvent::CharacterDamage(reflected)); }
                else if let Some(instance) = state.get_map_instance_from_character(&character) { instance.add_to_next_tick(MapEvent::MobDamage(reflected)); }
            }
            if let Some(Some(protector)) = devotion {
                redirected = Some(Damage { target_id: protector, damage: damage.damage, battle_flags: 0, skill_id: models::enums::skill_enums::SkillEnum::CrDevotion.id(),
                    skill_level: 0, landed: false, defenses_applied: true, magic_context: None, damage_motion: 0, ..damage });
                damage.damage = 0;
            }
        }
        if damage.damage > 0 {
            let no_walk_delay = target_snapshot.bonuses_raw().iter().any(|bonus| matches!(bonus, BonusType::EnableNoWalkDelay));
            if !no_walk_delay && damage.battle_flags != 0 {
                if character.is_moving() { self.character_service().cancel_movement(&mut character, tick); }
                character.timing.set_canmove_tick(tick + u128::from(damage.damage_motion));
            }
            let no_cast_cancel = ScriptSkillService::cast_interruption_protected(&target_snapshot, &map_flags);
            if damage.battle_flags != 0 && !no_cast_cancel && character.is_using_skill() && !character.skill_has_been_used() && self.script_skill_service().active_cast_cancelable(&character) {
                character.clear_skill_in_use();
                self.script_skill_service().cancel_queued_cast(&mut character);
                let mut packet = 0x01b9_u16.to_le_bytes().to_vec(); packet.extend_from_slice(&character.char_id.to_le_bytes());
                self.character_service().send_area_notification_around_characters(&character, packet);
            }
            if damage.battle_flags != 0 && !no_cast_cancel && character.script_skill_state.casting_until > tick && self.script_skill_service().active_cast_cancelable(&character) {
                self.script_skill_service().cancel_queued_cast(&mut character);
                let mut packet = 0x01b9_u16.to_le_bytes().to_vec(); packet.extend_from_slice(&character.char_id.to_le_bytes());
                self.character_service().send_area_notification_around_characters(&character, packet);
            }
            if damage.battle_flags != 0 { character.clear_pending_skill(); }
            if self.character_service().take_damage(&mut character, damage.damage) {
                character.pending_item_skill = None; character.pending_craft = None; character.clear_attack(); character.clear_skill_in_use(); character.clear_movement();
                self.script_skill_service().cancel_queued_cast(&mut character);
                for kind in StatusEffectService::remove_on_death(&mut character.status) { StatusEffectService::send_icon(&character, kind, false, tick, &sender); }
                StatusEffectService::send_visual_status(&character, &sender);
            }
        }
        state.insert_character(character);
        if damage.damage > 0 && damage.landed && damage.battle_flags != 0 && !source_player && other_mob_id.is_some()
            && GlobalConfigService::instance().config().game.pet_support.damage_support
            && state.characters().get(&damage.target_id).is_some_and(|owner|owner.game_systems.pet.as_ref().is_some_and(|pet|!pet.incubating&&pet.intimacy>0)) {
            self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                char_id:damage.target_id,request:crate::server::model::game_systems::ScriptWorldRequest::PetCombatTarget {target_id:damage.attacker_id,retaliation:true},
            }));
        }
        if let Some(redirected) = redirected {
            if let Some(protector) = state.characters_mut().get_mut(&redirected.target_id).filter(|protector| protector.is_sitting()) {
                self.character_service().stand(protector);
            }
            self.admit_character_damage(state, redirected, tick)?;
        }
        if damage.damage > 0 && damage.landed && damage.battle_flags != 0 {
            if damage.credit_id != 0 && damage.attacker_id != damage.credit_id && (1_100_000_000..1_200_000_000).contains(&damage.attacker_id) && damage.battle_flags & BattleFlag::Weapon.as_flag() != 0 {
                self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld { char_id: damage.credit_id, request: crate::server::model::game_systems::ScriptWorldRequest::CompanionAttackLanded { id: damage.attacker_id, damage: damage.damage } }));
            }
            let grand_cross_self = damage.attacker_id == damage.target_id && damage.skill_id == models::enums::skill_enums::SkillEnum::CrGrandcross.id();
            let hit_flags = if grand_cross_self { (damage.battle_flags & !BattleFlag::Magic.as_flag()) | BattleFlag::Weapon.as_flag() } else { damage.battle_flags };
            self.add_to_next_tick(GameEvent::ScriptCombat(script_combat_service::ScriptCombatRequest { source_id: damage.target_id, target_id: damage.attacker_id, trigger: CombatTrigger::Hit,
                battle_flags: hit_flags, skill_id: damage.skill_id, damage: damage.damage, other_mob_id, depth: damage.proc_depth, drop_position: None,
                right_hand_damage: original_damage.admitted_right_hand_damage(damage.damage),
                origin_map: state.characters().get(&damage.target_id).map(|target| target.map_instance_key.clone()) }));
            if source_player && !grand_cross_self {
                self.add_to_next_tick(GameEvent::ScriptCombat(script_combat_service::ScriptCombatRequest { source_id: damage.attacker_id, target_id: damage.target_id, trigger: CombatTrigger::Attack,
                    battle_flags: damage.battle_flags, skill_id: damage.skill_id, damage: damage.damage, other_mob_id: None, depth: damage.proc_depth, drop_position: None,
                    right_hand_damage: original_damage.admitted_right_hand_damage(damage.damage),
                    origin_map: state.characters().get(&damage.target_id).map(|target| target.map_instance_key.clone()) }));
            }
            if damage.skill_id != 0 && damage.skill_level != 0 && damage.battle_flags & BattleFlag::Skill.as_flag() != 0 {
                self.add_to_next_tick(GameEvent::ScriptSkillHit(crate::server::script::skill::ScriptSkillHit { source_id: damage.attacker_id, target_id: damage.target_id,
                    skill_id: damage.skill_id, skill_level: damage.skill_level, damage: damage.damage, depth: damage.proc_depth }));
            }
        }
        Ok(())
    }

    pub(crate) fn reward_monster_kill(&self, state: &mut ServerState, kill: crate::server::model::events::game_event::CharacterKillMonster, tick: u128) -> Result<(), String> {
        let instance = state.get_map_instance(kill.map_instance_key.map_name(), kill.map_instance_key.map_instance()).ok_or("Loot map is unavailable")?;
        instance.add_to_delayed_tick(MapEvent::MobDropItems(crate::server::model::events::map_event::MobDropItems {
            owner_id: kill.char_id, mob_id: kill.mob_id, mob_x: kill.mob_x, mob_y: kill.mob_y,
        }), 400);
        let config = &GlobalConfigService::instance().config().game;
        let flags = state.map_flags(&kill.map_instance_key);
        let shares = if flags.enabled(crate::server::model::map_flags::MapFlag::Pvp) && !config.pvp_exp { Default::default() }
            else { super::script_experience_service::monster_experience_awards_with_pets(state, &kill, config.party_even_share_bonus, config.pet_support.attack_exp_to_master, config.pet_support.attack_exp_rate) };
        let awards: Vec<_> = shares.into_iter().filter(|(_, (base, job))| *base != 0 || *job != 0).map(|(id, (base, job))| {
            let character = state.characters().get(&id).ok_or("Experience recipient is unavailable")?;
            Ok(crate::repository::script_character_repository::ScriptExperienceAward {
                char_id: id,
                account_id: character.account_id,
                plan: super::script_character_service::plan_raw_experience(character,
                    (base as f32 * config.base_exp_rate).ceil() as u32,
                    (job as f32 * config.job_exp_rate).ceil() as u32)?,
            })
        }).collect::<Result<_, String>>()?;
        if !awards.is_empty() {
            self.repository.character_commit_experience_awards(&awards).map_err(|error| error.to_string())?;
            for award in &awards {
                if let Some(character) = state.characters_mut().get_mut(&award.char_id) {
                    super::script_character_service::apply_experience(self, character, &award.plan);
                }
            }
        }
        let owners: std::collections::BTreeSet<_> = if kill.contributions.is_empty() {
            [kill.char_id].into_iter().collect()
        } else {
            kill.contributions.iter().filter(|entry| entry.damage > 0).map(|entry| entry.owner_id).collect()
        };
        let mut first_error = None;
        for id in owners {
            if let Some(mut character) = state.characters_mut().remove(&id) {
                if character.map_instance_key == kill.map_instance_key && !character.is_dead() && character.status.hp > 0 {
                    if let Err(error) = self.script_world_service().record_companion_kill(self, &mut character, &kill, &flags, tick as u64) {
                        first_error.get_or_insert(error);
                    }
                }
                state.insert_character(character);
            }
        }
        first_error.map_or(Ok(()), Err)
    }
}
