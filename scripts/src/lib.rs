use script_sdk::{ABI_VERSION, Context, Request};

mod battleground_arena;
mod battleground_kvm;
mod battleground_npcs;
mod battleground_tierra;
mod castle_npcs;
mod wedding_npcs;
mod npcs;
mod events;
mod pets;
mod rt;
mod generated;

#[no_mangle]
pub extern "C" fn script_abi() -> u32 {
    ABI_VERSION
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
pub extern "C" fn run_npc(id: u32) -> i32 {
    finish(npcs::run(&Context, id))
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

#[no_mangle]
pub extern "C" fn run_pet_auto_bonus(id: u32) -> i32 {
    finish(pets::run_auto_bonus(&Context, id))
}
