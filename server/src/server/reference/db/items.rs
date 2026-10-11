//! `/db/items/<id or name>`: an item with the monsters that drop it, the shops that sell it and the recipes, combos and groups it takes part in.
use serde_json::{Value, json};

use super::index::{self, DbIndex};
use super::{DbError, Link, ListQuery, matches, maps, mobs, not_found, npcs, pets, skills, to_json, with_fields};
use crate::repository::model::item_model::ItemModel;
use crate::server::model::item_combos;
use crate::server::script::game_data::{self, Recipe};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;
use crate::server::service::script_world_service;

pub fn link(id: i32) -> Link {
    let name = GlobalConfigService::instance().find_item(id).map_or_else(|| id.to_string(), |item| item.name_english.clone());
    Link::new("items", &id.to_string(), json!(id), &name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut items: Vec<&ItemModel> = GlobalConfigService::instance()
        .items()
        .filter(|item| matches(query.q.as_deref(), &[&item.name_english, &item.name_aegis]))
        .collect();
    items.sort_by_key(|item| item.id);
    items.into_iter().map(|item| link(item.id)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let item = find(key).ok_or_else(|| not_found("item", key))?;
    let id = item.id;
    let index = index::get();

    let script = ItemService::script_metadata(id as u32).map(|script| {
        json!({ "dynamic": script.dynamic, "calls": script.calls, "reads": script.reads, "interactive": script.interactive })
    });
    let bonuses: Vec<String> = item.bonuses.iter().map(|bonus| format!("{bonus:?}")).collect();
    let fields = vec![
        ("bonuses", json!(bonuses)),
        ("script_metadata", json!(script)),
        ("dropped_by", dropped_by(index, id)),
        ("sold_by", sold_by(index, id)),
        ("recipes", recipes(index, id)),
        ("arrow_crafting", arrow_crafting(index, id)),
        ("groups", groups(index, id)),
        ("combo_sets", combo_sets(index, id)),
        ("pet_items", pet_items(id)),
    ];
    Ok(with_fields(to_json(item), fields))
}

/// A name several items share answers with the lowest id, so the same key always gives the same item.
fn find(key: &str) -> Option<&'static ItemModel> {
    let config = GlobalConfigService::instance();
    match key.parse::<i32>() {
        Ok(id) => config.find_item(id),
        Err(_) => config.items().filter(|item| item.name_aegis.eq_ignore_ascii_case(key) || item.name_english.eq_ignore_ascii_case(key)).min_by_key(|item| item.id),
    }
}

fn dropped_by(index: &DbIndex, id: i32) -> Value {
    let drops: Vec<Value> = index
        .item_drops
        .get(&id)
        .into_iter()
        .flatten()
        .map(|drop| json!({ "mob": mobs::link(drop.mob_id), "rate": drop.rate, "mvp": drop.mvp, "card": drop.is_card }))
        .collect();
    json!(drops)
}

fn sold_by(index: &DbIndex, id: i32) -> Value {
    let offers: Vec<Value> = index
        .item_shops
        .get(&id)
        .into_iter()
        .flatten()
        .map(|shop| {
            json!({ "npc": npcs::link(&shop.map, &shop.npc), "map": maps::link(&shop.map), "x": shop.x, "y": shop.y, "price": shop.price })
        })
        .collect();
    json!(offers)
}

fn recipes(index: &DbIndex, id: i32) -> Value {
    let recipes_of = |ids: Option<&Vec<u32>>| -> Vec<Value> {
        ids.into_iter().flatten().filter_map(|recipe_id| game_data::data().recipes.iter().find(|recipe| recipe.id == *recipe_id)).map(recipe_json).collect()
    };
    json!({ "made_by": recipes_of(index.item_recipes_made.get(&id)), "used_in": recipes_of(index.item_recipes_used.get(&id)) })
}

fn recipe_json(recipe: &Recipe) -> Value {
    let materials: Vec<Value> = recipe.materials.iter().map(|material| json!({ "item": link(material.item_id), "amount": material.amount })).collect();
    json!({
        "id": recipe.id,
        "product": link(recipe.item_id),
        "level": recipe.level,
        "skill": skills::link(recipe.skill_id),
        "skill_level": recipe.skill_level,
        "materials": materials,
    })
}

fn arrow_crafting(index: &DbIndex, id: i32) -> Value {
    let arrows = &game_data::data().arrows;
    let makes: Vec<Value> = index
        .item_arrow_sources
        .get(&id)
        .into_iter()
        .flatten()
        .map(|position| {
            let products: Vec<Value> = arrows[*position].make.iter().map(|product| json!({ "item": link(product.item_id), "amount": product.amount })).collect();
            json!({ "makes": products })
        })
        .collect();
    let made_from: Vec<Value> = index
        .item_arrow_products
        .get(&id)
        .into_iter()
        .flatten()
        .map(|position| json!({ "from": link(arrows[*position].source) }))
        .collect();
    json!({ "as_source": makes, "as_product": made_from })
}

fn groups(index: &DbIndex, id: i32) -> Value {
    let groups: Vec<Value> = index
        .item_groups
        .get(&id)
        .into_iter()
        .flatten()
        .map(|group| json!({ "group": group.group_id, "name": group.group, "subgroup": group.subgroup_id, "rate": group.rate, "amount": group.amount }))
        .collect();
    json!(groups)
}

fn combo_sets(index: &DbIndex, id: i32) -> Value {
    let sets: Vec<Value> = index
        .item_combos
        .get(&id)
        .into_iter()
        .flatten()
        .filter_map(|set_id| item_combos::set_of_script(*set_id))
        .map(to_json)
        .collect();
    json!(sets)
}

fn pet_items(id: i32) -> Value {
    let roles: Vec<Value> = script_world_service::world_data()
        .pets
        .iter()
        .flat_map(|pet| {
            [("tame", pet.tame_item), ("egg", pet.egg_item), ("food", pet.food_item), ("equip", pet.equip_item)]
                .into_iter()
                .filter(move |(_, item)| *item == id)
                .map(move |(role, _)| json!({ "pet": pets::link(pet.class_id), "role": role }))
        })
        .collect();
    json!(roles)
}
