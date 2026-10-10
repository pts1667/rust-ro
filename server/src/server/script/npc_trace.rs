//! Runs every NPC and event on random but repeatable answers and compares what each asked the host to do with a recorded
//! snapshot. Consecutive `Mes` calls count as one dialogue, so splitting or joining dialogue lines does not change a trace.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value, VariableScope};

use super::entries::{NAMED_BASE, resolve};
use super::generated_npc_smoke::RandomHost;
use super::vm::ScriptVm;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::service::item_effect_service::script_variable_name;
use crate::server::service::script_service::ScriptService;

const SEEDS: u64 = 3;
const SNAPSHOT: &str = "src/server/script/npc_traces.json";

/// A script that loops on random answers runs until the fuel limit, which falls at a different step for each build. Only the
/// first steps are kept then, and a trap is not told apart from its location.
const MAX_STEPS: usize = 1000;

/// FNV-1a, which unlike `DefaultHasher` gives the same answer on every toolchain.
fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
}

/// The fields are read through `Debug`, which is what the digest hashes.
#[allow(dead_code)]
#[derive(Debug)]
enum Step {
    Say(String),
    Call(Function, Vec<Value>),
    /// A status the host updates when a script writes it by name: `Zeny` or `Hp`.
    Set(String, Value),
    Write(VariableScope, String, u32, Value),
    Increment(VariableScope, String, u32, Value),
    Error(String),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Trace {
    steps: Vec<Step>,
    result: Option<String>,
}

struct TraceHost {
    seed: u64,
    arguments: Vec<Value>,
    occurrences: HashMap<String, u64>,
    steps: Vec<Step>,
}

/// The host reads a prefixed name the same way as a scoped read of its first element, so both shapes trace alike. Writes
/// are recorded the same way for the same reason.
fn canonical(request: Request) -> Request {
    match request {
        Request::Read(name) => match script_variable_name(&name) {
            (VariableScope::Character, _) => Request::Read(name),
            (scope, name) => Request::VariableRead { scope, name, index: 0 },
        },
        request => request,
    }
}

#[async_trait::async_trait]
impl Host for TraceHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        let request = canonical(request);
        match &request {
            Request::Arguments => return Ok(Value::Array(self.arguments.clone())),
            Request::Call { function: Function::Mes, arguments } => self.steps.push(Step::Say(arguments.iter().map(Value::text).collect::<Vec<_>>().join("\n"))),
            Request::Call { function, arguments } => self.steps.push(Step::Call(function.clone(), arguments.clone())),
            Request::Write { name, value } if name == "Zeny" || name == "Hp" => self.steps.push(Step::Set(name.clone(), value.clone())),
            Request::Write { name, value } => {
                let (scope, name) = script_variable_name(name);
                self.steps.push(Step::Write(scope, name, 0, value.clone()));
            }
            Request::VariablesWrite(variables) => self.steps.extend(variables.iter().map(|variable| Step::Write(variable.scope, variable.name.clone(), variable.index, variable.value.clone()))),
            Request::VariablesIncrement(variables) => self.steps.extend(variables.iter().map(|variable| Step::Increment(variable.scope, variable.name.clone(), variable.index, variable.value.clone()))),
            Request::ReportError(error) => self.steps.push(Step::Error(error.clone())),
            _ => {}
        }
        // Each answer depends on its request and how often it came before, so a read added or dropped cannot shift the answers after it
        let key = format!("{request:?}");
        let occurrence = {
            let count = self.occurrences.entry(key.clone()).or_default();
            *count += 1;
            *count
        };
        RandomHost::new(fnv(format!("{}/{key}/{occurrence}", self.seed).as_bytes())).invoke(request).await
    }
}

/// Lookups without side effects. A script may run one before or after a dialogue without changing what the player sees; their
/// results still reach the dialogue text, which is compared.
const PURE_READS: [Function; 4] = [Function::StrCharInfo, Function::GetItemName, Function::GetItemInfo, Function::GetAreaUsers];

fn dialogues(steps: Vec<Step>) -> Vec<Step> {
    let mut merged: Vec<Step> = Vec::new();
    for step in steps {
        if matches!(&step, Step::Call(function, _) if PURE_READS.contains(function)) {
            continue;
        }
        if let (Some(Step::Say(previous)), Step::Say(text)) = (merged.last_mut(), &step) {
            previous.push('\n');
            previous.push_str(text);
        } else {
            merged.push(step);
        }
    }
    merged
}

fn trace(vm: &ScriptVm, handle: u32, arguments: &[Value], seed: u64) -> Trace {
    let host = TraceHost { seed, arguments: arguments.to_vec(), occurrences: HashMap::new(), steps: vec![] };
    let (host, result) = futures::executor::block_on(vm.run(host, handle));
    let mut steps = dialogues(host.steps);
    let result = match result.err() {
        _ if steps.len() > MAX_STEPS => {
            steps.truncate(MAX_STEPS);
            None
        }
        Some(error) if error.contains("wasm backtrace") => Some("trap".to_string()),
        other => other,
    };
    Trace { steps, result }
}

/// The key of a script run: the script, plus the placement when its arguments make each placement run differently.
fn key(kind: &str, handle: u32, placement: Option<&str>) -> String {
    assert!(handle >= NAMED_BASE, "handle {handle} is not routed to a named module");
    let script = resolve(handle).unwrap();
    let key = format!("{kind}:{}:{:?}", script.module, script.entry);
    placement.map_or(key.clone(), |placement| format!("{key}:{placement}"))
}

/// One digest per script and kind of entry over every seed. NPCs placed from one template share it, unless the placement
/// gives the script arguments.
fn digests() -> BTreeMap<String, String> {
    let wasm = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm");
    let npcs = ScriptLoader::load_scripts(wasm.join("npcs.json").to_str().unwrap()).unwrap();
    let events: serde_json::Value = serde_json::from_slice(&std::fs::read(wasm.join("events.json")).unwrap()).unwrap();
    let mut runs: BTreeMap<String, (u32, Vec<Value>)> = npcs
        .values()
        .flatten()
        .map(|npc| {
            let placement = (!npc.constructor_args.is_empty()).then_some(npc.name.as_str());
            (key("npc", npc.entry_id, placement), (npc.entry_id, npc.constructor_args.clone()))
        })
        .collect();
    runs.extend(events.as_object().unwrap().keys().map(|label| {
        let handle = ScriptService::event_entry(label).unwrap();
        (key("event", handle, None), (handle, vec![]))
    }));
    let runs: Vec<(String, (u32, Vec<Value>))> = runs.into_iter().collect();

    let vm = crate::tests::common::test_npc_vm();
    let next = AtomicUsize::new(0);
    let digests = Mutex::new(BTreeMap::new());
    std::thread::scope(|scope| {
        for _ in 0..std::thread::available_parallelism().map_or(1, |count| count.get()) {
            scope.spawn(|| {
                while let Some((key, (handle, arguments))) = runs.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let traces: String = (0..SEEDS).map(|seed| format!("{:?}", trace(&vm, *handle, arguments, seed))).collect();
                    digests.lock().unwrap().insert(key.clone(), format!("{:016x}", fnv(traces.as_bytes())));
                }
            });
        }
    });
    digests.into_inner().unwrap()
}

/// The recorded traces are what the scripts did when they were last recorded, starting from the retired numeric module. A
/// script changed on purpose changes its digest: rerun with `UPDATE_TRACES=1` to record the new one.
#[test]
fn every_npc_and_event_still_does_what_the_snapshot_recorded() {
    let digests = digests();
    assert!(digests.len() > 8_000, "only {} scripts ran", digests.len());
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT);
    if std::env::var_os("UPDATE_TRACES").is_some() {
        std::fs::write(&path, serde_json::to_string_pretty(&digests).unwrap() + "\n").unwrap();
        return;
    }
    let recorded: BTreeMap<String, String> = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let mut differences: Vec<String> = digests.iter().filter(|(key, digest)| recorded.get(*key) != Some(digest)).map(|(key, _)| format!("changed or new: {key}")).collect();
    differences.extend(recorded.keys().filter(|key| !digests.contains_key(*key)).map(|key| format!("no longer run: {key}")));
    assert!(
        differences.is_empty(),
        "{} scripts differ from the snapshot (UPDATE_TRACES=1 records the new traces):\n{}",
        differences.len(),
        differences.iter().take(30).cloned().collect::<Vec<_>>().join("\n")
    );
}
