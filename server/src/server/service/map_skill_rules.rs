use std::collections::BTreeMap;
use std::sync::OnceLock;

use models::enums::map::{MapActorType, SkillDamageMap};
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
use serde::Deserialize;

use crate::server::model::map_flags::{MapFlag, MapFlags};

#[derive(Deserialize)]
struct SkillDamageAdjustment {
    skill_id: u32,
    caster: u16,
    maps: u32,
    rates: [i32; 4],
}

fn adjustments() -> &'static BTreeMap<u32, SkillDamageAdjustment> {
    static ADJUSTMENTS: OnceLock<BTreeMap<u32, SkillDamageAdjustment>> = OnceLock::new();
    ADJUSTMENTS.get_or_init(|| {
        let entries: Vec<SkillDamageAdjustment> = serde_json::from_str(include_str!("../script/skill_damage_adjustments.json"))
            .expect("Embedded pre-renewal skill damage adjustments are invalid");
        entries.into_iter().map(|entry| (entry.skill_id, entry)).collect()
    })
}

pub(crate) fn database_damage_rate(flags: &MapFlags, skill_id: u32, caster: MapActorType, target: usize) -> i32 {
    let Some(entry) = adjustments().get(&skill_id) else {
        return 0;
    };
    if target >= entry.rates.len() || entry.caster & caster.as_flag() == 0 {
        return 0;
    }
    let restricted = flags.get(MapFlag::Restricted, None) as u32;
    let normal = !flags.enabled(MapFlag::Pvp)
        && !flags.is_gvg()
        && !flags.enabled(MapFlag::Battleground)
        && !flags.enabled(MapFlag::SkillDamage)
        && restricted == 0;
    let allowed = [
        (SkillDamageMap::Normal, normal),
        (SkillDamageMap::Pvp, flags.enabled(MapFlag::Pvp)),
        (SkillDamageMap::Gvg, flags.is_gvg()),
        (SkillDamageMap::Battleground, flags.enabled(MapFlag::Battleground)),
        (SkillDamageMap::Flag, flags.enabled(MapFlag::SkillDamage)),
    ]
    .into_iter()
    .any(|(kind, enabled)| enabled && entry.maps & kind.as_flag() != 0)
        || restricted & entry.maps != 0;
    if allowed { entry.rates[target].clamp(-100, 100000) } else { 0 }
}
