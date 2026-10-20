//! `/db/<category>` lists a category, `/db/<category>/<key>` answers one entry with everything linked to it. Entries name the
//! things they relate to as `{ id, name, href }` links, so a client can walk from an item to the monsters that drop it and on to
//! their spawns. Keys are the numeric id where there is one, otherwise the name; `/db/npcs/<map>/<name>` is keyed by placement.
mod destinations;
mod index;
mod items;
mod jobs;
mod maps;
mod mobs;
mod npcs;
mod pets;
mod quests;
mod route;
mod skills;
mod statuses;
mod transports;
mod warps;

use std::collections::BTreeMap;
use std::sync::OnceLock;

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const CATEGORIES: [&str; 11] = ["items", "mobs", "quests", "skills", "maps", "warps", "destinations", "npcs", "pets", "jobs", "statuses"];
const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

pub fn router() -> Router {
    Router::new()
        .route("/db", get(categories))
        .route("/db/route", get(route::answer))
        .route("/db/{category}", get(list))
        .route("/db/{category}/{*key}", get(entry))
}

#[derive(Debug)]
pub enum DbError {
    NotFound(String),
    Invalid(String),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::Invalid(message) => (StatusCode::BAD_REQUEST, message),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

type Reply = Result<Json<Value>, DbError>;

#[derive(Deserialize, Default)]
pub struct ListQuery {
    /// Part of the name, case-insensitive.
    q: Option<String>,
    /// Only the NPCs placed on this map.
    map: Option<String>,
    offset: Option<usize>,
    limit: Option<usize>,
}

/// Another entry, as every entry names what it relates to.
#[derive(Serialize)]
pub struct Link {
    pub id: Value,
    pub name: String,
    pub href: String,
}

impl Link {
    fn new(category: &str, key: &str, id: Value, name: &str) -> Self {
        Self { id, name: name.to_string(), href: format!("/db/{category}/{}", encode_path(key)) }
    }
}

async fn categories() -> Json<Value> {
    let categories: Vec<Value> = CATEGORIES
        .iter()
        .map(|category| {
            let count = category_links(category, &ListQuery::default()).map_or(0, |links| links.len());
            json!({ "name": category, "href": format!("/db/{category}"), "count": count })
        })
        .collect();
    Json(json!({ "categories": categories, "route": "/db/route?from=<map>&to=<map>", "wiki": "/wiki/index.md" }))
}

async fn list(Path(category): Path<String>, Query(query): Query<ListQuery>) -> Reply {
    let links = category_links(&category, &query).ok_or_else(|| unknown_category(&category))?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query.offset.unwrap_or(0);
    let total = links.len();
    let results: Vec<Link> = links.into_iter().skip(offset).take(limit).collect();
    Ok(Json(json!({ "total": total, "offset": offset, "limit": limit, "results": results })))
}

async fn entry(Path((category, key)): Path<(String, String)>) -> Reply {
    let value = match category.as_str() {
        "items" => items::entry(&key),
        "mobs" => mobs::entry(&key),
        "quests" => quests::entry(&key),
        "skills" => skills::entry(&key),
        "maps" => maps::entry(&key),
        "warps" => warps::entry(&key),
        "destinations" => destinations::entry(&key),
        "npcs" => npcs::entry(&key),
        "pets" => pets::entry(&key),
        "jobs" => jobs::entry(&key),
        "statuses" => statuses::entry(&key),
        _ => return Err(unknown_category(&category)),
    }?;
    Ok(Json(value))
}

fn category_links(category: &str, query: &ListQuery) -> Option<Vec<Link>> {
    Some(match category {
        "items" => items::links(query),
        "mobs" => mobs::links(query),
        "quests" => quests::links(query),
        "skills" => skills::links(query),
        "maps" => maps::links(query),
        "warps" => warps::links(query),
        "destinations" => destinations::links(query),
        "npcs" => npcs::links(query),
        "pets" => pets::links(query),
        "jobs" => jobs::links(query),
        "statuses" => statuses::links(query),
        _ => return None,
    })
}

fn unknown_category(category: &str) -> DbError {
    DbError::NotFound(format!("unknown category {category}, the categories are {}", CATEGORIES.join(", ")))
}

fn not_found(kind: &str, key: &str) -> DbError {
    DbError::NotFound(format!("no {kind} for {key}"))
}

/// Whether a name contains the query, case-insensitively; every name passes without a query.
fn matches(query: Option<&str>, names: &[&str]) -> bool {
    let Some(query) = query.filter(|query| !query.is_empty()) else { return true };
    let query = query.to_ascii_lowercase();
    names.iter().any(|name| name.to_ascii_lowercase().contains(&query))
}

fn to_json<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap_or(Value::Null)
}

/// `body` with the given fields set, replacing the ones it already has.
fn with_fields(mut body: Value, fields: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    if let Value::Object(object) = &mut body {
        object.extend(fields.into_iter().map(|(name, value)| (name.to_string(), value)));
    }
    body
}

/// Percent-encodes each `/`-separated part of a key, so the key stays one path with its slashes.
fn encode_path(key: &str) -> String {
    key.split('/')
        .map(|part| {
            part.bytes()
                .map(|byte| match byte {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => char::from(byte).to_string(),
                    _ => format!("%{byte:02X}"),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// A configuration file embedded at build time, parsed on first use.
fn embedded_json(cell: &'static OnceLock<Value>, text: &'static str) -> &'static Value {
    cell.get_or_init(|| serde_json::from_str(text).expect("Embedded configuration file is invalid"))
}

/// The Description section of an iRO Wiki page, copied by `tools/scripts-import/extract_wiki_descriptions.py`.
#[derive(Deserialize)]
struct WikiText {
    page: String,
    description: String,
}

static WIKI_TEXTS: OnceLock<BTreeMap<String, BTreeMap<String, WikiText>>> = OnceLock::new();

/// `{ page, description }` for the entry of `category` under `key`, `null` when no wiki page matches it.
fn wiki_text(category: &str, key: &str) -> Value {
    let texts = WIKI_TEXTS.get_or_init(|| {
        serde_json::from_str(include_str!("../../../../../config/wiki_descriptions.json")).expect("Embedded wiki descriptions are invalid")
    });
    texts.get(category).and_then(|entries| entries.get(key)).map_or(Value::Null, |text| {
        json!({ "page": format!("/wiki/{}", encode_path(&text.page)), "description": text.description })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use tower::ServiceExt;

    pub(crate) async fn get_json(uri: &str) -> (StatusCode, Value) {
        crate::tests::common::before_all();
        let response = router().oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap()).await.unwrap();
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
    }

    #[test]
    fn keys_keep_their_slashes_and_escape_the_rest() {
        assert_eq!(encode_path("prontera"), "prontera");
        assert_eq!(encode_path("prontera/Job Master#1"), "prontera/Job%20Master%231");
    }

    #[test]
    fn a_name_matches_case_insensitively() {
        assert!(matches(Some("POTION"), &["Red Potion"]));
        assert!(matches(None, &[]));
        assert!(!matches(Some("ring"), &["Red Potion"]));
    }

    #[tokio::test]
    async fn the_index_lists_the_categories_and_unknown_ones_are_refused() {
        let (status, body) = get_json("/db").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["categories"].as_array().unwrap().len(), CATEGORIES.len());
        let (status, body) = get_json("/db/potions").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body["error"].as_str().unwrap().contains("unknown category"));
    }

    #[tokio::test]
    async fn a_listing_is_paged_and_searched() {
        let (status, body) = get_json("/db/items?q=red%20potion&limit=5").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body["total"].as_u64().unwrap() >= 1);
        assert!(body["results"].as_array().unwrap().iter().any(|link| link["href"] == "/db/items/501"));
        assert!(body["results"].as_array().unwrap().len() <= 5);
    }

    #[tokio::test]
    async fn an_item_names_the_mobs_that_drop_it_and_they_name_it_back() {
        let (status, item) = get_json("/db/items/904").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(item["id"], 904);
        let drops = item["dropped_by"].as_array().unwrap();
        assert!(!drops.is_empty());
        let (status, mob) = get_json(drops[0]["mob"]["href"].as_str().unwrap()).await;
        assert_eq!(status, StatusCode::OK);
        let named = mob["drops"].as_array().unwrap().iter().chain(mob["mvp_drops"].as_array().unwrap()).any(|drop| drop["item"]["id"] == 904);
        assert!(named, "{mob}");
    }

    #[tokio::test]
    async fn a_monster_lists_its_spawns_skills_and_drops_by_name() {
        let (status, mob) = get_json("/db/mobs/PORING").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(mob["id"], 1002);
        assert!(mob["drops"].as_array().unwrap().iter().all(|drop| drop["item"]["href"].as_str().is_some()));
        assert!(mob["skills"].as_array().is_some());
    }

    #[tokio::test]
    async fn an_entry_carries_the_description_of_its_wiki_page_or_null() {
        let (status, skill) = get_json("/db/skills/5").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(skill["wiki"]["page"], "/wiki/Bash.md");
        let text = skill["wiki"]["description"].as_str().unwrap();
        assert!(text.contains("ATK") && text.contains("](/wiki/Stats.md)"), "{text}");
        let (_, base) = get_json("/db/skills/20").await;
        assert!(base["wiki"].is_null());
        let (_, status_effect) = get_json("/db/statuses/Angelus").await;
        assert_eq!(status_effect["wiki"]["page"], "/wiki/Angelus.md");
    }

    #[tokio::test]
    async fn an_item_carries_its_english_client_description_or_null() {
        let (status, potion) = get_json("/db/items/501").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(potion["description"]["identified"][0], "A potion made from grinded Red Herbs.");
        assert_eq!(potion["description"]["identified"][2], "Heal: 45 - 65 HP");
        let (_, abundance) = get_json("/db/items/6069").await;
        assert!(abundance["description"].is_null());
    }

    #[tokio::test]
    async fn a_quest_is_keyed_by_its_number_and_refuses_others() {
        let id = crate::server::model::quest::quest_catalog().keys().min().copied().unwrap();
        let (status, quest) = get_json(&format!("/db/quests/{id}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(quest["id"], id);
        assert!(quest["notes"].as_str().is_some());
        let (status, _) = get_json("/db/quests/999999999").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, body) = get_json("/db/quests/poring").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("keyed by their number"));
    }

    #[tokio::test]
    async fn a_skill_names_its_metadata_jobs_and_casters() {
        let (status, skill) = get_json("/db/skills/NV_BASIC").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(skill["id"], 1);
        assert!(skill["metadata"].is_object());
        assert!(skill["learned_by"].as_array().is_some_and(|jobs| !jobs.is_empty()));
    }

    #[test]
    fn a_map_link_is_named_in_english_where_the_client_names_it() {
        let field = maps::link("prt_fild08");
        assert_eq!(field.id, "prt_fild08");
        assert_eq!(field.name, "Prontera Field area 8");
        assert_eq!(field.href, "/db/maps/prt_fild08");
        assert_eq!(maps::display_name("prontera"), Some("Prontera, Capital of Rune Midgard"));
        assert_eq!(maps::display_name("no_such_map"), None);
        assert_eq!(maps::link("no_such_map").name, "no_such_map");
    }

    #[tokio::test]
    async fn a_route_needs_both_maps_and_known_ones() {
        let (status, body) = get_json("/db/route?from=prontera").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("from=<map>&to=<map>"));
        let (status, body) = get_json("/db/route?from=no_such_map&to=prontera").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body["error"].as_str().unwrap().contains("no_such_map"));
    }

    #[tokio::test]
    async fn a_map_and_an_npc_are_refused_when_they_do_not_exist() {
        let (status, _) = get_json("/db/maps/no_such_map").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, body) = get_json("/db/npcs/prontera").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("<map>/<name>"));
        let (status, _) = get_json("/db/npcs/no_such_map/Nobody").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn jobs_statuses_and_pets_answer_by_name_or_number() {
        let (status, job) = get_json("/db/jobs/Swordsman").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(job["id"], 1);
        assert!(job["skill_tree"]["tree"].as_array().is_some_and(|tree| !tree.is_empty()));
        let (status, _) = get_json("/db/statuses/Stone").await;
        assert_eq!(status, StatusCode::OK);
        let (status, listing) = get_json("/db/pets").await;
        assert_eq!(status, StatusCode::OK);
        let href = listing["results"][0]["href"].as_str().unwrap();
        let (status, _) = get_json(href).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn a_warper_destination_is_keyed_by_its_place_and_links_its_map() {
        let (status, listing) = get_json("/db/destinations?q=prontera").await;
        assert_eq!(status, StatusCode::OK);
        let href = listing["results"][0]["href"].as_str().unwrap().to_string();
        let (status, destination) = get_json(&href).await;
        assert_eq!(status, StatusCode::OK, "{href}");
        assert_eq!(destination["name"], "Prontera");
        assert_eq!(destination["category"], "Towns");
        assert_eq!(destination["map"]["href"], "/db/maps/prontera");
        let (status, _) = get_json("/db/destinations/prontera/1,1").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, _) = get_json("/db/destinations/prontera").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn a_warp_cell_is_refused_when_its_map_or_cell_is_unknown() {
        let (status, _) = get_json("/db/warps/no_such_map/1,1").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, _) = get_json("/db/warps/prontera/1,1").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let (status, body) = get_json("/db/warps/prontera/left").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("<map>/<x>,<y>"));
    }
}
