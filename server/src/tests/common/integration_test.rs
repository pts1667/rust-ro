use std::sync::{Arc, Once, OnceLock};
use std::{fs, thread};

use configuration::configuration::DatabaseConfig;
use models::status::KnownSkill;
use tokio::runtime::Runtime;

use crate::MAP_DIR;
use crate::repository::SledRepository;
use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::server::Server;
use crate::server::boot::map_loader::MapLoader;
use crate::server::boot::mob_spawn_loader::MobSpawnLoader;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::{CharacterJoinGame, GameEvent};
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::map::Map;
use crate::server::model::map_item::MapItems;
use crate::server::model::status::StatusFromDb;
use crate::server::state::character::Character;
use crate::tests::common;
use crate::tests::common::{CONFIGS, create_mpsc};

static INIT: Once = Once::new();
pub static SERVER: OnceLock<Arc<Server>> = OnceLock::new();

pub async fn before_all() -> Arc<Server> {
    INIT.call_once(|| unsafe {
        common::before_all();
        MAP_DIR = "../config/maps/pre-re";
        let runtime = Arc::new(Runtime::new().unwrap());

        let npc_script_vm = crate::tests::common::test_script_vm();
        let item_script_vm = crate::tests::common::test_script_vm();

        let database_config = DatabaseConfig {
            items_path: "../config/items.json".into(),
            mobs_path: "../config/mobs.json".into(),
            seed_path: Some("../db/seed.json".into()),
            ..DatabaseConfig::default()
        };
        let repository = SledRepository::temporary().unwrap();
        repository.seed_assets(&database_config).unwrap();
        let repository_arc = Arc::new(repository);
        let mut map_item_ids = MapItems::default();

        let mob_models = serde_json::from_str::<MobModels>(&fs::read_to_string("../config/mobs.json").unwrap());
        let mobs: Vec<MobModel> = mob_models.unwrap().into();

        let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
        let mob_spawns = {
            MobSpawnLoader::load_mob_spawns(CONFIGS.get().unwrap(), mobs_map, "../config/npc", runtime.clone())
                .join()
                .unwrap()
        };
        let maps = MapLoader::load_maps(
            Default::default(),
            mob_spawns,
            Default::default(),
            &mut map_item_ids,
            "../config/maps/pre-re",
        );
        crate::GlobalConfigService::instance_mut().maps = maps;
        let (_not_use_sender, not_use_receiver) = create_mpsc::<Notification>();
        let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
        let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
        let server = Server::new(
            CONFIGS.get().unwrap(),
            repository_arc.clone(),
            map_item_ids,
            npc_script_vm,
            item_script_vm,
            client_notification_sender.clone(),
            persistence_event_sender.clone(),
            runtime,
        );
        let server = SERVER.get_or_init(|| Arc::new(server)).clone();
        thread::spawn(move || {
            info!("Starting server");
            Server::start(
                server,
                client_notification_sender,
                not_use_receiver,
                persistence_event_receiver,
                persistence_event_sender,
                false,
            );
        });
        thread::Builder::new()
            .name("client_notification_thread".to_string())
            .spawn(move || {
                for _notification in client_notification_receiver.iter() {
                    // println!("Sent client notification {:?}", notification);
                }
            })
            .unwrap();
    });
    server()
}

pub fn server() -> Arc<Server> {
    SERVER.get().unwrap().clone()
}

pub async fn character_join_game() -> u32 {
    let server = server();
    let char_model: CharSelectModel = server.repository.character_fetch(2000000, 0).await.unwrap();
    let char_id = char_model.char_id as u32;
    let skills: Vec<KnownSkill> = server.repository.character_skills(char_id).await.unwrap();
    let mut character = Character::new(
        char_model.name.clone(),
        char_id,
        char_model.account_id as u32,
        StatusFromDb::from_char_model(&char_model, &server.configuration.game, skills),
        char_model.last_x as u16,
        char_model.last_y as u16,
        0,
        "prt_fild09".into(),
        1,
        vec![],
    );
    character.loaded_from_client_side = true;
    let mut state = server.state_mut();
    state.insert_character(character);
    let (map_name, x, y) = {
        let character = state.get_character_unsafe(char_id);
        (Map::name_without_ext(character.current_map_name()), character.x(), character.y())
    };
    server.add_to_next_tick(GameEvent::CharacterJoinGame(CharacterJoinGame { char_id }));
    server
        .server_service()
        .schedule_warp_to_walkable_cell(&mut state, &map_name, x, y, char_id);
    char_id
}
