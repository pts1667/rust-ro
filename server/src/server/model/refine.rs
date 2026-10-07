//! Refine rates, prices and ores of `db/pre-re/refine.yml`.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::repository::model::item_model::ItemModel;
use models::enums::item::ItemType;

pub const MAX_REFINE: i16 = 10;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct RefineCost {
    /// Out of 10000.
    pub rate: u16,
    pub price: u32,
    pub material: i32,
}

#[derive(Debug, Deserialize)]
struct RefineCosts {
    normal: Option<RefineCost>,
    enriched: Option<RefineCost>,
    hd: Option<RefineCost>,
}

#[derive(Debug, Deserialize)]
struct RefineLevel {
    group: String,
    level: i16,
    refine: i16,
    costs: RefineCosts,
}

/// `REFINE_COST_*` values of the script commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefineCostType {
    Normal,
    Hd,
    Enriched,
}

impl RefineCostType {
    pub fn from_number(number: i32) -> Option<Self> {
        Some(match number {
            0 => Self::Normal,
            1 => Self::Hd,
            2 => Self::Enriched,
            _ => return None,
        })
    }
}

fn levels() -> &'static [RefineLevel] {
    static LEVELS: OnceLock<Vec<RefineLevel>> = OnceLock::new();
    LEVELS.get_or_init(|| serde_json::from_str(include_str!("refine.json")).expect("Invalid refine database"))
}

pub fn is_refineable(item: &ItemModel) -> bool {
    item.refineable == Some(1) && matches!(item.item_type, ItemType::Weapon | ItemType::Armor)
}

/// Cost of the next refine of `item`, currently at `refine`.
pub fn refine_cost(item: &ItemModel, refine: i16, kind: RefineCostType) -> Option<RefineCost> {
    if !is_refineable(item) || !(0..MAX_REFINE).contains(&refine) {
        return None;
    }
    let (group, level) = if item.item_type == ItemType::Weapon { ("weapon", item.weapon_level) } else { ("armor", item.armor_level) };
    let costs = &levels().iter().find(|entry| entry.group == group && entry.level == level.unwrap_or(0) && entry.refine == refine + 1)?.costs;
    match kind {
        RefineCostType::Normal => costs.normal,
        RefineCostType::Enriched => costs.enriched,
        RefineCostType::Hd => costs.hd,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(kind: &str, level: i16) -> ItemModel {
        serde_json::from_value(serde_json::json!({"id":1,"name_aegis":"A","name_english":"A","weight":1,"item_type":kind,"job_flags":0,"class_flags":0,"location":0,
            "flags":0,"trade_flags":0,"refineable":1,"weapon_level":level,"armor_level":level})).unwrap()
    }

    #[test]
    fn rates_follow_the_weapon_level_and_stop_at_the_maximum() {
        let weapon = item("Weapon", 1);
        assert_eq!(refine_cost(&weapon, 0, RefineCostType::Normal).map(|cost| (cost.rate, cost.material)), Some((10000, 1010)));
        assert!(refine_cost(&weapon, 10, RefineCostType::Normal).is_none());
        assert_eq!(refine_cost(&item("Armor", 1), 4, RefineCostType::Enriched).map(|cost| cost.rate), Some(9000));
        assert!(refine_cost(&item("Armor", 1), 0, RefineCostType::Hd).is_none());
    }
}
