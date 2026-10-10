# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Running
```bash
# Build in release mode (optimized)
cargo build --release

# Run the server with its embedded sled database
cargo run --package server --bin server

# Run with visual debugger (requires visual_debugger feature)
cargo run --package server --bin server --features visual_debugger

# Build every NPC, event, item and pet script into config/wasm/modules/<name>.wasm
# (needs `rustup target add wasm32-unknown-unknown`; restart the server afterwards)
cargo run --package tools --bin scripts-build --release
```

### Testing
```bash
# Run all tests
cargo test --release

# Run integration tests (uses temporary sled databases)
cargo test --features integration_tests

# Run unit tests only
cargo test --features unit_tests

# Replay what every NPC and event asks the host to do against npc_traces.json.
# After changing a script on purpose, set UPDATE_TRACES=1 and rerun to record the new digests
cargo test --release -p server npc_trace
```

## Architecture Overview

This is a Ragnarok Online MMORPG server implementation written in Rust that combines the traditional Login/Char/Map server architecture into a single unified server. The architecture is built around message-passing and event-driven design patterns.

This project focus exclusively on "pre-re" (or "pre renewal") version of the game.

### Key Architectural Principles
- **Message Passing**: Uses message passing instead of shared memory with locks to avoid deadlocks and ensure thread safety
- **Event-Driven**: State changes occur through queued events processed by dedicated loops
- **Service Layer**: Business logic is separated into service layers
- **Repository Pattern**: Data access is abstracted through repository interfaces

### Core Components

#### Game Loop System
- Main game loop runs at fixed intervals (40ms tick rate)
- Events are queued for future ticks and processed sequentially
- State updates trigger persistence events and client notifications

#### Thread Architecture
1. **Game Loop Thread**: Processes game events and state updates
2. **Map Instance Threads**: Each map instance runs its own event loop for scalability
3. **Persistence Thread**: Handles all database write operations
4. **Client Notification Thread**: Sends packets to connected clients

#### Major Modules
- `server/`: Core server implementation
- `server/src/server/boot/`: Server initialization (map loading, script compilation, etc.)
- `server/src/server/service/`: Business logic layer (character, battle, inventory, etc.)
- `server/src/repository/`: Data access layer with sled integration
- `server/src/server/request_handler/`: Packet handling controllers
- `server/src/server/script/`: Typed game API for compiled Wasmtime NPC and item modules
- `server/src/server/state/`: Game state management (characters, maps, mobs)
- `server/src/server/mod.rs`: Implementation of server threads
- `server/src/server/game_loop.rs`: Implementation of the main game loop, latency of operation within the game loop should be low (<20ms) or server will lag. There is only one loop for the whole server. it handles action made by player
- `server/src/server/map_instance_loop.rs`: Implementation of map instance loop, responsible to handle action on a specific map, like interaction with MOB or NPC. There is one loop per map instance thread.
- `server/src/server/persistence.rs`: Implementation of an event loop for all database interaction which can be defered.
- `server/src/server/request_handler/mod.rs`: A function calling packet parser then by downcasting reference to the packet implementation route the request to the right function to handle it.
- `server/src/server/model/events/game_event/mod.rs`: Contains enumeration of game event, that are handled by the game loop. It is used for message passing between request handler thread and game loop.
- `server/src/server/model/events/client_notification.rs`: Enumeration for sending packet to the client
- `server/src/server/state/server.rs`: Access to server state, access should only be done from game loop, it is locked only by the game loop (see Server state below)
- `server/src/tests/`: unit and integration test of the server
- `lib/`: crate for specific logic implementation
  - `lib/configuration`: Structure for configuration the server. This is where configuration entry should be added 
  - `lib/models`: Structures shared accross crates
  - `lib/packets`: Structures of all packets exchanged between client and server
  - `lib/skills`: Implementation of all class skills, one file per skill under `skills/<job>/` (player) and `npc/` (monster). Each skill implements the `Skill` trait, which holds its behaviour as default-overridable hooks. These files are hand-maintained: do not run `tools/skills` without `--overwrite-hand-written-skills`, it overwrites them.
  - `lib/script-sdk`: wire protocol between the server and the Wasm script modules (`Function`, `Request`, `Value`, `Entry`)
  - `lib/script-sdk-2`: typed API that scripts are written against (`Ctx`, `script_module!`), see `docs/script-sdk-2.md`
  - `lib/script-runtime`: Wasmtime runtime for the modules (pooling allocator, memory and host-call budgets)
- `scripts/`: cargo workspace of the Wasm script modules, see Scripts below

### Scripts (NPC, event, item and pet modules)
- NPC and item behavior is Rust compiled to WebAssembly and run by Wasmtime. The sources are a cargo workspace in `scripts/` (`scripts/Cargo.toml`, one `Cargo.lock`, one `target/`); the original rathena `.txt` scripts are only import inputs and are never interpreted.
- One crate per module, built to `config/wasm/modules/<crate name>.wasm`: `scripts/{towns,misc,jobs,quests}/<module>` hold the NPCs converted from rathena (about 100 modules, one per rathena folder or large file, named after its path such as `quests_ein`), `scripts/systems` the hand-written NPCs (warper, job masters, castles, weddings, battlegrounds), `scripts/items` and `scripts/pets` the item and pet scripts keyed by id, `scripts/shared` the `function script` library every module can call. Design: `docs/adr/6-script-modules.md`.
- `config/wasm/npcs.json` and `events.json` place every NPC and event label: each entry names its `module` and `entry`. The server loads every `*.wasm` of `scripting.modules_dir` (`config/wasm/modules`), so a new module needs no registration beyond its manifest entries.
- Scripts are written against `lib/script-sdk-2` (`docs/script-sdk-2.md`), on top of the wire protocol of `lib/script-sdk` (`Function`, `Request`, `Value`, `Entry`). A module registers its scripts with `script_sdk_2::script_module!` in its `lib.rs`. Most converted files are hand-maintained now; a file whose first line is `// Generated by tools/scripts-import/convert_sdk2.py` is still generator output.
- `lib/script-runtime` runs the modules from one pooling allocator shared by all of them (4000 concurrent runs, 1 MiB of linear memory each, a 256 KiB guest stack). Guests have no WASI.
- A script request that needs the server goes through `server/src/server/script/game_api.rs` to a handler in `server/src/server/service/script_*.rs`. Behavior the server only partly implements is listed in `docs/script-server-gaps.md`: read it before relying on a command.
- `tools/scripts-import/convert_npcs.py --write` targets the layout from before the module split and must not be run until it is ported. `gen_sdk2_constants.py` and `gen_sdk2_bonus.py` still work.

### Configuration and Data
- `config.json`: Main server configuration (copy from `config.template.json`)
- `config/wasm/`: script assets: `modules/*.wasm` (built, tracked), `npcs.json`, `events.json`, `items.json`, `pets.json`, `map_flags.json`
- Embedded sled database for persistent data (accounts, characters, etc.)

### Sending packet to the client
- All services have an instance of `client_notification_sender: SyncSender<Notification>,` in their structure
- `Notification` is an enum implemented at `server/src/server/model/events/client_notification.rs`
- `Notification` inner structures contains raw packet data Vec<u8> which come from any packets structure `raw` field
- `server/src/server/mod.rs` contains implementation of `client_notification_thread` which route and send packet to the right socket(s) 

### Server state
- `ServerState` (characters and the data handlers touch) sits behind a `parking_lot::Mutex` in `Server`. Only the game loop and the movement loop lock it (`Server::lock_state()`), for the length of one iteration. Everything else receives `&ServerState`/`&mut ServerState` as a parameter, never call `lock_state()` from a service: the lock is not re-entrant and panics after a 10s timeout.
- A service that needs a `&mut Character` and the rest of the state removes the character first (`ServerState::with_character_taken`, or `characters_mut().remove` + `insert_character`); never hold a reference into `characters` while passing `state` on. `characters_mut_with_map_instances()` gives disjoint borrows for loops.
- `Server` and `ServerState` are `Send + Sync` through the type system; there is no `unsafe impl` or `MyUnsafeCell`. `Packet`, `Skill` and `InventoryRepository` require `Send + Sync`.
- Tests may call `server.state()`/`state_mut()` (cfg(test) shims returning the lock guard): keep the guard short, do not hold it across a call that locks (`Server::game_loop_iteration`, other `state()` calls).
- Request-handler and notification threads read `Server::sessions()` (`SessionRegistry`, a DashMap) and `Server::directory()` (`CharacterDirectory`, positions published at the end of each loop iteration, up to one tick stale). For anything else, enqueue a `GameEvent`.
- `MapInstanceState` is behind a `parking_lot::RwLock` (`state()`/`state_mut()` panic after a 10s timeout instead of hanging).
- Stores that only need character ids or are read from several threads are cloneable handles with their own lock, not `ServerState` fields: `Server::duels()`, `Server::map_flag_overrides()`, `Server::siege()`, `Server::battlegrounds()` (team definitions, scores, queues), `ScriptWorldService`'s party bookings and guild alliance requests. Battleground membership lives on the character (`Character::bg_id`, `bg_tracking`) and rosters are derived by scanning characters. `ServerState` keeps clones of the map flag and siege handles so `state.map_flags()` and `state.siege_active()` still work.


## Documentation
- When reading documentation, never read for "re" (or "renewal") version of the game

# How to 
This section contains guidance for common implementation tasks

## How to implement handling of a new packet?
- Add a new condition in `server/src/server/request_handler/mod.rs` to downcast_ref the packet to handle, the prompt MUST contains the packet structure
- Add a new function to handle this packet following this pattern: `pub fn handle_MY_PACKET_DESCRIPTION(server: &Server, context: Request)`
- Add a payload struct and its `impl GameEventHandler` in the matching domain file of `server/src/server/model/events/game_event/` (`character.rs`, `skill.rs`, `world.rs`, `script.rs`, `lifecycle.rs`), then register it in the `game_events!` list of `game_event/mod.rs`
- Payload fields are generally the `char_id` of type `u32` and other `Packet` structure fields except: `packet_id`, `raw`, `*_raw`
- The handler is `fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String>`, it runs in the game loop, and its logic lives next to the payload struct. Events tied to a character override `required_character()` (and `also_affects()` for other involved characters) so the game loop holds them back while that character is logging out
- Implement the business logic in a service present in `server/src/server/service/` a service function usually have following signature: `pub fn use_item(&self, server_ref: &Server, runtime: &Runtime, character: &mut Character, game_event_arguments: MyGameEventArguments)`
- In addition to business logic the service can also send packet to the client using: `self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_to_send.raw)))
                        .unwrap_or_else(|_| error!("Failed to send notification packet_to_send to client"));`
- Implement unit test for the newly added service function
- Map instance events follow the same pattern with `MapEventHandler` and `map_events!` in `server/src/server/model/events/map_event/`
- Script world requests are split by domain sub-enums in `server/src/server/service/script_world_requests.rs`, with each domain handler in its `script_world_*.rs` file

## How to implement or route a player skill?
- Every skill in `server/src/server/script/skill_metadata.json` has a `Route` (`SkillRoute` in `skill_metadata.rs`): `Damage`, `Status`, `AreaStatus`, `Ground`, `Recovery`, `Inventory`, `Movement`, `Spirit`, `Dispel`, `Tarot`, `Estimate` (script operations), `Menu`, `Guild`, `Native` (`lib/skills` offensive struct), `Passive`, `Actor` or `Unrouted` (no handler yet).
- The metadata is generated from rathena by `tools/scripts-import/import_skill_operations.py`, which only adds missing skills and keeps existing entries (hand-edited fields and `Route` included). A new skill has no `Route`: set it by hand.
- A plain buff (no damage, no unit, `Self`/`Support` target, a `Status` that exists in `StatusChangeKind`) needs no `Route` and no Rust: add the status kind in `lib/models/src/status_change.rs` (`bonuses()` for stat effects, `apply_status_for_target` for start values, `adjust_snapshot_for_target` for ASPD/speed) and regenerate the status metadata.
- Anything else: set the `Route`, then add the bespoke code in the matching `server/src/server/script/skill_*.rs` file (name branches live there). Lowering the number of `Unrouted` skills means editing `UNROUTED_BASELINE` in the tests of `script/skill.rs`.
- Passive effects are read through `known_skill_level`/`learned_level`: stat effects in `StatusService::passive_skill_bonuses` (`status_service.rs`), damage effects in `battle_service.rs`.
- Monster (`NPC_*`) skills with bespoke server behavior are `MonsterSkill` variants in `server/src/server/script/monster_skill.rs`. Read them with `metadata.monster_skill()`; do not compare skill names.

## How to change or add an NPC script?
- Edit the crate under `scripts/<group>/<module>/src/`, run `cargo run --package tools --bin scripts-build --release`, then `cargo test --release -p server npc_trace`. The trace test fails when what a script asks the host to do changes; if the change is intended, rerun it with `UPDATE_TRACES=1` and commit `server/src/server/script/npc_traces.json` with the script.
- A new NPC or event needs its handler in the `npcs`/`events` table of the module's `script_module!` and an entry in `config/wasm/npcs.json` or `events.json` (`module`, `entry`). Trace keys contain the module name, so renaming a module rewrites its keys.
- A new module is a directory with a `Cargo.toml` (package name = directory name, `crate-type = ["cdylib"]`, dependencies `script-sdk-2.workspace = true` and `shared.workspace = true`, copy a neighbour) under `scripts/<group>/`. The workspace glob and `scripts-build` pick it up.
- A command the host does not support yet is added as a `Function` variant in `lib/script-sdk`, an optional typed wrapper in `lib/script-sdk-2`, and a handler routed from `game_api.rs`. Rebuild every module after changing `lib/script-sdk`.
- After adding a constant a script names, run `python tools/scripts-import/gen_sdk2_constants.py`. It only sees `ctx.constant("NAME")` and `constants::NAME`, not names kept in arrays; the server retries a name it does not know in other spellings (upper, lower, title case).

## How to implement a bitflag?
- Create the enum in `lib/models/src/enums/` (add to existing file or create new one)
- Use derive macro: `#[derive(WithMaskValueU64)]` (or U32, U16, U8 depending on size needed)
- First variant must have explicit `#[mask_value = 1]` attribute
- Subsequent variants auto-double (2, 4, 8, 16...)
- Use `#[mask_value = X]` to override specific values
- Use `#[mask_all]` for a variant combining all flags
- If creating a new file, add `pub mod filename;` to `lib/models/src/enums/mod.rs`

Example:
```rust
#[derive(WithMaskValueU64, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MyFlags {
    #[mask_value = 1]
    FirstFlag,
    SecondFlag,      // = 2
    ThirdFlag,       // = 4
    #[mask_all]
    All,
}
```

Traits provided: `from_flag(value)`, `try_from_flag(value)`, `as_flag()`

**Important**: Never use hex values directly for flags. Always use enum variants with `.as_flag()` and bitwise OR:
```rust
// Good
let mode = MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag();

// Bad - never do this
let mode = 0x81;
```

## Reference Implementation

Relevant pre-renewal documentation and source files are available under `../rathena/doc` and `../rathena`.
Consider `rathena` a full, complete reference implmentation.