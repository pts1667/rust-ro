use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{SyncSender, TrySendError};
use std::sync::{Arc, Mutex, OnceLock};

use models::enums::action::ActionType;
use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::enums::size::Size;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithNumberValue, EnumWithStringValue};
use models::status::{Status, StatusSnapshot};
use models::status_bonus::{BattleFlag, StatusBonus};
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use movement::position::Position;
use packets::packets::{Packet, PacketZcNotifyMove, PacketZcNotifyStandentry7, PacketZcNotifyVanish, PacketZcUseSkill};
use script_sdk::{Function, Value};
use serde::Deserialize;

use crate::repository::Repository;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::map_event::{MapEvent, MobDamage};
use crate::server::model::game_systems::{
    CharacterGameSystems, CompanionPosition, HomunculusRecord, MercenaryRecord, PetCapture, PlayerOption, StoreSearch,
};
use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItemType};
use crate::server::service::battle_service::{BattleService, NormalAttackRoll};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[path = "script_world_companion_skills.rs"]
mod companion_skills;
#[path = "script_world_booking.rs"]
mod booking;
#[path = "script_world_family.rs"]
mod family;
#[path = "script_world_guild.rs"]
mod guild;
#[path = "script_world_party.rs"]
mod party;
pub use party::{party_can_pick_up, party_experience_awards, party_experience_awards_with_bonus, party_loot_candidates};
#[path = "script_world_homunculus.rs"]
mod homunculus;
#[path = "script_world_pet_combat.rs"]
mod pet_combat;
#[path = "script_world_pet_loot.rs"]
mod pet_loot;
#[path = "script_world_pet_support.rs"]
mod pet_support;
#[path = "script_world_pets.rs"]
mod pets;
#[path = "script_world_protocol.rs"]
mod protocol;
#[path = "script_world_requests.rs"]
mod requests;
pub use requests::{BattlegroundRequest, ContainerRequest, ScriptWorldRequest};
pub use booking::BookingRequest;
pub use companion_skills::CompanionRequest;
pub use family::FamilyRequest;
pub use guild::GuildRequest;
pub use homunculus::HomunculusRequest;
pub use party::PartyRequest;
pub use pets::PetRequest;
pub use vending::StoreRequest;
#[cfg(test)]
pub(crate) use pet_support::PetSupportHost;
pub use pet_support::pet_support_operation;
pub use pets::{advance_pet_hunger, pet_bonus_state, pet_constant, pet_information, pet_script_state};
#[path = "script_world_vending.rs"]
mod vending;
pub use guild::guild_actor_packet;
pub use protocol::{client_frame_length, decode_request, world_frame_length};
pub use vending::vending_store_sign_packet;

#[derive(Deserialize)]
pub struct ScriptWorldData {
    pub pets: Vec<PetDefinition>,
    pub mercenaries: Vec<MercenaryDefinition>,
    pub homunculi: Vec<HomunculusDefinition>,
    pub guild_experience: Vec<u64>,
    pub homunculus_experience: Vec<u64>,
    pub client_frames: BTreeMap<u16, i32>,
}

#[derive(Deserialize)]
pub struct PetDefinition {
    pub class_id: u16,
    pub name: String,
    pub level: u16,
    pub tame_item: i32,
    pub egg_item: i32,
    pub food_item: i32,
    pub equip_item: i32,
    pub capture_rate: i32,
    pub fullness: i32,
    pub hungry_delay: u64,
    pub hunger_increase: i32,
    pub intimacy_start: i32,
    pub intimacy_fed: i32,
    pub intimacy_overfed: i32,
    pub intimacy_hungry: i32,
    pub intimacy_owner_die: i32,
    #[serde(default)]
    pub has_bonus_script: bool,
    #[serde(default)]
    pub has_support_script: bool,
    #[serde(default = "default_pet_combat_rate")]
    pub attack_rate: u16,
    #[serde(default = "default_pet_combat_rate")]
    pub retaliation_rate: u16,
    #[serde(default = "default_pet_combat_rate")]
    pub change_target_rate: u16,
}

fn default_pet_combat_rate() -> u16 {
    10001
}

#[derive(Deserialize)]
pub struct MercenaryDefinition {
    pub class_id: u16,
    pub name: String,
    pub level: u16,
    pub hp: u32,
    pub sp: u32,
    pub attack: u16,
    pub attack2: u16,
    pub defense: u16,
    pub magic_defense: u16,
    pub stats: [u16; 6],
    pub range: u16,
    pub walk_speed: u16,
    pub attack_delay: u32,
    pub attack_motion: u32,
    pub skills: Vec<MercenarySkill>,
    pub size: String,
    pub race: String,
    pub element: String,
    pub element_level: u8,
}

#[derive(Deserialize)]
pub struct MercenarySkill {
    pub name: String,
    pub level: u8,
}

#[derive(Deserialize)]
pub struct HomunculusDefinition {
    pub class_id: u16,
    pub evolution_class: u16,
    pub name: String,
    pub base: [u32; 8],
    pub evolution_min: [u32; 8],
    pub evolution_max: [u32; 8],
    pub growth_min: [u32; 8],
    pub growth_max: [u32; 8],
    pub food_item: i32,
    pub hungry_delay: u64,
    pub attack_delay: u32,
    pub race: String,
    pub element: String,
    pub size: String,
    pub evolution_size: String,
    pub skills: Vec<HomunculusSkillDefinition>,
}

#[derive(Deserialize)]
pub struct HomunculusSkillDefinition {
    pub name: String,
    pub max_level: u8,
    pub required_level: u16,
    pub required_intimacy: u32,
    pub evolution: bool,
    pub required: Vec<HomunculusSkillRequirement>,
}

#[derive(Deserialize)]
pub struct HomunculusSkillRequirement {
    pub name: String,
    pub level: u8,
}

pub fn world_data() -> &'static ScriptWorldData {
    static DATA: OnceLock<ScriptWorldData> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("script_world_data.json")).expect("Invalid embedded pre-renewal world data"))
}

pub struct ScriptWorldService {
    pub(crate) notifications: SyncSender<Notification>,
    pending_notifications: Mutex<VecDeque<Notification>>,
    next_pet_capture_id: AtomicU64,
    next_pet_cast_id: AtomicU64,
    next_pet_loot_id: AtomicU64,
    #[cfg(test)]
    pet_configuration_override: Option<configuration::configuration::PetSupportConfig>,
    pub(crate) repository: Arc<dyn Repository>,
    pub(crate) configuration: &'static GlobalConfigService,
}

#[derive(Debug, Clone)]
pub struct WorldEffectPlan {
    pub expected_revision: u64,
    pub systems: Option<CharacterGameSystems>,
    pub guild_experience: u64,
    pub open_personal_storage: bool,
    pub guild_storage_receipts: Vec<crate::server::model::game_systems::GuildStorageOpenReceipt>,
    pub additional_grants: Vec<crate::repository::script_inventory_repository::ScriptItemGrant>,
    notices: Vec<WorldNotice>,
}

#[derive(Debug, Clone)]
enum WorldNotice {
    Font,
    Mercenary,
    HomunculusEvolution(bool),
    Guild,
    PersonalStorage,
    PetLoot { capacity: u8, returned_cargo: bool },
}

pub fn persistent_world_operation(function: Function) -> bool {
    matches!(
        function,
        Function::SetFont
            | Function::MercenaryCreate
            | Function::MercenaryHeal
            | Function::MercenaryStartStatus
            | Function::Homevolution
            | Function::GuildExperience
            | Function::OpenStorage
            | Function::PetLoot
    )
}

pub fn plan_persistent_effects(character: &Character, effects: &[(Function, Vec<Value>)], now: u64) -> Result<WorldEffectPlan, String> {
    let mut systems = character.game_systems.clone();
    let mut changed = false;
    let mut notices = Vec::new();
    let mut guild_experience = 0u64;
    let mut open_personal_storage = false;
    let mut additional_grants = Vec::new();
    for (function, args) in effects {
        match function {
            Function::PetLoot => {
                let had_cargo = systems.pet_loot.is_some();
                let (capacity, grants) = pet_loot::plan_reconfiguration(&mut systems, args)?;
                let returned_cargo = !grants.is_empty();
                additional_grants.extend(grants);
                changed |= had_cargo;
                notices.push(WorldNotice::PetLoot { capacity, returned_cargo });
            }
            Function::OpenStorage => {
                if optional_number(args, 0, 0)? != 0 {
                    return Err("Unknown personal storage identifier".into());
                }
                if character.game_systems.guild_storage_open.is_none() {
                    open_personal_storage = true;
                }
            }
            Function::SetFont => {
                systems.font = u16::try_from(number(args, 0)?).map_err(|_| "Invalid font")?;
                if systems.font > 9 {
                    return Err("Font must be between 0 and 9".into());
                }
                changed = true;
                notices.push(WorldNotice::Font);
            }
            Function::MercenaryCreate => {
                let class_id = u16::try_from(number(args, 0)?).map_err(|_| "Invalid mercenary class")?;
                let data = world_data()
                    .mercenaries
                    .iter()
                    .find(|data| data.class_id == class_id)
                    .ok_or("Unknown mercenary class")?;
                let duration = u64::try_from(number(args, 1)?).map_err(|_| "Invalid mercenary contract duration")?;
                if duration == 0 {
                    return Err("Mercenary contract must be positive".into());
                }
                if systems
                    .mercenary
                    .as_ref()
                    .is_some_and(|mercenary| mercenary_contract_active(mercenary, now))
                {
                    return Err("A mercenary is already under contract".into());
                }
                let guild = if (6017..=6026).contains(&class_id) {
                    0
                } else if (6027..=6036).contains(&class_id) {
                    1
                } else if (6037..=6046).contains(&class_id) {
                    2
                } else {
                    255
                };
                if let Some(previous) = &systems.mercenary {
                    if previous.guild < 3 {
                        let faith = systems.mercenary_faith.entry(previous.guild).or_default();
                        *faith = if previous.hp == 0 {
                            faith.saturating_sub(1)
                        } else {
                            faith.saturating_add(1).min(i16::MAX as u16)
                        };
                    }
                }
                systems.mercenary = Some(MercenaryRecord {
                    id: 0,
                    class_id,
                    name: data.name.clone(),
                    level: data.level,
                    hp: data.hp,
                    sp: data.sp,
                    max_hp: data.hp,
                    max_sp: data.sp,
                    expires_at: now.checked_add(duration).ok_or("Mercenary contract overflow")?,
                    contract_remaining: None,
                    skill_cooldowns: BTreeMap::new(),
                    cooldown_pause_at: None,
                    guild,
                    kill_count: 0,
                    statuses: Vec::new(),
                    last_attack_at: 0,
                    last_regen_hp_at: now,
                    last_regen_sp_at: now,
                });
                if guild < 3 {
                    let calls = systems.mercenary_calls.entry(guild).or_default();
                    *calls = calls.saturating_add(1).min(i32::MAX as u32);
                }
                changed = true;
                notices.push(WorldNotice::Mercenary);
            }
            Function::MercenaryHeal => {
                let hp = number(args, 0)?;
                let sp = number(args, 1)?;
                if let Some(mercenary) = systems
                    .mercenary
                    .as_mut()
                    .filter(|mercenary| mercenary_contract_active(mercenary, now))
                {
                    let hp = if hp > 0 && mercenary.statuses.iter().any(|status| status.kind == StatusChangeKind::Berserk) {
                        0
                    } else {
                        hp
                    };
                    mercenary.hp = (i64::from(mercenary.hp) + i64::from(hp)).clamp(0, i64::from(mercenary.max_hp)) as u32;
                    mercenary.sp = (i64::from(mercenary.sp) + i64::from(sp)).clamp(0, i64::from(mercenary.max_sp)) as u32;
                    changed = true;
                    notices.push(WorldNotice::Mercenary);
                }
            }
            Function::MercenaryStartStatus => {
                let kind = status_kind(args.first().ok_or("Missing mercenary status")?)?;
                let duration = number(args, 1)?;
                let value = number(args, 2)?;
                if duration < -1 {
                    return Err("Invalid mercenary status duration".into());
                }
                if let Some(mercenary) = systems
                    .mercenary
                    .as_mut()
                    .filter(|mercenary| mercenary_contract_active(mercenary, now))
                {
                    start_mercenary_status(mercenary, StatusChangeRequest::guaranteed(kind, duration, value), now)?;
                    if kind == StatusChangeKind::MercHpUp {
                        mercenary.hp = mercenary.max_hp;
                    }
                    if kind == StatusChangeKind::MercSpUp {
                        mercenary.sp = mercenary.max_sp;
                    }
                    changed = true;
                    notices.push(WorldNotice::Mercenary);
                }
            }
            Function::Homevolution => {
                let evolved = evolve_homunculus_systems(&mut systems)?;
                changed |= evolved;
                notices.push(WorldNotice::HomunculusEvolution(evolved));
            }
            Function::GuildExperience => {
                let amount = number(args, 0)?;
                if amount <= 0 {
                    return Err("Guild experience must be positive".into());
                }
                if systems.guild_id != 0 {
                    guild_experience = guild_experience.checked_add(amount as u64).ok_or("Guild experience overflow")?;
                    notices.push(WorldNotice::Guild);
                }
            }
            _ => {}
        }
    }
    if open_personal_storage {
        notices.push(WorldNotice::PersonalStorage);
    }
    Ok(WorldEffectPlan {
        expected_revision: character.game_systems.revision,
        systems: changed.then_some(systems),
        guild_experience,
        open_personal_storage,
        guild_storage_receipts: Vec::new(),
        additional_grants,
        notices,
    })
}

pub fn attach_guild_storage_receipts(
    character: &Character,
    plan: &mut WorldEffectPlan,
    receipts: &[crate::server::model::game_systems::GuildStorageOpenReceipt],
) -> Result<(), String> {
    for receipt in receipts {
        if receipt.result_code == 0 && (character.game_systems.buying_store.is_some() || character.game_systems.vending_store.is_some()) {
            return Err("Cannot open storage while operating a store".into());
        }
        if receipt.char_id != character.char_id
            || receipt.character_revision != character.game_systems.revision
            || receipt.original_personal_open != character.game_systems.storage_open
            || receipt.original_guild_open != character.game_systems.guild_storage_open
            || (receipt.personal_open_in_script && !plan.open_personal_storage)
            || (receipt.result_code == 0 && plan.open_personal_storage)
        {
            return Err("Storage state changed while the item script was running".into());
        }
    }
    plan.guild_storage_receipts = receipts.to_vec();
    Ok(())
}

fn evolve_homunculus_systems(systems: &mut CharacterGameSystems) -> Result<bool, String> {
    let Some(homunculus) = systems.homunculus.as_mut() else {
        return Ok(false);
    };
    if !homunculus.active || homunculus.hp == 0 || homunculus.evolved || homunculus.intimacy < 91_100 {
        return Ok(false);
    }
    let data = world_data()
        .homunculi
        .iter()
        .find(|data| data.class_id == homunculus.class_id)
        .ok_or("Unknown homunculus class")?;
    let growth = data
        .evolution_min
        .iter()
        .zip(data.evolution_max)
        .map(|(minimum, maximum)| fastrand::u32(*minimum..=maximum))
        .collect::<Vec<_>>();
    homunculus.class_id = data.evolution_class;
    if homunculus.base_max_hp == 0 {
        homunculus.base_max_hp = homunculus.max_hp;
    }
    if homunculus.base_max_sp == 0 {
        homunculus.base_max_sp = homunculus.max_sp;
    }
    homunculus.base_max_hp = homunculus.base_max_hp.saturating_add(growth[0]);
    homunculus.base_max_sp = homunculus.base_max_sp.saturating_add(growth[1]);
    for (stat, increase) in homunculus.stats.iter_mut().zip(growth.iter().skip(2)) {
        *stat = stat.saturating_add(*increase as u16);
    }
    homunculus.intimacy = 1000;
    homunculus.evolved = true;
    homunculus::recalculate_homunculus(homunculus);
    Ok(true)
}

const CALL_PARTNER_SKILL_ID: i32 = 336;

impl ScriptWorldService {
    pub fn handle_companion_damage(&self, server: &Server, state: &mut ServerState, mut damage: Damage, now: u128) -> Result<bool, String> {
        let owner_id = state
            .characters()
            .values()
            .find_map(|character| companion_health(character, damage.target_id).map(|_| character.char_id));
        let Some(owner_id) = owner_id else {
            return Ok((1_000_000_000..1_300_000_000).contains(&damage.target_id));
        };
        let character = state.characters().get(&owner_id).unwrap();
        if !damage.matches_notification_map(&character.map_instance_key) {
            return Ok(true);
        }
        if character
            .game_systems
            .pet
            .as_ref()
            .is_some_and(|pet| pet_world_id(pet.id) == damage.target_id)
        {
            damage.notify_admitted(&self.notifications, 0, self.configuration.packetver());
            return Ok(true);
        }
        let snapshot = companion_status_snapshot(character, damage.target_id).ok_or("Companion status is unavailable")?;
        if snapshot.hp() == 0 {
            damage.notify_admitted(&self.notifications, 0, self.configuration.packetver());
            return Ok(true);
        }
        let map_flags = state.map_flags(&character.map_instance_key);
        if damage.healing > 0 {
            crate::server::service::map_flag_service::apply_map_skill_damage(&map_flags, &mut damage, &snapshot);
        }
        if damage.healing > 0 {
            let character = state.characters_mut().get_mut(&owner_id).unwrap();
            let mut systems = character.game_systems.clone();
            if let Some(mercenary) = systems
                .mercenary
                .as_mut()
                .filter(|mercenary| mercenary_world_id(mercenary) == damage.target_id)
            {
                mercenary.hp = mercenary.hp.saturating_add(damage.healing).min(snapshot.max_hp());
            } else if let Some(homunculus) = systems
                .homunculus
                .as_mut()
                .filter(|homunculus| homunculus_world_id(homunculus) == damage.target_id)
            {
                homunculus.hp = homunculus.hp.saturating_add(damage.healing).min(snapshot.max_hp());
            }
            if systems != character.game_systems {
                let saved = self
                    .repository
                    .save_character_game_systems(owner_id, &systems)
                    .map_err(|error| error.to_string())?;
                install_state(character, saved);
                self.send_mercenary(character, now as u64)?;
                self.send_homunculus(character)?;
            }
            damage.notify_admitted(
                &self.notifications,
                -i64::from(damage.healing.min(snapshot.max_hp().saturating_sub(snapshot.hp()))),
                self.configuration.packetver(),
            );
            return Ok(true);
        }
        if damage.landed && damage.battle_flags & BattleFlag::Magic.as_flag() != 0 {
            if let Some(kind) = crate::server::service::combat_trigger_service::magic_reflection(
                &snapshot,
                damage.battle_flags,
                damage.skill_id,
                &mut fastrand::Rng::new(),
            ) {
                let request = crate::server::service::map_combat_service::MagicReflectionRequest {
                    damage,
                    reflector_id: damage.target_id,
                    reflector_credit_id: owner_id,
                    kind,
                    map_key: character.map_instance_key.clone(),
                };
                crate::server::service::map_combat_service::reflect_magic(server, state, request, now)?;
                damage.notify_admitted(&self.notifications, 0, self.configuration.packetver());
                return Ok(true);
            }
        }
        let versus = map_flags.versus(state.siege_active);
        let character = state.characters_mut().get_mut(&owner_id).unwrap();
        let mut systems = character.game_systems.clone();
        let mut admitted = 0;
        let mut healed = 0;
        let mut absorbed = false;
        let raw = if damage.defenses_applied {
            damage.damage
        } else if damage.battle_flags & BattleFlag::Weapon.as_flag() != 0 {
            (damage.damage as f32 * (1.0 - snapshot.def().clamp(0, 100) as f32 / 100.0) - f32::from(snapshot.vit()))
                .max(1.0)
                .floor() as u32
        } else if damage.battle_flags & BattleFlag::Magic.as_flag() != 0 {
            (damage.damage as f32 * (1.0 - snapshot.mdef().clamp(0, 100) as f32 / 100.0) - f32::from(snapshot.int()))
                .max(1.0)
                .floor() as u32
        } else {
            damage.damage
        };
        if let Some(mercenary) = systems
            .mercenary
            .as_mut()
            .filter(|mercenary| mercenary_world_id(mercenary) == damage.target_id)
        {
            if mercenary.hp == 0 {
                return Ok(true);
            }
            let mut status = mercenary_status(mercenary);
            absorbed =
                StatusEffectService::absorb_magic_rod(&mut status, damage.battle_flags, damage.skill_id, damage.skill_level).is_some();
            let received = if absorbed || damage.damage == 0 {
                0
            } else {
                StatusEffectService::apply_incoming_skill_damage_flags(&mut status, raw, damage.battle_flags, versus, damage.skill_id)
            };
            let received = crate::server::service::map_flag_service::apply_map_combat_damage(
                &map_flags,
                &self.configuration.config().game,
                received,
                damage.skill_id,
                damage.battle_flags,
            );
            let mut adjusted = Damage {
                damage: received,
                ..damage
            };
            crate::server::service::map_flag_service::apply_map_skill_damage(&map_flags, &mut adjusted, &snapshot);
            let received = adjusted.damage;
            healed = adjusted.healing.min(snapshot.max_hp().saturating_sub(mercenary.hp));
            admitted = received.min(mercenary.hp);
            mercenary.hp = mercenary
                .hp
                .saturating_sub(received)
                .saturating_add(adjusted.healing)
                .min(snapshot.max_hp());
            mercenary.sp = status.sp;
            mercenary.statuses = status.active_statuses;
            if mercenary.hp == 0 {
                if mercenary.guild != 255 {
                    let faith = systems.mercenary_faith.entry(mercenary.guild).or_default();
                    *faith = faith.saturating_sub(1);
                }
                systems.mercenary = None;
            }
        } else if let Some(homunculus) = systems
            .homunculus
            .as_mut()
            .filter(|homunculus| homunculus_world_id(homunculus) == damage.target_id)
        {
            let mut status = homunculus::homunculus_status(homunculus);
            absorbed =
                StatusEffectService::absorb_magic_rod(&mut status, damage.battle_flags, damage.skill_id, damage.skill_level).is_some();
            let received = if absorbed || damage.damage == 0 {
                0
            } else {
                StatusEffectService::apply_incoming_skill_damage_flags(&mut status, raw, damage.battle_flags, versus, damage.skill_id)
            };
            let received = crate::server::service::map_flag_service::apply_map_combat_damage(
                &map_flags,
                &self.configuration.config().game,
                received,
                damage.skill_id,
                damage.battle_flags,
            );
            let mut adjusted = Damage {
                damage: received,
                ..damage
            };
            crate::server::service::map_flag_service::apply_map_skill_damage(&map_flags, &mut adjusted, &snapshot);
            let received = adjusted.damage;
            healed = adjusted.healing.min(snapshot.max_hp().saturating_sub(homunculus.hp));
            admitted = received.min(homunculus.hp);
            homunculus.hp = homunculus
                .hp
                .saturating_sub(received)
                .saturating_add(adjusted.healing)
                .min(snapshot.max_hp());
            homunculus.sp = status.sp;
            homunculus.statuses = status.active_statuses;
            if homunculus.hp == 0 {
                homunculus.active = false;
                homunculus.statuses.clear();
            }
        }
        if systems != character.game_systems {
            let saved = self
                .repository
                .save_character_game_systems(owner_id, &systems)
                .map_err(|error| error.to_string())?;
            install_state(character, saved);
        }
        damage.notify_admitted(
            &self.notifications,
            if healed > 0 { -i64::from(healed) } else { i64::from(admitted) },
            self.configuration.packetver(),
        );
        if absorbed {
            self.send_mercenary(character, now as u64)?;
            self.send_homunculus(character)?;
            let mut packet = PacketZcUseSkill::new(server.packetver());
            packet.set_src_aid(damage.target_id);
            packet.set_target_aid(damage.target_id);
            packet.set_skid(models::enums::skill_enums::SkillEnum::SaMagicrod.id() as u16);
            packet.set_level(i16::from(damage.skill_level));
            packet.set_result(true);
            packet.fill_raw();
            self.area(character, packet.raw)?;
            return Ok(true);
        }
        if damage.damage == 0 {
            return Ok(true);
        }
        if admitted > 0 {
            self.interrupt_companion_cast(character, damage.target_id, false)?;
        }
        self.send_mercenary(character, now as u64)?;
        self.send_homunculus(character)?;
        self.render_companions(server, character, now as u64)?;
        let reflected = if damage.landed && damage.proc_depth == 0 {
            crate::server::service::combat_trigger_service::physical_reflection(&snapshot, damage.battle_flags, damage.skill_id, admitted)
        } else {
            0
        };
        if reflected > 0 && damage.attacker_id != damage.target_id {
            let reflected = Damage {
                notification: None,
                source_kind: *snapshot.combat_actor_kind(),
                skill_damage_adjusted: false,
                target_id: damage.attacker_id,
                attacker_id: damage.target_id,
                damage: reflected,
                attacked_at: now,
                damage_motion: 0,
                battle_flags: 0,
                skill_id: 0,
                skill_level: 0,
                landed: false,
                proc_depth: damage.proc_depth.saturating_add(1),
                credit_id: owner_id,
                defenses_applied: true,
                magic_context: None,
                right_hand_damage: None,
                healing: 0,
            }
            .with_action_notification(
                character.current_map_name(),
                character.current_map_instance(),
                character.x,
                character.y,
                now,
                1,
                0,
                ActionType::AttackNomotion,
                (reflected.min(i32::MAX as u32) as i32, 0),
            );
            if let Some(map) = server.state().get_map_instance_from_character(character) {
                if map.state().get_mob(reflected.target_id).is_some() {
                    map.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: reflected }));
                } else {
                    server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::CharacterDamage(crate::server::model::events::game_event::CharacterDamage { damage: reflected }));
                }
            }
        }
        Ok(true)
    }

    pub fn apply_committed_effects(
        &self,
        server: &Server,
        character: &mut Character,
        plan: &WorldEffectPlan,
        mut committed: crate::repository::game_system_repository::CommittedWorldEffects,
        now: u64,
    ) -> Result<(), String> {
        if let Some(saved) = committed.systems.take() {
            install_state(character, saved);
        }
        let mut mercenary = false;
        let mut guild = false;
        for notice in &plan.notices {
            match notice {
                WorldNotice::PetLoot { capacity, returned_cargo } => {
                    pet_loot::configure_runtime(character, *capacity, *returned_cargo, now);
                }
                WorldNotice::PersonalStorage => {
                    character.game_systems.storage_items = committed
                        .personal_storage
                        .take()
                        .ok_or("Committed personal storage snapshot is missing")?;
                    character.game_systems.storage_open = true;
                    self.send_storage(character)?;
                }
                WorldNotice::Font => {
                    let mut packet = protocol::header(0x02EF);
                    packet.extend_from_slice(&character.char_id.to_le_bytes());
                    packet.extend_from_slice(&character.game_systems.font.to_le_bytes());
                    self.area(character, packet)?;
                }
                WorldNotice::Mercenary => mercenary = true,
                WorldNotice::Guild => guild = true,
                WorldNotice::HomunculusEvolution(evolved) => {
                    if *evolved {
                        if let Some(homunculus) = &character.game_systems.homunculus {
                            let mut packet = protocol::header(0x01B0);
                            packet.extend_from_slice(&homunculus_world_id(homunculus).to_le_bytes());
                            packet.push(0);
                            packet.extend_from_slice(&u32::from(homunculus.class_id).to_le_bytes());
                            self.area(character, packet)?;
                            self.send_homunculus(character)?;
                        }
                    } else {
                        self.send(character.char_id, protocol::emotion(character.char_id, 14))?;
                    }
                }
            }
        }
        if mercenary {
            self.send_mercenary(character, now)?;
        }
        if guild {
            if let Some(guild) = committed.guild {
                self.broadcast_guild_summary(server, character, &guild)?;
            }
        }
        if let Some((guild_id, items)) = committed.guild_storage {
            character.game_systems.guild_storage_open = Some(guild_id);
            character.game_systems.guild_storage_items = items;
            self.send_storage(character)?;
        }
        self.render_companions(server, character, now)
    }

    pub fn new(
        notifications: SyncSender<Notification>,
        repository: Arc<dyn Repository>,
        configuration: &'static GlobalConfigService,
    ) -> Self {
        Self {
            notifications,
            pending_notifications: Mutex::new(VecDeque::new()),
            next_pet_capture_id: AtomicU64::new(1),
            next_pet_cast_id: AtomicU64::new(1),
            next_pet_loot_id: AtomicU64::new(fastrand::u64(1..u64::MAX / 2)),
            #[cfg(test)]
            pet_configuration_override: None,
            repository,
            configuration,
        }
    }

    fn pet_configuration(&self) -> &configuration::configuration::PetSupportConfig {
        #[cfg(test)]
        if let Some(configuration) = &self.pet_configuration_override {
            return configuration;
        }
        &self.configuration.config().game.pet_support
    }

    #[cfg(test)]
    pub(crate) fn with_pet_configuration(mut self, configuration: configuration::configuration::PetSupportConfig) -> Self {
        self.pet_configuration_override = Some(configuration);
        self
    }

    pub fn handles(function: Function) -> bool {
        if pet_support_operation(function) {
            return true;
        }
        matches!(
            function,
            Function::Pet
                | Function::SetFont
                | Function::BirthPet
                | Function::Homevolution
                | Function::MercenaryCreate
                | Function::MercenaryHeal
                | Function::MercenaryStartStatus
                | Function::GuildExperience
                | Function::BuyingStore
                | Function::SearchStores
                | Function::VipStatus
                | Function::GetPartnerId
                | Function::CheckCart
                | Function::CheckRiding
                | Function::CheckFalcon
                | Function::CheckMadogear
                | Function::IsMounting
                | Function::Marriage
                | Function::Divorce
                | Function::SetCart
                | Function::SetFalcon
                | Function::SetRiding
                | Function::OpenStorage
                | Function::GuildOpenStorage
                | Function::GetPetInfo
        )
    }

    pub fn validate_call(&self, state: &ServerState, character: &Character, function: Function, args: &[Value], now: u64) -> Result<(), String> {
        if matches!(function, Function::OpenStorage | Function::GuildOpenStorage | Function::BuyingStore)
            && (character.status.hp == 0 || character.game_systems.is_trading() || character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some()) {
            return Err("Storage and shop preparation are unavailable in the current state".into());
        }
        if function == Function::BuyingStore {
            if state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoBuyingStore) {
                return Err("Buying stores are disabled on this map".into());
            }
            if character.game_systems.storage_open || character.game_systems.guild_storage_open.is_some() || !(1..=5).contains(&number(args, 0)?) {
                return Err("Buying store preparation state or slot count is invalid".into());
            }
        }
        if pet_support_operation(function) {
            return self.validate_pet_support(character, function, args, now);
        }
        match function {
            Function::SetFont => {
                if !(0..=9).contains(&number(args, 0)?) {
                    return Err("Font must be between 0 and 9".into());
                }
            }
            Function::Pet => {
                if character.game_systems.pending_pet_capture.is_some() {
                    return Err("A pet capture is awaiting confirmation".into());
                }
                let item = pet_lure_id(number(args, 0)?)?;
                if self.configuration.find_item(item).is_none() {
                    return Err("Unknown pet lure item".into());
                }
                if !(0..=2).contains(&optional_number(args, 1, 0)?) {
                    return Err("Invalid pet capture flag".into());
                }
            }
            Function::BirthPet => {
                if character.game_systems.pet.is_some() {
                    return Err("A pet is already active".into());
                }
            }
            Function::GetPetInfo => {
                number(args, 0)?;
                optional_number(args, 1, character.char_id as i32)?;
            }
            Function::MercenaryCreate => {
                let class = number(args, 0)?;
                if !world_data()
                    .mercenaries
                    .iter()
                    .any(|mercenary| i32::from(mercenary.class_id) == class)
                {
                    return Err("Unknown mercenary class".into());
                }
                if number(args, 1)? <= 0 {
                    return Err("Mercenary contract must have a positive duration".into());
                }
                if character
                    .game_systems
                    .mercenary
                    .as_ref()
                    .is_some_and(|mercenary| mercenary_contract_active(mercenary, now))
                {
                    return Err("A mercenary is already under contract".into());
                }
            }
            Function::MercenaryHeal => {
                number(args, 0)?;
                number(args, 1)?;
            }
            Function::MercenaryStartStatus => {
                status_kind(args.first().ok_or("Missing mercenary status")?)?;
                if number(args, 1)? < -1 {
                    return Err("Invalid mercenary status duration".into());
                }
                number(args, 2)?;
            }
            Function::GuildExperience => {
                if number(args, 0)? <= 0 {
                    return Err("Guild experience must be positive".into());
                }
            }
            Function::BuyingStore => {
                if !(1..=5).contains(&number(args, 0)?) {
                    return Err("Buying stores support one to five item slots".into());
                }
                if character.game_systems.buying_store.is_some() {
                    return Err("A buying store is already open".into());
                }
            }
            Function::SearchStores => {
                if !(1..=255).contains(&number(args, 0)?) {
                    return Err("Store search uses must be between one and 255".into());
                }
                if !(0..=1).contains(&number(args, 1)?) {
                    return Err("Invalid store search effect".into());
                }
                if let Some(Value::String(map)) = args.get(2) {
                    if map != "this" && map != "all" && !self.configuration.maps.contains_key(map.trim_end_matches(".gat")) {
                        return Err("Unknown store search map".into());
                    }
                }
            }
            Function::VipStatus => {
                if !(1..=3).contains(&optional_number(args, 0, 1)?) {
                    return Err("Unknown VIP status query".into());
                }
            }
            Function::OpenStorage | Function::GuildOpenStorage => {
                if character.game_systems.buying_store.is_some() || character.game_systems.vending_store.is_some() {
                    return Err("Cannot open storage while operating a store".into());
                }
                if function == Function::OpenStorage && optional_number(args, 0, 0)? != 0 {
                    return Err("Unknown personal storage identifier".into());
                }
            }
            Function::Homevolution
            | Function::GetPartnerId
            | Function::CheckCart
            | Function::CheckRiding
            | Function::CheckFalcon
            | Function::CheckMadogear
            | Function::IsMounting => {}
            Function::SetCart => {
                if !(0..=5).contains(&optional_number(args, 0, 1)?) {
                    return Err("Unknown classic cart style".into());
                }
            }
            Function::SetFalcon | Function::SetRiding => {}
            Function::Marriage => {
                if args.first().and_then(|name| name.string_value().ok()).is_none_or(|name| name.is_empty()) {
                    return Err("Marriage requires a partner name".into());
                }
            }
            Function::Divorce => {}
            _ => return Err("Unknown world operation".into()),
        }
        Ok(())
    }

    pub fn call(&self, server: &Server, character: &mut Character, function: Function, args: &[Value], now: u64) -> Result<Value, String> {
        if character.game_systems.is_trading() && matches!(function, Function::OpenStorage | Function::GuildOpenStorage) {
            return Err("Storage is unavailable during trading".into());
        }
        self.validate_call(server.state(), character, function, args, now)?;
        if pet_support_operation(function) {
            return self.call_pet_support(server, character, function, args, now);
        }
        if persistent_world_operation(function) {
            let plan = plan_persistent_effects(character, &[(function, args.to_vec())], now)?;
            let committed = self
                .repository
                .commit_world_effects(character.char_id, &plan)
                .map_err(|error| error.to_string())?;
            self.apply_committed_effects(server, character, &plan, committed, now)?;
            return Ok(Value::default());
        }
        match function {
            Function::GetPetInfo => {
                let target_id = optional_number(args, 1, character.char_id as i32)? as u32;
                let pet = if target_id == character.char_id {
                    character.script_character_state().pet
                } else {
                    server
                        .state()
                        .get_character(target_id)
                        .and_then(|target| target.script_character_state().pet)
                };
                Ok(pet_information(pet.as_ref(), number(args, 0)?))
            }
            Function::VipStatus => {
                let target = if let Some(Value::String(name)) = args.get(1) {
                    if *name == character.name {
                        character.account_id
                    } else {
                        server
                            .state()
                            .characters()
                            .values()
                            .find(|target| target.name == *name)
                            .ok_or("VIP query character is offline")?
                            .account_id
                    }
                } else {
                    character.account_id
                };
                let expires = self
                    .repository
                    .account_game_systems(target)
                    .map_err(|error| error.to_string())?
                    .vip_expires_at;
                let now_seconds = now / 1000;
                let result = match optional_number(args, 0, 1)? {
                    1 => u64::from(expires > now_seconds),
                    2 => {
                        if expires > now_seconds {
                            expires
                        } else {
                            0
                        }
                    }
                    _ => expires.saturating_sub(now_seconds),
                };
                Ok(Value::Number(result.min(i32::MAX as u64) as i32))
            }
            Function::GetPartnerId => {
                let target = optional_number(args, 0, character.char_id as i32)? as u32;
                let partner = if target == character.char_id {
                    character.game_systems.partner_id
                } else {
                    self.repository
                        .character_game_systems(target)
                        .map_err(|error| error.to_string())?
                        .partner_id
                };
                Ok(Value::Number(partner as i32))
            }
            Function::CheckCart | Function::CheckRiding | Function::CheckFalcon | Function::CheckMadogear | Function::IsMounting => {
                let target = optional_number(args, 0, character.char_id as i32)? as u32;
                let other;
                let target = if target == character.char_id {
                    &*character
                } else {
                    other = server.state();
                    other.get_character(target).ok_or("State query character is offline")?
                };
                let present = match function {
                    Function::CheckCart => {
                        target.options
                            & (PlayerOption::Cart1.as_flag()
                                | PlayerOption::Cart2.as_flag()
                                | PlayerOption::Cart3.as_flag()
                                | PlayerOption::Cart4.as_flag()
                                | PlayerOption::Cart5.as_flag())
                            != 0
                    }
                    Function::CheckRiding => target.options & PlayerOption::Riding.as_flag() != 0,
                    Function::CheckFalcon => target.options & PlayerOption::Falcon.as_flag() != 0,
                    Function::CheckMadogear => target.options & PlayerOption::Madogear.as_flag() != 0,
                    _ => {
                        target.game_systems.mounting
                            || target.status.active_statuses.iter().any(|status| {
                                status.kind.name() == "SC_ALL_RIDING" && status.expires_at.is_none_or(|expires| expires > now as u128)
                            })
                    }
                };
                Ok(Value::Number(i32::from(present)))
            }
            Function::Pet => {
                character.game_systems.pet_capture = Some(PetCapture {
                    item_id: pet_lure_id(number(args, 0)?)?,
                    flag: optional_number(args, 1, 0)? as u8,
                    expires_at: now + 60_000,
                });
                self.send(character.char_id, 0x019Eu16.to_le_bytes().to_vec())?;
                Ok(Value::default())
            }
            Function::BirthPet => {
                let mut packet = protocol::header(0x01A6);
                packet.extend_from_slice(&0u16.to_le_bytes());
                for (index, item) in character
                    .inventory
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| item.as_ref().map(|item| (index, item)))
                {
                    if item.card0 == 256 && item.amount > 0 && world_data().pets.iter().any(|pet| pet.egg_item == item.item_id) {
                        packet.extend_from_slice(&(index as u16 + 2).to_le_bytes());
                    }
                }
                protocol::set_length(&mut packet);
                character.game_systems.pet_hatching = true;
                self.send(character.char_id, packet)?;
                Ok(Value::default())
            }
            Function::Homevolution => {
                if !Self::evolve_homunculus(character)? {
                    self.send(character.char_id, protocol::emotion(character.char_id, 14))?;
                    return Ok(Value::Number(0));
                }
                self.persist(character)?;
                if let Some(homunculus) = &character.game_systems.homunculus {
                    let mut sprite = protocol::header(0x01B0);
                    sprite.extend_from_slice(&homunculus_world_id(homunculus).to_le_bytes());
                    sprite.push(0);
                    sprite.extend_from_slice(&u32::from(homunculus.class_id).to_le_bytes());
                    self.area(character, sprite)?;
                }
                self.send_homunculus(character)?;
                Ok(Value::Number(1))
            }
            Function::MercenaryCreate => {
                let class = number(args, 0)? as u16;
                let data = world_data()
                    .mercenaries
                    .iter()
                    .find(|mercenary| mercenary.class_id == class)
                    .unwrap();
                let guild = if (6017..=6026).contains(&class) {
                    0
                } else if (6027..=6036).contains(&class) {
                    1
                } else {
                    2
                };
                character.game_systems.mercenary = Some(MercenaryRecord {
                    id: 0,
                    class_id: class,
                    name: data.name.clone(),
                    level: data.level,
                    hp: data.hp,
                    sp: data.sp,
                    max_hp: data.hp,
                    max_sp: data.sp,
                    expires_at: now + number(args, 1)? as u64,
                    contract_remaining: None,
                    skill_cooldowns: BTreeMap::new(),
                    cooldown_pause_at: None,
                    guild,
                    kill_count: 0,
                    statuses: Vec::new(),
                    last_attack_at: 0,
                    last_regen_hp_at: now,
                    last_regen_sp_at: now,
                });
                let calls = character.game_systems.mercenary_calls.entry(guild).or_default();
                *calls = calls.saturating_add(1).min(i32::MAX as u32);
                self.persist(character)?;
                self.send_mercenary(character, now)?;
                self.render_companions(server, character, now)?;
                Ok(Value::default())
            }
            Function::MercenaryHeal => {
                if let Some(mercenary) = character.game_systems.mercenary.as_mut() {
                    mercenary.hp = (i64::from(mercenary.hp) + i64::from(number(args, 0)?)).clamp(0, i64::from(mercenary.max_hp)) as u32;
                    mercenary.sp = (i64::from(mercenary.sp) + i64::from(number(args, 1)?)).clamp(0, i64::from(mercenary.max_sp)) as u32;
                    self.persist(character)?;
                    self.send_mercenary(character, now)?;
                }
                Ok(Value::default())
            }
            Function::MercenaryStartStatus => {
                if let Some(mercenary) = character.game_systems.mercenary.as_mut() {
                    let kind = status_kind(&args[0])?;
                    start_mercenary_status(
                        mercenary,
                        StatusChangeRequest::guaranteed(kind, number(args, 1)?, number(args, 2)?),
                        now,
                    )?;
                    if kind == StatusChangeKind::MercHpUp {
                        mercenary.hp = mercenary.max_hp;
                    }
                    if kind == StatusChangeKind::MercSpUp {
                        mercenary.sp = mercenary.max_sp;
                    }
                    self.persist(character)?;
                    self.send_mercenary(character, now)?;
                }
                Ok(Value::default())
            }
            Function::GuildExperience => {
                let guild = self
                    .repository
                    .guild_add_experience(character.char_id, number(args, 0)? as u64, &world_data().guild_experience)
                    .map_err(|error| error.to_string())?;
                if let Some(guild) = guild {
                    self.broadcast_guild_summary(server, character, &guild)?;
                }
                Ok(Value::default())
            }
            Function::BuyingStore => {
                if server.state().map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoBuyingStore) {
                    return Err("Buying stores are disabled on this map".into());
                }
                if character.status.hp == 0 || character.game_systems.is_trading() || character.game_systems.vending_store.is_some()
                    || character.game_systems.buying_store.is_some() || character.game_systems.storage_open || character.game_systems.guild_storage_open.is_some() {
                    return Err("Buying store cannot be prepared in the current state".into());
                }
                if !(1..=5).contains(&number(args, 0)?) { return Err("Buying store supports between one and five slots".into()); }
                character.game_systems.buying_slots = number(args, 0)? as u8;
                self.send(character.char_id, vec![0x10, 0x08, character.game_systems.buying_slots])?;
                Ok(Value::default())
            }
            Function::SearchStores => {
                let uses = number(args, 0)? as u8;
                let remote = number(args, 1)? == 1;
                let map = match args.get(2) {
                    Some(Value::String(map)) if map == "all" => None,
                    Some(Value::String(map)) if map != "this" => Some(map.trim_end_matches(".gat").to_string()),
                    _ => Some(character.map_instance_key.map_without_ext()),
                };
                character.game_systems.store_search = Some(StoreSearch {
                    remaining_uses: uses,
                    next_query_at: 0,
                    remote,
                    map,
                    results: Vec::new(),
                    next_page: 0,
                });
                let mut packet = protocol::header(0x083A);
                packet.extend_from_slice(&(u16::from(remote)).to_le_bytes());
                if server.packetver() >= 20100701 {
                    packet.push(uses);
                }
                self.send(character.char_id, packet)?;
                Ok(Value::default())
            }
            Function::OpenStorage => {
                if character.status.hp == 0 || character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some() {
                    return Ok(Value::Number(1));
                }
                if character.game_systems.guild_storage_open.is_some() {
                    return Ok(Value::Number(1));
                }
                character.game_systems.storage_items = self
                    .repository
                    .account_storage(character.account_id)
                    .map_err(|error| error.to_string())?;
                character.game_systems.storage_open = true;
                self.send_storage(character)?;
                Ok(Value::default())
            }
            Function::Marriage => {
                let name = args[0].string_value()?.to_string();
                let first = character.char_id;
                let mut state = server.state_mut();
                let partner = state
                    .characters_mut()
                    .values_mut()
                    .find(|other| other.name == name && other.char_id != first)
                    .ok_or("Marriage partner is offline")?;
                if character.game_systems.partner_id != 0 || partner.game_systems.partner_id != 0 {
                    return Ok(Value::Number(0));
                }
                self.repository
                    .marry_characters(first, partner.char_id)
                    .map_err(|error| error.to_string())?;
                partner.game_systems.partner_id = first;
                character.game_systems.partner_id = partner.char_id;
                partner.game_systems.revision = self
                    .repository
                    .character_game_systems(partner.char_id)
                    .map_err(|error| error.to_string())?
                    .revision;
                character.game_systems.revision = self
                    .repository
                    .character_game_systems(first)
                    .map_err(|error| error.to_string())?
                    .revision;
                for member in [&mut *partner, &mut *character] {
                    crate::server::service::script_character_service::grant_skill(
                        server,
                        member,
                        &[Value::Number(CALL_PARTNER_SKILL_ID), Value::Number(1), Value::Number(3)],
                    )?;
                    server.start_wedding(member)?;
                }
                Ok(Value::Number(1))
            }
            Function::Divorce => {
                if character.game_systems.partner_id == 0 {
                    return Ok(Value::Number(0));
                }
                let partner_id = self
                    .repository
                    .divorce_character(character.char_id)
                    .map_err(|error| error.to_string())?;
                character.game_systems.partner_id = 0;
                character.game_systems.revision = self
                    .repository
                    .character_game_systems(character.char_id)
                    .map_err(|error| error.to_string())?
                    .revision;
                let revoke = [Value::Number(CALL_PARTNER_SKILL_ID), Value::Number(0), Value::Number(3)];
                crate::server::service::script_character_service::grant_skill(server, character, &revoke)?;
                server.drop_wedding_ring(character);
                if let Some(partner) = server.state_mut().characters_mut().get_mut(&partner_id) {
                    server.drop_wedding_ring(partner);
                    partner.game_systems.partner_id = 0;
                    partner.game_systems.revision = self
                        .repository
                        .character_game_systems(partner_id)
                        .map_err(|error| error.to_string())?
                        .revision;
                    crate::server::service::script_character_service::grant_skill(server, partner, &revoke)?;
                }
                Ok(Value::Number(1))
            }
            Function::SetCart => {
                self.set_cart(character, optional_number(args, 0, 1)? as u8, false)?;
                Ok(Value::default())
            }
            Function::SetFalcon | Function::SetRiding => {
                let mount = if function == Function::SetFalcon { PlayerOption::Falcon } else { PlayerOption::Riding };
                self.set_mount_option(character, mount, optional_number(args, 0, 1)? != 0)?;
                server.character_service().reload_client_side_status(character);
                Ok(Value::default())
            }
            Function::GuildOpenStorage => Ok(Value::Number(i32::from(self.open_guild_storage(character)?))),
            _ => Err("Unknown world operation".into()),
        }
    }

    fn fame_list(&self, character: &Character, kind: u8) -> Result<(), String> {
        use crate::repository::fame_repository::FameCategory;
        let category = match kind {
            0 => FameCategory::Blacksmith,
            1 => FameCategory::Alchemist,
            2 => FameCategory::Taekwon,
            _ => {
                return self.send(character.char_id, protocol::fame_list(kind, &[], 0, GlobalConfigService::instance().packetver()));
            }
        };
        let rankings = self.repository.fame_rankings(category).map_err(|error| error.to_string())?;
        let own = self.repository.fame_points(category, character.char_id).map_err(|error| error.to_string())?;
        self.send(character.char_id, protocol::fame_list(kind, &rankings, own, GlobalConfigService::instance().packetver()))
    }

    fn call_partner(&self, server: &Server, state: &ServerState, character: &Character) -> Result<(), String> {
        use crate::server::model::map_flags::MapFlag;
        let partner = state
            .characters()
            .get(&character.game_systems.partner_id)
            .filter(|_| character.game_systems.partner_id != 0)
            .ok_or("Your partner is not online")?;
        if partner.status.hp == 0 || partner.is_dead() {
            return Err("Your partner cannot be called while dead".into());
        }
        if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoWarpTo) {
            return Err("Partners cannot be called to this map".into());
        }
        let origin = state.map_flags(&partner.map_instance_key);
        if origin.enabled(MapFlag::NoWarp) || origin.enabled(MapFlag::NoTeleport) {
            return Err("Your partner cannot leave their current map".into());
        }
        server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::ScriptWarp(crate::server::model::events::game_event::ScriptWarp {
            char_id: partner.char_id,
            map: character.current_map_name().clone(),
            x: character.x,
            y: character.y,
            destination_instance: Some(character.current_map_instance()),
        }));
        Ok(())
    }

    pub fn tick(&self, server: &Server, character: &mut Character, now: u64) -> Result<bool, String> {
        self.tick_in_state(server, server.state(), character, now)
    }

    pub fn tick_in_state(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64) -> Result<bool, String> {
        self.drain_notifications()?;
        self.party_tick(server, character, now)?;
        if now < character.game_systems.last_companion_tick + 40 {
            return Ok(false);
        }
        character.game_systems.last_companion_tick = now;
        let previous_pet_bonus = pet_bonus_state(character);
        if character
            .game_systems
            .pending_pet_capture
            .as_ref()
            .is_some_and(|pending| now >= pending.expires_at || character.status.hp == 0 || pending.map_key != character.map_instance_key)
        {
            self.cancel_pet_capture_in_state(server, state, character);
        }
        let mut changed = resume_companion_timers(&mut character.game_systems, now);
        if character
            .game_systems
            .pet_capture
            .as_ref()
            .is_some_and(|capture| now >= capture.expires_at)
        {
            character.game_systems.pet_capture = None;
        }
        if let Some(pet) = character.game_systems.pet.as_mut() {
            if let Some(data) = world_data().pets.iter().find(|data| data.class_id == pet.class_id) {
                if character.status.hp == 0 && !character.game_systems.pet_owner_dead {
                    pet.intimacy = (pet.intimacy + data.intimacy_owner_die).max(0);
                    changed = true;
                }
                changed |= advance_pet_hunger(pet, data, now);
            }
        }
        character.game_systems.pet_owner_dead = character.status.hp == 0;
        if character.game_systems.pet.as_ref().is_some_and(|pet| {
            pet.intimacy == 0
                && world_data()
                    .pets
                    .iter()
                    .any(|data| data.class_id == pet.class_id && data.intimacy_start > 0)
        }) {
            character.game_systems.pet = None;
            changed = true;
        }
        if let Some(mercenary) = character.game_systems.mercenary.as_mut() {
            if mercenary.hp == 0 || now >= mercenary.expires_at {
                let death = mercenary.hp == 0;
                if mercenary.guild < 3 {
                    let faith = character.game_systems.mercenary_faith.entry(mercenary.guild).or_default();
                    *faith = if death {
                        faith.saturating_sub(1)
                    } else {
                        faith.saturating_add(1).min(i16::MAX as u16)
                    };
                }
                character.game_systems.mercenary = None;
                changed = true;
            } else {
                let previous = mercenary.statuses.clone();
                let previous_hp = mercenary.hp;
                let previous_sp = mercenary.sp;
                let mut status = mercenary_status(mercenary);
                StatusEffectService::periodic_resources(&mut status, now as u128);
                StatusEffectService::expire_statuses(&mut status, now as u128);
                let periodic = StatusEffectService::periodic_damage_for_target(&mut status, now as u128, fastrand::u32(200..=799), false);
                status.hp = status.hp.saturating_sub(periodic);
                mercenary.hp = status.hp;
                mercenary.sp = status.sp;
                mercenary.statuses = status.active_statuses;
                let no_regen = mercenary.statuses.iter().any(|status| {
                    matches!(
                        status.kind,
                        StatusChangeKind::Poison
                            | StatusChangeKind::DeadlyPoison
                            | StatusChangeKind::Bleeding
                            | StatusChangeKind::TrickDead
                    )
                });
                if !no_regen && mercenary.hp > 0 {
                    let regeneration = companion_regeneration(&mercenary.statuses);
                    let data = world_data()
                        .mercenaries
                        .iter()
                        .find(|data| data.class_id == mercenary.class_id)
                        .unwrap();
                    if regeneration.0 && now >= mercenary.last_regen_hp_at + 6000 {
                        mercenary.hp = mercenary
                            .hp
                            .saturating_add(
                                (((mercenary.max_hp as f64 * f64::from(data.stats[2]) / 10000.0 + 1.0) * 6.0) as u32)
                                    .saturating_mul(regeneration.2)
                                    / 100,
                            )
                            .min(mercenary.max_hp);
                        mercenary.last_regen_hp_at = now;
                    }
                    if regeneration.1 && now >= mercenary.last_regen_sp_at + 8000 {
                        let regen = (mercenary.max_sp as f64 * (f64::from(data.stats[3]) + 10.0) / 750.0 + 1.0) as u32;
                        mercenary.sp = mercenary
                            .sp
                            .saturating_add(regen.saturating_mul(regeneration.3) / 100)
                            .min(mercenary.max_sp);
                        mercenary.last_regen_sp_at = now;
                    }
                }
                if previous != mercenary.statuses || previous_hp != mercenary.hp || previous_sp != mercenary.sp {
                    recalculate_mercenary(mercenary);
                    changed = true;
                }
            }
        }
        if let Some(homunculus) = character.game_systems.homunculus.as_mut() {
            if homunculus.active && homunculus.hp > 0 {
                let before = (
                    homunculus.hp,
                    homunculus.sp,
                    homunculus.max_hp,
                    homunculus.max_sp,
                    homunculus.statuses.clone(),
                );
                let mut status = homunculus::homunculus_status(homunculus);
                StatusEffectService::periodic_resources(&mut status, now as u128);
                StatusEffectService::expire_statuses(&mut status, now as u128);
                let periodic = StatusEffectService::periodic_damage_for_target(&mut status, now as u128, fastrand::u32(200..=799), false);
                homunculus.hp = status.hp.saturating_sub(periodic);
                homunculus.sp = status.sp;
                homunculus.statuses = status.active_statuses;
                homunculus::recalculate_homunculus(homunculus);
                if homunculus.hp == 0 {
                    homunculus.active = false;
                    homunculus.statuses.clear();
                }
                changed |= before
                    != (
                        homunculus.hp,
                        homunculus.sp,
                        homunculus.max_hp,
                        homunculus.max_sp,
                        homunculus.statuses.clone(),
                    );
                let id = homunculus_world_id(homunculus);
                let command = character.game_systems.companion_commands.entry(id).or_default();
                if command.last_regen_hp_at == 0 {
                    command.last_regen_hp_at = now;
                }
                if command.last_regen_sp_at == 0 {
                    command.last_regen_sp_at = now;
                }
                let stationary = command.next_move_at <= now
                    && homunculus.hp > 0
                    && !homunculus.statuses.iter().any(|status| {
                        matches!(
                            status.kind,
                            StatusChangeKind::Poison | StatusChangeKind::DeadlyPoison | StatusChangeKind::Bleeding
                        )
                    });
                let stats = homunculus::homunculus_stats(homunculus);
                let regeneration = companion_regeneration(&homunculus.statuses);
                if stationary && regeneration.0 && now >= command.last_regen_hp_at + 6000 {
                    let hp = homunculus.hp;
                    let skin = homunculus::homunculus_skill_level(homunculus, "HAMI_SKIN");
                    let regen = (u32::from(stats[2]) / 5 + (homunculus.max_hp / 200).max(1)) * (100 + 5 * u32::from(skin)) / 100;
                    homunculus.hp = hp.saturating_add(regen.saturating_mul(regeneration.2) / 100).min(homunculus.max_hp);
                    command.last_regen_hp_at = now;
                    changed |= hp != homunculus.hp;
                }
                if stationary && regeneration.1 && now >= command.last_regen_sp_at + 8000 {
                    let sp = homunculus.sp;
                    let brain = homunculus::homunculus_skill_level(homunculus, "HLIF_BRAIN");
                    let mut regen = 1 + u32::from(stats[3]) / 6 + homunculus.max_sp / 100;
                    if stats[3] >= 120 {
                        regen += u32::from(stats[3] - 120) / 2 + 4;
                    }
                    regen = regen * (100 + 3 * u32::from(brain)) / 100;
                    homunculus.sp = sp.saturating_add(regen.saturating_mul(regeneration.3) / 100).min(homunculus.max_sp);
                    command.last_regen_sp_at = now;
                    changed |= sp != homunculus.sp;
                }
                if homunculus.hp > 0 && homunculus.next_hunger_at == 0 {
                    homunculus.next_hunger_at = now + 60_000;
                    changed = true;
                } else if homunculus.hp > 0 && now >= homunculus.next_hunger_at {
                    if homunculus.hunger == 0 {
                        homunculus.intimacy = homunculus.intimacy.saturating_sub(100);
                    } else {
                        homunculus.hunger -= 1;
                    }
                    let delay = world_data()
                        .homunculi
                        .iter()
                        .find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id)
                        .map(|data| data.hungry_delay)
                        .unwrap_or(60_000);
                    homunculus.next_hunger_at = now + if homunculus.hunger <= 10 { 20_000 } else { delay };
                    changed = true;
                }
            }
        }
        if character
            .game_systems
            .homunculus
            .as_ref()
            .is_some_and(|homunculus| homunculus.intimacy == 0)
        {
            character.game_systems.homunculus = None;
            changed = true;
        }
        if changed {
            self.persist(character)?;
            self.send_pet(character)?;
            self.send_homunculus(character)?;
            self.send_mercenary(character, now)?;
        }
        self.tick_pet_support(server, state, character, now)?;
        self.tick_pet_loot(server, state, character, now)?;
        self.refresh_pet_bonuses(server, character, previous_pet_bonus);
        self.render_companions(server, character, now)?;
        self.reveal_companions(server, character, now)?;
        self.tick_companion_skills(server, character, now);
        self.companion_attack(server, state, character, now)?;
        Ok(changed)
    }

    pub fn persist(&self, character: &mut Character) -> Result<(), String> {
        match self
            .repository
            .save_character_game_systems(character.char_id, &character.game_systems)
        {
            Ok(state) => {
                install_state(character, state);
                Ok(())
            }
            Err(error) => {
                if let Ok(committed) = self.repository.character_game_systems(character.char_id) {
                    install_state(character, committed);
                }
                Err(error.to_string())
            }
        }
    }

    pub fn disconnect(&self, character: &mut Character) -> Result<(), String> {
        self.close_storage(character)?;
        pause_companion_timers(&mut character.game_systems, crate::util::tick::get_tick() as u64);
        self.close_vending(character)?;
        self.close_buying(character)?;
        for id in character.game_systems.rendered_companions.keys().copied().collect::<Vec<_>>() {
            self.area(character, self.vanish(id))?;
        }
        character.game_systems.rendered_companions.clear();
        character.game_systems.pet_capture = None;
        character.game_systems.pet_hatching = false;
        character.game_systems.store_search = None;
        character.game_systems.storage_open = false;
        character.game_systems.storage_items.clear();
        character.game_systems.guild_invitation = None;
        character.game_systems.companion_commands.clear();
        self.persist(character)
    }

    fn evolve_homunculus(character: &mut Character) -> Result<bool, String> {
        evolve_homunculus_systems(&mut character.game_systems)
    }

    pub fn create_homunculus(&self, character: &mut Character, class_id: u16) -> Result<(), String> {
        if character.game_systems.homunculus.is_some() {
            return Err("A homunculus already exists".into());
        }
        let data = world_data()
            .homunculi
            .iter()
            .find(|data| data.class_id == class_id)
            .ok_or("Unknown pre-renewal homunculus class")?;
        character.game_systems.homunculus = Some(HomunculusRecord {
            id: 0,
            class_id,
            name: data.name.clone(),
            level: 1,
            intimacy: 2100,
            hunger: 32,
            hp: 10,
            max_hp: data.base[0],
            sp: 0,
            max_sp: data.base[1],
            base_max_hp: data.base[0],
            base_max_sp: data.base[1],
            statuses: Vec::new(),
            stats: data.base[2..]
                .iter()
                .map(|stat| *stat as u16)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            active: true,
            evolved: false,
            skill_points: 0,
            skills: BTreeMap::new(),
            experience: 0,
            renamed: false,
            next_hunger_at: 0,
            skill_cooldowns: BTreeMap::new(),
            cooldown_pause_at: None,
        });
        self.persist(character)?;
        self.send_homunculus(character)
    }

    pub(crate) fn send(&self, char_id: u32, packet: Vec<u8>) -> Result<(), String> {
        self.enqueue_notification(Notification::Char(CharNotification::new(char_id, packet)))
    }

    pub(crate) fn area(&self, character: &Character, packet: Vec<u8>) -> Result<(), String> {
        self.enqueue_notification(Notification::Area(AreaNotification::from_character(character, packet)))
    }

    fn enqueue_notification(&self, notification: Notification) -> Result<(), String> {
        let mut pending = self
            .pending_notifications
            .lock()
            .map_err(|_| "World notification queue was poisoned")?;
        if pending.len() >= 16384 {
            return Err("World notification queue is full".into());
        }
        pending.push_back(notification);
        Self::deliver_notifications(&self.notifications, &mut pending)
    }

    pub(crate) fn drain_notifications(&self) -> Result<(), String> {
        let mut pending = self
            .pending_notifications
            .lock()
            .map_err(|_| "World notification queue was poisoned")?;
        Self::deliver_notifications(&self.notifications, &mut pending)
    }

    fn deliver_notifications(sender: &SyncSender<Notification>, pending: &mut VecDeque<Notification>) -> Result<(), String> {
        for _ in 0..64 {
            let Some(notification) = pending.pop_front() else {
                break;
            };
            match sender.try_send(notification) {
                Ok(()) => {}
                Err(TrySendError::Full(notification)) => {
                    pending.push_front(notification);
                    break;
                }
                Err(TrySendError::Disconnected(_)) => {
                    pending.clear();
                    return Err("Client notification queue is unavailable".into());
                }
            }
        }
        Ok(())
    }

    pub(crate) fn stop_for_store(&self, server: &Server, character: &mut Character) -> Result<(), String> {
        let casting = character.skill_in_use.is_some() || character.script_skill_state.casting_until > crate::util::tick::get_tick();
        character.clear_attack();
        character.movements.clear();
        character.pending_item_skill = None;
        character.clear_skill_in_use();
        server.script_skill_service().cancel_queued_cast(character);
        if casting {
            let mut packet = protocol::header(0x01B9);
            packet.extend_from_slice(&character.char_id.to_le_bytes());
            self.area(character, packet)?;
        }
        let mut packet = protocol::header(0x0088);
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&character.x.to_le_bytes());
        packet.extend_from_slice(&character.y.to_le_bytes());
        self.area(character, packet)
    }

    fn vanish(&self, id: u32) -> Vec<u8> {
        let mut packet = PacketZcNotifyVanish::new(self.configuration.packetver());
        packet.set_gid(id);
        packet.set_atype(0);
        packet.fill_raw();
        packet.raw
    }

    pub(crate) fn send_pet(&self, character: &Character) -> Result<(), String> {
        if let Some(pet) = &character.game_systems.pet {
            let mut packet = protocol::header(0x01A2);
            protocol::fixed_string(&mut packet, &pet.name, 24);
            packet.push(u8::from(pet.renamed));
            for value in [
                pet.level,
                pet.hunger as u16,
                pet.intimacy as u16,
                pet.equipped_item as u16,
                pet.class_id,
            ] {
                packet.extend_from_slice(&value.to_le_bytes());
            }
            self.send(character.char_id, packet)?;
        }
        Ok(())
    }

    pub(crate) fn send_mercenary(&self, character: &Character, now: u64) -> Result<(), String> {
        if let Some(mercenary) = &character.game_systems.mercenary {
            let data = world_data()
                .mercenaries
                .iter()
                .find(|data| data.class_id == mercenary.class_id)
                .ok_or("Unknown mercenary class")?;
            let mut packet = protocol::header(0x029B);
            packet.extend_from_slice(&mercenary_world_id(mercenary).to_le_bytes());
            let snapshot = mercenary_snapshot(mercenary);
            for value in [
                (snapshot.atk_left_side() + snapshot.atk_right_side()).clamp(0, 32767) as u16,
                snapshot.matk_max(),
                snapshot.hit().max(0) as u16,
                snapshot.crit().max(0.0).floor() as u16,
                snapshot.def().max(0) as u16,
                snapshot.mdef().max(0) as u16,
                snapshot.flee().max(0) as u16,
                ((200.0 - snapshot.aspd()) * 10.0).clamp(100.0, 2000.0) as u16,
            ] {
                packet.extend_from_slice(&value.to_le_bytes());
            }
            protocol::fixed_string(&mut packet, &mercenary.name, 24);
            packet.extend_from_slice(&mercenary.level.to_le_bytes());
            for value in [
                mercenary.hp,
                mercenary.max_hp,
                mercenary.sp,
                mercenary.max_sp,
                (mercenary.expires_at / 1000).min(u32::MAX as u64) as u32,
            ] {
                packet.extend_from_slice(&value.to_le_bytes());
            }
            packet.extend_from_slice(
                &character
                    .game_systems
                    .mercenary_faith
                    .get(&mercenary.guild)
                    .copied()
                    .unwrap_or_default()
                    .to_le_bytes(),
            );
            packet.extend_from_slice(
                &character
                    .game_systems
                    .mercenary_calls
                    .get(&mercenary.guild)
                    .copied()
                    .unwrap_or_default()
                    .to_le_bytes(),
            );
            packet.extend_from_slice(&mercenary.kill_count.to_le_bytes());
            packet.extend_from_slice(&data.range.to_le_bytes());
            self.send(character.char_id, packet)?;
            self.area(
                character,
                StatusEffectService::visual_state_packet(mercenary_world_id(mercenary), &mercenary_status(mercenary)),
            )?;
            if now == 0 {
                return Ok(());
            }
            let mut skills = protocol::header(0x029D);
            skills.extend_from_slice(&0u16.to_le_bytes());
            for skill in &data.skills {
                if let Some(metadata) = crate::server::script::skill::metadata::SkillMetadata::all()
                    .iter()
                    .find(|metadata| metadata.name == skill.name)
                {
                    let inf = match metadata.target_type.as_deref() {
                        Some("Attack") => 1u16,
                        Some("Ground") => 2,
                        Some("Self") => 4,
                        Some("Support") => 16,
                        _ => 0,
                    };
                    let sp = metadata
                        .requires
                        .as_ref()
                        .and_then(|requires| requires.get("SpCost"))
                        .and_then(|value| {
                            crate::server::script::skill::metadata::SkillMetadata::json_level_value(value, skill.level, "Amount")
                        })
                        .unwrap_or(0)
                        .clamp(0, i32::from(u16::MAX)) as u16;
                    let range = metadata
                        .range(skill.level)
                        .unwrap_or(i32::from(data.range))
                        .unsigned_abs()
                        .min(u32::from(u16::MAX)) as u16;
                    skills.extend_from_slice(&(metadata.id as u16).to_le_bytes());
                    skills.extend_from_slice(&inf.to_le_bytes());
                    skills.extend_from_slice(&0u16.to_le_bytes());
                    skills.extend_from_slice(&(skill.level as u16).to_le_bytes());
                    skills.extend_from_slice(&sp.to_le_bytes());
                    skills.extend_from_slice(&range.to_le_bytes());
                    protocol::fixed_string(&mut skills, &skill.name, 24);
                    skills.push(0);
                }
            }
            protocol::set_length(&mut skills);
            self.send(character.char_id, skills)?;
        }
        Ok(())
    }

    pub(crate) fn send_homunculus(&self, character: &Character) -> Result<(), String> {
        if let Some(homunculus) = &character.game_systems.homunculus {
            self.send(character.char_id, protocol::homunculus_info(homunculus))?;
            self.send_homunculus_skills(character)?;
            self.area(
                character,
                StatusEffectService::visual_state_packet(homunculus_world_id(homunculus), &homunculus::homunculus_status(homunculus)),
            )?;
        }
        Ok(())
    }

    pub(crate) fn send_storage(&self, character: &Character) -> Result<(), String> {
        let records = if character.game_systems.guild_storage_open.is_some() {
            &character.game_systems.guild_storage_items
        } else {
            &character.game_systems.storage_items
        };
        for packet in protocol::storage_packets(records, self.configuration, self.configuration.packetver()) {
            self.send(character.char_id, packet)?;
        }
        Ok(())
    }

    pub fn open_guild_storage(&self, character: &mut Character) -> Result<u8, String> {
        use crate::server::model::game_systems::GuildStorageAccess;
        if character.status.hp == 0 || character.game_systems.is_trading() || character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some() {
            return Ok(1);
        }
        if character.game_systems.guild_id == 0 {
            return Ok(3);
        }
        if character.game_systems.storage_open {
            return Ok(1);
        }
        if character.game_systems.guild_storage_open.is_some() {
            return Ok(2);
        }
        let result = self
            .repository
            .open_guild_storage(character.char_id)
            .map_err(|error| error.to_string())?;
        match result {
            GuildStorageAccess::Open { guild_id, items } => {
                character.game_systems.guild_storage_open = Some(guild_id);
                character.game_systems.guild_storage_items = items;
                self.send_storage(character)?;
                Ok(0)
            }
            GuildStorageAccess::Busy => Ok(2),
            GuildStorageAccess::NoGuild => Ok(3),
            GuildStorageAccess::NoPermission => Ok(5),
        }
    }

    pub(crate) fn close_storage(&self, character: &mut Character) -> Result<(), String> {
        let opened = character.game_systems.storage_open || character.game_systems.guild_storage_open.is_some();
        if let Some(guild_id) = character.game_systems.guild_storage_open {
            self.repository
                .close_guild_storage(character.char_id, guild_id)
                .map_err(|error| error.to_string())?;
        }
        character.game_systems.storage_open = false;
        character.game_systems.storage_items.clear();
        character.game_systems.guild_storage_open = None;
        character.game_systems.guild_storage_items.clear();
        if opened {
            self.send(character.char_id, protocol::header(0x00F8))?;
        }
        Ok(())
    }

    fn render_companions(&self, server: &Server, character: &mut Character, now: u64) -> Result<(), String> {
        if character.status.status_change(StatusChangeKind::Devotion).is_some_and(|status| {
            let protector = status.values[0] as u32;
            (1_200_000_000..1_300_000_000).contains(&protector)
                && !character
                    .game_systems
                    .mercenary
                    .as_ref()
                    .is_some_and(|mercenary| mercenary_world_id(mercenary) == protector && mercenary.hp > 0 && mercenary.expires_at > now)
        }) {
            server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::CharacterEndStatus(
                crate::server::model::events::game_event::CharacterEndStatus {
                    char_id: character.char_id,
                    kind: Some(StatusChangeKind::Devotion),
                },
            ));
        }
        let mut active = Vec::new();
        if let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating) {
            active.push((pet_world_id(pet.id), pet.class_id, 7, 1, 1));
        }
        if let Some(homunculus) = &character.game_systems.homunculus {
            if homunculus.active && homunculus.hp > 0 {
                active.push((
                    homunculus_world_id(homunculus),
                    homunculus.class_id,
                    8,
                    homunculus.hp,
                    homunculus.max_hp,
                ));
            }
        }
        if let Some(mercenary) = &character.game_systems.mercenary {
            if mercenary.hp > 0 {
                active.push((
                    mercenary_world_id(mercenary),
                    mercenary.class_id,
                    9,
                    mercenary.hp,
                    mercenary.max_hp,
                ));
            }
        }
        let map_changed = (!character.game_systems.last_companion_map.is_empty()
            && character.game_systems.last_companion_map != *character.current_map_name())
            || character
                .game_systems
                .rendered_companions
                .values()
                .any(|position| position.map_instance != character.current_map_instance());
        for (id, position) in character.game_systems.rendered_companions.clone() {
            if map_changed || !active.iter().any(|(active, ..)| *active == id) {
                let map = character.game_systems.last_companion_map.clone();
                self.notifications
                    .try_send(Notification::Area(AreaNotification::new(
                        map,
                        position.map_instance,
                        AreaNotificationRangeType::Fov {
                            x: position.x,
                            y: position.y,
                            exclude_id: None,
                        },
                        self.vanish(id),
                    )))
                    .map_err(|_| "Client notification queue is unavailable")?;
                character.game_systems.rendered_companions.remove(&id);
                character.game_systems.companion_commands.remove(&id);
            }
        }
        for (index, (id, class, kind, hp, max_hp)) in active.into_iter().enumerate() {
            let (x, y) = self.companion_next_position(server, character, id, index as u16, now);
            let position = CompanionPosition {
                x,
                y,
                map_instance: character.current_map_instance(),
            };
            if let Some(previous) = character.game_systems.rendered_companions.get(&id) {
                if *previous != position {
                    let from = Position {
                        x: previous.x,
                        y: previous.y,
                        dir: character.dir,
                    };
                    let to = Position {
                        x: position.x,
                        y: position.y,
                        dir: character.dir,
                    };
                    let mut packet = PacketZcNotifyMove::new(self.configuration.packetver());
                    packet.set_gid(id);
                    packet.move_data = from.to_move_data(&to);
                    packet.set_move_start_time(now as u32);
                    packet.fill_raw();
                    self.area(character, packet.raw)?;
                }
            } else {
                let mut packet = PacketZcNotifyStandentry7::new(self.configuration.packetver());
                packet.set_job(class as i16);
                packet.set_packet_length(PacketZcNotifyStandentry7::base_len(self.configuration.packetver()) as i16);
                packet.set_pos_dir(
                    Position {
                        x: position.x,
                        y: position.y,
                        dir: character.dir,
                    }
                    .to_pos(),
                );
                packet.set_objecttype(kind);
                packet.set_aid(id);
                packet.set_gid(id);
                packet.set_clevel(companion_health(character, id).map_or(1, |(_, _, level)| level) as i16);
                packet
                    .set_speed(companion_status_snapshot(character, id).map_or(character.status.speed, |snapshot| snapshot.speed()) as i16);
                packet.set_hp(hp);
                packet.set_max_hp(max_hp);
                packet.fill_raw_with_packetver(Some(self.configuration.packetver()));
                self.area(character, packet.raw)?;
                if character.game_systems.pet.as_ref().is_some_and(|pet| pet_world_id(pet.id) == id) {
                    if let Some(packet) = pet_accessory_packet(character, self.configuration) {
                        self.area(character, packet)?;
                    }
                }
            }
            character.game_systems.rendered_companions.insert(id, position);
        }
        character.game_systems.last_companion_map = character.current_map_name().clone();
        Ok(())
    }

    fn companion_next_position(&self, server: &Server, character: &mut Character, id: u32, index: u16, now: u64) -> (u16, u16) {
        let follow = companion_follow_position(server, character, index);
        let Some(previous) = character.game_systems.rendered_companions.get(&id).copied() else {
            return follow;
        };
        if previous.x.abs_diff(character.x).max(previous.y.abs_diff(character.y)) > 15 {
            character.game_systems.companion_commands.remove(&id);
            return follow;
        }
        let source = companion_status_snapshot(character, id);
        if source
            .as_ref()
            .is_some_and(|source| source.active_statuses().iter().any(|status| status.kind.blocks_movement()))
        {
            return (previous.x, previous.y);
        }
        let commanded = character.game_systems.companion_commands.get(&id);
        if commanded.is_some_and(|command| now < command.next_move_at || command.cast.is_some())
            || character
                .game_systems
                .pet_support
                .as_ref()
                .is_some_and(|support| pet_world_id(support.pet_id) == id && support.casting.is_some())
        {
            return (previous.x, previous.y);
        }
        let target = commanded.and_then(|command| command.target).or_else(|| {
            if commanded.is_some_and(|command| command.stay) {
                None
            } else if character.game_systems.pet.as_ref().is_some_and(|pet| pet_world_id(pet.id) == id) {
                None
            } else {
                source.as_ref().and_then(|_| character.attack.map(|attack| attack.target))
            }
        });
        let range = companion_attack_range(character, id);
        let Some(map) = server.state().get_map_instance_from_character(character) else {
            return (previous.x, previous.y);
        };
        let map_state = map.state();
        let destination = if let Some(target) = target {
            if let Some(mob) = map_state.get_mob(target).filter(|mob| {
                mob.hp() > 0
                    && mob.summon_ai == 0
                    && !server.state().contains_locked_map_item(target)
                    && source.as_ref().is_some_and(|source| {
                        companion_can_target(
                            source,
                            &mob.status,
                            crate::server::service::visibility_service::TargetingMode::Direct,
                        )
                    })
            }) {
                if previous.x.abs_diff(mob.x).max(previous.y.abs_diff(mob.y)) <= range {
                    return (previous.x, previous.y);
                }
                if character.x.abs_diff(mob.x).max(character.y.abs_diff(mob.y)) > 15 {
                    character.game_systems.companion_commands.entry(id).or_default().target = None;
                    follow
                } else {
                    (mob.x, mob.y)
                }
            } else {
                character.game_systems.companion_commands.entry(id).or_default().target = None;
                follow
            }
        } else if let Some(destination) = commanded.and_then(|command| command.destination) {
            destination
        } else if commanded.is_some_and(|command| command.stay) {
            return (previous.x, previous.y);
        } else {
            follow
        };
        if (previous.x, previous.y) == destination {
            return destination;
        }
        let path = movement::path::path_search_client_side_algorithm(
            map_state.x_size(),
            map_state.y_size(),
            map_state.cells(),
            previous.x,
            previous.y,
            destination.0,
            destination.1,
        );
        let Some(step) = path.first() else {
            return (previous.x, previous.y);
        };
        let delay = source.as_ref().map(|source| source.speed()).unwrap_or(character.status.speed);
        character.game_systems.companion_commands.entry(id).or_default().next_move_at =
            now + movement::Movement::delay(delay, step.is_diagonal) as u64;
        (step.x, step.y)
    }

    fn companion_attack(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64) -> Result<(), String> {
        let ids = companion_snapshots(character)
            .into_iter()
            .filter(|snapshot| {
                *snapshot.map_item().object_type() != MapItemType::Pet
                    || self.pet_configuration().attack_support
                    || self.pet_configuration().damage_support
            })
            .map(|snapshot| snapshot.map_item().id())
            .collect::<Vec<_>>();
        for id in ids {
            self.companion_attack_one(server, state, character, id, now)?;
        }
        Ok(())
    }

    fn companion_attack_one(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        id: u32,
        now: u64,
    ) -> Result<(), String> {
        let command = character.game_systems.companion_commands.get(&id);
        let Some(target) = command.and_then(|command| command.target).or_else(|| {
            if command.is_some_and(|command| command.stay) {
                None
            } else if character.game_systems.pet.as_ref().is_some_and(|pet| pet_world_id(pet.id) == id) {
                None
            } else {
                character.attack.map(|attack| attack.target)
            }
        }) else {
            return Ok(());
        };
        let source = companion_status_snapshot(character, id).ok_or("Companion status is unavailable")?;
        let delay = ((200.0 - source.aspd()) * 10.0).round().clamp(100.0, 30_000.0) as u32;
        if command
            .is_some_and(|command| now < command.last_attack_at + u64::from(delay) || now < command.can_act_at || command.cast.is_some())
            || source.active_statuses().iter().any(|status| status.kind.blocks_attack())
            || character
                .game_systems
                .pet_support
                .as_ref()
                .is_some_and(|support| pet_world_id(support.pet_id) == id && (support.casting.is_some() || now < support.can_act_at))
        {
            return Ok(());
        }
        let position = character
            .game_systems
            .rendered_companions
            .get(&id)
            .copied()
            .unwrap_or(CompanionPosition {
                x: character.x,
                y: character.y,
                map_instance: character.current_map_instance(),
            });
        let Some(map) = state.get_map_instance_from_character(character) else {
            return Ok(());
        };
        let Some(mob) = map.state().get_mob(target).cloned() else {
            return Ok(());
        };
        let range = companion_attack_range(character, id);
        if mob.hp() == 0
            || mob.summon_ai != 0
            || state.contains_locked_map_item(target)
            || position.x.abs_diff(mob.x).max(position.y.abs_diff(mob.y)) > range
            || !companion_can_target(
                &source,
                &mob.status,
                crate::server::service::visibility_service::TargetingMode::Direct,
            )
        {
            return Ok(());
        }
        if character.game_systems.pet.as_ref().is_some_and(|pet| pet_world_id(pet.id) == id)
            && self.try_pet_attack_skill(server, state, character, target, now)?
        {
            return Ok(());
        }
        if let Some(damage) = server.battle_service().magical_normal_attack(&source, &mob.status, id, target,
            character.char_id, &character.map_instance_key, position.x, position.y, u128::from(now), delay, mob.damage_motion) {
            let command = character.game_systems.companion_commands.entry(id).or_default();
            command.last_attack_at = now;
            if command.target.is_some() && !command.repeat { command.target = None; command.stay = true; }
            map.add_to_delayed_tick(MapEvent::MobDamage(MobDamage { damage }), u128::from(delay));
            return Ok(());
        }
        let mut rng = fastrand::Rng::new();
        let roll = BattleService::normal_attack_roll(&source, &mob.status, range > 3, &mut rng);
        let hit = !matches!(roll, NormalAttackRoll::Miss | NormalAttackRoll::LuckyDodge);
        let critical = roll == NormalAttackRoll::Critical;
        let attack = companion_attack_damage(character, id, critical, &mut rng).ok_or("Companion status is unavailable")?;
        let attack = (f64::from(attack) * f64::from(BattleService::weapon_skill_ratio(&source, 1.0, 0)))
            .floor()
            .clamp(0.0, f64::from(u32::MAX)) as u32;
        let command = character.game_systems.companion_commands.entry(id).or_default();
        command.last_attack_at = now;
        if command.target.is_some() && !command.repeat {
            command.target = None;
            command.stay = true;
        }
        let battle_flags = BattleFlag::Weapon.as_flag()
            | BattleFlag::Normal.as_flag()
            | if range > 3 {
                BattleFlag::Long.as_flag()
            } else {
                BattleFlag::Short.as_flag()
            };
        let element = server.battle_service().attack_element(&source, None);
        let mut signed_damage = if hit {
            server
                .battle_service()
                .actor_physical_damage_signed(attack, &source, &mob.status, false, critical, &element, battle_flags)
        } else {
            0
        };
        if roll == NormalAttackRoll::DoubleAttack {
            signed_damage = signed_damage.saturating_mul(2);
        }
        let action = match roll {
            NormalAttackRoll::LuckyDodge => ActionType::AttackLucky,
            NormalAttackRoll::Critical => ActionType::AttackCritical,
            NormalAttackRoll::DoubleAttack => ActionType::AttackMultiple,
            _ => ActionType::Attack,
        };
        let damage = Damage {
            notification: None,
            source_kind: *source.combat_actor_kind(),
            skill_damage_adjusted: false,
            target_id: target,
            attacker_id: id,
            damage: signed_damage.max(0) as u32,
            attacked_at: now as u128,
            damage_motion: mob.damage_motion,
            battle_flags,
            skill_id: 0,
            skill_level: 0,
            landed: hit,
            proc_depth: 0,
            credit_id: character.char_id,
            defenses_applied: true,
            magic_context: None,
            right_hand_damage: None,
            healing: if signed_damage < 0 { signed_damage.unsigned_abs() } else { 0 },
        }
        .with_action_notification(
            character.current_map_name(),
            character.current_map_instance(),
            position.x,
            position.y,
            now as u128,
            if roll == NormalAttackRoll::DoubleAttack { 2 } else { 1 },
            delay,
            action,
            (signed_damage, 0),
        );
        map.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage }));
        Ok(())
    }
}

pub fn pet_world_id(id: u32) -> u32 {
    1_000_000_000 + id
}

fn companion_follow_position(server: &Server, character: &Character, index: u16) -> (u16, u16) {
    let Some(map) = server.state().get_map_instance_from_character(character) else {
        return (character.x, character.y);
    };
    let state = map.state();
    for (dx, dy) in [(-1 - index as i32, 0), (0, -1), (0, 1), (1, 0), (0, 0)] {
        let x = i32::from(character.x) + dx;
        let y = i32::from(character.y) + dy;
        if x < 0 || y < 0 || x >= i32::from(state.x_size()) || y >= i32::from(state.y_size()) {
            continue;
        }
        let cell = state
            .cells()
            .get(y as usize * usize::from(state.x_size()) + x as usize)
            .copied()
            .unwrap_or_default();
        if cell & CellType::Walkable.as_flag() != 0 {
            return (x as u16, y as u16);
        }
    }
    (character.x, character.y)
}
pub fn homunculus_world_id(homunculus: &HomunculusRecord) -> u32 {
    1_100_000_000 + homunculus.id
}
pub fn mercenary_world_id(mercenary: &MercenaryRecord) -> u32 {
    1_200_000_000 + mercenary.id
}

pub fn companion_attack_bounds(character: &Character, id: u32) -> Option<(u16, u16, u16)> {
    let snapshot = companion_status_snapshot(character, id)?;
    if let Some(pet) = character
        .game_systems
        .pet
        .as_ref()
        .filter(|pet| pet_world_id(pet.id) == id && !pet.incubating)
    {
        let data = GlobalConfigService::instance().get_mob_safe(i32::from(pet.class_id))?;
        let minimum = data.atk1.clamp(0, i32::from(u16::MAX)) as u16;
        let maximum = data.atk2.clamp(i32::from(minimum), i32::from(u16::MAX)) as u16;
        return Some((minimum, maximum, 0));
    }
    if let Some(mercenary) = character
        .game_systems
        .mercenary
        .as_ref()
        .filter(|mercenary| mercenary_world_id(mercenary) == id)
    {
        let data = world_data().mercenaries.iter().find(|data| data.class_id == mercenary.class_id)?;
        return Some((data.attack, data.attack.saturating_add(data.attack2), 0));
    }
    let homunculus = character
        .game_systems
        .homunculus
        .as_ref()
        .filter(|homunculus| homunculus_world_id(homunculus) == id)?;
    if snapshot.has_status_change(StatusChangeKind::HomChange) {
        return Some((0, 0, snapshot.matk_max()));
    }
    let maximum = snapshot.str().saturating_add(homunculus.level);
    Some((
        snapshot.dex().min(maximum),
        maximum,
        snapshot.str().saturating_add((snapshot.str() / 10).pow(2)),
    ))
}

pub fn companion_attack_damage(character: &Character, id: u32, critical: bool, rng: &mut fastrand::Rng) -> Option<u32> {
    let snapshot = companion_status_snapshot(character, id)?;
    let (minimum, maximum, base) = companion_attack_bounds(character, id)?;
    let weapon = if critical { maximum } else { rng.u16(minimum..=maximum) };
    let rate = companion_attack_rate(&snapshot);
    let raw = (((u32::from(weapon) + u32::from(base)) as f32 * rate).floor() as i64 + i64::from(snapshot.bonus_atk()))
        .clamp(0, i64::from(u32::MAX)) as u32;
    Some(raw)
}

fn companion_attack_rate(snapshot: &StatusSnapshot) -> f32 {
    let rate = snapshot
        .bonuses()
        .iter()
        .filter_map(|bonus| match bonus.bonus() {
            models::enums::bonus::BonusType::AtkPercentage(rate) => Some(i32::from(*rate)),
            _ => None,
        })
        .sum::<i32>();
    (1.0 + rate as f32 / 100.0).max(0.0)
}

fn mercenary_contract_active(mercenary: &MercenaryRecord, now: u64) -> bool {
    mercenary.hp > 0
        && mercenary
            .contract_remaining
            .map_or(mercenary.expires_at > now, |remaining| remaining > 0)
}

fn pause_companion_timers(systems: &mut CharacterGameSystems, now: u64) {
    if let Some(mercenary) = &mut systems.mercenary {
        if mercenary.contract_remaining.is_none() {
            mercenary.contract_remaining = Some(mercenary.expires_at.saturating_sub(now));
        }
        mercenary.skill_cooldowns.retain(|_, expiry| *expiry > now);
        mercenary.cooldown_pause_at = Some(now);
    }
    if let Some(homunculus) = &mut systems.homunculus {
        homunculus.skill_cooldowns.retain(|_, expiry| *expiry > now);
        homunculus.cooldown_pause_at = Some(now);
        homunculus.next_hunger_at = 0;
    }
    if let Some(pet) = &mut systems.pet {
        pet.next_hunger_at = 0;
    }
}

fn resume_companion_timers(systems: &mut CharacterGameSystems, now: u64) -> bool {
    let mut changed = false;
    if let Some(mercenary) = &mut systems.mercenary {
        if let Some(remaining) = mercenary.contract_remaining.take() {
            mercenary.expires_at = now.saturating_add(remaining);
            mercenary.last_regen_hp_at = now;
            mercenary.last_regen_sp_at = now;
            changed = true;
        }
        if let Some(paused) = mercenary.cooldown_pause_at.take() {
            for expiry in mercenary.skill_cooldowns.values_mut() {
                *expiry = expiry.saturating_add(now.saturating_sub(paused));
            }
            changed = true;
        }
    }
    if let Some(homunculus) = &mut systems.homunculus {
        if let Some(paused) = homunculus.cooldown_pause_at.take() {
            for expiry in homunculus.skill_cooldowns.values_mut() {
                *expiry = expiry.saturating_add(now.saturating_sub(paused));
            }
            changed = true;
        }
    }
    if let Some(pet) = &mut systems.pet {
        if pet.next_hunger_at == 0 {
            if let Some(definition) = world_data().pets.iter().find(|definition| definition.class_id == pet.class_id) {
                pet.next_hunger_at = now.saturating_add(definition.hungry_delay);
                changed = true;
            }
        }
    }
    changed
}

fn companion_cooldown_until(character: &Character, id: u32, skill_id: u32) -> u64 {
    if let Some(mercenary) = character
        .game_systems
        .mercenary
        .as_ref()
        .filter(|mercenary| mercenary_world_id(mercenary) == id)
    {
        return mercenary.skill_cooldowns.get(&skill_id).copied().unwrap_or(0);
    }
    character
        .game_systems
        .homunculus
        .as_ref()
        .filter(|homunculus| homunculus_world_id(homunculus) == id)
        .and_then(|homunculus| homunculus.skill_cooldowns.get(&skill_id).copied())
        .unwrap_or(0)
}

pub fn pet_accessory_packet(character: &Character, configuration: &GlobalConfigService) -> Option<Vec<u8>> {
    let pet = character.game_systems.pet.as_ref()?;
    let view = configuration
        .find_item(pet.equipped_item)
        .map_or(0, |item| item.view.filter(|view| *view > 0).unwrap_or(pet.equipped_item));
    let mut packet = protocol::header(0x01A4);
    packet.push(3);
    packet.extend_from_slice(&pet_world_id(pet.id).to_le_bytes());
    packet.extend_from_slice(&(view as u32).to_le_bytes());
    Some(packet)
}

pub fn companion_snapshots(character: &Character) -> Vec<MapItemSnapshot> {
    let mut snapshots = Vec::new();
    let mut add = |id, class, kind| {
        let position = character
            .game_systems
            .rendered_companions
            .get(&id)
            .map(|position| Position {
                x: position.x,
                y: position.y,
                dir: character.dir,
            })
            .unwrap_or(Position {
                x: character.x.saturating_sub(1),
                y: character.y,
                dir: character.dir,
            });
        snapshots.push(MapItemSnapshot::new(MapItem::new(id, class, kind), position));
    };
    if let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating) {
        add(pet_world_id(pet.id), pet.class_id as i16, MapItemType::Pet);
    }
    if let Some(homunculus) = &character.game_systems.homunculus {
        if homunculus.active && homunculus.hp > 0 {
            add(
                homunculus_world_id(homunculus),
                homunculus.class_id as i16,
                MapItemType::Homunculus,
            );
        }
    }
    if let Some(mercenary) = &character.game_systems.mercenary {
        if mercenary.hp > 0 {
            add(mercenary_world_id(mercenary), mercenary.class_id as i16, MapItemType::Mercenary);
        }
    }
    snapshots
}

pub fn companion_name(character: &Character, id: u32) -> Option<String> {
    if let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating) {
        if pet_world_id(pet.id) == id {
            return Some(pet.name.clone());
        }
    }
    if let Some(homunculus) = &character.game_systems.homunculus {
        if homunculus_world_id(homunculus) == id {
            return Some(homunculus.name.clone());
        }
    }
    if let Some(mercenary) = &character.game_systems.mercenary {
        if mercenary_world_id(mercenary) == id {
            return Some(mercenary.name.clone());
        }
    }
    None
}

pub fn companion_health(character: &Character, id: u32) -> Option<(u32, u32, u16)> {
    if let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating) {
        if pet_world_id(pet.id) == id {
            return Some((1, 1, pet.level));
        }
    }
    if let Some(homunculus) = &character.game_systems.homunculus {
        if homunculus_world_id(homunculus) == id {
            return Some((homunculus.hp, homunculus.max_hp, homunculus.level));
        }
    }
    if let Some(mercenary) = &character.game_systems.mercenary {
        if mercenary_world_id(mercenary) == id {
            return Some((mercenary.hp, mercenary.max_hp, mercenary.level));
        }
    }
    None
}

pub fn companion_status_snapshot(character: &Character, id: u32) -> Option<StatusSnapshot> {
    if character
        .game_systems
        .pet
        .as_ref()
        .is_some_and(|pet| pet_world_id(pet.id) == id && !pet.incubating)
    {
        return pet_combat::pet_snapshot(character);
    }
    if let Some(mercenary) = &character.game_systems.mercenary {
        if mercenary_world_id(mercenary) == id {
            return Some(mercenary_snapshot(mercenary));
        }
    }
    if let Some(homunculus) = &character.game_systems.homunculus {
        if homunculus_world_id(homunculus) == id {
            return homunculus::homunculus_snapshot(homunculus, character.status.speed);
        }
    }
    None
}

pub fn companion_visual_packet(character: &Character, id: u32) -> Option<Vec<u8>> {
    if let Some(mercenary) = character
        .game_systems
        .mercenary
        .as_ref()
        .filter(|mercenary| mercenary_world_id(mercenary) == id)
    {
        return Some(StatusEffectService::visual_state_packet(id, &mercenary_status(mercenary)));
    }
    character
        .game_systems
        .homunculus
        .as_ref()
        .filter(|homunculus| homunculus_world_id(homunculus) == id)
        .map(|homunculus| StatusEffectService::visual_state_packet(id, &homunculus::homunculus_status(homunculus)))
}

fn companion_can_target(
    source: &StatusSnapshot,
    target: &StatusSnapshot,
    mode: crate::server::service::visibility_service::TargetingMode,
) -> bool {
    use crate::server::service::visibility_service::{StealthState, VisibilityObserver, can_target};
    can_target(VisibilityObserver::player(source), StealthState::from_snapshot(target), mode)
}

impl ScriptWorldService {
    fn reveal_companions(&self, server: &Server, character: &Character, now: u64) -> Result<(), String> {
        for actor in companion_snapshots(character) {
            let Some(status) = companion_status_snapshot(character, actor.map_item().id()) else {
                continue;
            };
            let Some(sight) = status.status_change(StatusChangeKind::Sight) else {
                continue;
            };
            let position = actor.position();
            let range = sight.values[2].max(0) as u16;
            if character.x.abs_diff(position.x).max(character.y.abs_diff(position.y)) <= range {
                for kind in [StatusChangeKind::Hiding, StatusChangeKind::Cloaking] {
                    if character.status.has_status_change(kind) {
                        server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::CharacterEndStatus(
                            crate::server::model::events::game_event::CharacterEndStatus {
                                char_id: character.char_id,
                                kind: Some(kind),
                            },
                        ));
                    }
                }
            }
            server.script_skill_service().reveal_from_actor(
                server,
                server.state(),
                &crate::server::script::skill::ScriptRevealActor {
                    actor_id: actor.map_item().id(),
                    credit_id: character.char_id,
                    map: character.current_map_name().clone(),
                    instance: character.current_map_instance(),
                    x: position.x,
                    y: position.y,
                    status,
                    kind: StatusChangeKind::Sight,
                    damage_mobs: false,
                    damage_players: false,
                },
                u128::from(now),
            )?;
        }
        Ok(())
    }
}

fn companion_attack_range(character: &Character, id: u32) -> u16 {
    if let Some(pet) = character
        .game_systems
        .pet
        .as_ref()
        .filter(|pet| pet_world_id(pet.id) == id && !pet.incubating)
    {
        return GlobalConfigService::instance()
            .get_mob_safe(i32::from(pet.class_id))
            .map_or(1, |mob| mob.range1.max(1) as u16);
    }
    if let Some(mercenary) = character
        .game_systems
        .mercenary
        .as_ref()
        .filter(|mercenary| mercenary_world_id(mercenary) == id)
    {
        return world_data()
            .mercenaries
            .iter()
            .find(|data| data.class_id == mercenary.class_id)
            .map(|data| data.range)
            .unwrap_or(1);
    }
    companion_status_snapshot(character, id)
        .map(|snapshot| 1 + snapshot.size().value() as u16)
        .unwrap_or(1)
}

fn companion_maximum_pools(base_hp: u32, base_sp: u32, bonuses: &[models::enums::bonus::BonusType], berserk: bool) -> (u32, u32) {
    use models::enums::bonus::BonusType;
    let (mut hp_flat, mut sp_flat, mut hp_rate, mut sp_rate) = (0i64, 0i64, if berserk { 300i64 } else { 100 }, 100i64);
    for bonus in bonuses {
        match bonus {
            BonusType::Maxhp(value) => hp_flat += i64::from(*value),
            BonusType::Maxsp(value) => sp_flat += i64::from(*value),
            BonusType::MaxhpPercentage(value) => hp_rate += i64::from(*value),
            BonusType::MaxspPercentage(value) => sp_rate += i64::from(*value),
            _ => {}
        }
    }
    let hp = ((i128::from(base_hp) + i128::from(hp_flat)).max(1) * i128::from(hp_rate.max(0)) / 100).clamp(1, i128::from(u32::MAX)) as u32;
    let sp = ((i128::from(base_sp) + i128::from(sp_flat)).max(0) * i128::from(sp_rate.max(0)) / 100).clamp(0, i128::from(u32::MAX)) as u32;
    (hp, sp)
}

fn mercenary_snapshot(mercenary: &MercenaryRecord) -> StatusSnapshot {
    let data = world_data()
        .mercenaries
        .iter()
        .find(|data| data.class_id == mercenary.class_id)
        .unwrap();
    let [strength, agility, vitality, intelligence, dexterity, luck] = data.stats;
    let mut snapshot = StatusSnapshot::new_for_mob(
        u32::from(data.class_id),
        mercenary.hp,
        mercenary.sp,
        data.hp,
        data.sp,
        strength,
        agility,
        vitality,
        intelligence,
        dexterity,
        luck,
        data.attack,
        data.attack.saturating_add(data.attack2),
        intelligence,
        intelligence,
        data.walk_speed,
        data.defense,
        data.magic_defense,
        Size::try_from_string(&data.size).unwrap_or(Size::Small),
        Element::try_from_string(&data.element).unwrap_or(Element::Neutral),
        MobRace::try_from_string(&data.race).unwrap_or(MobRace::Formless),
        data.element_level,
    );
    snapshot.set_base_level(u32::from(mercenary.level));
    snapshot.set_combat_actor_kind(models::enums::actor::CombatActorKind::Mercenary);
    snapshot.set_aspd(200.0 - data.attack_delay as f32 / 10.0);
    snapshot.set_base_atk(0);
    let bonuses = mercenary.statuses.iter().flat_map(|status| status.bonuses()).collect::<Vec<_>>();
    let (max_hp, max_sp) = companion_maximum_pools(
        data.hp,
        data.sp,
        &bonuses,
        mercenary.statuses.iter().any(|status| status.kind == StatusChangeKind::Berserk),
    );
    for bonus in &bonuses {
        bonus.add_bonus_to_status(&mut snapshot);
    }
    StatusEffectService::adjust_status_attributes(&mercenary_status(mercenary), &mut snapshot);
    snapshot.set_hit((i32::from(snapshot.hit()) + i32::from(data.level) + i32::from(snapshot.dex())).clamp(0, i32::from(i16::MAX)) as i16);
    snapshot
        .set_flee((i32::from(snapshot.flee()) + i32::from(data.level) + i32::from(snapshot.agi())).clamp(0, i32::from(i16::MAX)) as i16);
    snapshot.set_crit(snapshot.crit() + 1.0 + (u32::from(snapshot.luk()) * 10 / 3) as f32 / 10.0);
    snapshot.set_matk_min(snapshot.int().saturating_add((snapshot.int() / 7).pow(2)));
    snapshot.set_matk_max(snapshot.int().saturating_add((snapshot.int() / 5).pow(2)));
    for bonus in &bonuses {
        if !matches!(bonus, models::enums::bonus::BonusType::AspdPercentage(_)) {
            bonus.add_percentage_bonus_to_status(&mut snapshot);
        }
    }
    snapshot.bonuses_mut().extend(bonuses.into_iter().map(StatusBonus::new));
    snapshot.set_active_statuses(mercenary.statuses.clone());
    snapshot.set_atk_left_side(i32::from(snapshot.bonus_atk()));
    snapshot.set_atk_right_side((f32::from(data.attack.saturating_add(data.attack2)) * companion_attack_rate(&snapshot)).floor() as i32);
    StatusEffectService::adjust_snapshot_for_target(&mercenary_status(mercenary), &mut snapshot, false);
    snapshot.set_max_hp(max_hp);
    snapshot.set_max_sp(max_sp);
    snapshot.set_hp(mercenary.hp.min(snapshot.max_hp()));
    snapshot.set_sp(mercenary.sp.min(snapshot.max_sp()));
    snapshot
}

pub fn buying_store_sign_packet(store: &crate::server::model::game_systems::BuyingStore) -> Vec<u8> {
    protocol::store_sign(store)
}

pub fn install_state(character: &mut Character, mut state: CharacterGameSystems) {
    state.pet_capture = character.game_systems.pet_capture.take();
    state.pending_pet_capture = character.game_systems.pending_pet_capture.take();
    state.completed_pet_capture = character.game_systems.completed_pet_capture.take();
    state.pet_support = character.game_systems.pet_support.take();
    state.pending_pet_loot = character.game_systems.pending_pet_loot.take();
    state.completed_pet_loot = character.game_systems.completed_pet_loot.take();
    state.trade = character.game_systems.trade.take();
    state.pet_loot_retry_at = character.game_systems.pet_loot_retry_at;
    state.pet_hatching = character.game_systems.pet_hatching;
    state.buying_slots = character.game_systems.buying_slots;
    state.buying_store = character.game_systems.buying_store.take();
    state.store_search = character.game_systems.store_search.take();
    state.rendered_companions = std::mem::take(&mut character.game_systems.rendered_companions);
    state.companion_commands = std::mem::take(&mut character.game_systems.companion_commands);
    state.last_companion_map = std::mem::take(&mut character.game_systems.last_companion_map);
    state.last_companion_tick = character.game_systems.last_companion_tick;
    state.storage_open = character.game_systems.storage_open;
    state.storage_items = std::mem::take(&mut character.game_systems.storage_items);
    state.guild_storage_open = character.game_systems.guild_storage_open;
    state.guild_storage_items = std::mem::take(&mut character.game_systems.guild_storage_items);
    state.remote_store = character.game_systems.remote_store;
    state.cart_items = std::mem::take(&mut character.game_systems.cart_items);
    state.vending_store = character.game_systems.vending_store.take();
    state.vending_slots = character.game_systems.vending_slots;
    state.opened_vending_store = character.game_systems.opened_vending_store;
    state.remote_vending_store = character.game_systems.remote_vending_store;
    state.pet_owner_dead = character.game_systems.pet_owner_dead;
    state.potion_success_counter = character.game_systems.potion_success_counter;
    state.guild_invitation = character.game_systems.guild_invitation.take();
    state.party_invitation = character.game_systems.party_invitation.take();
    state.party = character.game_systems.party.take();
    state.last_party_update = character.game_systems.last_party_update;
    state.party_position = character.game_systems.party_position.take();
    state.party_health = character.game_systems.party_health;
    if let (Some(previous), Some(current)) = (&character.game_systems.mercenary, state.mercenary.as_mut()) {
        if previous.id == current.id {
            current.last_attack_at = previous.last_attack_at;
            current.last_regen_hp_at = previous.last_regen_hp_at;
            current.last_regen_sp_at = previous.last_regen_sp_at;
        }
    }
    character.game_systems = state;
    character.refresh_script_context();
}

fn mercenary_status(mercenary: &MercenaryRecord) -> Status {
    let data = world_data()
        .mercenaries
        .iter()
        .find(|data| data.class_id == mercenary.class_id)
        .unwrap();
    Status {
        job: u32::from(data.class_id),
        base_level: u32::from(data.level),
        hp: mercenary.hp,
        sp: mercenary.sp,
        max_hp: mercenary.max_hp,
        max_sp: mercenary.max_sp,
        str: data.stats[0],
        agi: data.stats[1],
        vit: data.stats[2],
        int: data.stats[3],
        dex: data.stats[4],
        luk: data.stats[5],
        speed: data.walk_speed,
        active_statuses: mercenary.statuses.clone(),
        ..Status::default()
    }
}

fn recalculate_mercenary(mercenary: &mut MercenaryRecord) {
    let snapshot = mercenary_snapshot(mercenary);
    mercenary.max_hp = snapshot.max_hp();
    mercenary.max_sp = snapshot.max_sp();
    mercenary.hp = mercenary.hp.min(mercenary.max_hp);
    mercenary.sp = mercenary.sp.min(mercenary.max_sp);
}

fn start_mercenary_status(mercenary: &mut MercenaryRecord, request: StatusChangeRequest, now: u64) -> Result<(), String> {
    let fresh_berserk = request.kind == StatusChangeKind::Berserk && !request.has_flag(models::status_change::StatusStartFlag::Loaded);
    let refill = request.values[1] == 0;
    let mut status = mercenary_status(mercenary);
    let request = StatusEffectService::normalize_request_for_target(request, &mercenary_snapshot(mercenary), false);
    let outcome = StatusEffectService::apply_status_for_target(&mut status, request, u128::from(now), 0, false)?;
    mercenary.hp = status.hp;
    mercenary.sp = status.sp;
    mercenary.statuses = status.active_statuses;
    recalculate_mercenary(mercenary);
    if fresh_berserk && outcome.started {
        let mut status = mercenary_status(mercenary);
        StatusEffectService::finalize_berserk_entry_with_refill(&mut status, mercenary.max_hp, refill);
        mercenary.hp = status.hp;
        mercenary.sp = status.sp;
        mercenary.statuses = status.active_statuses;
    }
    Ok(())
}

fn companion_regeneration(statuses: &[models::status_change::StatusChange]) -> (bool, bool, u32, u32) {
    let mut hp = true;
    let mut sp = true;
    let mut hp_rate = 100i32;
    let mut sp_rate = 100i32;
    for status in statuses {
        for bonus in status.bonuses() {
            match bonus {
                models::enums::bonus::BonusType::DisableHpRegen => hp = false,
                models::enums::bonus::BonusType::DisableSpRegen => sp = false,
                models::enums::bonus::BonusType::NaturalHpRecoveryPercentage(rate) => hp_rate += i32::from(rate),
                models::enums::bonus::BonusType::NaturalSpRecoveryPercentage(rate) => sp_rate += i32::from(rate),
                _ => {}
            }
        }
    }
    (hp, sp, hp_rate.max(0) as u32, sp_rate.max(0) as u32)
}

fn status_kind(value: &Value) -> Result<StatusChangeKind, String> {
    match value {
        Value::Number(id) => StatusChangeKind::from_id(*id),
        Value::String(name) => StatusChangeKind::from_name(name),
        _ => None,
    }
    .ok_or_else(|| "Unknown mercenary status effect".into())
}

fn pet_lure_id(id: i32) -> Result<i32, String> {
    if let Some(pet) = world_data().pets.iter().find(|pet| i32::from(pet.class_id) == id) {
        if pet.tame_item == 0 {
            return Err("This pet has no dedicated lure".into());
        }
        Ok(pet.tame_item)
    } else {
        Ok(id)
    }
}

pub(crate) fn number(args: &[Value], index: usize) -> Result<i32, String> {
    args.get(index)
        .ok_or_else(|| format!("Missing argument {}", index + 1))?
        .number_value()
}
fn optional_number(args: &[Value], index: usize, default: i32) -> Result<i32, String> {
    args.get(index).map_or(Ok(default), Value::number_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mercenary() -> MercenaryRecord {
        let data = world_data().mercenaries.iter().find(|data| data.class_id == 6017).unwrap();
        MercenaryRecord {
            id: 1,
            class_id: data.class_id,
            name: data.name.clone(),
            level: data.level,
            hp: 1,
            sp: 1,
            max_hp: data.hp,
            max_sp: data.sp,
            expires_at: 100_000,
            contract_remaining: None,
            skill_cooldowns: BTreeMap::new(),
            cooldown_pause_at: None,
            guild: 0,
            kill_count: 0,
            statuses: Vec::new(),
            last_attack_at: 0,
            last_regen_hp_at: 0,
            last_regen_sp_at: 0,
        }
    }

    #[test]
    fn mercenary_status_increases_maximum_hp_and_expires_back_to_base() {
        let mut mercenary = mercenary();
        let base = mercenary.max_hp;
        let mut status = mercenary_status(&mercenary);
        StatusEffectService::apply_status(
            &mut status,
            StatusChangeRequest::guaranteed(StatusChangeKind::MercHpUp, 1000, 2),
            100,
            0,
        )
        .unwrap();
        mercenary.statuses = status.active_statuses;
        recalculate_mercenary(&mut mercenary);
        assert_eq!(mercenary.max_hp, (u64::from(base) * 110 / 100) as u32);
        mercenary.hp = mercenary.max_hp;
        let mut status = mercenary_status(&mercenary);
        StatusEffectService::expire_statuses(&mut status, 1100);
        mercenary.statuses = status.active_statuses;
        recalculate_mercenary(&mut mercenary);
        assert_eq!((mercenary.max_hp, mercenary.hp), (base, base));
    }

    #[test]
    fn mercenary_script_berserk_uses_final_derived_pools_and_does_not_refill_on_recalculation() {
        let mut mercenary = mercenary();
        let base = mercenary.max_hp;
        start_mercenary_status(
            &mut mercenary,
            StatusChangeRequest::guaranteed(StatusChangeKind::MercHpUp, 60000, 2),
            100,
        )
        .unwrap();
        start_mercenary_status(
            &mut mercenary,
            StatusChangeRequest::guaranteed(StatusChangeKind::Berserk, 60000, 1),
            100,
        )
        .unwrap();
        let final_hp = mercenary.max_hp;
        assert_eq!(final_hp, (u64::from(base) * 310 / 100) as u32);
        assert_eq!((mercenary.hp, mercenary.sp), (final_hp, 0));
        let berserk = mercenary
            .statuses
            .iter()
            .find(|change| change.kind == StatusChangeKind::Berserk)
            .unwrap();
        assert_eq!(berserk.values[1], (u64::from(final_hp) * 5 / 100) as i32);
        mercenary.hp -= 17;
        recalculate_mercenary(&mut mercenary);
        assert_eq!(mercenary.hp, final_hp - 17);
        let mut character = crate::tests::common::character_helper::create_character();
        character.game_systems.mercenary = Some(mercenary);
        let plan = plan_persistent_effects(
            &character,
            &[(Function::MercenaryStartStatus, vec![
                Value::String("SC_BERSERK".into()),
                Value::Number(60000),
                Value::Number(1),
            ])],
            200,
        )
        .unwrap();
        assert_eq!(plan.systems.unwrap().mercenary.unwrap().hp, final_hp);
        assert_eq!(character.game_systems.mercenary.as_ref().unwrap().hp, final_hp - 17);
    }

    #[test]
    fn companion_contract_and_skill_cooldowns_suspend_offline_without_map_reset() {
        let mut mercenary = mercenary();
        mercenary.expires_at = 10_000;
        mercenary.skill_cooldowns.insert(8201, 5000);
        mercenary.skill_cooldowns.insert(8202, 500);
        let mut systems = CharacterGameSystems {
            mercenary: Some(mercenary),
            ..CharacterGameSystems::default()
        };
        pause_companion_timers(&mut systems, 1000);
        let paused = systems.mercenary.as_ref().unwrap();
        assert_eq!(paused.contract_remaining, Some(9000));
        assert!(mercenary_contract_active(paused, 100_000));
        let stored = serde_json::to_vec(&systems).unwrap();
        let mut restored: CharacterGameSystems = serde_json::from_slice(&stored).unwrap();
        assert!(resume_companion_timers(&mut restored, 100_000));
        let mercenary = restored.mercenary.as_ref().unwrap();
        assert_eq!(mercenary.expires_at, 109_000);
        assert_eq!(mercenary.skill_cooldowns.get(&8201), Some(&104_000));
        assert!(!mercenary.skill_cooldowns.contains_key(&8202));
        assert!(!resume_companion_timers(&mut restored, 100_040));
        let mut character = crate::tests::common::character_helper::create_character();
        character.game_systems = restored;
        character.game_systems.companion_commands.clear();
        let id = mercenary_world_id(character.game_systems.mercenary.as_ref().unwrap());
        assert_eq!(companion_cooldown_until(&character, id, 8201), 104_000);
    }

    #[test]
    fn world_planning_stages_multiple_effects_without_mutating_the_live_character() {
        let character = Character::new(
            "Planner".into(),
            1,
            2,
            Status::default(),
            150,
            150,
            0,
            "prontera.gat".into(),
            0,
            vec![],
        );
        let before = character.game_systems.clone();
        let plan = plan_persistent_effects(
            &character,
            &[
                (Function::MercenaryCreate, vec![Value::Number(6017), Value::Number(60_000)]),
                (Function::MercenaryHeal, vec![Value::Number(-10), Value::Number(0)]),
                (Function::SetFont, vec![Value::Number(7)]),
            ],
            1000,
        )
        .unwrap();
        assert_eq!(character.game_systems, before);
        let staged = plan.systems.unwrap();
        assert_eq!(staged.font, 7);
        let mercenary = staged.mercenary.unwrap();
        assert_eq!(mercenary.expires_at, 61_000);
        assert_eq!(
            mercenary.hp,
            world_data().mercenaries.iter().find(|data| data.class_id == 6017).unwrap().hp - 10
        );
        assert!(plan_persistent_effects(&character, &[(Function::SetFont, vec![Value::Number(10)])], 1000).is_err());
        assert_eq!(character.game_systems, before);
    }

    #[test]
    fn embedded_world_catalog_contains_only_classic_homunculus_families() {
        let data = world_data();
        assert_eq!(data.homunculi.len(), 8);
        assert!(
            data.homunculi
                .iter()
                .all(|homunculus| (6001..=6008).contains(&homunculus.class_id) && (6009..=6016).contains(&homunculus.evolution_class))
        );
        assert!(data.pets.iter().any(|pet| pet.class_id == 1002 && pet.tame_item == 619));
        assert!(data.mercenaries.iter().any(|mercenary| mercenary.class_id == 6046));
        assert_eq!(data.guild_experience[0], 2_000_000);
        assert_eq!(pet_lure_id(1002).unwrap(), 619);
        assert_eq!(pet_lure_id(619).unwrap(), 619);
    }

    #[test]
    fn homunculus_evolution_requires_loyalty_and_applies_each_familys_growth() {
        let mut character = crate::tests::common::character_helper::create_character();
        character.game_systems.homunculus = Some(HomunculusRecord {
            id: 1,
            class_id: 6001,
            name: "Lif".into(),
            level: 10,
            intimacy: 91_000,
            hunger: 50,
            hp: 100,
            sp: 20,
            max_hp: 200,
            max_sp: 40,
            base_max_hp: 200,
            base_max_sp: 40,
            statuses: Vec::new(),
            stats: [10; 6],
            active: true,
            evolved: false,
            skill_points: 3,
            skills: BTreeMap::new(),
            experience: 0,
            renamed: false,
            next_hunger_at: 0,
            skill_cooldowns: BTreeMap::new(),
            cooldown_pause_at: None,
        });
        assert!(!ScriptWorldService::evolve_homunculus(&mut character).unwrap());
        character.game_systems.homunculus.as_mut().unwrap().intimacy = 91_100;
        assert!(ScriptWorldService::evolve_homunculus(&mut character).unwrap());
        let homunculus = character.game_systems.homunculus.as_ref().unwrap();
        assert_eq!((homunculus.class_id, homunculus.intimacy), (6009, 1000));
        assert_eq!((homunculus.hp, homunculus.sp), (100, 20));
        assert!(homunculus.max_hp > 200 && homunculus.max_sp > 40);
        assert!(!ScriptWorldService::evolve_homunculus(&mut character).unwrap());
    }
}
