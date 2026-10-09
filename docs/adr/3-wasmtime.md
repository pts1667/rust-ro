# Compiled WebAssembly game scripts

The server runs NPC and item code compiled from Rust to WebAssembly with Wasmtime. The previous rAthena interpreter and custom bytecode compiler have been removed from the dependency graph.

`scripts/` is an independent Rust guest workspace targeting `wasm32-unknown-unknown`. It builds several modules (`config/wasm/<name>.wasm`: `towns`, `misc`, `jobs`, `quests`, `systems`, `items`, `pets`), each a separate crate. The NPC and event manifests name the module and entry of every script; items and pets are looked up by id. The server compiles each portable module once when starting and shares the compiled code across executions. The first design was a single module addressed by numeric ids; it is retired (see [ADR 4](4-typed-script-sdk.md)). Each invocation owns a separate Wasmtime store. Game state and NPC variables remain owned by the server.

`lib/script-sdk` defines the versioned guest API. Requests and replies contain typed enums, numbers, strings, and arrays serialized into bounded guest memory buffers. `lib/script-runtime` links that API to asynchronous Rust host functions, verifies the ABI before allowing host calls, and enforces fuel, memory, and host-call budgets. Guests receive no WASI filesystem, network, or process capabilities.

NPC conversations run as asynchronous tasks. Dialogue, menus, and input suspend the guest while awaiting typed client responses. Other requests pass through the game event queue and execute on the game loop. A session generation prevents queued operations from an old conversation affecting a replacement conversation. Disconnecting, opening another NPC, or reaching the configured timeout cancels the interaction.

Items run synchronously with a snapshot of character status and stage their effects. The server applies those effects only after the guest succeeds and the effect list passes validation. Static equipment bonuses are evaluated at startup; dynamic bonuses are evaluated with current status. NPC variable batches spanning character, account, and server scopes use a transaction across sled's numeric and string trees. Shop additions and charges use the existing inventory transaction, including configured price overrides. No explicit database flushing is performed.

The initial migration converted all 2,570 repository item sources to generated Rust, plus the enabled NPC content: 78 placements and 364 pre-renewal warp destinations. The original sources remain available as import inputs. They are not interpreted by the running server. Game operations that the server has not implemented return an error; their presence in an item source does not authorize silently consuming that item.

New NPC behavior should be authored in Rust using `script-sdk-2` and registered in the NPC manifest. Item sources can be converted offline with `tools/scripts-import/import_items.py`. Source hashes and a catalog fingerprint exported by the guest detect catalog or manifest changes without a matching script build. Rebuild the Wasm bundle and restart the server when changing executable script code. Persistent state is not stored in guest memory or native Wasmtime artifacts.

Build with `cargo run --package tools --bin scripts-build`. The Rust toolchain must have `wasm32-unknown-unknown` installed. The checked-in portable module permits running the server without a guest compiler installation.

Wasmtime traps and cancellation do not roll back host actions that have already committed. Related persistent changes must be expressed as a batch; NPC conversations commit individual host actions between player interactions. Sled background syncing remains the durability policy.
