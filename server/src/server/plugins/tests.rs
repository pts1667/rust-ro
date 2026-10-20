use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use configuration::configuration::{PluginSetting, ScriptingConfig};
use script_runtime::Host;
use script_sdk::{Capability, Entry, Function, Hook, HookKind, Interception, InterceptorKind, LevelKind, Reply, Request, Value};

use super::*;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::model::script::Script;
use crate::server::service::script_service::ScriptService;

fn plugins_dir() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../config/plugins").into()
}

fn load(plugins: Vec<PluginSetting>) -> Result<Plugins, String> {
    Plugins::load(&ScriptingConfig { plugins_dir: plugins_dir(), plugins, ..ScriptingConfig::default() })
}

fn example() -> Plugins {
    load(vec![PluginSetting::Name("double_xp".into())]).unwrap()
}

fn narrowed(capabilities: &[&str]) -> Result<Plugins, String> {
    load(vec![PluginSetting::Detailed {
        name: "double_xp".into(),
        enabled: true,
        capabilities: Some(capabilities.iter().map(|name| name.to_string()).collect()),
    }])
}

/// A folder of its own under the temp directory holding the example's module, for refusals that need a modified package.
fn copied_example(folder: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("rust-ro-plugins-{}-{folder}", std::process::id()));
    let package = root.join(folder);
    std::fs::create_dir_all(&package).unwrap();
    std::fs::copy(std::path::Path::new(&plugins_dir()).join("double_xp/plugin.wasm"), package.join("plugin.wasm")).unwrap();
    root
}

#[test]
fn the_example_plugin_loads_with_what_it_declares() {
    let plugins = example();
    let plugin = plugins.iter().next().unwrap();
    assert_eq!((plugin.name(), plugin.info.version.as_str()), ("double_xp", "1.0.0"));
    assert_eq!(plugin.capabilities.iter().collect::<Vec<_>>(), [Capability::Dialogue, Capability::Rates, Capability::Broadcast]);
    assert_eq!(plugin.module, "plugin:double_xp");
    assert_eq!(plugins.observers(HookKind::MobKilled).count(), 1);
    assert_eq!(plugins.observers(HookKind::Logout).count(), 0);
    assert!(plugins.intercepted(InterceptorKind::ExpGain));
    assert!(!plugins.intercepted(InterceptorKind::Chat));
    assert_eq!(plugin.npcs[0].name, "Rate Clerk");
    assert!(plugin.events.contains_key("Rate Clerk::OnMinute00"));
    assert_eq!(plugin.info.manifest.title, "Double XP");
    assert_eq!(serde_json::Value::Object(plugin.settings.clone()), serde_json::json!({ "exp_percent": 200, "drop_percent": 150, "milestone": 100 }));
}

/// The example's module as `double_xp` under a root of its own, so tests that write settings next to it do not race.
fn copied_double_xp(test: &str) -> std::path::PathBuf {
    let root = copied_example(test);
    std::fs::rename(root.join(test), root.join("double_xp")).unwrap();
    root
}

fn load_copied(root: &std::path::Path) -> Result<Plugins, String> {
    Plugins::load(&ScriptingConfig {
        plugins_dir: root.to_string_lossy().into(),
        plugins: vec![PluginSetting::Name("double_xp".into())],
        ..ScriptingConfig::default()
    })
}

#[test]
fn the_operator_file_overrides_the_defaults_of_info_json() {
    let root = copied_double_xp("overridden");
    std::fs::write(root.join("double_xp.json"), r#"{ "exp_percent": 300 }"#).unwrap();
    let plugins = load_copied(&root).unwrap();
    let plugin = plugins.iter().next().unwrap();
    assert_eq!(serde_json::Value::Object(plugin.settings.clone()), serde_json::json!({ "exp_percent": 300, "drop_percent": 150, "milestone": 100 }));
    let gained = plugins.intercept(Interception::ExpGain { char_id: 1, mob_id: 1002, base: 10, job: 7 }).unwrap();
    assert_eq!(gained, Interception::ExpGain { char_id: 1, mob_id: 1002, base: 30, job: 21 }, "the guest reads the override");
}

#[test]
fn overrides_the_plugin_does_not_declare_or_of_another_type_are_refused() {
    let root = copied_double_xp("misconfigured");
    for (overrides, reason) in [
        (r#"{ "exp_procent": 300 }"#, "exp_procent is not a setting"),
        (r#"{ "exp_percent": "300" }"#, "exp_percent must be a number"),
        ("{ not json", "double_xp.json is invalid"),
    ] {
        std::fs::write(root.join("double_xp.json"), overrides).unwrap();
        let error = load_copied(&root).err().unwrap();
        assert!(error.contains(reason), "{error}");
    }
}

#[test]
fn plugins_load_only_when_listed_and_enabled() {
    assert!(load(vec![]).unwrap().is_empty());
    let disabled = PluginSetting::Detailed { name: "double_xp".into(), enabled: false, capabilities: None };
    assert!(load(vec![disabled]).unwrap().is_empty());
    let twice = load(vec![PluginSetting::Name("double_xp".into()), PluginSetting::Name("double_xp".into())]);
    assert!(twice.err().unwrap().contains("listed twice"));
}

#[test]
fn the_configuration_narrows_capabilities_but_never_widens_them() {
    let plugins = narrowed(&["Rates", "World"]).unwrap();
    assert_eq!(plugins.iter().next().unwrap().capabilities.iter().collect::<Vec<_>>(), [Capability::Rates], "World was never declared");
    assert!(narrowed(&["Teleport"]).err().unwrap().contains("unknown capability Teleport"));
}

#[test]
fn a_plugin_is_refused_when_its_package_does_not_hold_together() {
    let root = copied_example("renamed");
    let error = Plugins::load(&ScriptingConfig {
        plugins_dir: root.to_string_lossy().into(),
        plugins: vec![PluginSetting::Name("renamed".into())],
        ..ScriptingConfig::default()
    })
    .err()
    .unwrap();
    assert!(error.contains("calls itself double_xp"), "{error}");

    let root = copied_double_xp("leftover");
    std::fs::write(root.join("double_xp/config.json"), "{}").unwrap();
    let error = load_copied(&root).err().unwrap();
    assert!(error.contains("settings moved") && error.contains("double_xp.json"), "a settings file left in the package is not ignored: {error}");

    let missing = load(vec![PluginSetting::Name("not_installed".into())]).err().unwrap();
    assert!(missing.contains("not_installed"), "{missing}");
}

#[test]
fn the_example_scales_monster_experience_by_its_configuration() {
    let plugins = example();
    let gained = plugins.intercept(Interception::ExpGain { char_id: 1, mob_id: 1002, base: 10, job: 7 }).unwrap();
    assert_eq!(gained, Interception::ExpGain { char_id: 1, mob_id: 1002, base: 20, job: 14 }, "exp_percent is 200 in info.json");
    let chat = Interception::Chat { char_id: 1, map: "prontera".into(), message: "hi".into() };
    assert_eq!(plugins.intercept(chat.clone()), Ok(chat), "kinds no plugin intercepts are not sent to the guests");
}

/// The ADR lands an interceptor only with a measure of its cost: one call per kill and recipient must stay far below the 20 ms
/// of a game loop iteration.
#[test]
fn an_interceptor_call_costs_microseconds() {
    let plugins = example();
    let decision = Interception::ExpGain { char_id: 1, mob_id: 1002, base: 1000, job: 700 };
    plugins.intercept(decision.clone()).unwrap();
    let mut timings: Vec<Duration> = (0..2000)
        .map(|_| {
            let started = Instant::now();
            plugins.intercept(decision.clone()).unwrap();
            started.elapsed()
        })
        .collect();
    timings.sort();
    let mean = timings.iter().sum::<Duration>() / timings.len() as u32;
    let p99 = timings[timings.len() * 99 / 100];
    println!("ExpGain interceptor: mean {} µs, p99 {} µs, max {} µs", mean.as_micros(), p99.as_micros(), timings.last().unwrap().as_micros());
    let plugin = plugins.iter().next().unwrap();
    assert!(plugin.intercepts(InterceptorKind::ExpGain), "no call overran often enough to turn the interceptor off");
    assert!(mean < plugins.deadline, "mean {mean:?} is over the {:?} deadline", plugins.deadline);
}

#[derive(Clone, Default)]
struct Recorder {
    requests: Arc<Mutex<Vec<Request>>>,
}

#[async_trait]
impl Host for Recorder {
    async fn invoke(&mut self, request: Request) -> Reply {
        let reply = match &request {
            Request::Read(name) if name == "double_xp_kills" => Ok(Value::Number(99)),
            Request::Call { function: Function::GetRate, .. } => Ok(Value::Number(150)),
            _ => Ok(Value::default()),
        };
        self.requests.lock().unwrap().push(request);
        reply
    }
}

impl Recorder {
    fn requests(&self) -> Vec<Request> {
        self.requests.lock().unwrap().clone()
    }
}

async fn run(plugins: &Plugins, entry: Entry) -> (Vec<Request>, Result<(), String>) {
    let recorder = Recorder::default();
    let (_, result) = plugins.iter().next().unwrap().runtime.execute_named(recorder.clone(), &entry).await;
    (recorder.requests(), result)
}

#[tokio::test]
async fn the_initialisation_scales_drops_under_the_plugin_name_and_announces() {
    let (requests, result) = run(&example(), Entry::PluginInit).await;
    result.unwrap();
    assert!(requests.contains(&Request::Call { function: Function::SetRate, arguments: vec!["Drop".into(), 150.into(), "double_xp".into()] }));
    assert!(requests.contains(&Request::Write { name: "$double_xp::started".into(), value: Value::Number(1) }), "{requests:?}");
    assert!(requests.iter().any(|request| matches!(request, Request::Call { function: Function::Announce, .. })));
}

#[tokio::test]
async fn a_capability_the_configuration_withholds_is_refused() {
    let (requests, result) = run(&narrowed(&["Rates"]).unwrap(), Entry::PluginInit).await;
    let error = result.unwrap_err();
    assert!(error.contains("Broadcast") || error.contains("error code"), "{error}");
    assert!(!requests.iter().any(|request| matches!(request, Request::Call { function: Function::Announce, .. })), "the host never saw it");
    assert!(requests.iter().any(|request| matches!(request, Request::ReportError(error) if error.contains("Broadcast"))), "{requests:?}");
}

#[tokio::test]
async fn the_kill_hook_counts_in_a_shared_character_variable() {
    let hook = Hook::MobKilled { char_id: 7, mob_id: 1002, map: "prt_fild08".into(), x: 10, y: 10 };
    let (requests, result) = run(&example(), Entry::Hook(hook)).await;
    result.unwrap();
    assert!(requests.contains(&Request::Write { name: "double_xp_kills".into(), value: Value::Number(100) }), "{requests:?}");
    assert!(requests.iter().any(|request| matches!(request, Request::Call { function: Function::Message, arguments }
        if arguments[0].text().starts_with("100 monsters"))));
    let (_, unobserved) = run(&example(), Entry::Hook(Hook::Logout { char_id: 7, account_id: 7 })).await;
    assert!(unobserved.is_err(), "a hook the plugin did not declare has no observer");
}

fn placed(name: &str, map: &str) -> Script {
    Script {
        appearance: None, speed: None,
        id: 0,
        scope_instance: 0,
        map_name: map.into(),
        name: name.into(),
        sprite: 111,
        x: 5,
        y: 6,
        dir: 2,
        x_size: 0,
        y_size: 0,
        entry_id: 0,
        constructor_args: vec![],
    }
}

fn npc(name: &str, replaces: bool) -> PluginNpc {
    PluginNpc { name: name.into(), replaces, ..Default::default() }
}

#[test]
fn plugin_npcs_are_placed_after_the_base_ones_and_replace_them_only_when_asked() {
    let mut plugins = example();
    let mut scripts = HashMap::from([("prontera".to_string(), vec![placed("Guard", "prontera")])]);
    ScriptLoader::place_plugin_npcs(&mut scripts, &plugins, false).unwrap();
    let clerk = scripts["prontera"].iter().find(|script| script.name == "Rate Clerk").unwrap();
    let entry = crate::server::script::entries::resolve(clerk.entry_id).unwrap();
    assert_eq!((entry.module.as_str(), entry.entry), ("plugin:double_xp", Entry::Npc("Rate Clerk".into())));
    assert_eq!((clerk.x, clerk.y), (161, 186));

    plugins.list[0].npcs = vec![npc("Guard", false)];
    let mut scripts = HashMap::from([("prontera".to_string(), vec![placed("Guard", "prontera")])]);
    let error = ScriptLoader::place_plugin_npcs(&mut scripts, &plugins, false).unwrap_err();
    assert!(error.contains("replaces"), "{error}");

    plugins.list[0].npcs = vec![npc("Guard", true)];
    let mut scripts = HashMap::from([("prontera".to_string(), vec![placed("Guard", "prontera")])]);
    ScriptLoader::place_plugin_npcs(&mut scripts, &plugins, false).unwrap();
    let [guard] = scripts["prontera"].as_slice() else { panic!("{:?}", scripts["prontera"].len()) };
    assert_eq!((guard.x, guard.y, guard.sprite), (5, 6, 111), "the replacement keeps the place it leaves out");
    assert_eq!(crate::server::script::entries::resolve(guard.entry_id).unwrap().module, "plugin:double_xp");

    plugins.list[0].npcs = vec![npc("Nobody", false)];
    assert!(ScriptLoader::place_plugin_npcs(&mut HashMap::new(), &plugins, false).unwrap_err().contains("map_name is required"));
}

#[test]
fn a_plugin_npc_appearance_and_speed_reach_the_drawn_npc() {
    use crate::server::script::skill::actor::NpcSkillState;
    use models::enums::look::LookType;
    use models::enums::EnumWithNumberValue;
    let mut plugins = example();
    let appearance = script_sdk::NpcAppearance { sex: 1, hair: 7, hair_color: 3, head_top: 5, ..Default::default() };
    plugins.list[0].npcs = vec![PluginNpc {
        name: "Townsfolk#1".into(),
        map_name: Some("prontera".into()),
        sprite: Some("4001".into()),
        x: Some(150),
        y: Some(150),
        appearance: Some(appearance),
        speed: Some(160),
        ..Default::default()
    }];
    let mut scripts = HashMap::new();
    ScriptLoader::place_plugin_npcs(&mut scripts, &plugins, false).unwrap();
    let npc = NpcSkillState::uninitialized(&scripts["prontera"][0]);
    assert_eq!((npc.sprite, npc.sex, npc.speed, npc.snapshot().speed()), (4001, 1, 160, 160));
    let look = |kind: LookType| npc.looks[kind.value() as usize];
    assert_eq!((look(LookType::Hair), look(LookType::HairColor), look(LookType::HeadTop)), (7, 3, 5));
}

/// `alive_towns` under a root of its own with `overrides` as the operator's settings.
fn alive_towns(test: &str, overrides: &str) -> Result<Plugins, String> {
    let root = std::env::temp_dir().join(format!("rust-ro-plugins-{}-{test}", std::process::id()));
    let package = root.join("alive_towns");
    std::fs::create_dir_all(&package).unwrap();
    for file in ["plugin.wasm", "events.json"] {
        std::fs::copy(std::path::Path::new(&plugins_dir()).join("alive_towns").join(file), package.join(file)).unwrap();
    }
    std::fs::write(root.join("alive_towns.json"), overrides).unwrap();
    Plugins::load(&ScriptingConfig {
        plugins_dir: root.to_string_lossy().into(),
        plugins: vec![PluginSetting::Name("alive_towns".into())],
        ..ScriptingConfig::default()
    })
}

#[test]
fn alive_towns_generates_its_townsfolk_from_the_settings() {
    let plugins = alive_towns("alive", r#"{ "towns": ["prontera"], "npcs_per_town": 5, "guards_per_town": 1 }"#).unwrap();
    let plugin = plugins.iter().next().unwrap();
    let [controller, townsfolk @ ..] = plugin.npcs.as_slice() else { panic!("no NPCs") };
    assert!(controller.name.starts_with("Alive Towns_") && controller.map_name.as_deref() == Some("prontera"), "one controller per town");
    assert_eq!(townsfolk.len(), 5);
    assert!(townsfolk.iter().all(|npc| npc.map_name.as_deref() == Some("prontera") && npc.entry.as_deref() == Some("townsfolk") && npc.speed.is_some()));
    assert!(townsfolk[0].name.starts_with("Guard#at"));
    assert!(plugin.events.contains_key("Alive Towns::OnTimer1000") && plugin.events.contains_key("Alive Towns::OnDoor"));
    assert_eq!(plugins.observers(HookKind::MapEnter).count(), 1);
    let mut scripts = HashMap::new();
    ScriptLoader::place_plugin_npcs(&mut scripts, &plugins, false).unwrap();
    assert_eq!(scripts["prontera"].len(), 6);

    let everyone = alive_towns("alive_default", "{}").unwrap();
    assert_eq!(everyone.iter().next().unwrap().npcs.len(), 22 * (1 + 100), "a guest of 1 MiB generates them page by page");

    let error = alive_towns("alive_unknown", r#"{ "towns": ["prt_fild08"] }"#).err().unwrap();
    assert!(error.contains("prt_fild08 is not a town"), "{error}");
}

#[test]
fn plugin_events_take_a_base_label_only_when_they_replace_it() {
    let mut plugins = example();
    let handles = ScriptService::plugin_event_handles(&plugins, HashSet::new()).unwrap();
    assert_eq!(handles.len(), 1);
    let entry = crate::server::script::entries::resolve(handles[0].1).unwrap();
    assert_eq!(entry.entry, Entry::Event("Rate Clerk::OnMinute00".into()));

    let base = HashSet::from(["Rate Clerk::OnMinute00".to_string()]);
    assert!(ScriptService::plugin_event_handles(&plugins, base.clone()).unwrap_err().contains("already defined"));
    plugins.list[0].events.get_mut("Rate Clerk::OnMinute00").unwrap().replaces = true;
    assert!(ScriptService::plugin_event_handles(&plugins, base).is_ok());
}

fn ui_shop() -> Plugins {
    load(vec![PluginSetting::Name("ui_shop".into())]).unwrap()
}

/// The host of a player with `zeny`, the shop window open or not, who can carry anything.
#[derive(Clone)]
struct Shopper {
    zeny: i32,
    shop_open: i32,
    requests: Arc<Mutex<Vec<Request>>>,
}

#[async_trait]
impl Host for Shopper {
    async fn invoke(&mut self, request: Request) -> Reply {
        let reply = match &request {
            Request::Read(name) if name == "Zeny" => Ok(Value::Number(self.zeny)),
            Request::Read(name) if name == "@ui_shop_open" => Ok(Value::Number(self.shop_open)),
            Request::VariableRead { name, .. } if name.ends_with("ui_shop_open") => Ok(Value::Number(self.shop_open)),
            Request::Call { function: Function::CheckWeight, .. } => Ok(Value::Number(1)),
            Request::Call { function: Function::GetItemName, .. } => Ok(Value::String("Red Potion".into())),
            _ => Ok(Value::default()),
        };
        self.requests.lock().unwrap().push(request);
        reply
    }
}

async fn shop_run(zeny: i32, shop_open: i32, entry: Entry) -> (Vec<Request>, Result<(), String>) {
    let host = Shopper { zeny, shop_open, requests: Default::default() };
    let plugins = ui_shop();
    let (host, result) = plugins.iter().next().unwrap().runtime.execute_named(host, &entry).await;
    let requests = host.requests.lock().unwrap().clone();
    (requests, result)
}

fn ui_call(requests: &[Request], wanted: Function) -> Vec<Vec<Value>> {
    requests
        .iter()
        .filter_map(|request| match request {
            Request::Call { function, arguments } if *function == wanted => Some(arguments.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn the_ui_example_declares_its_window_and_bot_action() {
    let plugins = ui_shop();
    let plugin = plugins.iter().next().unwrap();
    assert_eq!(plugin.capabilities.iter().collect::<Vec<_>>(), [Capability::Economy, Capability::Interface]);
    assert!(plugin.handles_window("shop"));
    assert!(plugin.has_bot_action("buy"));
    assert_eq!(plugins.observers(HookKind::LevelUp).count(), 1);
}

#[tokio::test]
async fn talking_to_the_trader_opens_the_shop_window_and_shows_bots_the_stock() {
    let (requests, result) = shop_run(1_000, 0, Entry::Npc("Curio Trader".into())).await;
    result.unwrap();
    let opened = ui_call(&requests, Function::UiOpen);
    assert_eq!(opened.len(), 1);
    assert_eq!(opened[0][1], Value::from("ui_shop"), "the runtime names the plugin");
    let window: plugin_ui::Window = serde_json::from_str(&opened[0][0].text()).unwrap();
    assert_eq!(window.id, "shop");
    assert!(matches!(window.widget("buy:501"), Some(plugin_ui::Node::Button { enabled: true, .. })));
    assert!(matches!(window.widget("buy:505"), Some(plugin_ui::Node::Button { enabled: false, .. })), "4,000 zeny is more than the player has");
    let stock = ui_call(&requests, Function::BotObserve);
    assert_eq!(stock[0][0], Value::from("stock"));
    assert!(stock[0][1].text().contains("\"item_id\":501"));
}

#[tokio::test]
async fn a_buy_click_takes_the_zeny_gives_the_items_and_refreshes_the_window() {
    let click = plugin_ui::UiEvent::click("shop", "buy:501").with_input("amount", plugin_ui::InputValue::Number(3));
    let (requests, result) = shop_run(1_000, 1, Entry::UiEvent(click)).await;
    result.unwrap();
    assert!(requests.contains(&Request::Write { name: "Zeny".into(), value: Value::Number(880) }), "{requests:?}");
    assert!(ui_call(&requests, Function::GetItem).contains(&vec![501.into(), 3.into()]));
    let refreshed: plugin_ui::Window = serde_json::from_str(&ui_call(&requests, Function::UiOpen)[0][0].text()).unwrap();
    assert!(serde_json::to_string(&refreshed).unwrap().contains("Bought 3 Red Potion."));
}

#[tokio::test]
async fn the_bot_action_needs_the_trader_visited_and_answers_what_it_bought() {
    let buy = || Entry::BotAction(script_sdk::BotAction { name: "buy".into(), args: r#"{"item_id":501,"amount":2}"#.into() });
    let (requests, result) = shop_run(1_000, 0, buy()).await;
    result.unwrap();
    assert!(ui_call(&requests, Function::BotReply)[0][0].text().contains("Talk to the Curio Trader first"));
    assert!(!requests.iter().any(|request| matches!(request, Request::Write { name, .. } if name == "Zeny")));

    let (requests, result) = shop_run(1_000, 1, buy()).await;
    result.unwrap();
    let reply: serde_json::Value = serde_json::from_str(&ui_call(&requests, Function::BotReply)[0][0].text()).unwrap();
    assert_eq!(reply, serde_json::json!({ "bought": 2, "item_id": 501, "zeny": 920 }));
}

#[tokio::test]
async fn a_level_up_shows_a_rising_star_above_the_player() {
    let hook = Hook::LevelUp { char_id: 7, kind: LevelKind::Base, level: 12 };
    let (requests, result) = shop_run(0, 0, Entry::Hook(hook)).await;
    result.unwrap();
    let shown = ui_call(&requests, Function::UiOverhead);
    assert_eq!(shown[0][0], Value::Number(0), "above the attached player");
    let overhead: plugin_ui::Overhead = serde_json::from_str(&shown[0][1].text()).unwrap();
    assert_eq!(overhead.text.unwrap().text, "LEVEL 12!");
    assert_eq!(overhead.image.unwrap().frames, ["@/star.png"], "the server resolves the plugin's own path");
}

fn lucky_charms() -> Plugins {
    load(vec![PluginSetting::Name("lucky_charms".into())]).unwrap()
}

/// Answers a fixed draw. The runtime answers the settings itself, with the defaults of `info.json`.
#[derive(Clone)]
struct Lucky {
    roll: i32,
    requests: Arc<Mutex<Vec<Request>>>,
}

#[async_trait]
impl Host for Lucky {
    async fn invoke(&mut self, request: Request) -> Reply {
        let reply = match &request {
            Request::Call { function: Function::Rand, .. } => Ok(Value::Number(self.roll)),
            Request::Call { function: Function::CountItem, .. } => Ok(Value::Number(2)),
            _ => Ok(Value::default()),
        };
        self.requests.lock().unwrap().push(request);
        reply
    }
}

#[test]
fn the_example_declares_its_commands_and_item_scripts() {
    let plugins = lucky_charms();
    let plugin = plugins.iter().next().unwrap();
    let commands: Vec<_> = plugin.info.commands.iter().map(|command| (command.name.as_str(), command.level)).collect();
    assert_eq!(commands, [("lucky", 0), ("luckytickets", 60)]);
    assert!(plugin.has_command("lucky") && !plugin.has_command("item"));
    assert_eq!((plugin.info.item_scripts.as_slice(), plugin.info.item_bonuses.as_slice()), (&[40001, 40003][..], &[40002][..]));
    assert_eq!(plugin.items.iter().map(|item| item.id()).collect::<Vec<_>>(), [40001, 40002, 40003]);
}

#[tokio::test]
async fn a_plugin_command_runs_with_its_arguments_and_replies() {
    let lucky = Lucky { roll: 0, requests: Arc::default() };
    let command = script_sdk::PluginCommand { name: "luckytickets".into(), args: vec!["4".into()] };
    let (host, result) = lucky_charms().iter().next().unwrap().runtime.execute_named(lucky, &Entry::Command(command)).await;
    result.unwrap();
    let requests = host.requests.lock().unwrap().clone();
    assert_eq!(ui_call(&requests, Function::GetItem), [vec![40001.into(), 4.into()]]);
    assert_eq!(ui_call(&requests, Function::CommandReply), [vec!["Here are 4 Lucky Tickets.".into(), "lucky_charms".into()]], "the runtime names the plugin");
}

#[tokio::test]
async fn the_item_vm_runs_plugin_items_in_their_plugin() {
    let plugins = lucky_charms();
    let vm = crate::server::script::ItemVm::new(crate::tests::common::test_item_runtime(), crate::tests::common::test_item_runtime()).with_plugin_items(&plugins);
    let (host, result) = vm.run_item(Lucky { roll: 52, requests: Arc::default() }, 40001).await;
    result.unwrap();
    assert_eq!(ui_call(&host.requests.lock().unwrap(), Function::GetItem), [vec![502.into(), 3.into()]], "past the 50 of Red Potions");
    let (host, result) = vm.run_item(Lucky { roll: 0, requests: Arc::default() }, 501).await;
    result.unwrap();
    assert!(ui_call(&host.requests.lock().unwrap(), Function::GetItem).is_empty(), "a game item runs in the item module");
}
