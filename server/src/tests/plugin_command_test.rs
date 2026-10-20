//! `@` commands of plugins with the `lucky_charms` example loaded: the permission groups know them by level, a player's
//! command runs the plugin and answers in the chat window, and a bot's command answers with the lines it said.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use configuration::configuration::{PluginSetting, ScriptingConfig};

use super::{ServerServiceTestContext, before_each_with_npc_vm};
use crate::repository::SledRepository;
use crate::server::Server;
use crate::server::bots::{BotCommand, BotRequest};
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEventHandler;
use crate::server::model::permission_groups::{CommandKind, PermissionGroups};
use crate::server::model::session::Session;
use crate::server::plugins::Plugins;
use crate::server::request_handler::atcommand::handle_atcommand;
use crate::server::script::ScriptVm;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::plugin_command_service::register_plugin_commands;
use crate::tests::common::TestContext;
use crate::tests::common::character_helper::create_character;

const PLAYER: u32 = 150_200;
const ACCOUNT: u32 = 2_150_200;

fn running_server() -> (Arc<Server>, TestContext) {
    let plugins = Plugins::load(&ScriptingConfig {
        plugins_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/../config/plugins").into(),
        plugins: vec![PluginSetting::Name("lucky_charms".into())],
        ..ScriptingConfig::default()
    })
    .unwrap();
    let ServerServiceTestContext { server, test_context, .. } =
        before_each_with_npc_vm(0, Arc::new(SledRepository::temporary().unwrap()), true, Arc::new(ScriptVm::with_plugins(HashMap::new(), plugins)));
    let server = Arc::new(server);
    server.bind_shared();
    let mut groups = PermissionGroups::from_json(include_str!("../../../config/groups.json")).unwrap();
    register_plugin_commands(server.plugins(), &mut groups);
    server.state_mut().set_permission_groups(groups);
    let mut character = create_character();
    character.char_id = PLAYER;
    character.account_id = ACCOUNT;
    server.state_mut().insert_character(character);
    let session = Session::create_empty(ACCOUNT, 42, 0, GlobalConfigService::instance().packetver()).recreate_with_character(PLAYER);
    server.sessions().add(ACCOUNT, Arc::new(session));
    (server, test_context)
}

/// Runs game loop iterations, which answer the plugin's calls, until `done` holds or two seconds pass.
fn run_until(server: &Server, mut done: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut tick = 0;
    while Instant::now() < deadline {
        tick += 40;
        Server::game_loop_iteration(server, tick);
        if done() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    false
}

fn chat_lines(test_context: &TestContext) -> Vec<String> {
    test_context
        .received_notification()
        .lock()
        .unwrap()
        .iter()
        .filter_map(|notification| match notification {
            Notification::Char(notification) if notification.char_id() == PLAYER => Some(String::from_utf8_lossy(notification.serialized_packet()).into_owned()),
            _ => None,
        })
        .collect()
}

#[test]
fn plugin_commands_are_open_to_the_groups_of_their_level() {
    let (server, _) = running_server();
    let state = server.state();
    let groups = state.permission_groups();
    assert!(groups.can_use_command(0, "lucky", CommandKind::At));
    assert!(groups.can_use_command(0, "LUCKY", CommandKind::At), "typed in any case");
    assert!(!groups.can_use_command(0, "luckytickets", CommandKind::At), "level 60");
    assert!(!groups.can_use_command(0, "lucky", CommandKind::Char), "plugins add @ commands only");
    assert!(groups.usable_commands(0, CommandKind::At).contains(&"lucky".to_string()));
}

#[test]
fn a_player_runs_a_plugin_command_and_reads_its_answer() {
    let (server, test_context) = running_server();
    handle_atcommand(&server, &mut server.state_mut(), PLAYER, "Lucky : @lucky");
    assert!(
        run_until(&server, || chat_lines(&test_context).iter().any(|line| line.contains("You hold 0 Lucky Tickets."))),
        "{:?}",
        chat_lines(&test_context)
    );
    handle_atcommand(&server, &mut server.state_mut(), PLAYER, "Lucky : @luckytickets 3");
    assert!(run_until(&server, || chat_lines(&test_context).iter().any(|line| line.contains("luckytickets is an Unknown Command"))), "level 0 may not");
}

#[test]
fn a_bot_command_of_a_plugin_answers_with_what_the_plugin_said() {
    let (server, _) = running_server();
    let (command, mut receiver) = BotCommand::new(PLAYER, ACCOUNT, BotRequest::Command("@lucky".into()));
    command.handle(&server, &mut server.state_mut(), 0).unwrap();
    let answer = Arc::new(Mutex::new(None));
    assert!(run_until(&server, || {
        if let Ok(result) = receiver.try_recv() {
            *answer.lock().unwrap() = Some(result);
        }
        answer.lock().unwrap().is_some()
    }));
    let answer = answer.lock().unwrap().take().unwrap().unwrap();
    assert_eq!(answer, serde_json::json!({ "command": "lucky", "replies": ["You hold 0 Lucky Tickets."] }));
}
