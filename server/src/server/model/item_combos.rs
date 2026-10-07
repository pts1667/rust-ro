//! Item combos (`db/pre-re/item_combos.yml`): sets of worn items and cards whose script applies when all are worn together.

use std::collections::HashSet;
use std::sync::OnceLock;

use models::enums::bonus::BonusType;
use models::item::special_card_metadata;
use models::status::Status;
use serde::Deserialize;

/// Script ids at and above this belong to combo sets, see `import_item_combos.py`.
pub const FIRST_COMBO_SCRIPT_ID: u32 = 1_000_000;

#[derive(Debug, Deserialize)]
pub struct ComboSet {
    /// Id of the compiled script, which also identifies the set.
    pub id: u32,
    /// Every entry that is fully worn applies the script once.
    pub combos: Vec<Vec<i32>>,
    /// Bonuses of a script that does not depend on the wearer, computed on first use.
    #[serde(skip)]
    pub static_bonuses: OnceLock<Vec<BonusType>>,
}

pub struct WornItem {
    pub item_id: i32,
    pub cards: [i16; 4],
}

pub fn catalog() -> &'static [ComboSet] {
    static CATALOG: OnceLock<Vec<ComboSet>> = OnceLock::new();
    CATALOG.get_or_init(|| serde_json::from_str(include_str!("../../../../config/item_combos.json")).expect("Invalid item combo catalog"))
}

pub fn set_of_script(id: u32) -> Option<&'static ComboSet> {
    catalog().iter().find(|set| set.id == id)
}

/// What the character wears, with the cards that count (forged and named equipment carry metadata instead of cards).
pub fn worn_items(status: &Status) -> Vec<WornItem> {
    let cards = |card0: i16, cards: [i16; 4]| if special_card_metadata(card0) { [0; 4] } else { cards };
    let weapons = status.weapons.iter().map(|item| WornItem { item_id: item.item_id, cards: cards(item.card0, [item.card0, item.card1, item.card2, item.card3]) });
    let gears = status.equipments.iter().map(|item| WornItem { item_id: item.item_id, cards: cards(item.card0, [item.card0, item.card1, item.card2, item.card3]) });
    let ammo = status.ammo.iter().map(|item| WornItem { item_id: item.item_id, cards: [0; 4] });
    weapons.chain(gears).chain(ammo).collect()
}

/// Every element needs its own worn item, or its own card slot for a card; items are matched in the order of the combo, like rathena.
fn is_worn(combo: &[i32], worn: &[WornItem], is_card: &dyn Fn(i32) -> bool) -> bool {
    let mut used_items = HashSet::new();
    let mut used_slots = HashSet::new();
    combo.iter().all(|&id| {
        if is_card(id) {
            let slot = worn.iter().enumerate().find_map(|(index, item)| {
                (0..4).find(|slot| item.cards[*slot] as i32 == id && !used_slots.contains(&(index, *slot))).map(|slot| (index, slot))
            });
            slot.is_some_and(|slot| used_slots.insert(slot))
        } else {
            let index = worn.iter().enumerate().find_map(|(index, item)| (item.item_id == id && !used_items.contains(&index)).then_some(index));
            index.is_some_and(|index| used_items.insert(index))
        }
    })
}

impl ComboSet {
    /// How many of the combos of the set are worn.
    pub fn active_count(&self, worn: &[WornItem], is_card: &dyn Fn(i32) -> bool) -> usize {
        self.combos.iter().filter(|combo| is_worn(combo, worn, is_card)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worn(item_id: i32, cards: [i16; 4]) -> WornItem {
        WornItem { item_id, cards }
    }

    #[test]
    fn a_combo_needs_every_element_on_its_own_item_or_card_slot() {
        let is_card = |id: i32| id >= 4000;
        assert!(is_worn(&[1201, 2101], &[worn(2101, [0; 4]), worn(1201, [0; 4])], &is_card));
        assert!(!is_worn(&[1201, 1201], &[worn(1201, [0; 4])], &is_card));
        assert!(is_worn(&[4001, 4002], &[worn(2101, [4002, 4001, 0, 0])], &is_card));
        assert!(!is_worn(&[4001, 4001], &[worn(2101, [4001, 0, 0, 0])], &is_card));
        assert!(is_worn(&[4001, 4001], &[worn(2101, [4001, 0, 0, 0]), worn(1201, [0, 0, 4001, 0])], &is_card));
    }

    #[test]
    fn every_combo_script_has_a_compiled_script() {
        for set in catalog() {
            assert!(set.id >= FIRST_COMBO_SCRIPT_ID && crate::server::service::item_service::ItemService::script_metadata(set.id).is_some(), "{}", set.id);
        }
    }
}
