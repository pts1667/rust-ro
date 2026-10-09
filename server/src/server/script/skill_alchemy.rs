use models::enums::skill_enums::SkillEnum;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use regex_lite::Regex;
use skills::TwilightStage;

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::events::map_event::{MapEvent, ScriptSpawn};
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const BERSERK_POTION_MIN_BASE_LEVEL: u32 = 85;
const BERSERK_POTION_DURATION_MS: i32 = 1_800_000;
const CULTIVATION_LIFETIME_MS: u32 = 300_000;
const CULTIVATION_FAILURE_PERCENT: u32 = 50;
const BLACK_MUSHROOM: u32 = 1084;
const WHITE_POTION: i32 = 504;
const WHITE_SLIM_POTION: i32 = 547;
const ALCOHOL: i32 = 970;
const ACID_BOTTLE: i32 = 7136;
const FIRE_BOTTLE: i32 = 7135;
const EMPTY_BOTTLE: i32 = 713;
const PHARMACY_RECIPE_LEVEL: u16 = 22;
/// Plant cultivated from a Stem, with its share in percent: Green, Red, Yellow, White, Blue and Shining Plant.
const STEM_PLANTS: [(u32, u32); 6] = [(1080, 30), (1078, 25), (1081, 25), (1082, 10), (1079, 8), (1083, 2)];

/// `itemheal` ranges of a potion script: (hp low, hp high), (sp low, sp high).
fn potion_heal_ranges(script: &str) -> Option<((u32, u32), (u32, u32))> {
    let amount = Regex::new(r"itemheal\s+(?:rand\((\d+)\s*,\s*(\d+)\)|(\d+))\s*,\s*(?:rand\((\d+)\s*,\s*(\d+)\)|(\d+))").ok()?;
    let captures = amount.captures(script)?;
    let number = |index: usize| captures.get(index).and_then(|group| group.as_str().parse::<u32>().ok());
    let range = |low: usize, high: usize, fixed: usize| match (number(low), number(high), number(fixed)) {
        (Some(low), Some(high), _) => (low, high),
        (_, _, Some(fixed)) => (fixed, fixed),
        _ => (0, 0),
    };
    Some((range(1, 2, 3), range(4, 5, 6)))
}

fn boost(base: u32, percent: u32) -> u32 {
    (u64::from(base) * u64::from(percent) / 100).min(u64::from(u32::MAX)) as u32
}

/// Pitched potions heal for the potion's amount, raised by the pitcher's skills and then by the target's vitality (or intelligence) and recovery skill.
fn pitched_amount(base: u32, pitcher_percent: u32, stat: u32, recovery_level: u32) -> u32 {
    boost(boost(boost(base, pitcher_percent), 100 + 2 * stat), 100 + 10 * recovery_level)
}

fn roll((low, high): (u32, u32)) -> u32 {
    if high <= low { low } else { fastrand::u32(low..=high) }
}

impl ScriptSkillService {
    /// The potion consumed at this level: the pitcher skills list one potion per skill level.
    fn pitched_potion(&self, skill_id: u32, level: u8) -> Result<((u32, u32), (u32, u32)), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Unknown pitcher skill")?;
        let name = metadata
            .requires
            .as_ref()
            .and_then(|requires| requires.get("ItemCost"))
            .and_then(|costs| costs.as_array())
            .and_then(|costs| costs.get(usize::from(level).saturating_sub(1)))
            .and_then(|cost| cost.get("Item"))
            .and_then(|item| item.as_str())
            .ok_or("This level pitches no potion")?;
        let item = self.configuration.find_item_by_name(name).ok_or("The potion is unavailable")?;
        item.script.as_deref().and_then(potion_heal_ranges).ok_or_else(|| "The potion does not heal".to_string())
    }

    pub(super) fn aid_potion(&self, server: &Server, state: &ServerState, target: &mut Character, effect: &ScriptSkillEffect) -> Result<(), String> {
        let caster = if effect.source_char_id == target.char_id { &*target } else { state.get_character(effect.source_char_id).ok_or("Caster disconnected")? };
        let (hp_range, sp_range) = self.pitched_potion(effect.skill_id, effect.level)?;
        let learned = |skill: SkillEnum| u32::from(learned_level(&caster.status, skill.id()));
        let linked_bonus = if super::ScriptSkillService::spirit_rules(caster.status.status_change(StatusChangeKind::Spirit)).alchemy_base_level_bonus {
            caster.status.base_level
        } else {
            0
        };
        let pitcher_percent = (100 + 10 * learned(SkillEnum::AmPotionpitcher) + 5 * learned(SkillEnum::AmLearningpotion)) * (100 + linked_bonus) / 100;
        let snapshot = StatusService::instance().to_snapshot(&target.status);
        let hp = pitched_amount(roll(hp_range), pitcher_percent, u32::from(snapshot.vit()), u32::from(learned_level(&target.status, SkillEnum::SmRecovery.id())));
        let sp = pitched_amount(roll(sp_range), pitcher_percent, u32::from(snapshot.int()), u32::from(learned_level(&target.status, SkillEnum::MgSrecovery.id())));
        Self::restore(server, target, hp, sp);
        Ok(())
    }

    pub(super) fn restore(server: &Server, target: &mut Character, hp: u32, sp: u32) {
        if target.status.hp == 0 || target.status.has_status_change(StatusChangeKind::NoRecovery) || target.status.has_status_change(StatusChangeKind::Berserk) {
            return;
        }
        let snapshot = StatusService::instance().to_snapshot(&target.status);
        server.character_service().update_hp_sp(target, target.status.hp.saturating_add(hp).min(snapshot.max_hp()), target.status.sp.saturating_add(sp).min(snapshot.max_sp()));
    }

    /// Twilight Alchemy prepares 200 White Potions, 200 Condensed White Potions, or 100 Alcohol with 50 Acid and 50 Fire Bottles.
    pub(super) fn twilight_alchemy(&self, server: &Server, character: &mut Character, stage: TwilightStage) -> Result<(), String> {
        let items = server.item_service();
        let batches: &[(i32, u16)] = match stage {
            TwilightStage::WhitePotion => &[(WHITE_POTION, 200)],
            TwilightStage::WhiteSlimPotion => &[(WHITE_SLIM_POTION, 200)],
            TwilightStage::Bottles => &[(ALCOHOL, 100), (ACID_BOTTLE, 50), (FIRE_BOTTLE, 50)],
        };
        if stage == TwilightStage::Bottles {
            let bottles: i32 = character.inventory_iter().filter(|(_, item)| item.item_id == EMPTY_BOTTLE).map(|(_, item)| i32::from(item.amount)).sum();
            if bottles < 200 {
                return Err("Twilight Alchemy needs 200 Empty Bottles".into());
            }
        }
        for &(item_id, quantity) in batches {
            items.make_batch(server, character, item_id, quantity, PHARMACY_RECIPE_LEVEL)?;
        }
        Ok(())
    }

    pub(super) fn aid_berserk_potion(&self, server: &Server, target: &mut Character, tick: u128) -> Result<(), String> {
        if target.status.base_level < BERSERK_POTION_MIN_BASE_LEVEL {
            return Err("The target is too low level for the Berserk Potion".into());
        }
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::AspdPotion2, BERSERK_POTION_DURATION_MS, 0);
        request.flags = 0;
        StatusEffectService::start(server, target, request, tick, &self.client_notification_sender).map(|_| ())
    }

    /// Slim Pitcher heals the party and guild members around the target cell with a condensed potion.
    pub(super) fn aid_condensed_potion(&self, server: &Server, state: &ServerState, caster: &mut Character, effect: &ScriptSkillEffect, x: u16, y: u16) -> Result<(), String> {
        let (hp_range, sp_range) = self.pitched_potion(effect.skill_id, effect.level)?;
        let learned = |skill: SkillEnum| u32::from(learned_level(&caster.status, skill.id()));
        let pitcher_percent = 100 + 10 * learned(SkillEnum::CrSlimpitcher) + 10 * learned(SkillEnum::AmPotionpitcher) + 5 * learned(SkillEnum::AmLearningpotion);
        let (hp_base, sp_base) = (boost(roll(hp_range), pitcher_percent), boost(roll(sp_range), pitcher_percent));
        let radius = SkillMetadata::find(effect.skill_id).and_then(|metadata| metadata.splash(effect.level)).unwrap_or(3).max(0) as u16;
        let (party_id, guild_id) = (caster.game_systems.party_id, caster.game_systems.guild_id);
        let allied = |party: u32, guild: u32| (party_id != 0 && party == party_id) || (guild_id != 0 && guild == guild_id);
        let healed = |character: &Character| {
            let snapshot = StatusService::instance().to_snapshot(&character.status);
            let recovery = |skill: SkillEnum| u32::from(learned_level(&character.status, skill.id()));
            (
                boost(boost(hp_base, 100 + 2 * u32::from(snapshot.vit())), 100 + 10 * recovery(SkillEnum::SmRecovery)),
                boost(boost(sp_base, 100 + 2 * u32::from(snapshot.int())), 100 + 10 * recovery(SkillEnum::MgSrecovery)),
            )
        };
        let in_area = |character: &Character| character.x.abs_diff(x).max(character.y.abs_diff(y)) <= radius;
        let recipients: Vec<(u32, u32, u32)> = state
            .characters()
            .values()
            .filter(|member| member.map_instance_key == caster.map_instance_key && in_area(member) && allied(member.game_systems.party_id, member.game_systems.guild_id))
            .map(|member| {
                let (hp, sp) = healed(member);
                (member.char_id, hp, sp)
            })
            .collect();
        for (char_id, hp, sp) in recipients {
            self.followup_action(server, effect, char_id, ScriptSkillAction::Heal { hp, sp });
        }
        if in_area(caster) && allied(party_id, guild_id) {
            let (hp, sp) = healed(caster);
            Self::restore(server, caster, hp, sp);
        }
        Ok(())
    }

    /// Cultivation plants a mushroom (from a Spore) or a plant (from a Stem) that lives five minutes, half the casts fail.
    pub(super) fn cultivate(&self, state: &ServerState, caster: &Character, effect: &ScriptSkillEffect, x: u16, y: u16) -> Result<(), String> {
        let occupied = state.characters().values().any(|other| other.map_instance_key == caster.map_instance_key && other.x == x && other.y == y);
        if occupied || (caster.x == x && caster.y == y) {
            return Err("Something stands on the target cell".into());
        }
        if fastrand::u32(0..100) < CULTIVATION_FAILURE_PERCENT {
            return Ok(());
        }
        let mob_id = if effect.level >= 2 {
            let mut roll = fastrand::u32(0..100);
            STEM_PLANTS
                .iter()
                .find(|(_, share)| {
                    let hit = roll < *share;
                    roll = roll.saturating_sub(*share);
                    hit
                })
                .map_or(STEM_PLANTS[0].0, |(plant, _)| *plant)
        } else {
            BLACK_MUSHROOM + fastrand::u32(0..2)
        };
        let instance = state.get_map_instance_from_character(caster).ok_or("Map instance is unavailable")?;
        instance.add_to_next_tick(MapEvent::ScriptSpawn(ScriptSpawn {
            is_guardian: false,
            mob_id: mob_id as i32,
            x: i32::from(x),
            y: i32::from(y),
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
            lifetime_ms: Some(CULTIVATION_LIFETIME_MS),
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
    fn potion_scripts_give_their_heal_ranges() {
        assert_eq!(potion_heal_ranges("itemheal rand(45,65),0;"), Some(((45, 65), (0, 0))));
        assert_eq!(potion_heal_ranges("itemheal 0,rand(40,60);"), Some(((0, 0), (40, 60))));
        assert_eq!(potion_heal_ranges("itemheal 500,50;"), Some(((500, 500), (50, 50))));
        assert_eq!(potion_heal_ranges("sc_start SC_ASPDPOTION2,1800000,0;"), None);
    }

    #[test]
    fn pitched_potions_scale_with_the_pitcher_skills_vitality_and_recovery() {
        assert_eq!(pitched_amount(100, 150, 0, 0), 150);
        assert_eq!(pitched_amount(100, 150, 50, 0), 300);
        assert_eq!(pitched_amount(100, 100, 0, 5), 150);
    }
}
