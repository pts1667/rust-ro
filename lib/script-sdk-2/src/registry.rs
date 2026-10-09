use script_sdk::{Entry, NAMED_ABI_VERSION, Request};

use crate::ctx::Ctx;
use crate::flow::{Script, finish};
use crate::transport::{Transport, WasmTransport};

/// A script as a module exports it. Plain functions coerce to this, so scripts need no wrapping.
pub type ScriptFn = fn(&Ctx<'_>) -> Script;

/// The ABI a module built with [`script_module!`](crate::script_module) reports to the host.
pub const ABI: u32 = NAMED_ABI_VERSION;

/// Whether `names` is strictly increasing. That order lets [`run`] binary-search the table, and it rules out duplicates.
pub const fn is_strictly_sorted(names: &[&str]) -> bool {
    let mut index = 1;
    while index < names.len() {
        if !less(names[index - 1], names[index]) {
            return false;
        }
        index += 1;
    }
    true
}

const fn less(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    let mut index = 0;
    while index < left.len() && index < right.len() {
        if left[index] != right[index] {
            return left[index] < right[index];
        }
        index += 1;
    }
    left.len() < right.len()
}

/// Runs the entry the host named and returns the status it expects: `0` on success.
pub fn run(npcs: &[(&str, ScriptFn)], events: &[(&str, ScriptFn)]) -> i32 {
    let transport = WasmTransport;
    let ctx = Ctx::new(&transport);
    let result = current_entry().and_then(|entry| {
        let script = match &entry {
            Entry::Npc(name) => find(npcs, name)?,
            Entry::Event(name) => find(events, name)?,
        };
        finish(script(&ctx))
    });
    match result {
        Ok(()) => 0,
        Err(error) => {
            let _ = transport.request(Request::ReportError(error));
            1
        }
    }
}

fn find(table: &[(&str, ScriptFn)], name: &str) -> Result<ScriptFn, String> {
    table
        .binary_search_by(|entry| entry.0.cmp(name))
        .map(|index| table[index].1)
        .map_err(|_| format!("Unknown script {name}"))
}

/// The entry the host asked this module to run, read through the `rust_ro::entry` import.
pub fn current_entry() -> Result<Entry, String> {
    #[cfg(target_arch = "wasm32")]
    {
        #[link(wasm_import_module = "rust_ro")]
        extern "C" {
            fn entry(buffer: *mut u8, capacity: usize) -> i32;
        }
        let mut buffer = vec![0u8; script_sdk::MAX_MESSAGE_BYTES];
        let length = unsafe { entry(buffer.as_mut_ptr(), buffer.len()) };
        if length < 0 || length as usize > buffer.len() {
            return Err("Invalid entry from host".into());
        }
        serde_json::from_slice(&buffer[..length as usize]).map_err(|e| e.to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    Err("Script entries require a WebAssembly host".into())
}

#[cfg(test)]
mod tests {
    use super::is_strictly_sorted;

    #[test]
    fn table_order_is_strict() {
        assert!(is_strictly_sorted(&[]));
        assert!(is_strictly_sorted(&["a", "ab", "b"]));
        assert!(!is_strictly_sorted(&["b", "a"]));
        assert!(!is_strictly_sorted(&["a", "a"]));
        assert!(!is_strictly_sorted(&["ab", "a"]));
    }
}
