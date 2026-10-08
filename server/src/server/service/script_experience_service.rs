use std::collections::BTreeMap;

use crate::server::model::events::game_event::CharacterKillMonster;
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::state::server::ServerState;

fn attacker_bonus_options() -> (u64, usize) {
    let battle = &crate::server::service::global_config_service::GlobalConfigService::instance().config().battle;
    (battle.get("exp_bonus_attacker") as u64, battle.get("exp_bonus_max_attacker") as usize)
}

#[cfg(test)]
pub fn monster_experience_awards(state: &ServerState, kill: &CharacterKillMonster, party_bonus: u16) -> BTreeMap<u32, (u32, u32)> {
    monster_experience_awards_with_pets(state, kill, party_bonus, false, 100)
}

pub fn monster_experience_awards_with_pets(
    state: &ServerState,
    kill: &CharacterKillMonster,
    party_bonus: u16,
    pet_exp_to_master: bool,
    pet_exp_rate: u16,
) -> BTreeMap<u32, (u32, u32)> {
    let flags = state.map_flags(&kill.map_instance_key);
    let owners: std::collections::BTreeSet<_> = if kill.contributions.is_empty() {
        [kill.char_id].into_iter().collect()
    } else {
        kill.contributions
            .iter()
            .filter(|entry| entry.damage > 0)
            .map(|entry| entry.owner_id)
            .collect()
    };
    let mut groups: BTreeMap<Vec<u32>, (u32, u32, u32)> = BTreeMap::new();
    for owner_id in owners {
        let Some(owner) = state
            .characters()
            .get(&owner_id)
            .filter(|owner| !owner.is_dead() && owner.status.hp > 0 && owner.map_instance_key == kill.map_instance_key)
        else {
            continue;
        };
        let (base, job) =
            if kill.contributions.is_empty() && !pet_exp_to_master && (1_000_000_000..1_100_000_000).contains(&kill.attacker_id) {
                (0, 0)
            } else if kill.contributions.is_empty() {
                let actor_rate = if (1_000_000_000..1_100_000_000).contains(&kill.attacker_id) {
                    pet_exp_rate
                } else {
                    100
                };
                (
                    map_experience(
                        kill.mob_base_exp,
                        experience_rate(&flags, MapFlag::NoBaseExp, MapFlag::Bexp),
                        actor_rate,
                    ),
                    map_experience(
                        kill.mob_job_exp,
                        experience_rate(&flags, MapFlag::NoJobExp, MapFlag::Jexp),
                        actor_rate,
                    ),
                )
            } else {
                owner_experience_share_with_map_and_pets(kill, owner_id, &flags, pet_exp_to_master, pet_exp_rate)
            };
        let blessing = 100 + crate::server::script::skill::star_gladiator::bless_percent(&owner.status, kill.mob_id as u32);
        let (base, job) = (base.saturating_mul(blessing) / 100, job.saturating_mul(blessing) / 100);
        if base == 0 && job == 0 {
            continue;
        }
        let mut eligible: Vec<_> = super::script_world_service::party_experience_awards(
            state,
            owner,
            0,
            0,
            kill.map_instance_key.map_name(),
            kill.map_instance_key.map_instance(),
        )
        .into_iter()
        .map(|(id, ..)| id)
        .collect();
        if eligible.is_empty() {
            continue;
        }
        eligible.sort_unstable();
        eligible.dedup();
        let group = groups.entry(eligible).or_insert((owner_id, 0, 0));
        group.1 = group.1.saturating_add(base);
        group.2 = group.2.saturating_add(job);
    }
    let mut awards: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
    for (_, (owner_id, base, job)) in groups {
        let owner = &state.characters()[&owner_id];
        for (id, base, job) in super::script_world_service::party_experience_awards_with_bonus(
            state,
            owner,
            base,
            job,
            kill.map_instance_key.map_name(),
            kill.map_instance_key.map_instance(),
            party_bonus,
        ) {
            let award = awards.entry(id).or_default();
            award.0 = award.0.saturating_add(base);
            award.1 = award.1.saturating_add(job);
        }
    }
    awards
}

#[cfg(test)]
pub fn actor_experience_share(kill: &CharacterKillMonster, actor_id: u32) -> (u32, u32) {
    actor_experience_share_with_map(kill, actor_id, &MapFlags::default())
}

fn experience_rate(flags: &MapFlags, no_exp: MapFlag, rate: MapFlag) -> Option<u32> {
    (!flags.enabled(no_exp)).then(|| flags.get(rate, None).max(0) as u32)
}

fn map_experience(experience: u32, rate: Option<u32>, actor_rate: u16) -> u32 {
    if experience == 0 {
        return 0;
    }
    match rate {
        Some(rate) => (u128::from(experience) * u128::from(rate) * u128::from(actor_rate) / 10000).clamp(1, u128::from(u32::MAX)) as u32,
        _ => 0,
    }
}

pub fn actor_experience_share_with_map(kill: &CharacterKillMonster, actor_id: u32, flags: &MapFlags) -> (u32, u32) {
    actor_experience_share_scaled(kill, actor_id, flags, 100)
}

fn actor_experience_share_scaled(kill: &CharacterKillMonster, actor_id: u32, flags: &MapFlags, actor_rate: u16) -> (u32, u32) {
    let total: u64 = kill.contributions.iter().map(|entry| u64::from(entry.damage)).sum();
    let damage: u64 = kill
        .contributions
        .iter()
        .filter(|entry| entry.actor_id == actor_id)
        .map(|entry| u64::from(entry.damage))
        .sum();
    if total == 0 || damage == 0 {
        return (0, 0);
    }
    let attackers = kill
        .contributions
        .iter()
        .filter(|entry| entry.damage > 0)
        .count()
        .min(attacker_bonus_options().1);
    let bonus = 100 + attackers.saturating_sub(1) as u64 * attacker_bonus_options().0;
    let share = |experience: u32, rate: Option<u32>| {
        if experience == 0 || rate.is_none() {
            0
        } else {
            (u128::from(experience)
                * u128::from(damage.min(total))
                * u128::from(bonus)
                * u128::from(rate.unwrap())
                * u128::from(actor_rate)
                / (u128::from(total) * 1000000))
                .clamp(1, u128::from(u32::MAX)) as u32
        }
    };
    let homunculus = (1_100_000_000..1_200_000_000).contains(&actor_id);
    (
        share(kill.mob_base_exp, experience_rate(flags, MapFlag::NoBaseExp, MapFlag::Bexp)),
        if homunculus {
            0
        } else {
            share(kill.mob_job_exp, experience_rate(flags, MapFlag::NoJobExp, MapFlag::Jexp))
        },
    )
}

#[cfg(test)]
pub fn owner_experience_share(kill: &CharacterKillMonster, owner_id: u32) -> (u32, u32) {
    owner_experience_share_with_map(kill, owner_id, &MapFlags::default())
}

#[cfg(test)]
pub fn owner_experience_share_with_map(kill: &CharacterKillMonster, owner_id: u32, flags: &MapFlags) -> (u32, u32) {
    owner_experience_share_with_map_and_pets(kill, owner_id, flags, true, 100)
}

pub fn owner_experience_share_with_map_and_pets(
    kill: &CharacterKillMonster,
    owner_id: u32,
    flags: &MapFlags,
    pet_exp_to_master: bool,
    pet_exp_rate: u16,
) -> (u32, u32) {
    kill.contributions
        .iter()
        .filter(|entry| entry.owner_id == owner_id && (pet_exp_to_master || !(1_000_000_000..1_100_000_000).contains(&entry.actor_id)))
        .fold((0_u32, 0_u32), |(base, job), entry| {
            let pet = (1_000_000_000..1_100_000_000).contains(&entry.actor_id);
            let (actor_base, actor_job) = actor_experience_share_scaled(kill, entry.actor_id, flags, if pet { pet_exp_rate } else { 100 });
            (base.saturating_add(actor_base), job.saturating_add(actor_job))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::model::action::DamageContribution;
    use crate::server::model::map_instance::MapInstanceKey;

    #[test]
    fn classic_homunculus_damage_keeps_owner_base_experience_and_discards_its_job_share() {
        let kill = CharacterKillMonster {
            attacker_id: 1,
            char_id: 1,
            mob_id: 1002,
            mob_x: 1,
            mob_y: 1,
            map_instance_key: MapInstanceKey::new("prontera".into(), 0),
            mob_base_exp: 1000,
            mob_job_exp: 500,
            mob_max_hp: 100,
            contributions: vec![
                DamageContribution {
                    actor_id: 1,
                    owner_id: 1,
                    damage: 60,
                },
                DamageContribution {
                    actor_id: 1_100_000_001,
                    owner_id: 1,
                    damage: 40,
                },
            ],
        };
        assert_eq!(actor_experience_share(&kill, 1), (750, 375));
        assert_eq!(actor_experience_share(&kill, 1_100_000_001), (500, 0));
        assert_eq!(owner_experience_share(&kill, 1), (1250, 375));
        assert_eq!(actor_experience_share(&kill, 3), (0, 0));
    }
}
