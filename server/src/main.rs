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

pub static CONFIGS: std::sync::OnceLock<Config> = std::sync::OnceLock::new();
pub static mut MAPS: Option<HashMap<String, &Map>> = None;
pub static mut MOB_ROOT_PATH: &str = "./config/npc";
pub static mut MAP_DIR: &str = "./config/maps/pre-re";
#[tokio::main]
pub async fn main() {
    let _start = Instant::now();
    let mut config = Config::load("").unwrap();
    let options = parse_arguments(std::env::args().skip(1)).unwrap_or_else(|error| exit_with_usage(&error));
    if let Some(host) = options.host {
        config.server.set_host(host);
        config.server.validate_host().unwrap_or_else(|error| exit_with_usage(&error));
    }
    CONFIGS.set(config).unwrap_or_else(|_| unreachable!("configuration is loaded once"));

    setup_logger(configs(), options.debug_log);
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
    let warps = WarpLoader::load_warps(configs()).await;
    let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
    let mob_spawns = unsafe {
        MobSpawnLoader::load_mob_spawns(configs(), mobs_map, MOB_ROOT_PATH, runtime.clone())
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
    GlobalConfigService::init(
        configs().clone(),
        items,
        mobs,
        job_configs,
        job_skills_tree,
        skills_config,
        maps,
    );
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

    let proxies = configs().server.enable_legacy_proxy.then(|| {
        let char_proxy = CharProxy::new(&configs().proxy, &configs().server);
        let map_proxy = MapProxy::new(&configs().proxy, &configs().server);
        handles.push(char_proxy.proxy(configs().server.packetver));
        handles.push(map_proxy.proxy(configs().server.packetver));
        (char_proxy, map_proxy)
    });


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
    if let Some((char_proxy, map_proxy)) = proxies {
        map_proxy.shutdown();
        char_proxy.shutdown();
    }
}

fn update_item_and_mob_static_db(_items: &mut Vec<ItemModel>, _mobs: &Vec<MobModel>) {
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

fn log_filter(level: &str, module_overrides: &[String], debug_log: bool) -> EnvFilter {
    let mut directives = module_overrides.to_vec();
    if debug_log {
        directives.push("script_debug=debug".to_string());
    }
    EnvFilter::builder()
        .with_default_directive(level.to_lowercase().parse().unwrap())
        .parse_lossy(directives.join(",").trim())
}

fn setup_logger(config: &'static Config, debug_log: bool) {
    let filter = log_filter(config.server.log_level.as_ref().unwrap(), &config.server.log_level_module_override, debug_log);
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

#[derive(Debug, Default, PartialEq, Eq)]
struct CliOptions {
    host: Option<String>,
    debug_log: bool,
}

fn parse_arguments(mut args: impl Iterator<Item = String>) -> Result<CliOptions, String> {
    let mut options = CliOptions::default();
    while let Some(argument) = args.next() {
        if argument == "--host" {
            options.host = Some(args.next().ok_or("--host requires an IP address")?);
        } else if let Some(value) = argument.strip_prefix("--host=") {
            options.host = Some(value.to_string());
        } else if argument == "--debug-log" {
            options.debug_log = true;
        } else {
            return Err(format!("unknown argument \"{argument}\""));
        }
    }
    Ok(options)
}

fn exit_with_usage(error: &str) -> ! {
    eprintln!("{error}
Usage: server [--host <ip>] [--debug-log]");
    std::process::exit(2)
}

pub fn load_scripts() -> HashMap<String, Vec<Script>> {
    ScriptLoader::load_scripts(&configs().scripting.npcs_path).expect("Failed to load Wasm NPC manifest")
}

pub fn configs() -> &'static Config {
    CONFIGS.get().expect("configuration is loaded in main")
}

pub fn create_script_vm() -> Arc<script_runtime::WasmRuntime> {
    script_runtime::WasmRuntime::from_file(&configs().scripting.module_path).expect("Failed to load compiled game scripts")
}

#[cfg(test)]
mod host_argument_tests {
    use super::{CliOptions, parse_arguments};

    fn parse(arguments: &[&str]) -> Result<CliOptions, String> {
        parse_arguments(arguments.iter().map(|argument| argument.to_string()))
    }

    fn host(arguments: &[&str]) -> Option<String> {
        parse(arguments).unwrap().host
    }

    #[test]
    fn host_is_read_from_either_form() {
        assert_eq!(parse(&[]), Ok(CliOptions::default()));
        assert_eq!(host(&["--host", "127.0.0.1"]), Some("127.0.0.1".to_string()));
        assert_eq!(host(&["--host=::1"]), Some("::1".to_string()));
    }

    #[test]
    fn debug_log_flag_combines_with_host() {
        assert!(!parse(&[]).unwrap().debug_log);
        let options = parse(&["--debug-log", "--host", "127.0.0.1"]).unwrap();
        assert!(options.debug_log);
        assert_eq!(options.host, Some("127.0.0.1".to_string()));
    }

    #[test]
    fn missing_value_and_unknown_arguments_are_errors() {
        assert!(parse(&["--host"]).is_err());
        assert!(parse(&["--port", "1"]).is_err());
    }

    #[derive(Clone, Default)]
    struct Captured(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Captured {
        fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn logged(debug_log: bool) -> String {
        let captured = Captured::default();
        let writer = captured.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(super::log_filter("info", &["info".to_string()], debug_log))
            .with_writer(move || writer.clone())
            .finish();
        tracing::subscriber::with_default(subscriber, || {
            script_debug!("npc conversation failed");
            tracing::debug!("unrelated debug line");
            tracing::info!("regular line");
        });
        let bytes = captured.0.lock().unwrap().clone();
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn script_diagnostics_are_only_logged_with_the_debug_log_flag() {
        let without = logged(false);
        assert!(!without.contains("npc conversation failed"), "{without}");
        assert!(without.contains("regular line"));

        let with = logged(true);
        assert!(with.contains("npc conversation failed"), "{with}");
        assert!(!with.contains("unrelated debug line"), "{with}");
    }
}
