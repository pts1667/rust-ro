use models::enums::action::ActionType;
use models::enums::element::Element;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use packets::packets::{Packet, PacketZcNotifyAct};

use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MobAttackCharacter;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::battle_service::{BattleService, NormalAttackRoll};
use crate::server::service::combat_trigger_service::MagicReflectionKind;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_world_service::companion_status_snapshot;
use crate::server::service::status_service::StatusService;
use crate::server::service::visibility_service::{StealthState, TargetingMode, VisibilityObserver, can_target};
use crate::server::state::server::ServerState;

#[derive(Debug, Clone, PartialEq)]
pub struct MobAttackRequest {
    pub attack: MobAttackCharacter,
    pub source_status: StatusSnapshot,
    pub source_key: MapInstanceKey,
    pub min_atk: u16,
    pub max_atk: u16,
    pub battle_flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagicAttackContext {
    pub matk: u16,
    pub modifier: f32,
    pub element: Element,
    pub hits: u16,
    pub skill_id: u32,
    pub grand_cross: Option<GrandCrossAttackContext>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrandCrossAttackContext {
    pub raw_atk: u32,
    pub refine_bonus: u16,
    pub self_target: bool,
}

impl MagicAttackContext {
    pub fn new(matk: u16, modifier: f32, element: Element, hits: u16, skill_id: u32) -> Self {
        Self {
            matk,
            modifier,
            element,
            hits,
            skill_id,
            grand_cross: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MagicReflectionRequest {
    pub damage: Damage,
    pub reflector_id: u32,
    pub reflector_credit_id: u32,
    pub kind: MagicReflectionKind,
    pub map_key: MapInstanceKey,
}

pub fn reflect_magic(server: &Server, state: &ServerState, request: MagicReflectionRequest, tick: u128) -> Result<(), String> {
    let source_id = request.damage.attacker_id;
    let player = state
        .get_character(source_id)
        .filter(|character| character.map_instance_key == request.map_key)
        .map(|character| StatusService::instance().to_snapshot(&character.status));
    let companion = state
        .characters()
        .values()
        .filter(|character| character.map_instance_key == request.map_key)
        .find_map(|character| companion_status_snapshot(character, source_id));
    let map = state
        .get_map_instance(request.map_key.map_name(), request.map_key.map_instance())
        .ok_or("Reflection map is unavailable")?;
    let is_player_actor = player.is_some() || companion.is_some();
    let caster = player
        .or(companion)
        .or_else(|| map.state().get_mob(source_id).map(|mob| mob.status.clone()));
    let Some(caster) = caster.filter(|status| status.hp() > 0) else {
        return Ok(());
    };
    let reflected = server.battle_service().reflected_magic_damage_signed(
        request.kind,
        request.damage.damage,
        &caster,
        request.damage.magic_context,
    )?;
    let damage = Damage {
        target_id: source_id,
        attacker_id: request.reflector_id,
        damage: reflected.max(0) as u32,
        healing: if reflected < 0 { reflected.unsigned_abs() } else { 0 },
        right_hand_damage: None,
        attacked_at: tick,
        damage_motion: 0,
        battle_flags: request.damage.battle_flags,
        skill_id: request.damage.skill_id,
        proc_depth: request.damage.proc_depth.saturating_add(1),
        credit_id: request.reflector_credit_id,
        defenses_applied: true,
        magic_context: request.damage.magic_context,
        skill_level: request.damage.skill_level,
        landed: false,
    };
    if is_player_actor {
        server.add_to_next_tick(GameEvent::CharacterDamage(damage));
    } else {
        map.add_to_next_tick(crate::server::model::events::map_event::MapEvent::MobDamage(damage));
    }
    Ok(())
}

pub fn handle(server: &Server, state: &mut ServerState, request: MobAttackRequest, tick: u128) {
    let attack = request.attack;
    let target = state
        .get_character(attack.target_char_id)
        .filter(|character| character.map_instance_key == request.source_key)
        .map(|character| {
            (
                StatusService::instance().to_snapshot(&character.status),
                StealthState::from_status_options(&character.status, character.options),
                true,
            )
        })
        .or_else(|| {
            state
                .characters()
                .values()
                .filter(|character| character.map_instance_key == request.source_key)
                .find_map(|character| {
                    companion_status_snapshot(character, attack.target_char_id).map(|status| {
                        let stealth = StealthState::from_snapshot(&status);
                        (status, stealth, false)
                    })
                })
        });
    let Some((target, stealth, player_target)) = target.filter(|(target, ..)| target.hp() > 0) else {
        return;
    };
    let mode = GlobalConfigService::instance()
        .get_mob_safe(request.source_status.job() as i32)
        .map_or(0, |mob| mob.mode as u32);
    if !can_target(
        VisibilityObserver::for_mob(mode, *request.source_status.race()),
        stealth,
        TargetingMode::Direct,
    ) {
        return;
    }
    let mut rng = fastrand::Rng::new();
    let ranged = request.battle_flags & BattleFlag::Long.as_flag() != 0;
    let outcome = BattleService::normal_attack_roll(&request.source_status, &target, ranged, &mut rng);
    let damage = if matches!(outcome, NormalAttackRoll::Miss | NormalAttackRoll::LuckyDodge) {
        0
    } else {
        let raw = if outcome == NormalAttackRoll::Critical
            || request
                .source_status
                .has_status_change(models::status_change::StatusChangeKind::MaximizePower)
        {
            request.max_atk.max(request.min_atk) as u32
        } else {
            rng.u32(request.min_atk as u32..=request.max_atk.max(request.min_atk) as u32)
        };
        let raw = (raw as f64 * f64::from(BattleService::weapon_skill_ratio(&request.source_status, 1.0, 0)))
            .clamp(0.0, u32::MAX as f64)
            .floor() as u32;
        server
            .battle_service()
            .actor_physical_damage_signed(
                raw,
                &request.source_status,
                &target,
                player_target,
                outcome == NormalAttackRoll::Critical,
                &server.battle_service().attack_element(&request.source_status, None),
                request.battle_flags,
            )
            .saturating_mul(if outcome == NormalAttackRoll::DoubleAttack { 2 } else { 1 })
    };
    let mut packet = PacketZcNotifyAct::new(GlobalConfigService::instance().packetver());
    packet.set_gid(attack.mob_id);
    packet.set_target_gid(attack.target_char_id);
    packet.set_action(
        match outcome {
            NormalAttackRoll::LuckyDodge => ActionType::AttackLucky,
            NormalAttackRoll::Critical => ActionType::AttackCritical,
            NormalAttackRoll::DoubleAttack => ActionType::AttackMultiple,
            _ => ActionType::Attack,
        }
        .value() as u8,
    );
    packet.set_attack_mt((attack.attack_motion / 2) as i32);
    packet.set_attacked_mt(480);
    packet.set_damage(damage.clamp(i16::MIN as i32, i16::MAX as i32) as i16);
    packet.set_count(if outcome == NormalAttackRoll::DoubleAttack { 2 } else { 1 });
    packet.fill_raw();
    let _ = server
        .server_service()
        .notification_sender()
        .send(Notification::Area(AreaNotification::new(
            request.source_key.map_name().clone(),
            request.source_key.map_instance(),
            AreaNotificationRangeType::Fov {
                x: attack.mob_x,
                y: attack.mob_y,
                exclude_id: None,
            },
            packet.raw,
        )));
    let delay = attack.attack_motion as u128 / 2;
    server.add_to_delayed_tick(
        GameEvent::CharacterDamage(Damage {
            target_id: attack.target_char_id,
            attacker_id: attack.mob_id,
            damage: damage.max(0) as u32,
            healing: if damage < 0 { damage.unsigned_abs() } else { 0 },
            right_hand_damage: None,
            attacked_at: tick.saturating_add(delay),
            damage_motion: 480,
            battle_flags: request.battle_flags,
            skill_id: 0,
            skill_level: 0,
            landed: !matches!(outcome, NormalAttackRoll::Miss | NormalAttackRoll::LuckyDodge),
            proc_depth: 0,
            credit_id: attack.mob_id,
            defenses_applied: true,
            magic_context: None,
        }),
        delay,
    );
}
