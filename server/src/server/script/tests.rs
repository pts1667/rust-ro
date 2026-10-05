use std::sync::Arc;

use models::enums::bonus::BonusType;
use models::status::Status;
use script_runtime::{Host, WasmRuntime};
use script_sdk::{Function, Reply, Request, Value};

use super::item_script_handler::{ItemEffect, ItemScriptHost};

fn runtime() -> Arc<WasmRuntime> {
    crate::tests::common::test_script_vm()
}

#[test]
fn compiled_potion_stages_healing_without_mutating_character_status() {
    let status = Status {
        hp: 10,
        sp: 4,
        ..Status::default()
    };
    let (host, result) = futures::executor::block_on(runtime().execute(ItemScriptHost::consumable(status, 501), "run_item", 501));
    assert!(result.is_ok(), "{:?}", host.error);
    assert_eq!(host.status.hp, 10);
    assert_eq!(host.status.sp, 4);
    assert!(matches!(host.effects.as_slice(), [ItemEffect::Heal {
        hp: 45..=65,
        sp: 0,
        percentage: false,
        item_scaling: true,
        item_id: 501
    }]));
}

#[test]
fn equipment_context_rejects_consumable_healing_effects() {
    let (host, result) = futures::executor::block_on(runtime().execute(
        ItemScriptHost::bonuses(
            Status {
                hp: 10,
                ..Status::default()
            },
            526,
        ),
        "run_item",
        526,
    ));
    assert!(result.is_err());
    assert_eq!(host.status.hp, 10);
    assert!(host.error.unwrap().contains("ItemHeal"));
    assert!(host.effects.is_empty());
}

#[test]
fn generated_static_bonus_matches_original_item() {
    let item: serde_json::Value =
        serde_json::from_slice(&std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/items.json")).unwrap())
            .unwrap();
    let id = item["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["script"] == "bonus bStr,1;")
        .unwrap()["id"]
        .as_u64()
        .unwrap() as u32;
    let (host, result) = futures::executor::block_on(runtime().execute(ItemScriptHost::bonuses(Status::default(), id), "run_item", id));
    assert!(result.is_ok(), "{:?}", host.error);
    assert_eq!(host.bonuses.drain(), vec![BonusType::Str(1)]);
}

#[test]
fn item_catalog_fingerprint_rejects_stale_bundle_metadata() {
    let bytes = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/items.json")).unwrap();
    let bytes: Vec<_> = bytes.into_iter().filter(|byte| *byte != b'\r').collect();
    let digest = md5::compute(&bytes);
    let fingerprint = u64::from_le_bytes(digest.0[..8].try_into().unwrap());
    assert_eq!(runtime().catalog_hash().unwrap(), fingerprint);
    let mut changed = bytes;
    changed.push(b' ');
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../target/stale-script-manifest-{nonce}.json"));
    std::fs::write(&path, changed).unwrap();
    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::server::service::item_service::ItemService::load_item_scripts(&mut vec![], runtime(), &path)
    }));
    std::fs::remove_file(path).unwrap();
    assert!(rejected.is_err());
}

#[test]
fn compiled_catalog_accepts_windows_line_endings() {
    let source = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/items.json")).unwrap();
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../target/crlf-script-manifest-{nonce}.json"));
    std::fs::write(&path, source.replace("\r\n", "\n").replace('\n', "\r\n")).unwrap();
    let counts = crate::server::service::item_service::ItemService::load_item_scripts(&mut vec![], runtime(), &path);
    std::fs::remove_file(path).unwrap();
    assert_eq!(counts, (0, 0));
}

#[test]
fn npc_manifest_preserves_enabled_placements() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/npcs.json");
    let scripts = crate::server::boot::script_loader::ScriptLoader::load_scripts(root.to_str().unwrap()).unwrap();
    assert_eq!(scripts.values().map(Vec::len).sum::<usize>(), 79);
    let prontera = &scripts["prontera"];
    assert!(prontera.iter().any(|npc| npc.name == "Job Master" && npc.entry_id == 5));
    assert!(prontera.iter().any(|npc| npc.entry_id == 6 && !npc.constructor_args.is_empty()));
}

#[test]
fn npc_dialogue_runs_compiled_code_across_player_wait() {
    struct Dialogue {
        messages: Vec<String>,
        waits: usize,
    }
    #[async_trait::async_trait]
    impl Host for Dialogue {
        async fn invoke(&mut self, request: Request) -> Reply {
            match request {
                Request::VariableRead { .. } => Ok(0.into()),
                Request::VariablesWrite(_) => Ok(Value::default()),
                Request::VariablesIncrement(_) => Ok(Value::Array(vec![1.into(), 1.into()])),
                Request::Call {
                    function: Function::Mes,
                    arguments,
                } => {
                    self.messages.extend(arguments.iter().map(Value::text));
                    Ok(Value::default())
                }
                Request::Call {
                    function: Function::Next, ..
                } => {
                    self.waits += 1;
                    tokio::task::yield_now().await;
                    Ok(Value::default())
                }
                Request::Call {
                    function: Function::Close, ..
                } => Ok(Value::default()),
                Request::ReportError(error) => Err(error),
                _ => Err("Unexpected dialogue call".into()),
            }
        }
    }
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let (host, result) = runtime.block_on(self::runtime().execute(
        Dialogue {
            messages: vec![],
            waits: 0,
        },
        "run_npc",
        1,
    ));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(host.waits, 1);
    assert_eq!(host.messages, [
        "Npc counter variable: 1\nNPC instance counter variable: 1",
        "Close"
    ]);
}
