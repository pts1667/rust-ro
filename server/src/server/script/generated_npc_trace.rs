//! Runs every NPC and event moved to a named module next to its legacy entry in `game_scripts.wasm`, on the same random answers,
//! and compares what each one asked the host to do. The new scripts send a dialogue page per `mes`, so consecutive `Mes` calls
//! are compared as one dialogue.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value};

use super::entries::NAMED_BASE;
use super::generated_npc_smoke::RandomHost;
use super::vm::ScriptVm;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::service::script_service::ScriptService;

const SEEDS: u64 = 3;

#[derive(Debug, PartialEq)]
enum Step {
    Say(String),
    Call(Function, Vec<Value>),
    Write(String, Value),
    Error(String),
}

#[derive(Debug, PartialEq)]
struct Trace {
    steps: Vec<Step>,
    result: Option<String>,
}

struct TraceHost {
    seed: u64,
    occurrences: HashMap<String, u64>,
    steps: Vec<Step>,
}

#[async_trait::async_trait]
impl Host for TraceHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        match &request {
            Request::Call { function: Function::Mes, arguments } => self.steps.push(Step::Say(arguments.iter().map(Value::text).collect::<Vec<_>>().join("\n"))),
            Request::Call { function, arguments } => self.steps.push(Step::Call(function.clone(), arguments.clone())),
            Request::Write { name, value } => self.steps.push(Step::Write(name.clone(), value.clone())),
            Request::VariablesWrite(variables) => self.steps.extend(variables.iter().map(|variable| Step::Write(variable.name.clone(), variable.value.clone()))),
            Request::ReportError(error) => self.steps.push(Step::Error(error.clone())),
            _ => {}
        }
        // Each answer depends on its request and how often it came before, so a read the other version does not make cannot shift the rest
        let key = format!("{request:?}");
        let occurrence = {
            let count = self.occurrences.entry(key.clone()).or_default();
            *count += 1;
            *count
        };
        let mut hasher = DefaultHasher::new();
        (self.seed, &key, occurrence).hash(&mut hasher);
        RandomHost::new(hasher.finish()).invoke(request).await
    }
}

/// Lookups without side effects. The generated code may run one before a dialogue the legacy code shows first; their results still
/// reach the dialogue text, which is compared.
const PURE_READS: [Function; 4] = [Function::StrCharInfo, Function::GetItemName, Function::GetItemInfo, Function::GetAreaUsers];

fn dialogues(steps: Vec<Step>) -> Vec<Step> {
    let mut merged: Vec<Step> = Vec::new();
    for step in steps {
        if matches!(&step, Step::Call(function, _) if PURE_READS.contains(function)) {
            continue;
        }
        let joined = match (merged.last_mut(), &step) {
            (Some(Step::Say(previous)), Step::Say(text)) => {
                previous.push('\n');
                previous.push_str(text);
                true
            }
            _ => false,
        };
        if !joined {
            merged.push(step);
        }
    }
    merged
}

fn trace(vm: &ScriptVm, export: &str, handle: u32, seed: u64) -> Trace {
    let host = TraceHost { seed, occurrences: HashMap::new(), steps: vec![] };
    let (host, result) = futures::executor::block_on(vm.run(host, export, handle));
    Trace { steps: dialogues(host.steps), result: result.err() }
}

fn first_difference(legacy: &Trace, named: &Trace) -> String {
    let index = legacy.steps.iter().zip(&named.steps).position(|(old, new)| old != new).unwrap_or(legacy.steps.len().min(named.steps.len()));
    format!(
        "{} vs {} steps, result {:?} vs {:?}, first difference at {index}: {:?} vs {:?}",
        legacy.steps.len(),
        named.steps.len(),
        legacy.result,
        named.result,
        legacy.steps.get(index),
        named.steps.get(index),
    )
}

struct Job {
    label: String,
    legacy: (&'static str, u32),
    named: (&'static str, u32),
}

/// Compares the two versions on every seed and returns how many host steps the migrated one made, with the first mismatch if any.
fn compare(vm: &ScriptVm, job: &Job) -> (usize, Option<String>) {
    let (label, legacy, named) = (&job.label, job.legacy, job.named);
    assert!(named.1 >= NAMED_BASE, "{label} is not routed to a named module");
    let mut steps = 0;
    for seed in 0..SEEDS {
        let old = trace(vm, legacy.0, legacy.1, seed);
        let new = trace(vm, named.0, named.1, seed);
        steps += new.steps.len();
        if old != new {
            return (steps, Some(format!("{label} (seed {seed}): {}", first_difference(&old, &new))));
        }
    }
    (steps, None)
}

#[test]
fn every_migrated_npc_and_event_does_what_its_legacy_entry_did() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/migration_map.json");
    let migration: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let npcs = ScriptLoader::load_scripts(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/npcs.json").to_str().unwrap()).unwrap();
    let mut jobs = vec![];
    for npc in npcs.values().flatten() {
        let Some(legacy) = migration.get(&npc.name) else { continue };
        jobs.push(Job { label: npc.name.clone(), legacy: ("run_npc", legacy["entry_id"].as_u64().unwrap() as u32), named: ("run_npc", npc.entry_id) });
        for (label, legacy_event) in legacy["events"].as_object().unwrap() {
            let named = ScriptService::event_entry(label).unwrap_or_else(|| panic!("{label} is not a registered event"));
            jobs.push(Job { label: label.clone(), legacy: ("run_event", legacy_event.as_u64().unwrap() as u32), named: ("run_event", named) });
        }
    }
    assert!(!jobs.is_empty(), "no migrated NPC in the migration map");

    let vm = crate::tests::common::test_npc_vm();
    let next = AtomicUsize::new(0);
    let outcomes = Mutex::new(Vec::with_capacity(jobs.len()));
    let workers = std::thread::available_parallelism().map_or(1, |count| count.get());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                while let Some(job) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let outcome = compare(&vm, job);
                    outcomes.lock().unwrap().push(outcome);
                }
            });
        }
    });

    let outcomes = outcomes.into_inner().unwrap();
    let steps: usize = outcomes.iter().map(|(steps, _)| steps).sum();
    let mut mismatches: Vec<String> = outcomes.into_iter().filter_map(|(_, mismatch)| mismatch).collect();
    mismatches.sort();
    assert!(steps > 0, "the migrated entries made no host calls, so the comparison proves nothing");
    assert!(mismatches.is_empty(), "{} of {} migrated entries differ:\n{}", mismatches.len(), jobs.len(), mismatches.join("\n"));
}
