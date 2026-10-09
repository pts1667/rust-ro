use models::enums::skill_enums::SkillEnum;

use super::metadata::SkillMetadata;
use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::model::events::map_event::{MapEvent, ScriptSpawn};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const MARINE_SPHERE: i32 = 1142;
const FLORA_CLASSES: [i32; 5] = [1589, 1579, 1575, 1555, 1590];
const SPHERE_AI: u16 = 2;
const FLORA_AI: u16 = 3;

struct Summon {
    class: i32,
    ai: u16,
    limit: Option<usize>,
    max_hp: u32,
}

impl Summon {
    fn for_skill(skill_id: u32, level: u8, base_level: u32) -> Option<Self> {
        let level_index = usize::from(level.clamp(1, 5)) - 1;
        let level = u32::from(level);
        if skill_id == SkillEnum::AmSpheremine.id() {
            Some(Self { class: MARINE_SPHERE, ai: SPHERE_AI, limit: None, max_hp: 2000 + 400 * level })
        } else if skill_id == SkillEnum::AmCannibalize.id() {
            Some(Self {
                class: FLORA_CLASSES[level_index],
                ai: FLORA_AI,
                limit: Some(6usize.saturating_sub(level as usize)),
                max_hp: 1500 + 200 * level + 10 * base_level,
            })
        } else {
            None
        }
    }
}

impl ScriptSkillService {
    fn owned_summons(state: &ServerState, character: &Character, class: i32) -> usize {
        state.get_map_instance_from_character(character).map_or(0, |instance| {
            instance
                .state()
                .mobs()
                .values()
                .filter(|mob| mob.is_present() && i32::from(mob.mob_id) == class && mob.summon_owner == Some(character.char_id))
                .count()
        })
    }

    pub(super) fn validate_summon_limit(&self, state: &ServerState, character: &Character, skill_id: u32, level: u8) -> Result<(), String> {
        let summon = Summon::for_skill(skill_id, level, character.get_base_level()).ok_or("Skill does not summon a creature")?;
        match summon.limit {
            Some(limit) if Self::owned_summons(state, character, summon.class) >= limit => Err("Too many creatures are already summoned".into()),
            _ => Ok(()),
        }
    }

    pub(super) fn summon_alchemist_creature(
        &self,
        state: &ServerState,
        character: &Character,
        effect: &ScriptSkillEffect,
        x: u16,
        y: u16,
    ) -> Result<(), String> {
        self.validate_summon_limit(state, character, effect.skill_id, effect.level)?;
        let summon = Summon::for_skill(effect.skill_id, effect.level, character.get_base_level()).ok_or("Skill does not summon a creature")?;
        let lifetime = SkillMetadata::find(effect.skill_id)
            .and_then(|metadata| metadata.duration(effect.level, false))
            .filter(|duration| *duration > 0)
            .ok_or("Summon skill has no duration")?;
        let instance = state.get_map_instance_from_character(character).ok_or("Map instance is unavailable")?;
        instance.add_to_next_tick(MapEvent::ScriptSpawn(ScriptSpawn {
            mob_id: summon.class,
            x: i32::from(x),
            y: i32::from(y),
            name: character.name.clone(),
            amount: 1,
            event: String::new(),
            event_npc: None,
            size: Some(0),
            ai: Some(summon.ai),
            owner_id: character.char_id,
            guardian: None,
            bg_id: 0,
            max_hp: Some(summon.max_hp),
            lifetime_ms: Some(lifetime as u32),
        reserved_id: None,
        area_end: None,
        }));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summons_follow_skill_level() {
        let sphere = Summon::for_skill(SkillEnum::AmSpheremine.id(), 3, 99).unwrap();
        assert_eq!((sphere.class, sphere.ai, sphere.limit, sphere.max_hp), (1142, 2, None, 3200));
        let flora = Summon::for_skill(SkillEnum::AmCannibalize.id(), 4, 80).unwrap();
        assert_eq!((flora.class, flora.ai, flora.limit, flora.max_hp), (1555, 3, Some(2), 1500 + 800 + 800));
        assert!(Summon::for_skill(SkillEnum::AlHeal.id(), 1, 1).is_none());
    }
}
