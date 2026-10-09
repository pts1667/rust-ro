use async_trait::async_trait;
use configuration::configuration::PetSupportConfig;
use models::status::Status;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value};

use super::{ScriptWorldService, pet_world_id, protocol, world_data};
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, ScriptWorld, CharacterStatusChange, CharacterEndStatus};
use crate::server::model::game_systems::{PetRecord, PetRecovery, PetSupportCast, PetSupportRuntime, PetSupportSkill, PetTimedBonus, ScriptWorldRequest, CompanionRequest, PetRequest};
use crate::server::script::bonus::BonusScriptHandler;
use crate::server::script::item_script_handler::ItemScriptHost;
use crate::server::script::skill::{metadata::SkillMetadata, ScriptSkillService};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub fn pet_support_operation(function: Function) -> bool {
    matches!(function, Function::PetSkillBonus | Function::PetRecovery | Function::PetSkillSupport | Function::PetSkillAttack | Function::PetSkillAttack2 | Function::PetLoot
        | Function::PetAutoBonus | Function::PetAutoBonus2 | Function::PetAutoBonus3)
}

fn milliseconds(args: &[Value], index: usize) -> Result<u64, String> {
    let value = args.get(index).ok_or("Missing pet timer")?.number_value()?;
    u16::try_from(value).map(|value| u64::from(value) * 1000).map_err(|_| "Pet timer is out of range".into())
}

fn configure(support: &mut PetSupportRuntime, pet: &PetRecord, config: &PetSupportConfig, function: Function, args: &[Value], now: u64) -> Result<(), String> {
    let armed = !config.require_accessory || pet.equipped_item > 0;
    support.pet_id = pet.id;
    support.requires_accessory = config.require_accessory;
    match function {
        Function::PetAutoBonus | Function::PetAutoBonus2 | Function::PetAutoBonus3 => {
            let handler = BonusScriptHandler::new();
            handler.bonuses.write().unwrap().extend(support.auto_bonuses.iter().copied());
            handler.register_pet_auto_bonus(function, args, pet.id, pet.class_id)?;
            support.auto_bonuses = handler.drain();
        }
        Function::PetLoot => {
            support.loot = Some(crate::server::model::game_systems::PetLootRuntime {
                capacity: super::pet_loot::loot_capacity(args)?, next_at: now, return_requested: false, target: None, claim_queued: false,
            });
        }
        Function::PetSkillBonus => {
            let bonus = BonusScriptHandler::new();
            bonus.apply(Function::Bonus, args.get(..2).ok_or("Pet bonus requires type and value")?.to_vec())?;
            let delay_ms = milliseconds(args, 3)?;
            support.bonus = Some(PetTimedBonus { bonuses: bonus.drain(), duration_ms: milliseconds(args, 2)?, delay_ms,
                next_at: armed.then_some(now.saturating_add(delay_ms)), active: false });
        }
        Function::PetRecovery => {
            let kind = match args.first().ok_or("Missing pet recovery status")? {
                Value::String(name) => StatusChangeKind::from_name(name),
                Value::Number(id) => StatusChangeKind::from_id(*id),
                _ => None,
            }.ok_or("Unknown pet recovery status")?;
            support.recovery = Some(PetRecovery { kind, delay_ms: milliseconds(args, 1)?, next_at: None });
        }
        Function::PetSkillSupport => {
            let skill = crate::server::service::global_config_service::GlobalConfigService::instance()
                .find_skill_config(args.first().ok_or("Missing pet support skill")?).ok_or("Unknown pet support skill")?;
            let metadata = SkillMetadata::find(skill.id).ok_or("Pet skill has no classic definition")?;
            if !matches!(crate::server::script::skill::ScriptSkillService::actor_behaviour(metadata, metadata.max_level), skills::ActorBehaviour::Heal)
                && metadata.status.as_deref().and_then(StatusChangeKind::from_name).is_none()
            {
                return Err("Pet support skill has no healing or status effect".into());
            }
            let level = u8::try_from(args.get(1).ok_or("Missing pet skill level")?.number_value()?).map_err(|_| "Pet skill level is out of range")?;
            if level == 0 || level > metadata.max_level { return Err("Invalid pet support skill level".into()); }
            let threshold = |index: usize| -> Result<u8, String> {
                let value = u8::try_from(args.get(index).ok_or("Missing pet support threshold")?.number_value()?).map_err(|_| "Pet threshold is out of range")?;
                if value > 100 { return Err("Pet threshold must be between 0 and 100".into()); }
                Ok(value)
            };
            let delay_ms = milliseconds(args, 2)?;
            support.skill = Some(PetSupportSkill { skill_id: skill.id, level, delay_ms, hp_threshold: threshold(3)?, sp_threshold: threshold(4)?,
                next_at: armed.then_some(now.saturating_add(delay_ms)) });
            support.casting = None;
        }
        Function::PetSkillAttack | Function::PetSkillAttack2 => {
            support.attack = Some(super::pet_combat::configure_pet_attack(function, args)?);
        }
        _ => return Err("Unknown pet support operation".into()),
    }
    Ok(())
}

pub(crate) struct PetSupportHost {
    inner: ItemScriptHost,
    pub support: PetSupportRuntime,
    pet: PetRecord,
    config: PetSupportConfig,
    now: u64,
    error: Option<String>,
}

impl PetSupportHost {
    pub(crate) fn new(status: Status, pet: PetRecord, config: PetSupportConfig, now: u64) -> Self {
        Self { inner: ItemScriptHost::bonuses(status, 0), support: PetSupportRuntime { pet_id: pet.id, initialized: true,
            requires_accessory: config.require_accessory, ..PetSupportRuntime::default() }, pet, config, now, error: None }
    }

    pub(crate) fn into_support(mut self) -> Result<PetSupportRuntime, String> {
        if let Some(error) = self.error.or(self.inner.error) { return Err(error); }
        self.support.base_bonuses = self.inner.bonuses.drain();
        Ok(self.support)
    }
}

#[async_trait]
impl Host for PetSupportHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        if let Request::Call { function, arguments } = &request {
            if pet_support_operation(*function) {
                if let Err(error) = configure(&mut self.support, &self.pet, &self.config, *function, arguments, self.now) {
                    self.error = Some(error.clone());
                    return Err(error);
                }
                return Ok(Value::default());
            }
        }
        let result = self.inner.invoke(request).await;
        if let Err(error) = &result { self.error = Some(error.clone()); }
        result
    }
}

fn active_pet(character: &Character) -> Result<&PetRecord, String> {
    character.game_systems.pet.as_ref().filter(|pet| !pet.incubating && pet.intimacy > 0).ok_or("Pet support requires an active pet".into())
}

impl ScriptWorldService {
    pub(crate) fn validate_pet_support(&self, character: &Character, function: Function, args: &[Value], now: u64) -> Result<(), String> {
        if function == Function::PetLoot {
            super::plan_persistent_effects(character, &[(function, args.to_vec())], now)?;
            return Ok(());
        }
        let pet = active_pet(character)?;
        let mut support = character.game_systems.pet_support.clone().filter(|support| support.pet_id == pet.id).unwrap_or_default();
        configure(&mut support, pet, self.pet_configuration(), function, args, now)
    }

    pub(crate) fn initialize_pet_support(&self, server: &Server, character: &mut Character, now: u64) -> Result<(), String> {
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating).cloned() else {
            character.game_systems.pet_support = None;
            return Ok(());
        };
        if character.game_systems.pet_support.as_ref().is_some_and(|support| support.pet_id == pet.id && support.initialized) { return Ok(()); }
        let config = self.pet_configuration().clone();
        let configured_loot = character.game_systems.pet_support.as_ref().filter(|support| support.pet_id == pet.id).and_then(|support| support.loot.clone());
        let runtime = PetSupportRuntime { pet_id: pet.id, initialized: true, requires_accessory: config.require_accessory, ..PetSupportRuntime::default() };
        character.game_systems.pet_support = Some(runtime);
        if config.status_support && world_data().pets.iter().any(|definition| definition.class_id == pet.class_id && definition.has_support_script) {
            character.refresh_script_context();
            let host = PetSupportHost::new(character.status.clone(), pet.clone(), config, now);
            let (host, result) = futures::executor::block_on(server.item_service().item_script_vm.run_pet_support(host, u32::from(pet.class_id)));
            result?;
            character.game_systems.pet_support = Some(host.into_support()?);
        }
        if let Some(loot) = configured_loot { character.game_systems.pet_support.as_mut().unwrap().loot = Some(loot); }
        Ok(())
    }

    pub(crate) fn call_pet_support(&self, server: &Server, state: &ServerState, character: &mut Character, function: Function, args: &[Value], now: u64) -> Result<Value, String> {
        self.initialize_pet_support(server, character, now)?;
        if function == Function::PetLoot { return self.call_pet_loot(server, state, character, args, now); }
        let pet = active_pet(character)?.clone();
        let mut support = character.game_systems.pet_support.clone().ok_or("Pet support state is unavailable")?;
        configure(&mut support, &pet,
            self.pet_configuration(), function, args, now)?;
        character.game_systems.pet_support = Some(support);
        Ok(Value::default())
    }

    pub fn pet_status_started(&self, character: &mut Character, kind: StatusChangeKind, now: u128) {
        let config = self.pet_configuration();
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating && (!config.require_accessory || pet.equipped_item > 0)) else { return; };
        let Some(recovery) = character.game_systems.pet_support.as_mut().filter(|support| support.pet_id == pet.id).and_then(|support| support.recovery.as_mut()) else { return; };
        if recovery.kind == kind && recovery.next_at.is_none() { recovery.next_at = Some((now as u64).saturating_add(recovery.delay_ms)); }
    }

    pub(crate) fn tick_pet_support(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64) -> Result<(), String> {
        let previous = super::pet_bonus_state(character);
        self.initialize_pet_support(server, character, now)?;
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating).cloned() else { return Ok(()); };
        let config = self.pet_configuration();
        let armed = pet.intimacy > 0 && (!config.require_accessory || pet.equipped_item > 0);
        let mut cure = None;
        let mut cast = None;
        let mut cancelled_cast = false;
        let no_skill = state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoSkill);
        if !(config.attack_support || config.damage_support) || pet.hunger == 0 || pet.intimacy < i32::from(config.minimum_intimacy) || character.status.hp == 0 {
            if let Some(command) = character.game_systems.companion_commands.get_mut(&pet_world_id(pet.id)) { command.target = None; }
        }
        if let Some(support) = character.game_systems.pet_support.as_mut() {
            support.requires_accessory = config.require_accessory;
            if let Some(casting) = &support.casting {
                if !armed || character.status.hp == 0 || no_skill || casting.map_key != character.map_instance_key {
                    support.casting = None;
                    cancelled_cast = true;
                }
            }
            if let Some(bonus) = &mut support.bonus {
                if !armed { bonus.next_at = None; bonus.active = false; }
                else {
                    let next = bonus.next_at.get_or_insert(now.saturating_add(bonus.delay_ms));
                    if now >= *next {
                        bonus.active = !bonus.active || bonus.delay_ms == 0;
                        *next = now.saturating_add(if bonus.active { bonus.duration_ms } else { bonus.delay_ms }.max(40));
                    }
                }
            }
            if let Some(recovery) = &mut support.recovery {
                if !armed { recovery.next_at = None; }
                else if recovery.next_at.is_some_and(|next| now >= next) {
                    cure = Some(recovery.kind);
                    recovery.next_at = None;
                }
            }
        }
        if cancelled_cast {
            let mut packet = protocol::header(0x01B9);
            packet.extend_from_slice(&pet_world_id(pet.id).to_le_bytes());
            self.area(character, packet)?;
        }
        if super::pet_bonus_state(character) != previous { character.refresh_script_context(); }
        if let Some(support) = character.game_systems.pet_support.as_mut() {
            if let Some(casting) = support.casting.as_mut().filter(|casting| !casting.queued && now >= casting.completes_at) {
                casting.queued = true;
                server.add_to_next_tick(GameEvent::ScriptWorld(ScriptWorld {
                    char_id: character.char_id, request: ScriptWorldRequest::Pet(PetRequest::FinishPetSupport(casting.id)),
                }));
            }
            if let Some(skill) = &mut support.skill {
                if !armed { skill.next_at = None; }
                else {
                    let next = skill.next_at.get_or_insert(now.saturating_add(skill.delay_ms));
                    if now >= *next {
                        let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
                        let sp = u64::from(character.status.sp) * 100 / u64::from(snapshot.max_sp().max(1));
                        let hp = u64::from(character.status.hp) * 100 / u64::from(snapshot.max_hp().max(1));
                        let wait = if sp > u64::from(skill.sp_threshold) { sp } else { hp };
                        if character.status.hp > 0 && hp <= u64::from(skill.hp_threshold) && sp <= u64::from(skill.sp_threshold)
                            && !no_skill && support.casting.is_none() && now >= support.can_act_at {
                            cast = Some((skill.skill_id, skill.level));
                            *next = now.saturating_add(skill.delay_ms.max(40));
                        } else { *next = now.saturating_add(wait.max(10).saturating_mul(100)); }
                    }
                }
            }
        }
        let actor_id = pet_world_id(pet.id);
        if let Some(kind) = cure.filter(|kind| character.status.has_status_change(*kind)) {
            server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: character.char_id, kind: Some(kind) }));
            self.pet_skill_packet(server, character, actor_id, models::enums::skill_enums::SkillEnum::TfDetoxify.id(), 1)?;
            self.area(character, protocol::emotion(actor_id, 0))?;
        }
        if let Some((skill_id, level)) = cast { self.begin_pet_support(server, state, character, &pet, skill_id, level, now)?; }
        Ok(())
    }

    fn begin_pet_support(&self, server: &Server, state: &ServerState, character: &mut Character, pet: &PetRecord, skill_id: u32, level: u8, now: u64) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Pet support skill disappeared")?;
        let data = self.configuration.get_mob_safe(i32::from(pet.class_id)).ok_or("Pet monster data is unavailable")?;
        let snapshot = crate::server::model::status::StatusFromDb::from_mob_model(data);
        let cast_time = super::companion_skills::companion_cast_time(&snapshot, metadata, level);
        let id = self.next_pet_cast_id.try_update(std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed, |value| value.checked_add(1))
            .map_err(|_| "Pet cast identifiers exhausted")?;
        let support = character.game_systems.pet_support.as_mut().ok_or("Pet support state is unavailable")?;
        support.casting = Some(PetSupportCast { id, skill_id, level, target_id: character.char_id, attack: None,
            map_key: character.map_instance_key.clone(), completes_at: now.saturating_add(cast_time), queued: false });
        let actor_id = pet_world_id(pet.id);
        let command = character.game_systems.companion_commands.entry(actor_id).or_default();
        command.target = None;
        command.destination = None;
        command.can_act_at = now.saturating_add(cast_time);
        command.next_move_at = command.can_act_at;
        if cast_time == 0 { return self.finish_pet_support(server, state, character, id, now); }
        let mut packet = protocol::header(0x013E);
        packet.extend_from_slice(&actor_id.to_le_bytes());
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&character.x.to_le_bytes());
        packet.extend_from_slice(&character.y.to_le_bytes());
        packet.extend_from_slice(&(skill_id as u16).to_le_bytes());
        packet.extend_from_slice(&0u32.to_le_bytes());
        packet.extend_from_slice(&(cast_time.min(u64::from(u32::MAX)) as u32).to_le_bytes());
        self.area(character, packet)
    }

    pub(crate) fn finish_pet_support(&self, server: &Server, state: &ServerState, character: &mut Character, id: u64, now: u64) -> Result<(), String> {
        let Some(support) = character.game_systems.pet_support.as_mut() else { return Ok(()); };
        if support.casting.as_ref().is_none_or(|casting| casting.id != id || now < casting.completes_at) { return Ok(()); }
        let casting = support.casting.take().unwrap();
        let metadata = SkillMetadata::find(casting.skill_id).ok_or("Pet support skill disappeared")?;
        let delay = metadata.after_cast_act_delay.as_ref().and_then(|delay| delay.value(casting.level, "Time")).unwrap_or(0).max(0) as u64;
        support.can_act_at = now.saturating_add(delay);
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| pet.id == support.pet_id && !pet.incubating && pet.intimacy > 0
            && (!support.requires_accessory || pet.equipped_item > 0)).cloned() else { return Ok(()); };
        if character.status.hp == 0 || casting.map_key != character.map_instance_key || state.map_flags(&casting.map_key).enabled(crate::server::model::map_flags::MapFlag::NoSkill) { return Ok(()); }
        if casting.attack.is_some() {
            let motion = self.configuration.get_mob_safe(i32::from(pet.class_id)).map_or(100, |mob| mob.atk_motion.max(100) as u64);
            support.can_act_at = now.saturating_add(delay.max(motion));
        }
        let command = character.game_systems.companion_commands.entry(pet_world_id(pet.id)).or_default();
        command.can_act_at = support.can_act_at;
        if let Some(attack) = casting.attack {
            return self.finish_pet_attack(server, state, character, &pet, &attack, casting.target_id, now);
        }
        self.cast_pet_support(server, character, &pet, casting.skill_id, casting.level, now)
    }

    fn cast_pet_support(&self, server: &Server, character: &Character, pet: &PetRecord, skill_id: u32, level: u8, now: u64) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Pet support skill disappeared")?;
        let actor_id = pet_world_id(pet.id);
        if matches!(crate::server::script::skill::ScriptSkillService::actor_behaviour(metadata, level), skills::ActorBehaviour::Heal) {
            let data = self.configuration.get_mob_safe(i32::from(pet.class_id)).ok_or("Pet monster data is unavailable")?;
            let snapshot = crate::server::model::status::StatusFromDb::from_mob_model(data);
            let hp = ScriptSkillService::heal_amount(&snapshot, u32::from(pet.level), level);
            server.add_to_next_tick(GameEvent::ScriptWorld(ScriptWorld { char_id: character.char_id,
                request: ScriptWorldRequest::Companion(CompanionRequest::HealByCompanion { source_id: actor_id, hp, sp: 0 }) }));
        } else if let Some(kind) = metadata.status.as_deref().and_then(StatusChangeKind::from_name) {
            server.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange { char_id: character.char_id,
                request: StatusChangeRequest::guaranteed(kind, metadata.duration(level, false).unwrap_or(0), i32::from(level)) }));
        }
        let _ = now;
        self.pet_skill_packet(server, character, actor_id, skill_id, level)
    }

    fn pet_skill_packet(&self, server: &Server, character: &Character, actor_id: u32, skill_id: u32, level: u8) -> Result<(), String> {
        let mut packet = packets::packets::PacketZcUseSkill::new(server.packetver());
        use packets::packets::Packet;
        packet.set_src_aid(actor_id);
        packet.set_target_aid(character.char_id);
        packet.set_skid(skill_id as u16);
        packet.set_level(i16::from(level));
        packet.set_result(true);
        packet.fill_raw();
        self.area(character, packet.raw)
    }
}
