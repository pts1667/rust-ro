use std::sync::Arc;
use std::sync::atomic::Ordering;

use models::enums::EnumWithMaskValueU32;
use models::enums::bonus::BonusType;
use models::status_bonus::{BattleFlag, CombatTrigger};

use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterUseSkill, GameEvent, ScriptEvent, CharacterDamage};
use crate::server::model::events::map_event::{MapEvent, MobDamage};
use crate::server::model::movement::Movable;
use crate::server::model::script::Script;
use crate::server::script::NpcScriptHost;
use crate::server::script::skill::{PendingItemSkill, ScriptSkillService};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::service::{map_combat_service, script_combat_service};
use crate::server::model::map_flags::MapFlag;
use crate::server::state::server::ServerState;

fn devotion_protector(
    state: &ServerState,
    protected: &crate::server::state::character::Character,
    status: &models::status_change::StatusChange,
    tick: u128,
) -> Option<u32> {
    if status.expired(tick) || !(0..5).contains(&status.values[1]) {
        return None;
    }
    let id = u32::try_from(status.values[0]).ok().filter(|id| *id != protected.char_id)?;
    let range = u16::try_from(status.values[2]).ok()?;
    if let Some(protector) = state.characters().get(&id) {
        return (!protector.is_dead()
            && protector.status.hp > 0
            && protector.map_instance_key == protected.map_instance_key
            && protected.x.abs_diff(protector.x).max(protected.y.abs_diff(protector.y)) <= range)
            .then_some(id);
    }
    let mercenary = protected
        .game_systems
        .mercenary
        .as_ref()
        .filter(|mercenary| super::script_world_service::mercenary_world_id(mercenary) == id && mercenary.hp > 0)?;
    let snapshot = super::script_world_service::companion_snapshots(protected)
        .into_iter()
        .find(|snapshot| snapshot.map_item().id() == super::script_world_service::mercenary_world_id(mercenary))?;
    (protected.x.abs_diff(snapshot.x()).max(protected.y.abs_diff(snapshot.y())) <= range).then_some(id)
}

const INSURANCE_ITEM_ID: i32 = 6413;

impl Server {
    pub(crate) fn player_target_allowed(
        &self,
        state: &ServerState,
        source: &crate::server::state::character::Character,
        target_id: u32,
        mode: super::visibility_service::TargetingMode,
    ) -> bool {
        use super::visibility_service::{StealthState, VisibilityObserver, can_target};
        if target_id == source.char_id {
            return true;
        }
        if let Some(unit) = state.ground_unit(target_id, source.current_map_name(), source.current_map_instance()) {
            return !unit.used;
        }
        let stealth = if let Some(target) = state
            .characters()
            .get(&target_id)
            .filter(|target| target.map_instance_key == source.map_instance_key)
        {
            Some(StealthState::from_status_options(&target.status, target.options))
        } else if let Some(snapshot) = super::script_world_service::companion_status_snapshot(source, target_id) {
            Some(StealthState::from_snapshot(&snapshot))
        } else if let Some(instance) = state.get_map_instance_from_character(source) {
            instance
                .state()
                .get_mob(target_id)
                .map(|mob| StealthState::from_status(&mob.status_effects))
                .or_else(|| {
                    state
                        .characters()
                        .values()
                        .filter(|owner| owner.map_instance_key == source.map_instance_key)
                        .find_map(|owner| {
                            super::script_world_service::companion_status_snapshot(owner, target_id)
                                .map(|snapshot| StealthState::from_snapshot(&snapshot))
                        })
                })
        } else {
            None
        };
        stealth.is_some_and(|stealth| {
            can_target(
                VisibilityObserver::player(&StatusService::instance().to_snapshot(&source.status)),
                stealth,
                mode,
            )
        })
    }

    pub(crate) fn player_skill_target_allowed(
        &self,
        state: &ServerState,
        source: &crate::server::state::character::Character,
        target_id: u32,
        skill_id: u32,
        completed: bool,
    ) -> bool {
        use models::enums::skill_enums::SkillEnum;

        use super::visibility_service::TargetingMode;
        if skill_id != 0 && self.is_emperium_target(state, source, target_id) {
            let targets_emperium = crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
                .is_some_and(|metadata| metadata.flags.get("TargetEmperium").copied().unwrap_or(false));
            if !targets_emperium {
                return false;
            }
        }
        if let Some(unit) = state.ground_unit(target_id, source.current_map_name(), source.current_map_instance()) {
            return crate::server::script::skill::metadata::SkillMetadata::find(skill_id).is_some_and(|metadata| {
                metadata.target_type.as_deref() == Some("Trap") || !unit.used && metadata.flags.get("TargetTrap").copied().unwrap_or(false)
            });
        }
        if crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .is_some_and(|metadata| metadata.target_type.as_deref() == Some("Trap"))
        {
            return self
                .script_skill_service()
                .ground_unit_position(
                    source.current_map_name(),
                    source.current_map_instance(),
                    target_id,
                    crate::util::tick::get_tick(),
                )
                .is_some();
        }
        if skill_id == SkillEnum::SaDispell.id() {
            let own_party = state.characters().get(&target_id).is_some_and(|target| {
                source.game_systems.party_id > 0
                    && source.game_systems.party_id == target.game_systems.party_id
                    && source.map_instance_key == target.map_instance_key
            });
            if target_id != source.char_id && !own_party && !self.player_combat_target_allowed(state, source, target_id) {
                return false;
            }
        } else if Self::player_skill_requires_hostile_target(skill_id) && !self.player_combat_target_allowed(state, source, target_id) {
            return false;
        }
        if [SkillEnum::AlHeal.id(), SkillEnum::AllResurrection.id(), SkillEnum::PrAspersio.id()].contains(&skill_id) {
            let target = self
                .server_service()
                .get_target_status(state, source, Some(target_id), 0)
                .or_else(|| super::script_world_service::companion_status_snapshot(source, target_id));
            let undead = target.is_some_and(|target| {
                *target.race() == models::enums::mob::MobRace::RUndead || *target.element() == models::enums::element::Element::Undead
            });
            if undead && !self.player_combat_target_allowed(state, source, target_id) {
                return false;
            }
        }
        let hidden = crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .is_some_and(|skill| skill.flags.get("TargetHidden").copied().unwrap_or(false));
        let mode = if completed {
            TargetingMode::SkillCompletion { can_hit_hidden: hidden }
        } else if hidden {
            TargetingMode::DirectHiddenSkill
        } else {
            TargetingMode::Direct
        };
        self.player_target_allowed(state, source, target_id, mode)
    }

    pub(crate) fn refresh_forged_rank(&self, character: &mut crate::server::state::character::Character) {
        if let Ok(rankings) = self
            .repository
            .fame_rankings(crate::repository::fame_repository::FameCategory::Blacksmith)
        {
            let creators: Vec<_> = rankings.iter().map(|entry| entry.char_id).collect();
            crate::repository::fame_repository::refresh_forged_rank(&mut character.status, &creators);
        }
    }

    pub(crate) fn refresh_taekwon_rank(&self, character: &mut crate::server::state::character::Character) {
        if let Ok(rankings) = self
            .repository
            .fame_rankings(crate::repository::fame_repository::FameCategory::Taekwon)
        {
            let ranked: Vec<_> = rankings.iter().map(|entry| entry.char_id).collect();
            match self.update_taekwon_rank(character, &ranked) {
                Ok(true) => self.skill_tree_service().send_skill_tree(character),
                Ok(false) => {}
                Err(error) => warn!("Taekwon rank refresh failed: {error}"),
            }
        }
    }

    pub(crate) fn update_taekwon_rank(&self, character: &mut crate::server::state::character::Character, ranked: &[u32]) -> Result<bool, String> {
        let previous = character.status.taekwon_ranked;
        if !super::script_character_service::refresh_rank_status(character, ranked) {
            return Ok(false);
        }
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let (max_hp, max_sp) = (snapshot.max_hp(), snapshot.max_sp());
        let (hp, sp) = (character.status.hp.min(max_hp), character.status.sp.min(max_sp));
        let change = crate::repository::script_inventory_repository::ScriptInventoryTransaction {
            char_id: character.char_id,
            account_id: character.account_id,
            consumption: None,
            exact_removals: vec![],
            removals: vec![],
            identifications: vec![],
            grants: vec![],
            variables: vec![],
            zeny: None,
            hp: None,
            sp: None,
            max_weight: self.character_service().max_weight(character),
            max_slots: 100,
            world: None,
            reset_skills: None,
            fame: None,
            pool_draws: vec![],
            character_changes: vec![
                crate::repository::script_inventory_repository::ScriptCharacterChange::ResourcePools { hp, sp, max_hp, max_sp },
            ],
        };
        if let Err(error) = self.repository.script_inventory_transaction(&change) {
            character.status.taekwon_ranked = previous;
            return Err(error.to_string());
        }
        character.status.max_hp = max_hp;
        character.status.max_sp = max_sp;
        if character.status.hp != hp || character.status.sp != sp {
            self.character_service().update_hp_sp(character, hp, sp);
        }
        Ok(true)
    }

    #[cfg(test)]
    pub(crate) fn handle_script_event(&self, state: &mut ServerState, event: GameEvent, tick: u128) -> Result<(), String> {
        event.dispatch(self, state, tick)
    }

    pub(crate) fn run_compiled_event(&self, state: &ServerState, event: ScriptEvent) -> Result<(), String> {
        let character = state.characters().get(&event.char_id).ok_or("Event player disconnected")?;
        let session = state.find_session(character.account_id).ok_or("Event session expired")?;
        let server = self.shared().ok_or("Server runtime is not bound")?;
        let (_, inputs) = tokio::sync::mpsc::channel(1);
        let host = NpcScriptHost {
            server,
            generation: session.script_generation.load(Ordering::Acquire),
            background: true,
            event_depth: 0,
            event_arguments: None,
            timer_context: None,
            logout_token: None,
            session,
            script: Arc::new(Script {
                id: 0,
                scope_instance: character.current_map_instance(),
                entry_id: event.entry_id,
                map_name: character.current_map_name().clone(),
                name: "CompiledEvent".into(),
                sprite: 0,
                x: character.x(),
                y: character.y(),
                dir: 0,
                x_size: 0,
                y_size: 0,
                constructor_args: event.args,
            }),
            inputs,
            notifications: self.server_service().notification_sender(),
            map_instance: character.current_map_instance(),
            dialog_open: false,
            error: None,
        };
        let vm = self.script_service().vm.clone();
        self.runtime().spawn(async move {
            let (host, result) = vm.execute(host, "run_event", event.entry_id).await;
            if let Err(error) = result {
                warn!("Compiled event failed: {}", host.error.unwrap_or(error));
            }
        });
        Ok(())
    }

    pub(crate) fn handle_character_skill(&self, state: &mut ServerState, event: CharacterUseSkill, tick: u128) -> Result<(), String> {
        let mut character = state.characters_mut().remove(&event.char_id).ok_or("Skill source disconnected")?;
        let previous_pending = character.pending_item_skill.clone();
        let result = (|| {
            if character.is_dead()
                || character.status.blocks_casting()
                || character.timing.skill_menu_blocked()
                || character.game_systems.is_trading()
                || character.game_systems.buying_store.is_some()
                || character.game_systems.vending_store.is_some()
            {
                return Err("Character cannot use skills now".into());
            }
            if self.character_service().is_overweight_for_combat(&character) {
                return Err("Too heavy to use skills".into());
            }
            self.script_skill_service().validate_performing(state, &character, event.skill_id)?;
            if (8001..=8016).contains(&event.skill_id) || (8201..=8240).contains(&event.skill_id) {
                self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                    char_id: character.char_id,
                    request: crate::server::model::game_systems::ScriptWorldRequest::Companion(crate::server::model::game_systems::CompanionRequest::UseCompanionSkill {
                        skill_id: event.skill_id,
                        skill_level: event.skill_level,
                        target_id: event.target_id,
                    }),
                }));
                return Ok(());
            }
            if crate::server::service::guild_skill_service::is_active_guild_skill(event.skill_id) {
                return self.use_guild_skill(state, &mut character, event.skill_id, event.skill_level, tick);
            }
            if let Some(pending) = character.pending_item_skill.clone() {
                if !self.player_skill_target_allowed(state, &character, event.target_id, event.skill_id, false) {
                    return Err("Skill target is hidden or unavailable".into());
                }
                self.script_skill_service().validate_pending_cast(state, &character, &event, tick)?;
                self.item_service().defer_skill_requirements_in_state(
                    self,
                    state,
                    &mut character,
                    event.skill_id,
                    event.skill_level,
                    tick,
                    pending.keep_requirements,
                    pending.item_index,
                )?;
                if let Some(pending) = character.pending_item_skill.as_mut() {
                    pending.keep_requirements = false;
                    pending.item_index = None;
                }
                self.script_skill_service()
                    .cast_pending(self, state, &mut character, &event, tick)?;
                return Ok(());
            }
            let learned = StatusService::instance()
                .to_snapshot(&character.status)
                .known_skills()
                .iter()
                .find(|skill| skill.value.id() == event.skill_id)
                .map_or(0, |skill| skill.level);
            if event.skill_level == 0 || event.skill_level > learned {
                return Err("Requested skill level is not learned".into());
            }
            let configuration = GlobalConfigService::instance();
            let skill = configuration
                .find_skill_config(&(event.skill_id as i32).into())
                .ok_or("Unknown skill")?;
            if skill.name() == "SA_CASTCANCEL" {
                return self.script_skill_service().cast_cancel(self, &mut character, event.skill_id, event.skill_level, tick);
            }
            if skill.name() == "MC_VENDING" || skill.name() == "MC_PUSHCART" {
                self.script_skill_service()
                    .validate_native_environment(state, &character, event.skill_id, event.skill_level, tick)?;
                self.item_service()
                    .pay_skill_requirements(self, &mut character, event.skill_id, event.skill_level, tick, true, None)?;
                let request = if skill.name() == "MC_VENDING" {
                    crate::server::model::game_systems::ScriptWorldRequest::Store(crate::server::model::game_systems::StoreRequest::PrepareVending {
                        skill_level: event.skill_level,
                    })
                } else {
                    crate::server::model::game_systems::ScriptWorldRequest::Container(crate::server::model::game_systems::ContainerRequest::SetCart(1))
                };
                self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                    char_id: character.char_id,
                    request,
                }));
                return Ok(());
            }
            if crate::server::service::skill_menu_service::is_menu_skill(skill.name()) {
                return self.open_skill_menu(state, &mut character, skill.name(), event.skill_id, event.skill_level, event.target_id, tick);
            }
            self.script_skill_service().validate_skill(skill, u32::from(event.skill_level))?;
            if !self.player_skill_target_allowed(state, &character, event.target_id, event.skill_id, false) {
                return Err("Skill target is hidden or unavailable".into());
            }
            if ScriptSkillService::operation(skill.name()).is_some_and(|operation| {
                operation != crate::server::script::skill::callbacks::SkillOperation::Damage
                    || ScriptSkillService::uses_metadata_magic(skill.name())
            }) {
                if skill.name() == "MC_IDENTIFY" {
                    self.item_service().defer_skill_requirements_in_state(
                        self,
                        state,
                        &mut character,
                        event.skill_id,
                        event.skill_level,
                        tick,
                        true,
                        None,
                    )?;
                    return self
                        .script_skill_service()
                        .handle_skill(self, &mut character, skill, u32::from(event.skill_level), false);
                }
                if skill.name() == "AL_TELEPORT" {
                    return self
                        .script_skill_service()
                        .start_native_teleport_menu(self, state, &mut character, event.skill_level, tick);
                }
                character.pending_item_skill = Some(PendingItemSkill {
                    skill_id: event.skill_id,
                    level: event.skill_level,
                    keep_requirements: true,
                    item_index: None,
                    source_item: None,
                    expires_at: tick + 120_000,
                });
                self.script_skill_service().validate_pending_cast(state, &character, &event, tick)?;
                self.item_service().defer_skill_requirements_in_state(
                    self,
                    state,
                    &mut character,
                    event.skill_id,
                    event.skill_level,
                    tick,
                    true,
                    None,
                )?;
                if let Some(pending) = character.pending_item_skill.as_mut() {
                    pending.keep_requirements = false;
                }
                self.script_skill_service()
                    .cast_pending(self, state, &mut character, &event, tick)?;
            } else {
                self.server_service()
                    .character_start_use_skill(self, state, &mut character, event, tick);
            }
            Ok(())
        })();
        if result.is_err() && previous_pending.is_none() {
            character.pending_item_skill = None;
            character.script_skill_state.deferred_requirements = None;
        }
        state.insert_character(character);
        result
    }

    pub(crate) fn admit_script_map_damage(
        &self,
        state: &mut ServerState,
        request: crate::server::model::events::game_event::ScriptMapDamage,
        tick: u128,
    ) -> Result<(), String> {
        if self
            .script_skill_service()
            .apply_ground_unit_damage(self, state, &request.map, request.damage, tick)
        {
            self.script_skill_service()
                .refresh_ground_unit_snapshot(state, &request.map, request.damage.target_id, tick);
            return Ok(());
        }
        if let Some(target) = state.get_character(request.damage.target_id) {
            if target.map_instance_key != request.map {
                return Ok(());
            }
            self.admit_character_damage(state, request.damage, tick)
        } else if state
            .companion_owner(request.damage.target_id, request.map.map_name(), request.map.map_instance())
            .is_some()
        {
            self.admit_character_damage(state, request.damage, tick)
        } else {
            if let Some(instance) = state.get_map_instance(request.map.map_name(), request.map.map_instance()) {
                instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: request.damage }));
            }
            Ok(())
        }
    }

    /// A defender in Auto Counter stance who faces a melee attacker negates the hit and strikes back.
    fn auto_counter_reaction(&self, state: &ServerState, damage: &Damage, tick: u128) -> bool {
        use models::enums::skill_enums::SkillEnum;
        use models::status_change::StatusChangeKind;
        let melee = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag();
        if damage.battle_flags & melee != melee || damage.attacker_id == damage.target_id {
            return false;
        }
        let Some(defender) = state.characters().get(&damage.target_id) else {
            return false;
        };
        let Some(stance) = defender.status.status_change(StatusChangeKind::AutoCounter) else {
            return false;
        };
        let level = stance.values[0].clamp(1, i32::from(u8::MAX)) as u8;
        let attacker = state
            .characters()
            .get(&damage.attacker_id)
            .map(|attacker| (attacker.x, attacker.y, StatusService::instance().to_snapshot(&attacker.status), true))
            .or_else(|| {
                state.get_map_instance_from_character(defender).and_then(|instance| {
                    instance
                        .state()
                        .get_mob(damage.attacker_id)
                        .map(|mob| (mob.x, mob.y, mob.status.clone(), false))
                })
            });
        let Some((attacker_x, attacker_y, attacker_status, attacker_is_player)) = attacker else {
            return false;
        };
        let defender_status = StatusService::instance().to_snapshot(&defender.status);
        let distance = defender.x.abs_diff(attacker_x).max(defender.y.abs_diff(attacker_y));
        let toward_attacker = ScriptSkillService::direction_to(defender.x, defender.y, attacker_x, attacker_y, defender.dir);
        let facing_offset = (toward_attacker + 8 - defender.dir % 8) % 8;
        if distance > 0 && (!matches!(facing_offset, 0 | 1 | 7) || distance > u16::from(defender_status.attack_range()) + 1) {
            return false;
        }
        let Some(object) = skills::skill_enums::to_object(SkillEnum::KnAutocounter, level) else {
            return false;
        };
        let Some(offensive) = object.as_offensive_skill() else {
            return false;
        };
        let (amount, context) = self
            .battle_service()
            .calculate_damage_with_context(&defender_status, &attacker_status, Some(offensive));
        let mut counter = Damage {
            notification: None,
            source_kind: *defender_status.combat_actor_kind(),
            skill_damage_adjusted: false,
            target_id: damage.attacker_id,
            attacker_id: defender.char_id,
            credit_id: defender.char_id,
            damage: 0,
            healing: 0,
            right_hand_damage: None,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: melee,
            skill_id: SkillEnum::KnAutocounter.id(),
            skill_level: level,
            proc_depth: damage.proc_depth + 1,
            defenses_applied: true,
            magic_context: context,
            landed: true,
        };
        counter.set_signed_damage(amount);
        counter = counter.with_skill_notification(defender.current_map_name(), defender.current_map_instance(), defender.x, defender.y, tick, 1, 0);
        self.add_to_next_tick(GameEvent::CharacterEndStatus(crate::server::model::events::game_event::CharacterEndStatus {
            char_id: defender.char_id,
            kind: Some(StatusChangeKind::AutoCounter),
        }));
        if attacker_is_player {
            self.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage: counter }));
        } else if let Some(instance) = state.get_map_instance_from_character(defender) {
            instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: counter }));
        }
        true
    }

    fn apply_nightmare_drops(&self, state: &mut ServerState, char_id: u32) {
        use models::enums::EnumWithMaskValueU64;
        use models::enums::item::ItemTradeFlag;
        use models::enums::EnumWithMaskValueU8;
        let Some(character) = state.characters().get(&char_id) else {
            return;
        };
        let flags = state.map_flags(&character.map_instance_key);
        if !flags.enabled(MapFlag::PvpNightmareDrop) || flags.enabled(MapFlag::NoDrop) || flags.nightmare_drops.is_empty() {
            return;
        }
        let Some(mut character) = state.characters_mut().remove(&char_id) else {
            return;
        };
        if let Some(instance) = state.get_map_instance_from_character(&character) {
            for (item_id, location, rate) in flags.nightmare_drops.iter().copied() {
                for (wanted, equipped) in [
                    (models::enums::map::NightmareDropLocation::Inventory, false),
                    (models::enums::map::NightmareDropLocation::Equipped, true),
                ] {
                    if location & wanted.as_flag() == 0 {
                        continue;
                    }
                    let candidates: Vec<usize> = character
                        .inventory_iter()
                        .filter(|(_, item)| {
                            (item.equip != 0) == equipped
                                && (item_id == -1 || item.item_id == item_id)
                                && GlobalConfigService::instance().get_item(item.item_id).trade_flags as u64 & ItemTradeFlag::NoDrop.as_flag() == 0
                        })
                        .map(|(index, _)| index)
                        .collect();
                    if candidates.is_empty() || fastrand::u32(0..10_000) >= u32::from(rate) {
                        continue;
                    }
                    let index = if item_id == -1 {
                        candidates[fastrand::usize(..candidates.len())]
                    } else {
                        candidates[0]
                    };
                    if equipped && self.inventory_service().takeoff_equip_item(&mut character, index).is_none() {
                        continue;
                    }
                    let drop = crate::server::model::events::game_event::CharacterRemoveItem {
                        char_id,
                        index,
                        amount: 1,
                        price: 0,
                    };
                    if let Err(error) = self.inventory_service().character_drop_items(
                        self.runtime(),
                        &mut character,
                        crate::server::model::events::game_event::CharacterRemoveItems {
                            char_id,
                            sell: false,
                            items: vec![drop],
                            notify_client: true,
                        },
                        &instance,
                    ) {
                        error!("Nightmare drop for {char_id} failed: {error}");
                    }
                }
            }
        }
        state.insert_character(character);
    }

    pub(crate) fn enter_pvp_ranking(&self, state: &mut ServerState, char_id: u32) {
        let Some(character) = state.characters().get(&char_id) else {
            return;
        };
        let key = character.map_instance_key.clone();
        let flags = state.map_flags(&key);
        if !flags.enabled(MapFlag::Pvp) || flags.enabled(MapFlag::PvpNoCalcRank) {
            return;
        }
        if let Some(character) = state.characters_mut().get_mut(&char_id) {
            character.pvp_point = 5;
            character.pvp_won = 0;
            character.pvp_lost = 0;
        }
        self.send_pvp_ranks(state, &key);
    }

    fn send_pvp_ranks(&self, state: &ServerState, key: &crate::server::model::map_instance::MapInstanceKey) {
        let players: Vec<(u32, i32)> = state
            .characters()
            .values()
            .filter(|player| player.map_instance_key == *key)
            .map(|player| (player.char_id, player.pvp_point))
            .collect();
        let total = players.len() as u32;
        for (char_id, points) in &players {
            let rank = 1 + players.iter().filter(|(_, other)| other > points).count() as u32;
            let mut packet = 0x019A_u16.to_le_bytes().to_vec();
            packet.extend_from_slice(&char_id.to_le_bytes());
            packet.extend_from_slice(&rank.to_le_bytes());
            packet.extend_from_slice(&total.to_le_bytes());
            let _ = self.server_service().notification_sender().send(
                crate::server::model::events::client_notification::Notification::Char(
                    crate::server::model::events::client_notification::CharNotification::new(*char_id, packet),
                ),
            );
        }
    }

    fn apply_pvp_death(&self, state: &mut ServerState, victim_id: u32, killer_id: u32) {
        if self.duels().duel_of(victim_id).is_some() {
            self.leave_duel(state, victim_id);
        }
        self.duels().reject(victim_id);
        let Some(victim) = state.characters().get(&victim_id) else {
            return;
        };
        let victim_key = victim.map_instance_key.clone();
        let flags = state.map_flags(&victim.map_instance_key);
        let mut forced_respawn = flags.is_gvg() || flags.enabled(MapFlag::Battleground);
        if flags.enabled(MapFlag::Pvp) && !flags.enabled(MapFlag::PvpNoCalcRank) {
            if let Some(victim) = state.characters_mut().get_mut(&victim_id) {
                victim.pvp_point -= 5;
                victim.pvp_lost += 1;
                forced_respawn |= victim.pvp_point < 0;
            }
            if let Some(killer) = state.characters_mut().get_mut(&killer_id).filter(|killer| killer.char_id != victim_id) {
                killer.pvp_point += 1;
                killer.pvp_won += 1;
            }
            self.send_pvp_ranks(state, &victim_key);
        }
        if !forced_respawn {
            return;
        }
        if let Some(session) = state
            .characters()
            .get(&victim_id)
            .and_then(|victim| state.find_session(victim.account_id))
        {
            self.add_to_delayed_tick(
                GameEvent::CharacterRespawn(crate::server::model::character_lifecycle::CharacterRespawn { session }),
                1000,
            );
        }
    }

    fn apply_death_penalty(&self, state: &mut ServerState, char_id: u32) {
        let Some(character) = state.characters().get(&char_id) else {
            return;
        };
        let flags = state.map_flags(&character.map_instance_key);
        let exempt = flags.enabled(MapFlag::NoExpPenalty)
            || flags.enabled(MapFlag::NoPenalty)
            || flags.is_gvg()
            || flags.enabled(MapFlag::Battleground)
            || character.status.has_status_change(models::status_change::StatusChangeKind::ProtectExp);
        let plan = match super::script_character_service::plan_death_penalty(character, exempt) {
            Ok(Some(plan)) => plan,
            Ok(None) => return,
            Err(error) => {
                error!("Death penalty for {char_id} failed: {error}");
                return;
            }
        };
        let account_id = character.account_id;
        let zeny_exempt = exempt || flags.enabled(MapFlag::NoZenyPenalty);
        let Some(mut character) = state.characters_mut().remove(&char_id) else {
            return;
        };
        let insurance = character
            .inventory_iter()
            .find(|(_, item)| item.item_id == INSURANCE_ITEM_ID && item.equip == 0)
            .map(|(index, _)| index);
        let insured = insurance.is_some_and(|index| {
            self.inventory_service()
                .remove_single_item_from_inventory(self.runtime(), index, &mut character, true)
                .is_ok()
        });
        if !insured {
            let award = crate::repository::script_character_repository::ScriptExperienceAward { char_id, account_id, plan };
            match self.repository.character_commit_experience_awards(std::slice::from_ref(&award)) {
                Ok(()) => super::script_character_service::apply_experience(self, &mut character, &award.plan),
                Err(error) => error!("Death penalty for {char_id} was not committed: {error}"),
            }
        }
        let zeny_rate = GlobalConfigService::instance().config().game.death_penalty.zeny;
        if zeny_rate > 0 && !zeny_exempt {
            let loss = (u64::from(character.get_zeny()) * u64::from(zeny_rate) / 10_000) as u32;
            if loss > 0 {
                self.character_service().update_zeny(
                    self.runtime(),
                    crate::server::model::events::game_event::CharacterZeny {
                        char_id,
                        zeny: Some(character.get_zeny() - loss),
                    },
                    &mut character,
                );
            }
        }
        state.characters_mut().insert(char_id, character);
    }

    pub(crate) fn admit_character_damage(&self, state: &mut ServerState, mut damage: Damage, tick: u128) -> Result<(), String> {
        if let Some(map) = state.ground_units().map_of(damage.target_id) {
            if self
                .script_skill_service()
                .apply_ground_unit_damage(self, state, &map, damage, tick)
            {
                self.script_skill_service()
                    .refresh_ground_unit_snapshot(state, &map, damage.target_id, tick);
                return Ok(());
            }
        }
        if state
            .get_character(damage.target_id)
            .is_some_and(|target| !damage.matches_notification_map(&target.map_instance_key))
        {
            return Ok(());
        }
        if damage.landed && damage.attacker_id != damage.target_id {
            let owner = state.characters().get(&damage.attacker_id).or_else(|| {
                state
                    .characters()
                    .values()
                    .find(|owner| super::script_world_service::companion_status_snapshot(owner, damage.attacker_id).is_some())
            });
            if owner.is_some_and(|owner| !self.player_combat_target_allowed(state, owner, damage.target_id)) {
                damage.notify_admitted(&self.server_service().notification_sender(), 0, self.packetver());
                return Ok(());
            }
        }
        let original_damage = damage;
        if self.script_world_service().handle_companion_damage(self, state, damage, tick)? {
            return Ok(());
        }
        let Some(target) = state.characters().get(&damage.target_id) else {
            return Ok(());
        };
        let origin_map = target.map_instance_key.clone();
        if target.is_dead() || target.status.hp == 0 {
            damage.notify_admitted(&self.server_service().notification_sender(), 0, self.packetver());
            return Ok(());
        }
        let mut target_snapshot = StatusService::instance().to_snapshot(&target.status);
        if damage.healing > 0 {
            super::map_flag_service::apply_map_skill_damage(&state.map_flags(&origin_map), &mut damage, &target_snapshot);
        }
        if damage.healing > 0 {
            let hp = target
                .status
                .hp
                .saturating_add(damage.healing)
                .min(StatusService::instance().to_snapshot(&target.status).max_hp());
            let healed = hp.saturating_sub(target.status.hp);
            let sp = target.status.sp;
            self.repository
                .script_inventory_transaction(&crate::repository::script_inventory_repository::ScriptInventoryTransaction {
                    char_id: target.char_id,
                    account_id: target.account_id,
                    consumption: None,
                    exact_removals: vec![],
                    removals: vec![],
                    identifications: vec![],
                    grants: vec![],
                    variables: vec![],
                    zeny: None,
                    hp: Some(hp),
                    sp: None,
                    max_weight: self.character_service().max_weight(target),
                    max_slots: usize::from(GlobalConfigService::instance().config().game.max_inventory),
                    world: None,
                    reset_skills: None,
                    fame: None,
                    character_changes: vec![],
                    pool_draws: vec![],
                })
                .map_err(|error| error.to_string())?;
            let target = state.characters_mut().get_mut(&damage.target_id).unwrap();
            self.character_service().update_hp_sp(target, hp, sp);
            damage.notify_admitted(
                &self.server_service().notification_sender(),
                -i64::from(healed),
                self.packetver(),
            );
            return Ok(());
        }
        if damage.damage == 0 {
            damage.notify_admitted(&self.server_service().notification_sender(), 0, self.packetver());
            return Ok(());
        }
        if damage.landed && damage.skill_id == 0 && damage.proc_depth < 8 && self.auto_counter_reaction(state, &damage, tick) {
            damage.notify_admitted(&self.server_service().notification_sender(), 0, self.packetver());
            return Ok(());
        }
        if damage.landed && damage.proc_depth < 8 {
            if let Some(kind) = super::combat_trigger_service::magic_reflection(
                &target_snapshot,
                damage.battle_flags,
                damage.skill_id,
                &mut fastrand::Rng::new(),
            ) {
                let request = map_combat_service::MagicReflectionRequest {
                    damage,
                    reflector_id: target.char_id,
                    reflector_credit_id: target.char_id,
                    kind,
                    map_key: target.map_instance_key.clone(),
                };
                damage.notify_admitted(&self.server_service().notification_sender(), 0, self.packetver());
                return map_combat_service::reflect_magic(self, state, request, tick);
            }
        }
        let magic_rod = target
            .status
            .status_change(models::status_change::StatusChangeKind::MagicRod)
            .is_some_and(|change| !change.expired(tick));
        if magic_rod {
            let mut character = state.characters_mut().remove(&damage.target_id).unwrap();
            let absorbed =
                StatusEffectService::absorb_magic_rod(&mut character.status, damage.battle_flags, damage.skill_id, damage.skill_level)
                    .is_some();
            if absorbed {
                damage.notify_admitted(&self.server_service().notification_sender(), 0, self.packetver());
                let (hp, sp) = (character.status.hp, character.status.sp);
                self.character_service().update_hp_sp(&mut character, hp, sp);
                self.script_skill_service()
                    .notify_magic_rod_absorption(&character, damage.skill_level);
            }
            state.insert_character(character);
            if absorbed {
                return Ok(());
            }
        }
        let target = state.characters().get(&damage.target_id).unwrap();
        let source_player = state.characters().contains_key(&damage.attacker_id);
        let other_mob_id = state
            .get_map_instance_from_character(target)
            .and_then(|instance| instance.state().get_mob(damage.attacker_id).map(|mob| mob.mob_id as u32));
        let devotion = if damage.battle_flags != 0
            && damage.skill_id != models::enums::skill_enums::SkillEnum::PaPressure.id()
            && damage.skill_id != models::enums::skill_enums::SkillEnum::CrReflectshield.id()
            && damage.skill_id != models::enums::skill_enums::SkillEnum::PaPressure.id()
        {
            target
                .status
                .status_change(models::status_change::StatusChangeKind::Devotion)
                .map(|change| devotion_protector(state, target, change, tick))
        } else {
            None
        };
        let mut character = state.characters_mut().remove(&damage.target_id).unwrap();
        if devotion == Some(None) {
            StatusEffectService::end(
                self,
                &mut character,
                Some(models::status_change::StatusChangeKind::Devotion),
                tick,
                &self.server_service().notification_sender(),
            );
            target_snapshot = StatusService::instance().to_snapshot(&character.status);
        }
        if !damage.defenses_applied {
            damage.damage = if damage.battle_flags & BattleFlag::Weapon.as_flag() != 0 {
                (damage.damage as f32 * (1.0 - target_snapshot.def().clamp(0, 100) as f32 / 100.0) - f32::from(target_snapshot.vit()))
                    .max(1.0) as u32
            } else if damage.battle_flags & BattleFlag::Magic.as_flag() != 0 {
                (damage.damage as f32 * (1.0 - target_snapshot.mdef().clamp(0, 100) as f32 / 100.0) - f32::from(target_snapshot.int()))
                    .max(1.0) as u32
            } else {
                damage.damage
            };
        }
        if ScriptSkillService::try_dodge(&character, damage.battle_flags) {
            damage.damage = 0;
        }
        let statuses: Vec<_> = character.status.active_statuses.iter().map(|change| change.kind).collect();
        let map_flags = state.map_flags(&character.map_instance_key);
        let utsusemi_hits = character.status.status_change(models::status_change::StatusChangeKind::Utsusemi).map(|change| change.values[1]);
        damage.damage = StatusEffectService::apply_incoming_skill_damage_flags(
            &mut character.status,
            damage.damage,
            damage.battle_flags,
            map_flags.versus(state.siege_active()),
            damage.skill_id,
        );
        let utsusemi_blocked = utsusemi_hits.is_some_and(|hits| {
            character
                .status
                .status_change(models::status_change::StatusChangeKind::Utsusemi)
                .is_none_or(|change| change.values[1] < hits)
        });
        if utsusemi_blocked {
            self.script_skill_service().utsusemi_block_knockback(self, state, &mut character, damage.attacker_id, tick)?;
        }
        damage.damage = super::map_flag_service::apply_map_combat_damage(
            &map_flags,
            &GlobalConfigService::instance().config().game,
            damage.damage,
            damage.skill_id,
            damage.battle_flags,
        );
        super::map_flag_service::apply_map_skill_damage(&map_flags, &mut damage, &target_snapshot);
        let attacker_uses_blade = state.characters().get(&damage.attacker_id).is_none_or(|attacker| {
            use models::enums::weapon::WeaponType::{Dagger, Sword1H, Sword2H};
            matches!(StatusService::instance().to_snapshot(&attacker.status).right_hand_weapon_type(), Dagger | Sword1H | Sword2H)
        });
        let rejected = StatusEffectService::reject_sword(&mut character.status, attacker_uses_blade, damage.battle_flags, damage.damage, fastrand::u8(0..100));
        damage.damage -= rejected;
        let sender = self.server_service().notification_sender();
        if damage.damage > target_snapshot.max_hp() / 4 && character.status.has_status_change(models::status_change::StatusChangeKind::Dancing) {
            StatusEffectService::end(self, &mut character, Some(models::status_change::StatusChangeKind::Dancing), tick, &sender);
        }
        let mut reported = 0_i64;
        for kind in statuses {
            if !character.status.has_status_change(kind) {
                StatusEffectService::send_icon(&character, kind, false, tick, &sender);
            }
        }
        if damage.healing > 0 {
            let hp = character.status.hp.saturating_add(damage.healing).min(target_snapshot.max_hp());
            reported = -i64::from(hp.saturating_sub(character.status.hp));
            let sp = character.status.sp;
            self.character_service().update_hp_sp(&mut character, hp, sp);
        }
        let mut redirected = None;
        if damage.damage > 0 {
            let reflected = super::combat_trigger_service::physical_reflection(&target_snapshot, damage.battle_flags, damage.skill_id, damage.damage)
                .saturating_add(rejected);
            if reflected > 0 && damage.proc_depth < 8 {
                let reflected = Damage {
                    notification: None,
                    source_kind: models::enums::actor::CombatActorKind::Player,
                    skill_damage_adjusted: false,
                    target_id: damage.attacker_id,
                    attacker_id: character.char_id,
                    credit_id: character.char_id,
                    damage: reflected,
                    healing: 0,
                    attacked_at: tick,
                    damage_motion: 0,
                    battle_flags: 0,
                    skill_id: 0,
                    skill_level: 0,
                    proc_depth: damage.proc_depth + 1,
                    defenses_applied: true,
                    landed: false,
                    magic_context: None,
                    right_hand_damage: None,
                }
                .with_action_notification(
                    character.current_map_name(),
                    character.current_map_instance(),
                    character.x,
                    character.y,
                    tick,
                    1,
                    0,
                    models::enums::action::ActionType::AttackNomotion,
                    (reflected.min(i32::MAX as u32) as i32, 0),
                );
                let player_actor =
                    state.characters().contains_key(&reflected.target_id) || (1_000_000_000..1_300_000_000).contains(&reflected.target_id);
                if player_actor {
                    self.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage: reflected }));
                } else if let Some(instance) = state.get_map_instance_from_character(&character) {
                    instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: reflected }));
                }
            }
            if let Some(Some(protector)) = devotion {
                let notification = damage.notification.map(|mut notification| {
                    notification.target_override = Some(damage.target_id);
                    notification
                });
                redirected = Some(Damage {
                    notification,
                    target_id: protector,
                    damage: damage.damage,
                    battle_flags: 0,
                    skill_id: models::enums::skill_enums::SkillEnum::CrDevotion.id(),
                    skill_level: 0,
                    landed: false,
                    defenses_applied: true,
                    magic_context: None,
                    damage_motion: 0,
                    ..damage
                });
                damage.damage = 0;
            }
        }
        if damage.damage > 0 {
            let no_walk_delay = target_snapshot
                .bonuses_raw()
                .iter()
                .any(|bonus| matches!(bonus, BonusType::EnableNoWalkDelay));
            if !no_walk_delay && damage.battle_flags != 0 {
                if character.is_moving() {
                    self.character_service().cancel_movement(&mut character, tick);
                }
                let walk_delay = u128::from(damage.damage_motion) * GlobalConfigService::battle_option("pc_damage_walk_delay_rate") as u128 / 100;
                character.timing.set_canmove_tick(tick + walk_delay.max(1));
            }
            let no_cast_cancel = ScriptSkillService::cast_interruption_protected(&target_snapshot, &map_flags);
            if damage.battle_flags != 0
                && !no_cast_cancel
                && character.is_using_skill()
                && !character.skill_has_been_used()
                && self.script_skill_service().active_cast_cancelable(&character)
            {
                character.clear_skill_in_use();
                self.script_skill_service().cancel_queued_cast(&mut character);
                let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes());
                self.character_service()
                    .send_area_notification_around_characters(&character, packet);
            }
            if damage.battle_flags != 0
                && !no_cast_cancel
                && character.script_skill_state.casting_until > tick
                && self.script_skill_service().active_cast_cancelable(&character)
            {
                self.script_skill_service().cancel_queued_cast(&mut character);
                let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes());
                self.character_service()
                    .send_area_notification_around_characters(&character, packet);
            }
            if damage.battle_flags != 0 {
                character.clear_pending_skill();
            }
            damage.damage = damage.damage.min(character.status.hp);
            reported = i64::from(damage.damage);
            if self.character_service().take_damage(&mut character, damage.damage) {
                character.pending_item_skill = None;
                character.pending_craft = None;
                character.clear_attack();
                character.clear_skill_in_use();
                character.clear_movement();
                self.script_skill_service().cancel_queued_cast(&mut character);
                for kind in StatusEffectService::remove_on_death(&mut character.status) {
                    StatusEffectService::send_icon(&character, kind, false, tick, &sender);
                }
                StatusEffectService::send_visual_status(&character, &sender);
            }
        }
        state.insert_character(character);
        if redirected.is_none() {
            original_damage.notify_admitted(&sender, reported, self.packetver());
        }
        if state
            .characters()
            .get(&damage.target_id)
            .is_some_and(|target| target.status.hp == 0 || target.is_dead())
        {
            self.cancel_player_trade(state, damage.target_id)?;
            self.apply_death_penalty(state, damage.target_id);
            self.apply_nightmare_drops(state, damage.target_id);
            self.apply_pvp_death(state, damage.target_id, damage.credit_id.max(damage.attacker_id));
            self.battleground_member_died(state, damage.target_id);
            let killer_id = damage.credit_id.max(damage.attacker_id);
            let killer_account = state.characters().get(&killer_id).filter(|killer| killer.char_id != damage.target_id).map(|killer| killer.account_id);
            let victim_account = state.characters().get(&damage.target_id).map(|victim| victim.account_id);
            self.broadcast_player_npc_event(state, damage.target_id, "OnPCDieEvent", killer_account.map(|account| ("killerrid", account as i32)));
            if let (Some(_), Some(victim_account)) = (killer_account, victim_account) {
                self.broadcast_player_npc_event(state, killer_id, "OnPCKillEvent", Some(("killedrid", victim_account as i32)));
            }
        }
        if damage.damage > 0
            && damage.landed
            && damage.battle_flags != 0
            && !source_player
            && other_mob_id.is_some()
            && GlobalConfigService::instance().config().game.pet_support.damage_support
            && state.characters().get(&damage.target_id).is_some_and(|owner| {
                owner
                    .game_systems
                    .pet
                    .as_ref()
                    .is_some_and(|pet| !pet.incubating && pet.intimacy > 0)
            })
        {
            self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                char_id: damage.target_id,
                request: crate::server::model::game_systems::ScriptWorldRequest::Pet(crate::server::model::game_systems::PetRequest::PetCombatTarget {
                    target_id: damage.attacker_id,
                    retaliation: true,
                }),
            }));
        }
        if let Some(redirected) = redirected {
            if let Some(protector) = state
                .characters_mut()
                .get_mut(&redirected.target_id)
                .filter(|protector| protector.is_sitting())
            {
                self.character_service().stand(protector);
            }
            self.admit_character_damage(state, redirected, tick)?;
        }
        if damage.damage > 0 && damage.landed && damage.battle_flags != 0 {
            if damage.credit_id != 0
                && damage.attacker_id != damage.credit_id
                && (1_100_000_000..1_200_000_000).contains(&damage.attacker_id)
                && damage.battle_flags & BattleFlag::Weapon.as_flag() != 0
            {
                self.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                    char_id: damage.credit_id,
                    request: crate::server::model::game_systems::ScriptWorldRequest::Companion(crate::server::model::game_systems::CompanionRequest::CompanionAttackLanded {
                        id: damage.attacker_id,
                        damage: damage.damage,
                    }),
                }));
            }
            let grand_cross_self =
                damage.attacker_id == damage.target_id && damage.skill_id == models::enums::skill_enums::SkillEnum::CrGrandcross.id();
            let hit_flags = if grand_cross_self {
                (damage.battle_flags & !BattleFlag::Magic.as_flag()) | BattleFlag::Weapon.as_flag()
            } else {
                damage.battle_flags
            };
            self.add_to_next_tick(GameEvent::ScriptCombat(script_combat_service::ScriptCombatRequest {
                source_id: damage.target_id,
                target_id: damage.attacker_id,
                trigger: CombatTrigger::Hit,
                battle_flags: hit_flags,
                skill_id: damage.skill_id,
                damage: damage.damage,
                other_mob_id,
                depth: damage.proc_depth,
                drop_position: None,
                right_hand_damage: original_damage.admitted_right_hand_damage(damage.damage),
                origin_map: state
                    .characters()
                    .get(&damage.target_id)
                    .map(|target| target.map_instance_key.clone()),
            }));
            if source_player && !grand_cross_self {
                self.add_to_next_tick(GameEvent::ScriptCombat(script_combat_service::ScriptCombatRequest {
                    source_id: damage.attacker_id,
                    target_id: damage.target_id,
                    trigger: CombatTrigger::Attack,
                    battle_flags: damage.battle_flags,
                    skill_id: damage.skill_id,
                    damage: damage.damage,
                    other_mob_id: None,
                    depth: damage.proc_depth,
                    drop_position: None,
                    right_hand_damage: original_damage.admitted_right_hand_damage(damage.damage),
                    origin_map: state
                        .characters()
                        .get(&damage.target_id)
                        .map(|target| target.map_instance_key.clone()),
                }));
            }
            if damage.skill_id != 0 && damage.skill_level != 0 && damage.battle_flags & BattleFlag::Skill.as_flag() != 0 {
                self.add_to_next_tick(GameEvent::ScriptSkillHit(crate::server::script::skill::ScriptSkillHit {
                    source_map: Some(origin_map.map_name().clone()),
                    source_instance: Some(origin_map.map_instance()),
                    source_id: damage.attacker_id,
                    target_id: damage.target_id,
                    skill_id: damage.skill_id,
                    skill_level: damage.skill_level,
                    damage: damage.damage,
                    depth: damage.proc_depth,
                }));
            }
        }
        Ok(())
    }

    fn guild_experience_tax(&self, character: &crate::server::state::character::Character, base: u32) -> u32 {
        if character.game_systems.guild_id == 0 || base == 0 {
            return 0;
        }
        match self.repository.guild(character.game_systems.guild_id) {
            Ok(Some(guild)) => (u64::from(base) * u64::from(guild.exp_tax(character.char_id)) / 100) as u32,
            Ok(None) => 0,
            Err(error) => {
                error!("Guild tax lookup failed: {error}");
                0
            }
        }
    }

    fn transfer_castle_on_emperium_break(
        &self,
        state: &ServerState,
        kill: &crate::server::model::events::game_event::CharacterKillMonster,
    ) {
        if kill.mob_id != 1288
            || !state.siege_active()
            || !state.map_flags(&kill.map_instance_key).enabled(MapFlag::GvgCastle)
        {
            return;
        }
        let guild = state
            .characters()
            .get(&kill.char_id)
            .map_or(0, |character| character.game_systems.guild_id);
        self.add_to_next_tick(GameEvent::CastleLifecycle(crate::server::model::events::game_event::CastleLifecycle::EmperiumBroken {
            map: kill.map_instance_key.map_name().to_string(),
            guild_id: guild,
        }));
    }

    pub(crate) fn reward_monster_kill(
        &self,
        state: &mut ServerState,
        kill: crate::server::model::events::game_event::CharacterKillMonster,
        tick: u128,
    ) -> Result<(), String> {
        let instance = state
            .get_map_instance(kill.map_instance_key.map_name(), kill.map_instance_key.map_instance())
            .ok_or("Loot map is unavailable")?;
        instance.add_to_delayed_tick(
            crate::server::model::events::map_event::MobDropItems {
                owner_id: kill.char_id,
                mob_id: kill.mob_id,
                mob_x: kill.mob_x,
                mob_y: kill.mob_y,
            }
            .into_event(state.get_character(kill.char_id).map(|character| character.game_systems.autoloot).unwrap_or_default()),
            400,
        );
        self.transfer_castle_on_emperium_break(state, &kill);
        self.reward_mvp(state, &kill);
        let config = &GlobalConfigService::instance().config().game;
        let battle = &GlobalConfigService::instance().config().battle;
        let (base_rate, job_rate) = (config.base_exp_rate * battle.get("base_exp_rate") as f32 / 100.0, config.job_exp_rate * battle.get("job_exp_rate") as f32 / 100.0);
        let flags = state.map_flags(&kill.map_instance_key);
        let shares = if flags.enabled(crate::server::model::map_flags::MapFlag::Pvp) && !config.pvp_exp {
            Default::default()
        } else {
            super::script_experience_service::monster_experience_awards_with_pets(
                state,
                &kill,
                config.party_even_share_bonus,
                config.pet_support.attack_exp_to_master,
                config.pet_support.attack_exp_rate,
            )
        };
        let mut guild_payouts: Vec<(u32, u64)> = Vec::new();
        let awards: Vec<_> = shares
            .into_iter()
            .filter(|(_, (base, job))| *base != 0 || *job != 0)
            .map(|(id, (base, job))| {
                let character = state.characters().get(&id).ok_or("Experience recipient is unavailable")?;
                let scaled_base = (base as f32 * base_rate).ceil() as u32;
                let tax = self.guild_experience_tax(character, scaled_base);
                if tax > 0 {
                    guild_payouts.push((id, u64::from(tax)));
                }
                Ok(crate::repository::script_character_repository::ScriptExperienceAward {
                    char_id: id,
                    account_id: character.account_id,
                    plan: super::script_character_service::plan_raw_experience(
                        character,
                        scaled_base - tax,
                        (job as f32 * job_rate).ceil() as u32,
                    )?,
                })
            })
            .collect::<Result<_, String>>()?;
        if !awards.is_empty() {
            self.repository
                .character_commit_experience_awards(&awards)
                .map_err(|error| error.to_string())?;
            let guild_exp_rate = GlobalConfigService::instance().config().char_server.guild_exp_rate;
            for (char_id, amount) in guild_payouts {
                let amount = super::char_server_service::guild_exp_gain(amount, guild_exp_rate);
                if amount == 0 {
                    continue;
                }
                if let Err(error) =
                    self.repository
                        .guild_add_experience(char_id, amount, &super::script_world_service::world_data().guild_experience)
                {
                    error!("Guild experience tax for {char_id} was not recorded: {error}");
                }
            }
            for award in &awards {
                if let Some(character) = state.characters_mut().get_mut(&award.char_id) {
                    super::script_character_service::apply_experience(self, character, &award.plan);
                }
            }
        }
        let owners: std::collections::BTreeSet<_> = if kill.contributions.is_empty() {
            [kill.char_id].into_iter().collect()
        } else {
            kill.contributions
                .iter()
                .filter(|entry| entry.damage > 0)
                .map(|entry| entry.owner_id)
                .collect()
        };
        let mut first_error = None;
        for id in owners {
            if let Some(mut character) = state.characters_mut().remove(&id) {
                if character.map_instance_key == kill.map_instance_key && !character.is_dead() && character.status.hp > 0 {
                    if let Err(error) = self
                        .script_world_service()
                        .record_companion_kill(self, &mut character, &kill, &flags, tick as u64)
                    {
                        first_error.get_or_insert(error);
                    }
                }
                state.insert_character(character);
            }
        }
        first_error.map_or(Ok(()), Err)
    }

    fn reward_mvp(&self, state: &mut ServerState, kill: &crate::server::model::events::game_event::CharacterKillMonster) {
        let mob = GlobalConfigService::instance().get_mob(i32::from(kill.mob_id));
        if !mob.is_mvp() {
            return;
        }
        let flags = state.map_flags(&kill.map_instance_key);
        let Some(mvp) = state
            .characters()
            .get(&kill.char_id)
            .filter(|character| !character.is_dead() && character.map_instance_key == kill.map_instance_key)
        else {
            return;
        };
        let (char_id, account_id, x, y) = (mvp.char_id, mvp.account_id, mvp.x, mvp.y);
        let sender = self.server_service().notification_sender();
        let mut effect = 0x010C_u16.to_le_bytes().to_vec();
        effect.extend_from_slice(&char_id.to_le_bytes());
        let _ = sender.try_send(Notification::Area(AreaNotification::new(
            kill.map_instance_key.map_name().clone(),
            kill.map_instance_key.map_instance(),
            AreaNotificationRangeType::Fov { x, y, exclude_id: None },
            effect,
        )));
        let config = &GlobalConfigService::instance().config().game;
        if mob.mvp_exp > 0 && !flags.enabled(crate::server::model::map_flags::MapFlag::NoBaseExp) {
            let exp = mob.mvp_exp as u32;
            let plan = state
                .characters()
                .get(&char_id)
                .ok_or_else(|| "MVP is unavailable".to_string())
                .and_then(|character| super::script_character_service::plan_raw_experience(character, exp, 0));
            match plan {
                Ok(plan) => {
                    let award = crate::repository::script_character_repository::ScriptExperienceAward { char_id, account_id, plan };
                    match self.repository.character_commit_experience_awards(std::slice::from_ref(&award)) {
                        Ok(_) => {
                            if let Some(character) = state.characters_mut().get_mut(&char_id) {
                                super::script_character_service::apply_experience(self, character, &award.plan);
                            }
                            let mut packet = 0x010B_u16.to_le_bytes().to_vec();
                            packet.extend_from_slice(&exp.to_le_bytes());
                            let _ = sender.try_send(Notification::Char(CharNotification::new(char_id, packet)));
                        }
                        Err(error) => error!("MVP experience for {char_id} was not recorded: {error}"),
                    }
                }
                Err(error) => warn!("MVP experience skipped: {error}"),
            }
        }
        if flags.enabled(crate::server::model::map_flags::MapFlag::NoMvpLoot) {
            return;
        }
        let prize = mob.mvp_drops.iter().find(|drop| {
            let rate = (drop.rate as f32 * config.drop_rate_mvp).round() as u32;
            rate >= 10_000 || fastrand::u32(0..10_000) < rate
        });
        let Some(prize) = prize else {
            return;
        };
        let item = GlobalConfigService::instance().get_item(prize.item_id);
        let mut packet = 0x010A_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&(prize.item_id as u16).to_le_bytes());
        let _ = sender.try_send(Notification::Char(CharNotification::new(char_id, packet)));
        let identified = !item.item_type.should_be_identified_when_dropped();
        let prize_item = crate::repository::model::item_model::InventoryItemModel::from_item_model(&item, 1, identified);
        let fits = state.characters().get(&char_id).is_some_and(|character| self.mvp_prize_fits(character, &prize_item));
        if fits {
            self.add_to_next_tick(GameEvent::CharacterAddItems(crate::server::model::events::game_event::CharacterAddItems {
                char_id,
                should_perform_check: true,
                buy: false,
                items: vec![prize_item],
            }));
        } else if let Some(map) = state.characters().get(&char_id).and_then(|character| state.get_map_instance_from_character(character)) {
            map.add_to_next_tick(MapEvent::ScriptDropItem(crate::server::model::events::map_event::ScriptDropItem {
                owner_id: char_id,
                item_id: prize.item_id,
                amount: 1,
                x,
                y,
            }));
        }
    }

    pub(crate) fn mvp_prize_fits(&self, character: &crate::server::state::character::Character, item: &crate::repository::model::item_model::InventoryItemModel) -> bool {
        let weight = u64::from(character.weight()) + item.weight.max(0) as u64 * item.amount.max(0) as u64;
        if weight > u64::from(self.character_service().max_weight(character)) {
            return false;
        }
        let stacks_onto_existing = item.item_type().is_stackable()
            && character.inventory.iter().flatten().any(|held| held.item_id == item.item_id && held.amount.checked_add(item.amount).is_some());
        stacks_onto_existing || character.inventory.iter().flatten().count() < usize::from(GlobalConfigService::instance().config().game.max_inventory)
    }
}
