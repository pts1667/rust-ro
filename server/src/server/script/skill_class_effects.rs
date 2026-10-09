use models::enums::actor::CombatActorKind;
use models::enums::class::JobName;
use models::enums::mob::{MobCapability, MobStealFlag};
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithMaskValueU8, EnumWithNumberValue};
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use skills::ClassEffect;

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterAddItems, CharacterDamage, CharacterZeny, GameEvent};
use crate::server::model::events::map_event::{MapEvent, MobDamage, MobEndStatus, MobHeal, MobMarkStolen, MobStatusChange, ScriptMobCombat, ScriptSpawn};
use crate::server::model::map_instance::MapInstance;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::mob::Mob;
use crate::server::state::server::ServerState;

const EMOTION_QUESTION: u8 = 1;
const SOUL_LINK_MISUSE_STUN_MS: i32 = 500;
const ES_REPEAT_STUN_MS: i32 = 10_000;
const SP_PER_SPHERE: u32 = 7;
const REDEMPTIO_RESURRECTION_LEVEL: u8 = 3;
/// Chance for Absorb Spirits to drain SP from a monster.
const ABSORB_MOB_CHANCE_PERCENT: u32 = 20;
const SOUL_CHANGE_MOB_SP_PERCENT: u32 = 3;
const MINDBREAKER_BASE_CHANCE: u32 = 55;
const RANDOM_MONSTER_ATTEMPTS: usize = 64;
const MAX_RANDOM_MONSTER_LEVEL: i32 = 99;
const PORING_MOB_ID: u32 = 1002;

impl ScriptSkillService {
    /// Whether the class dispatch handles this skill: a declared class effect, or a performance song.
    fn is_class_skill(metadata: &SkillMetadata, level: u8) -> bool {
        Self::class_effect(metadata, level).is_some() || Self::is_performance_skill(metadata, level)
    }

    pub(super) fn timed_request(effect: &ScriptSkillEffect, kind: StatusChangeKind) -> StatusChangeRequest {
        StatusChangeRequest {
            kind,
            duration_ms: SkillMetadata::find(effect.skill_id).and_then(|metadata| metadata.duration(effect.level, false)).unwrap_or(0),
            values: [i32::from(effect.level), 0, 0, 0],
            rate: 10_000,
            flags: 0,
        }
    }

    fn punish_caster(&self, server: &Server, caster: &Character, effect: &ScriptSkillEffect, duration_ms: i32) {
        let mut on_caster = effect.clone();
        on_caster.target_id = effect.source_char_id;
        let mut stun = StatusChangeRequest::guaranteed(StatusChangeKind::Stun, duration_ms, i32::from(effect.level));
        stun.flags = 0;
        self.queue_delayed_status(server, caster, &on_caster, stun);
    }

    /// The Estin family of buffs only works between linked players: the caster, their partner or child, any Soul Linker, or anyone while the caster is linked by a Soul Linker.
    fn soul_linker_target_allowed(source: &Character, target: &Character) -> bool {
        source.char_id == target.char_id
            || target.status.job == JobName::SoulLinker.value() as u32
            || source.game_systems.partner_id == target.char_id
            || source.game_systems.child_id == target.char_id
            || super::ScriptSkillService::spirit_rules(source.status.status_change(StatusChangeKind::Spirit)).links_soul_linker_targets
    }

    fn kill_damage(effect: &ScriptSkillEffect, target_id: u32, amount: u32, source_kind: CombatActorKind) -> Damage {
        Damage {
            notification: None,
            source_kind,
            skill_damage_adjusted: false,
            target_id,
            attacker_id: effect.source_char_id,
            damage: amount,
            healing: 0,
            right_hand_damage: None,
            attacked_at: 0,
            damage_motion: 0,
            battle_flags: BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
            skill_id: effect.skill_id,
            skill_level: effect.level,
            landed: true,
            proc_depth: effect.proc_depth,
            credit_id: effect.source_char_id,
            defenses_applied: true,
            magic_context: None,
        }
    }

    fn grant_zeny(&self, server: &Server, character: &mut Character, amount: u32) -> Result<(), String> {
        let zeny = server
            .repository
            .character_adjust_zeny(character.char_id, character.account_id, character.status.zeny, i64::from(amount))
            .map_err(|error| error.to_string())?;
        character.status.zeny = zeny;
        server.add_to_next_tick(GameEvent::CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }));
        Ok(())
    }

    pub fn consume_status_charge(&self, server: &Server, char_id: u32, kind: StatusChangeKind) {
        let effect = ScriptSkillEffect {
            source_char_id: char_id,
            target_id: char_id,
            skill_id: SkillEnum::SlKaite.id(),
            level: 1,
            heal_value: 0,
            proc_depth: 0,
            skill_event_emitted: true,
            cast_generation: 0,
            action: ScriptSkillAction::ConsumeCharge { kind },
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        };
        server.add_to_next_tick(GameEvent::CharacterScriptSkill(effect));
    }

    fn grant_sp(server: &Server, character: &mut Character, sp: u32) {
        let max_sp = StatusService::instance().to_snapshot(&character.status).max_sp();
        server.character_service().update_hp_sp(character, character.status.hp, character.status.sp.saturating_add(sp).min(max_sp));
    }

    fn mind_breaker_request(effect: &ScriptSkillEffect) -> StatusChangeRequest {
        StatusChangeRequest {
            kind: StatusChangeKind::MindBreaker,
            duration_ms: SkillMetadata::find(effect.skill_id).and_then(|metadata| metadata.duration(effect.level, true)).unwrap_or(30_000),
            values: [i32::from(effect.level), 0, 0, 0],
            rate: ((MINDBREAKER_BASE_CHANCE + 5 * u32::from(effect.level)) * 100) as u16,
            flags: 0,
        }
    }

    /// Effects whose target is a player (or the caster themselves), `character` is the target.
    pub(super) fn apply_class_skill(&self, server: &Server, state: &ServerState, character: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<bool, String> {
        let Some(metadata) = SkillMetadata::find(effect.skill_id) else { return Ok(false) };
        if !Self::is_class_skill(&metadata, effect.level) {
            return Ok(false);
        }
        let on_self = effect.source_char_id == character.char_id;
        match Self::class_effect(&metadata, effect.level) {
            Some(ClassEffect::AbsorbSpirits) => {
                if character.status.job == JobName::Gunslinger.value() as u32 {
                    return Err("Coins cannot be absorbed".into());
                }
                if !on_self && !state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::Pvp) && !state.map_flags(&character.map_instance_key).is_gvg() {
                    return Err("Absorb Spirits only works on enemies".into());
                }
                character.script_skill_state.expire_spheres(tick);
                let spheres = character.script_skill_state.spirit_spheres.len() as u32;
                if spheres == 0 {
                    return Err("The target has no spirit spheres".into());
                }
                character.script_skill_state.spirit_spheres.clear();
                character.script_skill_state.coins = 0;
                character.status.spirit_sphere_count = 0;
                self.notify_spheres(character);
                if on_self {
                    Self::grant_sp(server, character, spheres * SP_PER_SPHERE);
                } else {
                    self.followup_action(server, effect, effect.source_char_id, ScriptSkillAction::Heal { hp: 0, sp: spheres * SP_PER_SPHERE });
                }
            }
            Some(ClassEffect::KiTranslation) => {
                let party_id = state.get_character(effect.source_char_id).map_or(0, |source| source.game_systems.party_id);
                if on_self || party_id == 0 || character.game_systems.party_id != party_id {
                    return Err("Ki Translation only works on another party member".into());
                }
                if character.status.job == JobName::Gunslinger.value() as u32 {
                    return Err("Gunslingers cannot hold spirit spheres".into());
                }
                character.script_skill_state.expire_spheres(tick);
                if character.script_skill_state.spirit_spheres.len() >= 5 {
                    return Err("The target already holds five spirit spheres".into());
                }
                let expiry = tick + metadata.duration(effect.level, false).unwrap_or(600_000).max(0) as u128;
                character.script_skill_state.add_sphere(expiry, 5);
                character.status.spirit_sphere_count = character.script_skill_state.spirit_spheres.len().min(u8::MAX as usize) as u8;
                self.notify_spheres(character);
            }
            Some(ClassEffect::Redemptio) => {
                let flags = state.map_flags(&character.map_instance_key);
                if flags.is_gvg() || flags.enabled(crate::server::model::map_flags::MapFlag::Battleground) {
                    return Err("Redemptio is disabled on this map".into());
                }
                let party_id = character.game_systems.party_id;
                let radius = metadata.splash(effect.level).unwrap_or(14).max(0) as u16;
                let fallen: Vec<u32> = state
                    .characters()
                    .values()
                    .filter(|member| {
                        party_id != 0
                            && member.game_systems.party_id == party_id
                            && member.status.hp == 0
                            && member.map_instance_key == character.map_instance_key
                            && member.x.abs_diff(character.x).max(member.y.abs_diff(character.y)) <= radius
                    })
                    .map(|member| member.char_id)
                    .collect();
                if fallen.is_empty() {
                    return Err("No fallen party member is in range".into());
                }
                for char_id in fallen {
                    server.add_to_next_tick(GameEvent::CharacterScriptSkill(ScriptSkillEffect {
                        source_char_id: character.char_id,
                        target_id: char_id,
                        skill_id: SkillEnum::AllResurrection.id(),
                        level: REDEMPTIO_RESURRECTION_LEVEL,
                        heal_value: 0,
                        proc_depth: effect.proc_depth,
                        skill_event_emitted: true,
                        cast_generation: 0,
                        action: ScriptSkillAction::Cast,
                        deferred_requirements: None,
                        prepared_outcome: None,
                        source_index: None,
                        source_item: None,
                    }));
                }
                server.character_service().update_hp_sp(character, 1, 0);
            }
            Some(ClassEffect::Marionette) => self.start_marionette(server, state, character, effect, tick)?,
            Some(ClassEffect::StarComfort { slot, kind }) => self.star_comfort(server, character, slot, kind, effect, tick)?,
            Some(ClassEffect::StarHate) => return Err("Hatred of the Sun, Moon and Stars only targets monsters".into()),
            Some(ClassEffect::ConjugalShare { hp }) => {
                let source_partner = if on_self { character.game_systems.partner_id } else { state.get_character(effect.source_char_id).map_or(0, |source| source.game_systems.partner_id) };
                let partner_id = if on_self { source_partner } else { character.char_id };
                if partner_id == 0 || partner_id != source_partner {
                    return Err("The caster is not married to the target".into());
                }
                let share = |partner: &Character| {
                    let snapshot = StatusService::instance().to_snapshot(&partner.status);
                    if hp { (snapshot.max_hp() / 10, 0) } else { (0, snapshot.max_sp() / 10) }
                };
                if on_self {
                    let partner = state.get_character(partner_id).ok_or("The partner is not online")?;
                    let (hp, sp) = share(partner);
                    self.followup_action(server, effect, partner_id, ScriptSkillAction::Heal { hp, sp });
                } else {
                    let (hp, sp) = share(character);
                    Self::restore(server, character, hp, sp);
                }
            }
            Some(ClassEffect::AidPotion) => self.aid_potion(server, state, character, effect)?,
            Some(ClassEffect::AidBerserkPotion) => self.aid_berserk_potion(server, character, tick)?,
            Some(ClassEffect::Twilight(stage)) => self.twilight_alchemy(server, character, stage)?,
            Some(ClassEffect::FullProtection) => {
                use models::enums::item::ItemType;
                let slots = [
                    (StatusChangeKind::ProtectWeapon, models::enums::item::EquipmentLocation::HandRight.as_flag() | models::enums::item::EquipmentLocation::HandLeft.as_flag(), ItemType::Weapon),
                    (StatusChangeKind::ProtectShield, models::enums::item::EquipmentLocation::HandLeft.as_flag(), ItemType::Armor),
                    (StatusChangeKind::ProtectArmor, models::enums::item::EquipmentLocation::Armor.as_flag(), ItemType::Armor),
                    (StatusChangeKind::ProtectHelm, models::enums::item::EquipmentLocation::HeadTop.as_flag(), ItemType::Armor),
                ];
                let protected: Vec<_> = slots
                    .into_iter()
                    .filter(|(_, location, item_type)| character.inventory.iter().flatten().any(|item| item.item_type() == *item_type && item.equip as u64 & location != 0))
                    .map(|(kind, _, _)| kind)
                    .collect();
                if protected.is_empty() {
                    return Err("The target wears nothing to protect".into());
                }
                for kind in protected {
                    StatusEffectService::start(server, character, Self::timed_request(effect, kind), tick, &self.client_notification_sender)?;
                }
            }
            Some(ClassEffect::Question) => {
                let mut packet = 0x00c0_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes());
                packet.push(EMOTION_QUESTION);
                self.notify_area(character, packet);
            }
            Some(ClassEffect::Gravity) => {}
            Some(ClassEffect::LevelUp) => {
                let required = server.character_service().next_base_level_required_exp(&character.status);
                if required != u32::MAX {
                    server.character_service().gain_base_exp_unrated(character, required / 10);
                }
            }
            Some(ClassEffect::InstantDeath) => {
                let damage = Self::kill_damage(effect, character.char_id, character.status.hp, CombatActorKind::Player);
                server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage }));
            }
            Some(ClassEffect::FullRecovery) => {
                if character.status.hp > 0 {
                    let snapshot = StatusService::instance().to_snapshot(&character.status);
                    server.character_service().update_hp_sp(character, snapshot.max_hp(), snapshot.max_sp());
                }
            }
            Some(ClassEffect::Coma) => {
                if character.status.hp > 0 {
                    server.character_service().update_hp_sp(character, character.status.hp.min(1), character.status.sp.min(1));
                }
            }
            Some(ClassEffect::Fortune) => {
                if !on_self {
                    return Err("Fortune only works on the caster".into());
                }
                self.grant_zeny(server, character, character.status.base_level.saturating_mul(100))?;
            }
            Some(ClassEffect::SummonMonster) => {
                let config = GlobalConfigService::instance();
                let mob_id = (0..RANDOM_MONSTER_ATTEMPTS)
                    .map(|_| fastrand::i32(1001..2000))
                    .find(|id| config.get_mob_safe(*id).is_some_and(|mob| mob.level <= MAX_RANDOM_MONSTER_LEVEL && mob.mvp_exp == 0))
                    .ok_or("No monster could be picked")?;
                let instance = state.get_map_instance_from_character(character).ok_or("Map instance is unavailable")?;
                instance.add_to_next_tick(MapEvent::ScriptSpawn(ScriptSpawn {
                    is_guardian: false,
                    mob_id,
                    x: i32::from(character.x),
                    y: i32::from(character.y),
                    name: "--ja--".into(),
                    amount: 1,
                    event: String::new(),
                    event_npc: None,
                    size: None,
                    ai: None,
                    owner_id: 0,
                    guardian: None,
                    bg_id: 0,
                    max_hp: None,
                    lifetime_ms: None,
                    reserved_id: None,
                    area_end: None,
                }));
            }
            Some(ClassEffect::HpConversion) => {
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                let cost = snapshot.max_hp() / 10;
                if character.status.hp <= cost {
                    return Err("Not enough HP to convert".into());
                }
                let sp = cost.saturating_mul(u32::from(effect.level));
                server.character_service().update_hp_sp(character, character.status.hp - cost, character.status.sp.saturating_add(sp).min(snapshot.max_sp()));
            }
            Some(ClassEffect::SoulChange) => {
                if on_self {
                    return Err("Soul Change needs another player".into());
                }
                let source = state.get_character(effect.source_char_id).ok_or("Caster disconnected")?;
                let max_sp = StatusService::instance().to_snapshot(&character.status).max_sp();
                let (source_sp, target_sp) = (source.status.sp, character.status.sp);
                let target_keeps_sp = character.status.has_status_change(StatusChangeKind::NoRecovery);
                if !target_keeps_sp {
                    server.character_service().update_hp_sp(character, character.status.hp, source_sp.min(max_sp));
                }
                self.followup_action(server, effect, effect.source_char_id, ScriptSkillAction::SetResources { hp: None, sp: Some(target_sp) });
            }
            Some(ClassEffect::Stance(kind)) => self.apply_stance(server, character, effect, kind, tick)?,
            Some(ClassEffect::SevenWind) => self.apply_seven_wind(server, character, effect, tick)?,
            Some(ClassEffect::SoulLink) => {
                let mut request = Self::timed_request(effect, StatusChangeKind::Spirit);
                request.values[1] = effect.skill_id as i32;
                if !StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)? {
                    return Err("The soul link does not match the target".into());
                }
                let source = if on_self { &*character } else { state.get_character(effect.source_char_id).ok_or("Caster disconnected")? };
                let on_caster = ScriptSkillEffect { target_id: effect.source_char_id, skill_id: SkillEnum::SlSma.id(), ..effect.clone() };
                self.queue_delayed_status(server, source, &on_caster, Self::timed_request(&on_caster, StatusChangeKind::Sma));
            }
            Some(ClassEffect::SoulLinkBuff(kind)) => {
                let source = if on_self { &*character } else { state.get_character(effect.source_char_id).ok_or("Caster disconnected")? };
                if !Self::soul_linker_target_allowed(source, character) {
                    self.punish_caster(server, source, effect, SOUL_LINK_MISUSE_STUN_MS);
                    return Err("The target is not linked to the caster".into());
                }
                StatusEffectService::start(server, character, Self::timed_request(effect, kind), tick, &self.client_notification_sender)?;
            }
            Some(ClassEffect::Estin(_)) => {
                let source = if on_self { &*character } else { state.get_character(effect.source_char_id).ok_or("Caster disconnected")? };
                self.punish_caster(server, source, effect, SOUL_LINK_MISUSE_STUN_MS);
                return Err("Estin skills only work on monsters".into());
            }
            Some(ClassEffect::MindBreaker) => {
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                if Self::undead_target(&snapshot) || character.status.has_status_change(StatusChangeKind::MindBreaker) {
                    return Err("Mind Breaker has no effect".into());
                }
                if !StatusEffectService::start(server, character, Self::mind_breaker_request(effect), tick, &self.client_notification_sender)? {
                    return Err("Mind Breaker failed".into());
                }
            }
            _ if Self::is_performance_skill(metadata, effect.level) => self.apply_performance_skill(server, state, character, effect, tick)?,
            _ => return Err(format!("{} cannot target a player", metadata.name)),
        }
        Ok(true)
    }

    /// Effects whose target is a monster, `caster` is the player using the skill; `None` when the skill is not one of these.
    pub(super) fn apply_class_mob_skill(
        &self,
        server: &Server,
        caster: &mut Character,
        effect: &ScriptSkillEffect,
        instance: &MapInstance,
        mob: &Mob,
    ) -> Result<Option<bool>, String> {
        let Some(metadata) = SkillMetadata::find(effect.skill_id) else { return Ok(None) };
        if !Self::is_class_skill(&metadata, effect.level) {
            return Ok(None);
        }
        let immune = mob.status.has_mob_capability(MobCapability::StatusImmune);
        let mob_level = mob.status_effects.base_level;
        let source = StatusService::instance().to_snapshot(&caster.status);
        let succeeded = match Self::class_effect(&metadata, effect.level) {
            Some(ClassEffect::AbsorbSpirits) => {
                let drained = !immune && fastrand::u32(0..100) < ABSORB_MOB_CHANCE_PERCENT;
                if drained {
                    Self::grant_sp(server, caster, 2 * mob_level);
                }
                drained
            }
            Some(ClassEffect::StealItem) => self.steal_item(server, caster, effect, instance, mob, &source),
            Some(ClassEffect::StealCoin) => self.steal_coin(server, caster, effect, instance, mob, &source)?,
            Some(ClassEffect::Death) => {
                let kill = !immune;
                if kill {
                    let damage = Self::kill_damage(effect, mob.id, mob.status.hp(), *source.combat_actor_kind());
                    instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage }));
                }
                kill
            }
            Some(ClassEffect::FullRecovery) => {
                instance.add_to_next_tick(MapEvent::MobHeal(MobHeal { mob_id: mob.id, hp: mob.status.max_hp(), sp: mob.status.max_sp() }));
                true
            }
            Some(ClassEffect::Fortune) => {
                self.grant_zeny(server, caster, mob_level.saturating_mul(100))?;
                true
            }
            Some(ClassEffect::ElementChange(element)) => {
                if !immune {
                    let mut request = Self::timed_request(effect, StatusChangeKind::ElementalChange);
                    request.values = [i32::from(effect.level), element.value() as i32, 0, 0];
                    instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: mob.id, request }));
                }
                !immune
            }
            Some(ClassEffect::ClassChange { monocell }) => {
                let replacement = if monocell {
                    Some(PORING_MOB_ID)
                } else {
                    crate::server::script::game_data::random_summon("CLASSCHANGE", &mut fastrand::Rng::new())
                };
                match replacement.filter(|_| !immune) {
                    Some(mob_id) => {
                        instance.add_to_next_tick(MapEvent::ScriptMobCombat(ScriptMobCombat {
                            source_id: caster.char_id,
                            target_id: mob.id,
                            effect: crate::server::service::script_combat_service::MobCombatEffect::ClassChange { mob_id },
                        }));
                        true
                    }
                    None => false,
                }
            }
            Some(ClassEffect::StarHate) => self.star_hate_mob(server, caster, effect, mob)?,
            Some(ClassEffect::MindBreaker) => {
                let blocked = immune || Self::undead_target(&mob.status) || mob.status_effects.has_status_change(StatusChangeKind::MindBreaker);
                if !blocked {
                    instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: mob.id, request: Self::mind_breaker_request(effect) }));
                }
                !blocked
            }
            Some(ClassEffect::SoulChange) => {
                if mob.steal_flags & MobStealFlag::SoulChange.as_flag() != 0 {
                    false
                } else {
                    instance.add_to_next_tick(MapEvent::MobMarkStolen(MobMarkStolen { mob_id: mob.id, flag: MobStealFlag::SoulChange }));
                    Self::grant_sp(server, caster, source.max_sp() * SOUL_CHANGE_MOB_SP_PERCENT / 100);
                    true
                }
            }
            Some(ClassEffect::Estin(kind)) => {
                if mob.status_effects.has_status_change(kind) {
                    if kind != StatusChangeKind::Ske {
                        instance.add_to_next_tick(MapEvent::MobEndStatus(MobEndStatus { mob_id: mob.id, kind: Some(StatusChangeKind::Swoo) }));
                        self.punish_caster(server, caster, effect, ES_REPEAT_STUN_MS);
                    }
                    false
                } else {
                    instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: mob.id, request: Self::timed_request(effect, kind) }));
                    if kind == StatusChangeKind::Ske {
                        let on_caster = ScriptSkillEffect { target_id: effect.source_char_id, skill_id: SkillEnum::SlSma.id(), ..effect.clone() };
                        self.queue_delayed_status(server, caster, &on_caster, Self::timed_request(&on_caster, StatusChangeKind::Sma));
                    }
                    true
                }
            }
            _ => return Err(format!("{} cannot target a monster", metadata.name)),
        };
        self.notify_support_skill_result(caster, effect, succeeded);
        Ok(Some(succeeded))
    }

    /// `pc_steal_item`: protected, summoned and already robbed monsters are skipped, the chance scales every drop rate.
    fn steal_item(&self, server: &Server, thief: &Character, effect: &ScriptSkillEffect, instance: &MapInstance, mob: &Mob, source: &models::status::StatusSnapshot) -> bool {
        use models::enums::mob::MobStealFlag::Item;
        let disabled = [StatusChangeKind::Stone, StatusChangeKind::StoneWait, StatusChangeKind::Freeze, StatusChangeKind::Stun, StatusChangeKind::Sleep];
        if mob.steal_flags & Item.as_flag() != 0
            || mob.summon_owner.is_some()
            || mob.status.has_mob_capability(MobCapability::StatusImmune)
            || disabled.iter().any(|kind| mob.status_effects.has_status_change(*kind))
        {
            instance.add_to_next_tick(MapEvent::MobMarkStolen(MobMarkStolen { mob_id: mob.id, flag: Item }));
            return false;
        }
        let rate = (i32::from(source.dex()) - i32::from(mob.status.dex())) / 2 + i32::from(effect.level) * 6 + 4;
        if rate < 1 {
            return false;
        }
        let config = GlobalConfigService::instance();
        let drop = config
            .get_mob(i32::from(mob.mob_id))
            .drops
            .iter()
            .find(|drop| config.find_item(drop.item_id).is_some() && f64::from(fastrand::u32(0..10_000)) < f64::from(drop.rate) * f64::from(rate) / 100.0);
        let Some(item) = drop.and_then(|drop| config.find_item(drop.item_id)) else { return false };
        instance.add_to_next_tick(MapEvent::MobMarkStolen(MobMarkStolen { mob_id: mob.id, flag: Item }));
        server.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems {
            char_id: thief.char_id,
            should_perform_check: true,
            buy: false,
            items: vec![crate::repository::model::item_model::InventoryItemModel::from_item_model(item, 1, true)],
        }));
        true
    }

    /// `RG_STEALCOIN`: a once per monster zeny theft.
    fn steal_coin(&self, server: &Server, thief: &mut Character, effect: &ScriptSkillEffect, instance: &MapInstance, mob: &Mob, source: &models::status::StatusSnapshot) -> Result<bool, String> {
        if mob.steal_flags & MobStealFlag::Coin.as_flag() != 0 || mob.status.has_mob_capability(MobCapability::StatusImmune) {
            return Ok(false);
        }
        let level = mob.status_effects.base_level as i32;
        let chance = 10 * i32::from(effect.level) + i32::from(source.dex()) / 2 + i32::from(source.luk()) / 2 + 2 * (thief.status.base_level as i32 - level);
        if fastrand::i32(0..1000) >= chance {
            return Ok(false);
        }
        instance.add_to_next_tick(MapEvent::MobMarkStolen(MobMarkStolen { mob_id: mob.id, flag: MobStealFlag::Coin }));
        let amount = fastrand::i32(8 * level..=10 * level) + i32::from(effect.level) * level / 10;
        self.grant_zeny(server, thief, amount.max(0) as u32)?;
        Ok(true)
    }
}
