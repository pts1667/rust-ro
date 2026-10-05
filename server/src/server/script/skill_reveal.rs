use models::enums::EnumWithMaskValueU32;
use models::enums::element::Element;
use models::enums::skill_enums::SkillEnum;
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::StatusChangeKind;

use super::ScriptSkillService;
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{GameEvent, CharacterDamage};
use crate::server::model::events::map_event::{MapEvent, MobDamage, MobEndStatus};
use crate::server::service::map_combat_service::MagicAttackContext;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptRevealActor {
    pub actor_id: u32,
    pub credit_id: u32,
    pub map: String,
    pub instance: u8,
    pub x: u16,
    pub y: u16,
    pub status: StatusSnapshot,
    pub kind: StatusChangeKind,
    pub damage_mobs: bool,
    pub damage_players: bool,
}

impl ScriptSkillService {
    pub(super) fn reveal_actor(&self, character: &Character, kind: StatusChangeKind) -> ScriptRevealActor {
        ScriptRevealActor {
            actor_id: character.char_id,
            credit_id: character.char_id,
            map: character.current_map_name().clone(),
            instance: character.current_map_instance(),
            x: character.x,
            y: character.y,
            status: StatusService::instance().to_snapshot(&character.status),
            kind,
            damage_mobs: true,
            damage_players: false,
        }
    }

    pub fn tick_revealing_statuses(&self, server: &Server, state: &ServerState, character: &mut Character, tick: u128) {
        if character.status.hp == 0 {
            return;
        }
        for kind in [StatusChangeKind::Sight, StatusChangeKind::Ruwach] {
            let Some(change) = character
                .status
                .active_statuses
                .iter_mut()
                .find(|change| change.kind == kind && !change.expired(tick) && tick >= change.next_periodic_at)
            else {
                continue;
            };
            change.next_periodic_at = tick + 20;
            change.values[3] = change.values[3].saturating_add(20);
            if let Err(error) = self.reveal_from_actor(server, state, &self.reveal_actor(character, kind), tick) {
                warn!("Unable to reveal nearby actors: {}", error);
            }
        }
    }

    pub fn reveal_from_actor(&self, server: &Server, state: &ServerState, source: &ScriptRevealActor, tick: u128) -> Result<(), String> {
        let skill_id = match source.kind {
            StatusChangeKind::Sight => SkillEnum::MgSight.id(),
            StatusChangeKind::Ruwach => SkillEnum::AlRuwach.id(),
            StatusChangeKind::Concentrate => SkillEnum::AcConcentration.id(),
            _ => return Err("Status does not reveal hidden actors".into()),
        };
        let range = source
            .status
            .status_change(source.kind)
            .filter(|_| source.kind != StatusChangeKind::Concentrate)
            .map(|change| change.values[2])
            .or_else(|| SkillMetadata::find(skill_id).and_then(|skill| skill.splash(1)))
            .unwrap_or(0)
            .max(0) as u16;
        let instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Reveal map instance is unavailable")?;
        let instance_state = instance.state();
        for mob in instance_state
            .mobs()
            .values()
            .filter(|mob| mob.hp() > 0 && mob.x.abs_diff(source.x).max(mob.y.abs_diff(source.y)) <= range)
        {
            let hidden = [StatusChangeKind::Hiding, StatusChangeKind::Cloaking]
                .iter()
                .filter(|kind| mob.status.has_status_change(**kind))
                .copied()
                .collect::<Vec<_>>();
            if hidden.is_empty() {
                continue;
            }
            for kind in hidden {
                instance.add_to_next_tick(MapEvent::MobEndStatus(MobEndStatus {
                    mob_id: mob.id,
                    kind: Some(kind),
                }));
            }
            if source.kind == StatusChangeKind::Ruwach && source.damage_mobs && mob.summon_owner != Some(source.credit_id) {
                instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: Self::ruwach_damage(
                    server,
                    source,
                    &mob.status,
                    mob.id,
                    tick,
                ) }));
            }
        }
        for character in state.characters().values().filter(|character| {
            character.char_id != source.actor_id
                && character.status.hp > 0
                && character.current_map_name() == &source.map
                && character.current_map_instance() == source.instance
                && character.x.abs_diff(source.x).max(character.y.abs_diff(source.y)) <= range
        }) {
            let hidden = [StatusChangeKind::Hiding, StatusChangeKind::Cloaking]
                .iter()
                .filter(|kind| character.status.has_status_change(**kind))
                .copied()
                .collect::<Vec<_>>();
            if hidden.is_empty() {
                continue;
            }
            for kind in hidden {
                server.add_to_next_tick(GameEvent::CharacterEndStatus(
                    crate::server::model::events::game_event::CharacterEndStatus {
                        char_id: character.char_id,
                        kind: Some(kind),
                    },
                ));
            }
            if source.kind == StatusChangeKind::Ruwach && source.damage_players && character.char_id != source.credit_id {
                let target = StatusService::instance().to_snapshot(&character.status);
                server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage: Self::ruwach_damage(
                    server,
                    source,
                    &target,
                    character.char_id,
                    tick,
                ) }));
            }
        }
        Ok(())
    }

    fn ruwach_damage(server: &Server, source: &ScriptRevealActor, target: &StatusSnapshot, target_id: u32, tick: u128) -> Damage {
        let lower = source.status.matk_min().min(source.status.matk_max());
        let upper = source.status.matk_max().max(lower);
        let context = MagicAttackContext::new(fastrand::u16(lower..=upper), 1.45, Element::Ghost, 1, SkillEnum::AlRuwach.id());
        let amount = server.battle_service().magic_damage_from_context(&source.status, target, context);
        let mut damage = Damage {
            notification: None,
            source_kind: *source.status.combat_actor_kind(),
            skill_damage_adjusted: false,
            healing: 0,
            right_hand_damage: None,
            target_id,
            attacker_id: source.actor_id,
            damage: 0,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
            skill_id: SkillEnum::AlRuwach.id(),
            skill_level: 1,
            landed: true,
            proc_depth: 0,
            credit_id: source.credit_id,
            defenses_applied: true,
            magic_context: Some(context),
        };
        damage.set_signed_damage(amount);
        damage.with_skill_notification(&source.map, source.instance, source.x, source.y, tick, 1, 0)
    }
}
