//! `/db/pets/<class id or name>`: a pet of the pet database with the items that tame, feed and equip it.
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, items, matches, not_found, to_json, with_fields};
use crate::server::service::script_world_service::{self, PetDefinition};

pub fn link(class_id: u16) -> Link {
    let name = script_world_service::world_data().pets.iter().find(|pet| pet.class_id == class_id).map_or_else(|| class_id.to_string(), |pet| pet.name.clone());
    Link::new("pets", &class_id.to_string(), json!(class_id), &name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    script_world_service::world_data().pets.iter().filter(|pet| matches(query.q.as_deref(), &[&pet.name])).map(|pet| link(pet.class_id)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let pets = &script_world_service::world_data().pets;
    let pet: &PetDefinition = match key.parse::<u16>() {
        Ok(class_id) => pets.iter().find(|pet| pet.class_id == class_id),
        Err(_) => pets.iter().find(|pet| pet.name.eq_ignore_ascii_case(key)),
    }
    .ok_or_else(|| not_found("pet", key))?;
    Ok(with_fields(
        to_json(pet),
        [
            ("tame_item", item_value(pet.tame_item)),
            ("egg_item", item_value(pet.egg_item)),
            ("food_item", item_value(pet.food_item)),
            ("equip_item", item_value(pet.equip_item)),
        ],
    ))
}

fn item_value(item: i32) -> Value {
    if item > 0 { to_json(&items::link(item)) } else { Value::Null }
}
