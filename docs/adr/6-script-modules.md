# Script modules and name dispatch

Refines [the typed script SDK](4-typed-script-sdk.md). It replaces the single numeric-ABI module of [compiled WebAssembly scripts](3-wasmtime.md).

**Date**: 2026-10-09

# Context
All scripts used to live in one 41 MB module (`game_scripts.wasm`). The server called an export by name (`run_npc`, `run_event`, `run_pet`) with a numeric id. Ids were positional (`npc_10506`), so they shifted whenever the source list changed, and several event ids collided (event 104849 reached the wrong NPC). A change to one script rebuilt and recompiled everything.

# Decision
- **One crate per module** in one cargo workspace (`scripts/Cargo.toml`, one `Cargo.lock`, one `target/`), each built to `config/wasm/modules/<name>.wasm`:
  - `scripts/{towns,misc,jobs,quests}/<module>` hold the NPCs converted from rathena, about 100 modules of 1,000 to 20,000 lines. A module is one rathena folder, or one large source file, named after its path (`quests_ein`, `misc_other_arena_aco`). The loose small files of a folder share a `*_misc` module, in chunks of at most about 10,000 lines.
  - `systems` holds the NPCs that front server-side systems (warper, job masters, castles, weddings, battlegrounds).
  - `items` holds the item scripts and `pets` the pet scripts, both keyed by id.
  - `shared` is the `function script` library that `callfunc` reaches from any module.
- **One ABI.** A module exports `script_abi` (2) and `script_run`. The host passes the script as a `script_sdk::Entry` (`Npc(name)`, `Event("name::Label")`, `Item(id)`, `Program(id)`, `Pet(id)`, `PetSupport(id)`, `PetProgram(id)`), which the guest reads through `rust_ro::entry`. The registry binary-searches sorted tables and a compile-time assertion rejects unsorted or duplicate keys. ABI 1 (a numeric export per script) is removed.
- **Manifests route by name.** Every entry of `npcs.json` and `events.json` carries `module` and `entry`. The server interns each pair into a `u32` handle (`script/entries.rs`), so the `u32` that NPCs and events carry keeps its shape. Code that needs one particular script finds its handle by name (`system_npc("shop")`).
- **`ScriptVm`** runs NPCs and events in the module the handle names. **`ItemVm`** runs items and pets.
- **Guard.** `npc_traces.json` records a digest of what every NPC and event asks the host to do on repeatable random answers. `npc_trace.rs` replays it. NPCs whose placement passes arguments (castles, battlegrounds, breeders, shops) are traced once per placement, with its arguments. Request shapes the host treats alike, such as a write by prefixed name and a scoped write, trace alike. A script changed on purpose changes its digest: rerun with `UPDATE_TRACES=1`. The digests were recorded after every migrated script matched the numeric module, trace for trace.

# Consequences
- A script edit rebuilds one module, and the generated `.cwasm` caches are per module. The server loads every `*.wasm` of `scripting.modules_dir` (`config/wasm/modules`), compiling them in parallel, so a module needs no registration.
- Names are stable, so a manifest or a log line says which script ran.
- The `systems` scripts are written against `script-sdk-2` like the rest. Their port replayed the snapshot unchanged before two deliberate changes: the wedding ceremony variables took rathena's `$@wedding` names, and the castle flag and steward read the guild master through `getguildmaster`, which says `null` when the master is unknown.
- Generated item and NPC calls are mostly `ctx.call(Function::X, ..)`. Typed wrappers can replace them one function at a time, and the trace snapshot flags any change in behaviour.
- The numeric module had one known defect the migration fixed: the colliding event id above. The trace test skips nothing for it now, because its digest was recorded from the correct script.
- Adding a module means a crate directory with a `cdylib` manifest under `scripts/` (a `scripts/<group>/<name>/Cargo.toml` is picked up by the workspace glob, and `scripts-build` installs it), and `module` entries in the manifests. `scripting.modules` only replaces the file of a module by name.
