//! Plugin windows on the server side, with the `ui_shop` example loaded: script calls store windows and send them only to
//! clients that draw plugin UI, events are checked against the window shown, overheads reach the players around the unit,
//! and bots observe the windows, the data and the actions.
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use configuration::configuration::{PluginSetting, ScriptingConfig};
use plugin_ui::wire::{self, FEATURE_PLUGIN_UI, ServerMessage};
use plugin_ui::{Node, Overhead, Placement, UiEvent, Window};
use script_sdk::{Function, Reply, Request, Value};
use tokio::sync::oneshot;

use super::{ServerServiceTestContext, before_each_with_npc_vm};
use crate::server::model::events::client_notification::Notification;
use crate::server::model::session::Session;
use crate::server::plugins::Plugins;
use crate::server::script::{ScriptRequest, ScriptVm};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::tests::common::character_helper::create_character;
use crate::tests::common::mocked_repository::MockedRepository;

const BUYER: u32 = 150_100;
const NEIGHBOUR: u32 = 150_101;
const FAR_AWAY: u32 = 150_102;
const PLUGIN: &str = "ui_shop";

fn account(char_id: u32) -> u32 {
    char_id + 2_000_000
}

fn context() -> ServerServiceTestContext {
    let plugins = Plugins::load(&ScriptingConfig {
        plugins_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/../config/plugins").into(),
        plugins: vec![PluginSetting::Name(PLUGIN.into())],
        ..ScriptingConfig::default()
    })
    .unwrap();
    let context = before_each_with_npc_vm(0, Arc::new(MockedRepository), true, Arc::new(ScriptVm::with_plugins(HashMap::new(), plugins)));
    for (char_id, x) in [(BUYER, 150), (NEIGHBOUR, 155), (FAR_AWAY, 190)] {
        let mut character = create_character();
        character.char_id = char_id;
        character.account_id = account(char_id);
        character.x = x;
        character.y = 150;
        context.server.state_mut().insert_character(character);
        let session = Session::create_empty(account(char_id), 42, 0, GlobalConfigService::instance().packetver()).recreate_with_character(char_id);
        context.server.sessions().add(account(char_id), Arc::new(session));
    }
    context
}

/// A call of the plugin as the runtime hands it to the host, the plugin's name appended.
fn call(context: &ServerServiceTestContext, function: Function, mut arguments: Vec<Value>) -> Reply {
    arguments.push(PLUGIN.into());
    let session = context.server.sessions().find(account(BUYER)).unwrap();
    let (sender, mut receiver) = oneshot::channel();
    let request = ScriptRequest {
        char_id: BUYER,
        account_id: account(BUYER),
        npc_id: 0,
        npc_entry: 0,
        npc_scope_instance: 0,
        map_instance: 0,
        generation: session.script_generation.load(Ordering::Acquire),
        background: true,
        event_depth: 0,
        timer_context: None,
        logout_token: None,
        request: Request::Call { function, arguments },
        response: Arc::new(Mutex::new(Some(sender))),
    };
    context.server.script_service().handle_request(&context.server, &mut context.server.state_mut(), request);
    receiver.try_recv().expect("the game loop answers at once")
}

fn shop() -> Window {
    Window::new("shop", "Shop", 200, Node::column([Node::image(plugin_ui::ImageRef::plugin("coin_0.png"), 24, 24), Node::button("buy:501", "Buy"), Node::button("off", "Off").disabled()]))
}

fn open(context: &ServerServiceTestContext, window: &Window) -> Reply {
    call(context, Function::UiOpen, vec![serde_json::to_string(window).unwrap().into()])
}

/// Plugin UI packets sent so far, by recipient, waiting a little for the notification thread.
fn plugin_packets(context: &ServerServiceTestContext, expected: usize) -> Vec<(u32, ServerMessage)> {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let packets: Vec<_> = context
            .test_context
            .received_notification()
            .lock()
            .unwrap()
            .iter()
            .filter_map(|notification| match notification {
                Notification::Char(notification) => wire::decode_server(notification.serialized_packet()).ok().map(|message| (notification.char_id(), message)),
                _ => None,
            })
            .collect();
        if packets.len() >= expected || Instant::now() > deadline {
            return packets;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn a_window_is_stored_with_its_assets_resolved_and_sent_only_to_a_client_that_draws_it() {
    let context = context();
    open(&context, &shop()).unwrap();
    let stored = context.server.plugin_ui().window(BUYER, PLUGIN, "shop").unwrap();
    let Node::Column(column) = &stored.node else { panic!("{stored:?}") };
    assert!(matches!(&column.children[0], Node::Image { image, .. } if image.frames == ["plugin/ui_shop/coin_0.png"]));
    std::thread::sleep(Duration::from_millis(50));
    assert!(plugin_packets(&context, 0).is_empty(), "the client never said it draws plugin UI");

    context.server.plugin_ui().set_client_features(BUYER, FEATURE_PLUGIN_UI);
    open(&context, &shop()).unwrap();
    call(&context, Function::UiClose, vec!["shop".into()]).unwrap();
    let packets = plugin_packets(&context, 2);
    assert!(matches!(&packets[0], (BUYER, ServerMessage::Open(open)) if open.plugin == PLUGIN && open.window == stored), "{packets:?}");
    assert!(matches!(&packets[1], (BUYER, ServerMessage::Close(close)) if close.id == "shop"));
    assert!(context.server.plugin_ui().window(BUYER, PLUGIN, "shop").is_none());
}

#[test]
fn a_window_above_a_player_names_their_account_and_goes_when_the_map_changes() {
    let context = context();
    context.server.plugin_ui().set_client_features(BUYER, FEATURE_PLUGIN_UI);
    open(&context, &shop().anchored(NEIGHBOUR, 4)).unwrap();
    let stored = context.server.plugin_ui().window(BUYER, PLUGIN, "shop").unwrap();
    assert_eq!(stored.placement, Placement::Anchored { unit_id: account(NEIGHBOUR), offset_y: 4 }, "the client knows players by account");
    assert!(!stored.closable);
    let ghost = Window::new("ghost", "Ghost", 100, Node::text("Boo")).anchored(999_999, 0);
    assert!(open(&context, &ghost).unwrap_err().contains("not in the game"));
    open(&context, &Window::new("floating", "Stays", 100, Node::text("x"))).unwrap();

    context.server.close_anchored_plugin_windows(BUYER);
    let packets = plugin_packets(&context, 3);
    assert!(matches!(&packets[2], (BUYER, ServerMessage::Close(close)) if close.id == "shop"), "{packets:?}");
    assert!(context.server.plugin_ui().window(BUYER, PLUGIN, "shop").is_none());
    assert!(context.server.plugin_ui().window(BUYER, PLUGIN, "floating").is_some(), "floating windows follow the player");
}

#[test]
fn invalid_windows_and_calls_without_a_plugin_are_refused() {
    let context = context();
    let other = Window::new("w", "W", 200, Node::image(plugin_ui::ImageRef::texture("plugin/double_xp/x.png"), 8, 8));
    assert!(open(&context, &other).unwrap_err().contains("another plugin"));
    assert!(call(&context, Function::UiOpen, vec!["{".into()]).unwrap_err().contains("Invalid window"));
    let mut unowned = vec![serde_json::to_string(&shop()).unwrap().into()];
    unowned.push("not_loaded".into());
    let session = context.server.sessions().find(account(BUYER)).unwrap();
    let (sender, mut receiver) = oneshot::channel();
    let request = ScriptRequest {
        char_id: BUYER,
        account_id: account(BUYER),
        npc_id: 0,
        npc_entry: 0,
        npc_scope_instance: 0,
        map_instance: 0,
        generation: session.script_generation.load(Ordering::Acquire),
        background: true,
        event_depth: 0,
        timer_context: None,
        logout_token: None,
        request: Request::Call { function: Function::UiOpen, arguments: unowned },
        response: Arc::new(Mutex::new(Some(sender))),
    };
    context.server.script_service().handle_request(&context.server, &mut context.server.state_mut(), request);
    assert!(receiver.try_recv().unwrap().unwrap_err().contains("Only plugins"));
}

#[test]
fn an_overhead_reaches_the_players_around_who_draw_plugin_ui() {
    let context = context();
    for char_id in [BUYER, NEIGHBOUR, FAR_AWAY] {
        context.server.plugin_ui().set_client_features(char_id, FEATURE_PLUGIN_UI);
    }
    let overhead = Overhead::image(plugin_ui::ImageRef::plugin("star.png"), 48, 48, 1000).caption("LEVEL 2!", None).rise(20.0, 800);
    call(&context, Function::UiOverhead, vec![0.into(), serde_json::to_string(&overhead).unwrap().into()]).unwrap();
    let packets = plugin_packets(&context, 2);
    let mut recipients: Vec<u32> = packets.iter().map(|(char_id, _)| *char_id).collect();
    recipients.sort();
    assert_eq!(recipients, [BUYER, NEIGHBOUR], "the far player is out of range");
    let ServerMessage::Overhead { unit_id, message } = &packets[0].1 else { panic!("{packets:?}") };
    assert_eq!(*unit_id, account(BUYER), "clients know players by account id");
    assert_eq!(message.overhead.image.as_ref().unwrap().frames, ["plugin/ui_shop/star.png"]);

    let only_me = overhead.only_self();
    call(&context, Function::UiOverhead, vec![0.into(), serde_json::to_string(&only_me).unwrap().into()]).unwrap();
    let packets = plugin_packets(&context, 3);
    assert_eq!(packets.len(), 3);
    assert_eq!(packets[2].0, BUYER);
}

#[test]
fn events_must_match_the_window_shown() {
    let context = context();
    open(&context, &shop()).unwrap();
    let refused = |event: UiEvent| {
        let (sender, mut receiver) = oneshot::channel();
        let reply = Arc::new(std::sync::Mutex::new(Some(sender)));
        context.server.plugin_ui_event(&context.server.state(), BUYER, PLUGIN, event, Some(reply));
        receiver.try_recv().unwrap().unwrap_err()
    };
    assert!(refused(UiEvent::click("shop", "missing")).contains("no widget"));
    assert!(refused(UiEvent::click("shop", "off")).contains("does not take"));
    assert!(refused(UiEvent::click("elsewhere", "buy:501")).contains("not open"));

    let (sender, _receiver) = oneshot::channel();
    context.server.plugin_ui_event(&context.server.state(), BUYER, PLUGIN, UiEvent::closed("shop"), Some(Arc::new(std::sync::Mutex::new(Some(sender)))));
    assert!(context.server.plugin_ui().window(BUYER, PLUGIN, "shop").is_none(), "closing removes the window before its handler runs");
}

#[test]
fn bots_observe_windows_data_and_actions_and_unknown_actions_are_refused() {
    let context = context();
    open(&context, &shop()).unwrap();
    call(&context, Function::BotObserve, vec!["stock".into(), "[501]".into()]).unwrap();
    let view = crate::server::bots::observe(&context.server, &context.server.state(), BUYER, 0).unwrap();
    let json = serde_json::to_value(&view).unwrap();
    let plugin = &json["plugins"][PLUGIN];
    assert_eq!(plugin["windows"][0]["id"], "shop");
    assert_eq!(plugin["data"]["stock"], serde_json::json!([501]));
    assert_eq!(plugin["actions"][0]["name"], "buy");

    call(&context, Function::BotObserve, vec!["stock".into(), "null".into()]).unwrap();
    assert!(context.server.plugin_ui().observations(BUYER).is_empty());

    let (sender, mut receiver) = oneshot::channel();
    context.server.plugin_bot_action(&context.server.state(), BUYER, PLUGIN, "steal".into(), "{}".into(), Arc::new(std::sync::Mutex::new(Some(sender))));
    assert!(receiver.try_recv().unwrap().unwrap_err().contains("no bot action steal"));
}

#[test]
fn leaving_the_game_forgets_everything_of_the_character_and_answers_waiting_bots() {
    let context = context();
    context.server.plugin_ui().set_client_features(BUYER, FEATURE_PLUGIN_UI);
    open(&context, &shop()).unwrap();
    let job = |reply| crate::server::plugins::ui::PluginJob::BotAction { action: script_sdk::BotAction { name: "buy".into(), args: "{}".into() }, reply };
    let (running, _) = oneshot::channel();
    let (waiting, mut answer) = oneshot::channel();
    assert!(matches!(context.server.plugin_ui().enqueue(BUYER, PLUGIN, job(Arc::new(Mutex::new(Some(running))))), Ok(Some(_))));
    assert!(matches!(context.server.plugin_ui().enqueue(BUYER, PLUGIN, job(Arc::new(Mutex::new(Some(waiting))))), Ok(None)));
    context.server.forget_plugin_ui(BUYER);
    assert!(answer.try_recv().unwrap().unwrap_err().contains("left the game"));
    assert!(context.server.plugin_ui().windows(BUYER).is_empty());
    assert!(!context.server.plugin_ui().has_client(BUYER));
}
