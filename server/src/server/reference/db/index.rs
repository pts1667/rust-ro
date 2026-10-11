//! Reverse lookups behind `/db`: where an item drops or is sold, which monsters spawn where, which recipes and combos use an item.
//! Built on the first request from the data loaded at boot, so each entry answers without scanning every map.
use std::collections::HashMap;
use std::sync::OnceLock;

use models::enums::EnumWithMaskValueU32;
use models::enums::mob::MobMode;
use script_sdk::Entry;

use crate::server::model::item_combos;
use crate::server::model::mob_spawn::MobType;
use crate::server::model::quest::quest_catalog;
use crate::server::model::script::Script;
use crate::server::model::warp::Warp;
use crate::server::script::entries;
use crate::server::script::game_data;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::mob_skill_ai::MobSkillDatabase;

pub fn get() -> &'static DbIndex {
    static INDEX: OnceLock<DbIndex> = OnceLock::new();
    INDEX.get_or_init(DbIndex::build)
}

pub struct DropRef {
    pub mob_id: i32,
    /// Out of 10000, like the mob data.
    pub rate: u16,
    pub mvp: bool,
    pub is_card: bool,
}

pub struct ShopRef {
    pub map: String,
    pub npc: String,
    pub x: u16,
    pub y: u16,
    pub price: i32,
}

pub struct SpawnRef {
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub x_size: u16,
    pub y_size: u16,
    pub amount: i16,
    pub fixed_delay_ms: u32,
    pub random_delay_ms: u32,
    pub kind: &'static str,
}

pub struct GroupRef {
    pub group_id: i32,
    pub group: String,
    pub subgroup_id: i32,
    pub rate: u32,
    pub amount: i16,
}

#[derive(Default)]
pub struct DbIndex {
    pub item_drops: HashMap<i32, Vec<DropRef>>,
    pub item_shops: HashMap<i32, Vec<ShopRef>>,
    pub item_recipes_made: HashMap<i32, Vec<u32>>,
    pub item_recipes_used: HashMap<i32, Vec<u32>>,
    /// Positions in `game_data::data().arrows`.
    pub item_arrow_sources: HashMap<i32, Vec<usize>>,
    pub item_arrow_products: HashMap<i32, Vec<usize>>,
    pub item_groups: HashMap<i32, Vec<GroupRef>>,
    pub item_combos: HashMap<i32, Vec<u32>>,
    pub mob_spawns: HashMap<i32, Vec<SpawnRef>>,
    pub mob_quests: HashMap<i32, Vec<u32>>,
    /// Summon group ids that can call the monster.
    pub mob_summons: HashMap<i32, Vec<i32>>,
    pub skill_mobs: HashMap<u32, Vec<i32>>,
    /// Warp cells that land on a map, keyed by the map's name.
    pub map_arrivals: HashMap<String, Vec<&'static Warp>>,
}

impl DbIndex {
    fn build() -> Self {
        let config = GlobalConfigService::instance();
        let mut index = Self::default();
        index.add_mobs();
        for map in config.maps().values() {
            for warp in map.warps() {
                index.map_arrivals.entry(warp.dest_map_name.clone()).or_default().push(warp);
            }
            for spawn in map.mob_spawns() {
                index.mob_spawns.entry(i32::from(spawn.mob_id)).or_default().push(SpawnRef {
                    map: map.name().to_string(),
                    x: spawn.x,
                    y: spawn.y,
                    x_size: spawn.x_size,
                    y_size: spawn.y_size,
                    amount: spawn.to_spawn_amount,
                    fixed_delay_ms: spawn.fixed_delay_in_ms,
                    random_delay_ms: spawn.random_variance_delay_in_ms,
                    kind: kind_name(&spawn.mob_type),
                });
            }
            for script in map.scripts() {
                for (item_id, price) in shop_offers(script).unwrap_or_default() {
                    index.item_shops.entry(item_id).or_default().push(ShopRef {
                        map: script.map_name.clone(),
                        npc: script.name.clone(),
                        x: script.x,
                        y: script.y,
                        price,
                    });
                }
            }
        }
        index.add_game_data();
        for set in item_combos::catalog() {
            let items: std::collections::BTreeSet<i32> = set.combos.iter().flatten().copied().collect();
            for item in items {
                index.item_combos.entry(item).or_default().push(set.id);
            }
        }
        for quest in quest_catalog().values() {
            for objective in &quest.objectives {
                let mobs = if objective.mob != 0 { vec![objective.mob] } else { objective.mobs_allowed.clone() };
                for mob in mobs {
                    index.mob_quests.entry(i32::from(mob)).or_default().push(quest.id);
                }
            }
        }
        let skills = MobSkillDatabase::instance();
        for mob in config.mobs() {
            for entry in skills.entries_of(mob.id, is_boss(mob.mode)) {
                index.skill_mobs.entry(entry.skill_id).or_default().push(mob.id);
            }
        }
        index.sort_and_dedup();
        index
    }

    fn add_mobs(&mut self) {
        for mob in GlobalConfigService::instance().mobs() {
            for (drops, mvp) in [(&mob.drops, false), (&mob.mvp_drops, true)] {
                for drop in drops {
                    self.item_drops.entry(drop.item_id).or_default().push(DropRef { mob_id: mob.id, rate: drop.rate, mvp, is_card: drop.is_card });
                }
            }
        }
    }

    fn add_game_data(&mut self) {
        let data = game_data::data();
        for recipe in &data.recipes {
            self.item_recipes_made.entry(recipe.item_id).or_default().push(recipe.id);
            for material in &recipe.materials {
                self.item_recipes_used.entry(material.item_id).or_default().push(recipe.id);
            }
        }
        for (position, arrow) in data.arrows.iter().enumerate() {
            self.item_arrow_sources.entry(arrow.source).or_default().push(position);
            for product in &arrow.make {
                self.item_arrow_products.entry(product.item_id).or_default().push(position);
            }
        }
        for group in &data.groups {
            for subgroup in &group.subgroups {
                for entry in &subgroup.entries {
                    self.item_groups.entry(entry.item_id).or_default().push(GroupRef {
                        group_id: group.id,
                        group: group.name.clone(),
                        subgroup_id: subgroup.id,
                        rate: entry.rate,
                        amount: entry.amount,
                    });
                }
            }
        }
        for group in &data.summons {
            for entry in &group.entries {
                self.mob_summons.entry(entry.mob_id as i32).or_default().push(group.id);
            }
        }
    }

    fn sort_and_dedup(&mut self) {
        for ids in self.item_recipes_made.values_mut().chain(self.item_recipes_used.values_mut()) {
            ids.sort_unstable();
            ids.dedup();
        }
        for ids in self.item_combos.values_mut().chain(self.mob_quests.values_mut()) {
            ids.sort_unstable();
            ids.dedup();
        }
        for ids in self.mob_summons.values_mut() {
            ids.sort_unstable();
            ids.dedup();
        }
        for ids in self.skill_mobs.values_mut() {
            ids.sort_unstable();
            ids.dedup();
        }
    }
}

/// The kind of spawn in the listings of a map and a monster.
pub fn kind_name(kind: &MobType) -> &'static str {
    match kind {
        MobType::Monster => "normal",
        MobType::Miniboss => "miniboss",
        MobType::Mvp => "mvp",
    }
}

/// Monsters with the boss mode use the boss skills, see `MobSkillDatabase::entries_of`.
pub fn is_boss(mode: i16) -> bool {
    u32::from(mode as u16) & MobMode::Boss.as_flag() != 0
}

/// `(item id, price)` for each offer of a shop NPC, `None` for any other NPC. A price of -1 is the item's buy price.
pub fn shop_offers(script: &Script) -> Option<Vec<(i32, i32)>> {
    let shop = entries::resolve(script.entry_id)?;
    if shop.module != "systems" || shop.entry != Entry::Npc("shop".into()) {
        return None;
    }
    let config = GlobalConfigService::instance();
    let offers = script.constructor_args.get(1..).unwrap_or_default().chunks_exact(2).filter_map(|pair| {
        let item_id = pair[0].number_value().ok()?;
        let price = pair[1].number_value().ok()?;
        let price = if price == -1 { config.find_item(item_id)?.price_buy.unwrap_or(0) } else { price };
        Some((item_id, price))
    });
    Some(offers.collect())
}
