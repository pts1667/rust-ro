//! Runs every NPC and event converted from rathena against a host that answers with random values, so that
//! faults of the generated code (unresolved constants, type confusion, bad jumps) show up without a client.

use std::collections::BTreeMap;

use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value};

use crate::server::boot::script_loader::ScriptLoader;
use crate::server::service::script_service::ScriptService;

const RUNS_PER_ENTRY: u64 = 4;
const FIRST_GENERATED_NPC: u32 = 10_000;
const FIRST_GENERATED_EVENT: u32 = 100_000;

pub(super) struct RandomHost {
    rng: fastrand::Rng,
    errors: Vec<String>,
}

impl RandomHost {
    pub(super) fn new(seed: u64) -> Self {
        Self { rng: fastrand::Rng::with_seed(seed), errors: vec![] }
    }

    fn number(&mut self) -> Value {
        Value::Number(if self.rng.bool() { self.rng.i32(0..3) } else { self.rng.i32(0..60) })
    }

    fn variable(&mut self, name: &str) -> Value {
        if name.ends_with('$') {
            Value::String(["", "abc", "Player"][self.rng.usize(0..3)].into())
        } else {
            self.number()
        }
    }
}

#[async_trait::async_trait]
impl Host for RandomHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        Ok(match request {
            Request::Constant(name) => return super::item_script_handler::constant(&name),
            Request::Read(name) => self.variable(&name),
            Request::VariableRead { name, .. } => self.variable(&name),
            Request::VariablesIncrement(variables) => Value::Array(variables.iter().map(|_| Value::Number(1)).collect()),
            Request::Arguments => Value::Array(vec![]),
            Request::ReportError(error) => {
                self.errors.push(error);
                Value::default()
            }
            Request::Call { function, arguments } => match function {
                Function::Select => Value::Number(1 + self.rng.i32(0..arguments.len().max(1) as i32)),
                Function::InputNumber => Value::Number(self.rng.i32(0..100)),
                Function::InputString => Value::String("abc".into()),
                Function::Rand => match arguments.as_slice() {
                    [limit] => Value::Number(self.rng.i32(0..limit.number_value()?.max(1))),
                    [low, high] => {
                        let (low, high) = (low.number_value()?, high.number_value()?);
                        Value::Number(self.rng.i32(low.min(high)..=high.max(low)))
                    }
                    _ => Value::Number(0),
                },
                Function::ArrayGet => Value::Array(vec![]),
                Function::GetMapXy => Value::Array(vec![Value::String("prontera".into()), Value::Number(100), Value::Number(100)]),
                Function::GetPartyMember => Value::Array(vec![Value::Number(150_000), Value::Number(150_001)]),
                Function::CheckWeight => Value::Number(1),
                Function::CountItem => Value::Number(self.rng.i32(0..4)),
                Function::CheckQuest => Value::Number(self.rng.i32(-1..3)),
                Function::StrCharInfo | Function::StrNpcInfo | Function::GetTimeStr | Function::GetGuildInfo | Function::GetItemName => Value::String("abc".into()),
                _ => Value::Number(self.rng.i32(0..2)),
            },
            _ => Value::default(),
        })
    }
}

/// Handles of the NPCs and events that run generated code: legacy ids from `first` up, and every named entry.
fn generated_entries(entries: impl Iterator<Item = (String, u32)>, first: u32) -> Vec<(String, u32)> {
    let by_handle: BTreeMap<u32, String> = entries.filter(|(_, id)| *id >= first).map(|(name, id)| (id, name)).collect();
    by_handle.into_iter().map(|(id, name)| (name, id)).collect()
}

fn npc_entries() -> Vec<(String, u32)> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/npcs.json");
    let npcs = ScriptLoader::load_scripts(path.to_str().unwrap()).unwrap();
    generated_entries(npcs.into_values().flatten().map(|npc| (npc.name, npc.entry_id)), FIRST_GENERATED_NPC)
}

fn event_entries() -> Vec<(String, u32)> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/events.json");
    let labels: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let events = labels.as_object().unwrap().keys().filter_map(|label| Some((label.clone(), ScriptService::event_entry(label)?)));
    generated_entries(events, FIRST_GENERATED_EVENT)
}

fn run_all(entry: &str, entries: Vec<(String, u32)>) -> BTreeMap<String, Vec<String>> {
    let runtime = crate::tests::common::test_npc_vm();
    let mut failures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, id) in entries {
        for seed in 0..RUNS_PER_ENTRY {
            let host = RandomHost { rng: fastrand::Rng::with_seed(u64::from(id) * 31 + seed), errors: vec![] };
            let (host, result) = futures::executor::block_on(runtime.run(host, entry, id));
            if let Err(error) = result {
                let error = host.errors.into_iter().next().unwrap_or(error);
                failures.entry(error).or_default().push(format!("{name} ({id})"));
            }
        }
    }
    failures
}

fn report(failures: &BTreeMap<String, Vec<String>>) -> String {
    failures.iter().map(|(error, who)| format!("{} x {error}: {}", who.len(), who.iter().take(3).cloned().collect::<Vec<_>>().join(", "))).collect::<Vec<_>>().join("\n")
}

/// Faults that are the random host's doing, not the generated code's.
fn expected(error: &str) -> bool {
    error.contains("budget exhausted") || error.contains("fuel") || error.contains("Conversation cancelled") || error.contains("Invalid menu selection") || error.contains("Negative array index")
}

#[test]
fn every_converted_npc_runs_against_a_random_host() {
    let failures = run_all("run_npc", npc_entries());
    let unexpected: BTreeMap<_, _> = failures.into_iter().filter(|(error, _)| !expected(error)).collect();
    assert!(unexpected.is_empty(), "{}", report(&unexpected));
}

#[test]
fn every_converted_event_runs_against_a_random_host() {
    let failures = run_all("run_event", event_entries());
    let unexpected: BTreeMap<_, _> = failures.into_iter().filter(|(error, _)| !expected(error)).collect();
    assert!(unexpected.is_empty(), "{}", report(&unexpected));
}
