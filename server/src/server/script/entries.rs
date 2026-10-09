use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use script_sdk::Entry;

/// Handles from this value up name a script of a named-ABI module. Lower handles are legacy numeric entry ids, so the `u32`
/// carried by NPCs and events keeps its shape.
pub const NAMED_BASE: u32 = 0x8000_0000;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedScript {
    pub module: String,
    pub entry: Entry,
}

#[derive(Default)]
struct Interner {
    handles: HashMap<NamedScript, u32>,
    scripts: Vec<NamedScript>,
}

fn interner() -> &'static Mutex<Interner> {
    static INTERNER: OnceLock<Mutex<Interner>> = OnceLock::new();
    INTERNER.get_or_init(Default::default)
}

/// Handles are allocated in the order scripts are first named, so they are only meaningful within one process.
pub fn intern(module: &str, entry: Entry) -> u32 {
    let mut interner = interner().lock().unwrap();
    let script = NamedScript { module: module.into(), entry };
    if let Some(handle) = interner.handles.get(&script) {
        return *handle;
    }
    let handle = NAMED_BASE + interner.scripts.len() as u32;
    interner.scripts.push(script.clone());
    interner.handles.insert(script, handle);
    handle
}

pub fn resolve(handle: u32) -> Option<NamedScript> {
    let index = handle.checked_sub(NAMED_BASE)? as usize;
    interner().lock().unwrap().scripts.get(index).cloned()
}
