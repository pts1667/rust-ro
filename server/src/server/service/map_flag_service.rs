use std::sync::mpsc::TrySendError;

use configuration::configuration::GameConfig;
use models::enums::map::MapPropertyFlags;
use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64};
use models::status_bonus::BattleFlag;
use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::map_event::{MapEvent, SetMapFlags};
use crate::server::model::map::Map;
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

pub fn normalize_map(name: &str) -> String {
    Map::name_without_ext(name).to_ascii_lowercase()
}

pub(crate) fn apply_map_skill_damage(
    flags: &MapFlags,
    damage: &mut crate::server::model::action::Damage,
    target: &models::status::StatusSnapshot,
) {
    use models::enums::actor::CombatActorKind;
    use models::enums::mob::MobClass;
    if damage.skill_damage_adjusted || damage.skill_id == 0 {
        return;
    }
    damage.skill_damage_adjusted = true;
    let target_kind = match target.combat_actor_kind() {
        CombatActorKind::Player => 0,
        CombatActorKind::Monster if *target.mob_class() == MobClass::Boss => 2,
        CombatActorKind::Monster => 1,
        _ => 3,
    };
    let caster = damage.source_kind.into();
    let rate = i64::from(flags.skill_damage_rate(damage.skill_id, caster, target_kind))
        + i64::from(super::map_skill_rules::database_damage_rate(
            flags,
            damage.skill_id,
            caster,
            target_kind,
        ));
    let amount = if damage.healing > 0 {
        -i64::from(damage.healing)
    } else {
        i64::from(damage.damage)
    };
    let adjusted = amount + amount * rate / 100;
    damage.damage = adjusted.max(0).min(i64::from(u32::MAX)) as u32;
    damage.healing = if adjusted < 0 {
        adjusted.unsigned_abs().min(u64::from(u32::MAX)) as u32
    } else {
        0
    };
}

pub(crate) fn ground_skill_duration(flags: &MapFlags, skill_id: u32, duration: i32) -> u128 {
    let duration = duration.max(0) as u128;
    flags
        .skill_duration_rate(skill_id)
        .map_or(duration, |rate| duration / 100 * u128::from(rate))
}

pub fn apply_map_combat_damage(flags: &MapFlags, config: &GameConfig, damage: u32, skill_id: u32, battle_flags: u32) -> u32 {
    if damage == 0 {
        return 0;
    }
    let (rates, immunity) = if flags.enabled(MapFlag::Battleground) {
        (&config.battleground_damage_rates, "IgnoreBgReduction")
    } else if flags.is_gvg() {
        (&config.gvg_damage_rates, "IgnoreGvgReduction")
    } else {
        return damage;
    };
    if crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
        .is_some_and(|skill| skill.flags.get(immunity).copied().unwrap_or(false))
    {
        return damage;
    }
    let mut damage = u64::from(damage);
    let adjustments = if battle_flags & BattleFlag::Skill.as_flag() != 0 {
        [
            (BattleFlag::Weapon, rates.weapon_skill),
            (BattleFlag::Magic, rates.magic_skill),
            (BattleFlag::Misc, rates.misc_skill),
        ]
    } else {
        [
            (BattleFlag::Short, rates.normal_short),
            (BattleFlag::Long, rates.normal_long),
            (BattleFlag::Skill, 100),
        ]
    };
    for (flag, rate) in adjustments {
        if battle_flags & flag.as_flag() != 0 {
            damage = (damage * u64::from(rate) / 100).min(u64::from(u32::MAX));
        }
    }
    damage.clamp(1, u64::from(u32::MAX)) as u32
}

impl ServerState {
    pub fn map_flags(&self, key: &MapInstanceKey) -> MapFlags {
        self.map_flags_for(key.map_name(), key.map_instance())
    }

    pub fn map_flags_for(&self, name: &str, instance: u8) -> MapFlags {
        let name = normalize_map(name);
        self.runtime_map_flags
            .get(&(name.clone(), instance))
            .cloned()
            .or_else(|| {
                self.get_map_instance(&name, instance)
                    .map(|instance| instance.map().flags().clone())
            })
            .or_else(|| GlobalConfigService::instance().find_map(&name).map(|map| map.flags().clone()))
            .unwrap_or_default()
    }
}

pub fn map_property_packet(flags: &MapFlags, packetver: u32) -> Vec<u8> {
    map_property_packet_in_siege(flags, packetver, false)
}

pub fn map_property_packet_in_siege(flags: &MapFlags, packetver: u32, siege_active: bool) -> Vec<u8> {
    let property: u16 = if flags.enabled(MapFlag::Pvp) {
        1
    } else if flags.is_gvg() {
        3
    } else {
        0
    };
    if packetver < 20121010 {
        return [0x0199_u16.to_le_bytes(), property.to_le_bytes()].concat();
    }
    let mut mask = 0_u64;
    if flags.enabled(MapFlag::Pvp) {
        mask |= MapPropertyFlags::IsParty.as_flag() | MapPropertyFlags::CountPk.as_flag();
    }
    if flags.is_gvg() || flags.enabled(MapFlag::Battleground) {
        mask |= MapPropertyFlags::IsGuild.as_flag() | MapPropertyFlags::IsSiege.as_flag();
    }
    if flags.enabled(MapFlag::ForceMinEffect) || flags.is_gvg() {
        mask |= MapPropertyFlags::UseSimpleEffect.as_flag();
    }
    if flags.enabled(MapFlag::NoLockOn) || flags.versus(siege_active) {
        mask |= MapPropertyFlags::IsNoLockOn.as_flag();
    }
    for (flag, value) in [
        (MapFlag::PartyLock, MapPropertyFlags::PartyLock),
        (MapFlag::Battleground, MapPropertyFlags::IsBattleground),
        (MapFlag::NoCostume, MapPropertyFlags::IsNoCostum),
    ] {
        if flags.enabled(flag) {
            mask |= value.as_flag();
        }
    }
    if !flags.enabled(MapFlag::NoUseCart) {
        mask |= MapPropertyFlags::IsUseCart.as_flag();
    }
    if !flags.enabled(MapFlag::NoSunMoonStarMiracle) {
        mask |= MapPropertyFlags::IsSummonstarMiracle.as_flag();
    }
    [
        0x099B_u16.to_le_bytes().to_vec(),
        property.to_le_bytes().to_vec(),
        (mask as u32).to_le_bytes().to_vec(),
    ]
    .concat()
}

/// Shows the PvP cursor and disables lock-on for duelists, as rathena does for `duel_group`.
fn apply_duel_property(packet: &mut [u8]) {
    packet[2..4].copy_from_slice(&1_u16.to_le_bytes());
    if packet.len() >= 8 {
        let mask = u32::from_le_bytes(packet[4..8].try_into().unwrap())
            | MapPropertyFlags::IsParty.as_flag() as u32
            | MapPropertyFlags::IsNoLockOn.as_flag() as u32;
        packet[4..8].copy_from_slice(&mask.to_le_bytes());
    }
}

pub fn is_map_flag_call(function: Function) -> bool {
    matches!(
        function,
        Function::GetMapFlag
            | Function::SetMapFlag
            | Function::SetMapFlagNoSave
            | Function::RemoveMapFlag
            | Function::PvpOn
            | Function::PvpOff
            | Function::GvgOn
            | Function::GvgOff
    )
}

impl Server {
    pub(crate) fn drain_map_notifications(&self, state: &mut ServerState) {
        for _ in 0..64 {
            let Some(notification) = state.pending_map_notifications.pop_front() else {
                break;
            };
            match self.server_service().notification_sender().try_send(notification) {
                Ok(()) => {}
                Err(TrySendError::Full(notification)) => {
                    state.pending_map_notifications.push_front(notification);
                    break;
                }
                Err(TrySendError::Disconnected(_)) => {
                    state.pending_map_notifications.clear();
                    break;
                }
            }
        }
    }

    pub(crate) fn set_siege_active(&self, state: &mut ServerState, active: bool) -> bool {
        if state.siege_active == active {
            return false;
        }
        state.siege_active = active;
        let castle_characters: Vec<u32> = state
            .characters()
            .values()
            .filter(|character| state.map_flags(&character.map_instance_key).enabled(MapFlag::GvgCastle))
            .map(|character| character.char_id)
            .collect();
        for char_id in castle_characters {
            if !active {
                if let Some(character) = state.characters_mut().get_mut(&char_id) {
                    character.clear_attack();
                }
            }
            self.notify_map_property(state, char_id);
        }
        self.broadcast_npc_event(state, if active { "OnAgitStart" } else { "OnAgitEnd" });
        let lifecycle = if active {
            crate::server::model::events::game_event::CastleLifecycle::AgitStart
        } else {
            crate::server::model::events::game_event::CastleLifecycle::AgitEnd
        };
        self.add_to_next_tick(crate::server::model::events::game_event::GameEvent::CastleLifecycle(lifecycle));
        true
    }

    pub(crate) fn notify_map_property(&self, state: &mut ServerState, char_id: u32) {
        if let Some(character) = state.characters().get(&char_id) {
            let mut data = map_property_packet_in_siege(
                &state.map_flags(&character.map_instance_key),
                self.packetver(),
                state.siege_active,
            );
            if self.duels().duel_of(char_id).is_some() {
                apply_duel_property(&mut data);
            }
            state
                .pending_map_notifications
                .push_back(Notification::Char(CharNotification::new(char_id, data)));
        }
        self.drain_map_notifications(state);
    }

    pub(crate) fn install_map_flags(&self, state: &mut ServerState, key: &MapInstanceKey, flags: MapFlags) -> Result<(), String> {
        let name = normalize_map(key.map_name());
        let instance = state.get_map_instance(&name, key.map_instance());
        if instance.is_none() && GlobalConfigService::instance().find_map(&name).is_none() {
            return Err("Map is unavailable".into());
        }
        let previous = state.map_flags(key);
        state.runtime_map_flags.insert((name, key.map_instance()), flags.clone());
        if let Some(instance) = instance {
            instance.add_to_next_tick(MapEvent::SetMapFlags(SetMapFlags { flags: flags.clone() }));
        }
        let packet = map_property_packet_in_siege(&flags, self.packetver(), state.siege_active);
        let stop_attacks = previous.versus(state.siege_active) && !flags.versus(state.siege_active);
        let mut notifications = Vec::new();
        for character in state
            .characters_mut()
            .values_mut()
            .filter(|character| character.map_instance_key == *key)
        {
            if stop_attacks {
                character.clear_attack();
            }
            if character.loaded_from_client_side {
                notifications.push(Notification::Char(CharNotification::new(character.char_id, packet.clone())));
            }
        }
        state.pending_map_notifications.extend(notifications);
        self.drain_map_notifications(state);
        Ok(())
    }

    pub(crate) fn map_flag_call(&self, state: &mut ServerState, char_id: u32, function: Function, arguments: &[Value]) -> Reply {
        let source = state.characters().get(&char_id).map(|character| character.map_instance_key.clone());
        self.map_flag_call_from(state, source.as_ref(), function, arguments)
    }

    pub(crate) fn map_flag_call_from(
        &self,
        state: &mut ServerState,
        source: Option<&MapInstanceKey>,
        function: Function,
        arguments: &[Value],
    ) -> Reply {
        let map = normalize_map(arguments.first().ok_or("Map name is required")?.string_value()?);
        let instance = source
            .filter(|source| map == normalize_map(source.map_name()))
            .map_or(0, |source| source.map_instance());
        let key = MapInstanceKey::new(map.clone(), instance);
        if state.get_map_instance(&map, instance).is_none() && GlobalConfigService::instance().find_map(&map).is_none() {
            return Err("Map is unavailable".into());
        }
        if function == Function::SetMapFlagNoSave {
            if arguments.len() != 4 {
                return Err("NoSave requires a map, alternate map, x, and y".into());
            }
            let alternate = arguments[1].string_value()?;
            let alternate = if alternate.eq_ignore_ascii_case("SavePoint") {
                "SavePoint".to_string()
            } else {
                normalize_map(alternate)
            };
            let x = arguments[2].number_value()?;
            let y = arguments[3].number_value()?;
            let coordinates = if (x, y) == (-1, -1) {
                crate::server::model::map::RANDOM_CELL
            } else {
                (
                    u16::try_from(x).map_err(|_| "Invalid NoSave x coordinate")?,
                    u16::try_from(y).map_err(|_| "Invalid NoSave y coordinate")?,
                )
            };
            if alternate != "SavePoint" {
                let destination = GlobalConfigService::instance()
                    .find_map(&alternate)
                    .ok_or("NoSave destination is unavailable")?;
                if coordinates != crate::server::model::map::RANDOM_CELL
                    && (coordinates.0 >= destination.x_size() || coordinates.1 >= destination.y_size())
                {
                    return Err("NoSave destination is outside the map".into());
                }
            }
            let mut flags = state.map_flags(&key);
            flags.set(MapFlag::NoSave, true, &[])?;
            flags.save = Some((alternate, coordinates.0, coordinates.1));
            self.install_map_flags(state, &key, flags)?;
            return Ok(0.into());
        }
        let (flag, enabled, offset) = match function {
            Function::PvpOn => (MapFlag::Pvp, true, 1),
            Function::PvpOff => (MapFlag::Pvp, false, 1),
            Function::GvgOn => (MapFlag::Gvg, true, 1),
            Function::GvgOff => (MapFlag::Gvg, false, 1),
            _ => (
                MapFlag::from_id(arguments.get(1).ok_or("Map flag is required")?.number_value()?)?,
                function == Function::SetMapFlag,
                2,
            ),
        };
        let args: Vec<_> = arguments[offset..].iter().map(Value::number_value).collect::<Result<_, _>>()?;
        let mut flags = state.map_flags(&key);
        if function == Function::GetMapFlag {
            return Ok(flags.get(flag, args.first().copied()).into());
        }
        if flag == MapFlag::SkillDuration && enabled {
            let id = args.first().copied().ok_or("Skill duration requires a skill")?;
            if crate::server::script::skill::metadata::SkillMetadata::find(id as u32).is_none() {
                return Err("Unknown skill duration skill".into());
            }
        }
        flags.set(flag, enabled, &args)?;
        self.install_map_flags(state, &key, flags)?;
        Ok(0.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet_mask(flags: &MapFlags) -> u32 {
        u32::from_le_bytes(map_property_packet(flags, 20131223)[4..8].try_into().unwrap())
    }

    #[test]
    fn client_map_property_flags_enable_cart_and_miracle_on_an_ordinary_map() {
        let mut flags = MapFlags::default();
        assert_eq!(
            packet_mask(&flags),
            (MapPropertyFlags::IsUseCart.as_flag() | MapPropertyFlags::IsSummonstarMiracle.as_flag()) as u32
        );
        flags.set(MapFlag::NoUseCart, true, &[]).unwrap();
        flags.set(MapFlag::NoSunMoonStarMiracle, true, &[]).unwrap();
        assert_eq!(packet_mask(&flags), 0);
    }

    #[test]
    fn client_combat_properties_include_siege_effects_and_shift_targeting() {
        let mut flags = MapFlags::default();
        flags.set(MapFlag::Gvg, true, &[]).unwrap();
        let mask = packet_mask(&flags) as u64;
        for bit in [
            MapPropertyFlags::IsGuild,
            MapPropertyFlags::IsSiege,
            MapPropertyFlags::UseSimpleEffect,
            MapPropertyFlags::IsNoLockOn,
        ] {
            assert_ne!(mask & bit.as_flag(), 0);
        }
        assert_eq!(
            map_property_packet(&flags, 20100101),
            [0x0199_u16.to_le_bytes(), 3_u16.to_le_bytes()].concat()
        );
        flags.set(MapFlag::Pvp, true, &[]).unwrap();
        let mask = packet_mask(&flags) as u64;
        assert_ne!(mask & MapPropertyFlags::IsParty.as_flag(), 0);
        assert_ne!(mask & MapPropertyFlags::CountPk.as_flag(), 0);
        assert_eq!(mask & MapPropertyFlags::IsGuild.as_flag(), 0);
    }

    #[test]
    fn imported_pre_renewal_map_rules_keep_reset_permissions_castle_rules_and_trap_duration() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/map_flags.json");
        let flags = crate::server::boot::script_loader::ScriptLoader::load_map_flags(path.to_str().unwrap()).unwrap();
        let town = &flags["prontera"];
        assert!(town.enabled(MapFlag::Town));
        assert!(town.enabled(MapFlag::Reset));
        assert!(town.enabled(MapFlag::NoBranch));
        let castle = &flags["aldeg_cas01"];
        assert!(castle.enabled(MapFlag::GvgCastle));
        assert!(castle.enabled(MapFlag::NoTeleport));
        assert_eq!(castle.get(MapFlag::InvincibleTime, None), 10000);
        assert_eq!(
            flags["guild_vs1"].get(
                MapFlag::SkillDuration,
                Some(models::enums::skill_enums::SkillEnum::HtSkidtrap.id() as i32)
            ),
            400
        );
        assert!(flags["pvp_y_1-1"].enabled(MapFlag::Pvp));
        assert_eq!(flags["pvp_n_1-1"].nightmare_drops, vec![(-1, 2, 300)]);
    }
}
