#![feature(test)]
#[macro_use]
extern crate accessor;
extern crate core;
extern crate models;
extern crate packets;
extern crate test;
#[macro_use]
extern crate tracing;

mod proxy;
#[macro_use]
mod util;
#[cfg(feature = "visual_debugger")]
mod debugger;
mod repository;
pub mod server;
mod tests;

use std::collections::HashMap;
#[cfg(feature = "static_db_update")]
use std::fs::File;
#[cfg(feature = "static_db_update")]
use std::io::Write;
#[cfg(feature = "static_db_update")]
use std::path::Path;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;

use configuration::configuration::Config;
use proxy::map::MapProxy;
use server::Server;
use tokio::runtime::Runtime;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

use self::server::model::events::client_notification::Notification;
use self::server::model::events::persistence_event::PersistenceEvent;
use crate::proxy::char::CharProxy;
use crate::repository::model::item_model::ItemModel;
#[cfg(feature = "static_db_update")]
use crate::repository::model::item_model::ItemModels;
use crate::repository::model::mob_model::MobModel;
#[cfg(feature = "static_db_update")]
use crate::repository::model::mob_model::MobModels;
use crate::repository::{ItemRepository, MobRepository, Repository, SledRepository};
use crate::server::boot::map_loader::MapLoader;
use crate::server::boot::mob_spawn_loader::MobSpawnLoader;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::boot::warps_loader::WarpLoader;
use crate::server::model::map::Map;
use crate::server::model::map_item::MapItems;
use crate::server::model::script::Script;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;

pub static mut CONFIGS: Option<Config> = None;
pub static mut MAPS: Option<HashMap<String, &Map>> = None;
pub static mut MOB_ROOT_PATH: &str = "./config/npc";
pub static mut MAP_DIR: &str = "./config/maps/pre-re";
#[tokio::main]
pub async fn main() {
    let _start = Instant::now();
    unsafe {
        CONFIGS = Some(Config::load("").unwrap());
    }

    setup_logger(configs());
    let runtime = Arc::new(Runtime::new().unwrap());
    let repository = SledRepository::open(&configs().database).expect("Failed to open sled database and seed assets");
    let repository_arc = Arc::new(repository);
    // Load all items in memory, it takes only few mb
    let mut items = repository_arc.get_all_items().await.unwrap();
    // Load all mobs in memory, it takes only few mb
    let mobs = repository_arc.get_all_mobs().await.unwrap();
    // Initializing global id pool
    let mut map_item_ids = MapItems::new(300000);
    // Unit Tests needs items and mob db,to avoid starting an actual db, we dump
    // items and mob db into json files
    update_item_and_mob_static_db(&mut items, &mobs);

    // Setup script virtual machine for NPC
    let npc_script_vm = create_script_vm();
    let item_script_vm = npc_script_vm.clone();
    let scripts = load_scripts();

    // Loading configs
    let skills_config = Config::load_skills_config(".").unwrap();
    let job_configs = Config::load_jobs_config(".").unwrap();
    let job_skills_tree = Config::load_jobs_skill_tree(".").unwrap();
    // Loading map-cache and warps
    let start = Instant::now();
    let warps = unsafe { WarpLoader::load_warps(CONFIGS.as_ref().unwrap()).await };
    let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
    let mob_spawns = unsafe {
        MobSpawnLoader::load_mob_spawns(CONFIGS.as_ref().unwrap(), mobs_map, MOB_ROOT_PATH, runtime.clone())
            .join()
            .unwrap()
    };
    let mut maps = MapLoader::load_maps(warps, mob_spawns, scripts, &mut map_item_ids, unsafe { MAP_DIR });
    let map_flags = ScriptLoader::load_map_flags(&configs().scripting.map_flags_path).expect("Failed to load map flags");
    for (name, flags) in map_flags {
        if let Some(map) = maps.get_mut(&name) { map.set_flags(flags); }
    }
    info!("Loaded {} map-cache in {}ms", maps.len(), start.elapsed().as_millis());
    // Executing items' script and cache result when possible (e.g: script like
    // `bonus bStr, 3;` result will be cached and item will have a bonus +3 str
    // associated)
    let start = Instant::now();
    let item_script_executed = ItemService::load_item_scripts(&mut items, item_script_vm.clone(), &configs().scripting.items_path);
    info!(
        "Executed and cached {} item scripts, skipped {} item scripts (requiring runtime data) in {}ms",
        item_script_executed.0,
        item_script_executed.1,
        start.elapsed().as_millis()
    );

    // Creating global config instance, used by all services
    unsafe {
        GlobalConfigService::init(
            CONFIGS.clone().unwrap(),
            items,
            mobs,
            job_configs,
            job_skills_tree,
            skills_config,
            maps,
        );
    }
    // Init channel for inter-thread communication
    let (client_notification_sender, single_client_notification_receiver) = std::sync::mpsc::sync_channel::<Notification>(2048);
    let (persistence_event_sender, persistence_event_receiver) = std::sync::mpsc::sync_channel::<PersistenceEvent>(2048);
    // Create server
    let server = Server::new(
        configs(),
        repository_arc.clone(),
        map_item_ids,
        npc_script_vm,
        item_script_vm,
        client_notification_sender.clone(),
        persistence_event_sender.clone(),
        runtime,
    );
    let server_ref = Arc::new(server);
    let server_ref_clone = server_ref;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // Create proxies for other emulator (rathena/hercules). TODO: add a
    // configuration to disable this.
    let char_proxy = CharProxy::new(&configs().proxy);
    let map_proxy = MapProxy::new(&configs().proxy);
    let _ = &handles.push(char_proxy.proxy(configs().server.packetver));
    let _ = &handles.push(map_proxy.proxy(configs().server.packetver));

    if configs().server.enable_visual_debugger {
        #[cfg(feature = "visual_debugger")]
        {
            crate::debugger::visual_debugger::VisualDebugger::run(server_ref_clone.clone()).await;
        }
        #[cfg(not(feature = "visual_debugger"))]
        {
            warn!(
                "Visual debugger has been enable in configuration, but feature has not been compiled. Please consider enabling \
                 \"visual-debugger\" feature."
            );
        }
    }
    info!("Server started in {}ms", _start.elapsed().as_millis());
    Server::start(
        server_ref_clone,
        client_notification_sender,
        single_client_notification_receiver,
        persistence_event_receiver,
        persistence_event_sender,
        true,
    );
    map_proxy.shutdown();
    char_proxy.shutdown();
}

fn update_item_and_mob_static_db(items: &mut Vec<ItemModel>, mobs: &Vec<MobModel>) {
    #[cfg(feature = "static_db_update")]
    {
        // items.json is used in tests
        let item_db: ItemModels = items.clone().into();
        let json = serde_json::to_string_pretty(&item_db).unwrap();
        let output_path = Path::new("config");
        let mut file = File::create(output_path.join("items.json")).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        // mobs.json is used in tests
        let mob_db: MobModels = mobs.clone().into();
        let json = serde_json::to_string_pretty(&mob_db).unwrap();
        let output_path = Path::new("config");
        let mut file = File::create(output_path.join("mobs.json")).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

fn setup_logger(config: &'static Config) {
    let filter = EnvFilter::builder()
        .with_default_directive(config.server.log_level.as_ref().unwrap().to_lowercase().parse().unwrap())
        .parse_lossy(config.server.log_level_module_override.join(",").trim());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .compact()
        .with_line_number(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_thread_names(true)
        .with_target(true)
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
        .init();
}

pub fn load_scripts() -> HashMap<String, Vec<Script>> {
    ScriptLoader::load_scripts(&configs().scripting.npcs_path).expect("Failed to load Wasm NPC manifest")
}

pub fn configs() -> &'static Config {
    unsafe { CONFIGS.as_ref().unwrap() }
}

pub fn create_script_vm() -> Arc<script_runtime::WasmRuntime> {
    script_runtime::WasmRuntime::from_file(&configs().scripting.module_path).expect("Failed to load compiled game scripts")
}
