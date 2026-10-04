use script_sdk::{ABI_VERSION, Context, Request};

mod items;
mod npcs;
mod functions;
mod events;
mod pets;

#[no_mangle]
pub extern "C" fn script_abi() -> u32 {
    ABI_VERSION
}

#[no_mangle]
pub extern "C" fn script_catalog_hash() -> u64 {
    items::CATALOG_HASH
}

fn finish(result: Result<(), String>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(error) => {
            let _ = Context.request(Request::ReportError(error));
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn run_item(id: u32) -> i32 {
    finish(items::run(&Context, id))
}

#[no_mangle]
pub extern "C" fn run_npc(id: u32) -> i32 {
    finish(npcs::run(&Context, id))
}

#[no_mangle]
pub extern "C" fn run_bonus(id: u32) -> i32 {
    finish(items::run_bonus(&Context, id))
}

#[no_mangle]
pub extern "C" fn run_event(id: u32) -> i32 {
    finish(events::run(&Context, id))
}

#[no_mangle]
pub extern "C" fn run_pet(id: u32) -> i32 {
    finish(pets::run_bonus(&Context, id))
}

#[no_mangle]
pub extern "C" fn run_pet_support(id: u32) -> i32 {
    finish(pets::run_support(&Context, id))
}
