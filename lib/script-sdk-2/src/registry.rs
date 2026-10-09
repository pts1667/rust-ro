use script_sdk::{ABI_VERSION, Entry, Request};

use crate::ctx::Ctx;
use crate::flow::{Script, finish};
use crate::item::{ItemBonus, ItemUse};
use crate::transport::{Transport, WasmTransport};

/// A script as a module exports it. Plain functions coerce to this, so scripts need no wrapping.
pub type ScriptFn = fn(&Ctx<'_>) -> Script;

/// The ABI a module built with [`script_module!`](crate::script_module) reports to the host.
pub const ABI: u32 = ABI_VERSION;

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

/// A use item script or a program. It runs in an [`ItemUse`], so its effects are queued.
pub type ItemFn = fn(&ItemUse<'_, '_>) -> Script;

/// A passive item script. It runs in an [`ItemBonus`], so it can describe bonuses and nothing else.
pub type BonusFn = fn(&ItemBonus<'_, '_>) -> Script;

/// The scripts a module provides. A module fills the tables of the entries it serves and leaves the others empty.
#[derive(Default)]
pub struct Tables<'a> {
    pub npcs: &'a [(&'a str, ScriptFn)],
    pub events: &'a [(&'a str, ScriptFn)],
    pub items: &'a [(u32, ItemFn)],
    pub bonuses: &'a [(u32, BonusFn)],
    pub programs: &'a [(u32, ItemFn)],
    pub pets: &'a [(u32, ItemFn)],
    pub pet_supports: &'a [(u32, ItemFn)],
    pub pet_programs: &'a [(u32, ItemFn)],
}

/// Runs the entry the host named and returns the status it expects: `0` on success.
pub fn run(tables: Tables<'_>) -> i32 {
    let transport = WasmTransport;
    let ctx = Ctx::new(&transport);
    let result = current_entry().and_then(|entry| match &entry {
        Entry::Npc(name) => finish(find(tables.npcs, name)?(&ctx)),
        Entry::Event(name) => finish(find(tables.events, name)?(&ctx)),
        Entry::Item(id) => match find_id(tables.items, *id, "item") {
            Ok(script) => finish(script(&ItemUse::new(&ctx))),
            Err(_) => finish(find_id(tables.bonuses, *id, "item")?(&ItemBonus::new(&ctx))),
        },
        Entry::Program(id) => finish(find_id(tables.programs, *id, "item program")?(&ItemUse::new(&ctx))),
        Entry::Pet(id) => finish(find_id(tables.pets, *id, "pet")?(&ItemUse::new(&ctx))),
        Entry::PetSupport(id) => finish(find_id(tables.pet_supports, *id, "pet support")?(&ItemUse::new(&ctx))),
        Entry::PetProgram(id) => finish(find_id(tables.pet_programs, *id, "pet program")?(&ItemUse::new(&ctx))),
    });
    match result {
        Ok(()) => 0,
        Err(error) => {
            let _ = transport.request(Request::ReportError(error));
            1
        }
    }
}

/// Whether the ids are strictly increasing, for the same reason as [`is_strictly_sorted`].
pub const fn is_strictly_sorted_ids(ids: &[u32]) -> bool {
    let mut index = 1;
    while index < ids.len() {
        if ids[index - 1] >= ids[index] {
            return false;
        }
        index += 1;
    }
    true
}

fn find_id<T: Copy>(table: &[(u32, T)], id: u32, kind: &str) -> Result<T, String> {
    table.binary_search_by_key(&id, |entry| entry.0).map(|index| table[index].1).map_err(|_| format!("Unknown {kind} script {id}"))
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
    use super::{is_strictly_sorted, is_strictly_sorted_ids};

    #[test]
    fn table_order_is_strict() {
        assert!(is_strictly_sorted(&[]));
        assert!(is_strictly_sorted(&["a", "ab", "b"]));
        assert!(!is_strictly_sorted(&["b", "a"]));
        assert!(!is_strictly_sorted(&["a", "a"]));
        assert!(!is_strictly_sorted(&["ab", "a"]));
    }

    #[test]
    fn item_ids_must_be_strictly_increasing() {
        assert!(is_strictly_sorted_ids(&[1, 5, 9]));
        assert!(!is_strictly_sorted_ids(&[1, 1]));
        assert!(!is_strictly_sorted_ids(&[3, 2]));
    }
}
