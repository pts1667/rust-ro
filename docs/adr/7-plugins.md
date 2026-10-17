# Server plugins

Builds on [script modules](6-script-modules.md) and [the typed script SDK](4-typed-script-sdk.md). Author and operator guide:
[docs/plugins.md](../plugins.md).

**Date**: 2026-10-17

**Status**: Accepted

# Context
Custom server behaviour is written today by editing the repository: a crate under `scripts/`, entries in the shared
`config/wasm/npcs.json` and `events.json`, and Rust in `server/` for anything a script cannot do. A server operator who wants a
custom shop, a rate change or an event has to fork. We want plugins: a package that is added to a server, declares what it is,
initialises itself and adds to or changes server, map and NPC behaviour, without editing the base game.

What the script system gives us already:
- The server loads every `*.wasm` in `scripting.modules_dir`, and each module says which ABI it speaks (`script_abi`, 2).
- Modules are sandboxed by Wasmtime: no WASI, 1 MiB of linear memory, 10,000 host calls per run (`lib/script-runtime`).
- Scripts can place behaviour on events (`OnInit`, `OnTimer*`, `OnTouch`, `OnMyMobDead`...) and on five player events the server
  broadcasts to NPCs by name (`OnPCLoginEvent`, `OnPCLogoutEvent`, `OnPCDieEvent`, `OnPCKillEvent`, `OnPCLoadMapEvent`).
- A host function is added in a known way: a `Function` variant in `lib/script-sdk`, a wrapper in `lib/script-sdk-2`, a handler
  routed from `server/src/server/script/game_api.rs`.

What constrains the design:
- **Scripts run off the game loop.** A run is an async task; each host call is a request the game loop answers
  (`ScriptRequest` in `game_api.rs`). The game loop never waits for Wasm, which is how it stays under its 20 ms budget. A script
  can do things and react to things, but it cannot sit inside a game rule and return a value to it.
- **Runs had no CPU limit.** The runtime bounded memory and host calls, not time: a guest that loops without calling the host
  never yields its thread. That is acceptable for scripts we build ourselves, not for third-party code, and not at all for code
  the game loop would wait on.
- **Placement is global.** NPC and event names share one namespace in the base manifests; a module has no way to bring its own.

# Decision
A plugin is a script module with an identity, its own placements and configuration, and access to hooks, rates and
interceptors. All three phases below are implemented; `scripts/plugins/double_xp` is the example plugin and uses each of them.

## Package
```
config/plugins/<name>/
  plugin.wasm     the module
  npcs.json       optional, same shape as config/wasm/npcs.json without `module`
  events.json     optional, same shape as config/wasm/events.json
  config.json     optional, the plugin's own settings
```
- `scripting.plugins_dir` (default `config/plugins`) and `scripting.plugins` (the plugins to load, in order; empty loads none).
  A plugin is never loaded just because its folder exists. An entry is a name, or an object
  `{ "name", "enabled" (default true), "capabilities" (optional, narrows the declared list) }`, so an operator can turn a
  plugin off without deleting its line. A plugin listed twice, or one that is refused, stops the boot.
- The module exports `script_abi` and `script_run` like any script module, plus `plugin_buffer() -> ptr` (a 16 KiB guest
  buffer, `PLUGIN_BUFFER_BYTES`), `plugin_info() -> len`, `plugin_configure(len) -> status` and `plugin_intercept(len) -> len`.
  Messages through the buffer are JSON. `plugin_info` writes a `PluginInfo` (`lib/script-sdk/src/plugin.rs`): `name`,
  `version`, `sdk_abi`, `requires`, `capabilities`, `hooks` and `interceptors`. The server refuses a plugin whose `name` differs
  from its folder, whose `sdk_abi` differs from its own, whose `requires` are not loaded before it, or whose JSON files do not
  parse.
- In the server a plugin's module is known as `plugin:<name>`, which is what logs and `/db/npcs` show for its NPCs.
- `script_sdk_2::plugin_module!` generates the exports from one declaration:
  ```rust
  script_sdk_2::plugin_module! {
      info { name: "double_xp", version: "1.0.0", capabilities: [Rates, Broadcast, Dialogue] }
      init => init,
      npcs { "Rate Clerk" => rate_clerk }
      events { "Rate Clerk::OnMinute00" => announce }
      hooks { Login => welcome, MobKilled => count_kill }
      interceptors { ExpGain => scale_experience }
  }
  ```
- `scripts/plugins/*` crates belong to the scripts workspace; `scripts-build` installs them as
  `config/plugins/<name>/plugin.wasm`.

## Phase 1: content plugins
- **Placements.** The plugin's `npcs.json`/`events.json` are merged after the base manifests. A name already placed is a load
  error unless the entry says `"replaces": true`, in which case the plugin's entry takes the place of the base one; the fields
  the entry leaves out (map, cell, sprite, direction) are the base NPC's. `"test"` keeps its meaning. `"entry"` names the
  plugin's handler when it differs from the NPC name or label.
- **Init.** `Entry::PluginInit` runs once per plugin, in load order, queued after the base `OnInit` events. It runs like an
  event with no player attached. Dialogue windows are refused in runs without an NPC (init and hooks), and announcements there
  are allowed for the whole server only.
- **Configuration.** `config.json` is read and validated at boot and handed to the plugin as JSON text: `ctx.plugin_config::<T>()`
  decodes it with serde (`ctx.plugin_config_json()` gives the text). The runtime answers `PluginConfig` itself, without a game
  loop round trip. A plugin never touches the filesystem.
- **Capabilities.** A plugin lists what it needs: `Dialogue`, `World` (warps, spawns, map flags, NPCs, instances), `Economy`
  (items, zeny, shops, storage), `Character` (job, levels, skills, statuses, quests), `Rates`, `Accounts`, `Broadcast`.
  `Character` was added to the original list: changing a character is neither world nor economy. Every `Function` belongs to
  one capability or to none (reads, the plugin's own variables and timers), checked by an exhaustive match, and writing `Zeny`
  or `Hp` or buying and selling count as their capability. The runtime refuses a request outside the plugin's list before it
  leaves the guest. Base modules hold every capability. `scripting.plugins` may narrow a plugin's list, never widen it.
- **Isolation of state.** Server and account variables a plugin uses (`$`, `$@`, `#`, `##`) are renamed `<sigil><plugin>::name`
  on their way to the host, so two plugins (or a plugin and the base scripts) cannot overwrite each other's by accident.
  Character variables are shared, as rathena scripts expect.

## Prerequisite for phase 2: a CPU deadline
- Wasmtime epoch interruption is on for the shared engine. A `wasm-epoch` thread increments the epoch every millisecond.
  Fuel was rejected: it instruments every basic block, a cost every script pays on every instruction, where the epoch check is
  a load and compare at loop heads and function entries.
- Async runs (all scripts and plugins) yield their thread at each epoch tick and continue on the next poll. Each run has a CPU
  budget, `scripting.cpu_budget_ms` (default 2000), that counts guest time only: time waiting for host replies or for the next
  poll is not charged. A run past its budget traps with "Script exceeded its CPU budget". Base scripts get the same limits; the
  trace test and the generated-NPC smoke test pass with them.

## Phase 2: observer hooks
- `script_sdk::Hook` is a closed, typed enum: `Login`, `Logout`, `MapEnter`, `LevelUp` (base or job), `JobChange`, `Kill`,
  `Die`, `MobKilled`, `ItemObtained`, `ItemUsed`, `TradeCompleted`, `SkillUsed`, `PartyChanged`, `GuildChanged`. Each carries
  its own fields (who, what, where), not a loose argument list.
- A hook runs as `Entry::Hook(hook)` with the payload, attached to the character it is about while that character is online,
  fire-and-forget like an event. A call site queues a `PluginHook` game event only when a loaded plugin declared that kind: one
  atomic load and bit test otherwise.
- `Logout` runs detached: the session is going away. `MapEnter` also fires for the placement at login.
- Hooks see what already happened; they cannot cancel it. A plugin acts on it through ordinary host calls, which the game
  loop answers in its next iteration.
- The rathena `OnPC*Event` labels stay as they are, for converted scripts.

## Rates
- `ctx.rates().set(Rate::BaseExp | JobExp | Drop | CardDrop, percent)` sets the plugin's share of a rate (`Rates` capability);
  `get` reads the effective value. Shares of all plugins multiply, so two plugins that each double exp give four times, and a
  plugin that sets its share back to 100 removes only its own effect. The effective percent is kept in atomics: exp rates are
  applied in the game loop on each kill, drop and card rates in the map instance threads, neither takes a lock.

## Phase 3: interceptors
An interceptor is a pure function the game loop calls and waits for, to change or refuse what the game is about to do.
- **Shape.** The host writes the decision (`Interception`) as JSON into the plugin buffer and calls `plugin_intercept(len)`; the
  guest writes back a `Verdict`: `Allow`, `Refuse(message)`, `Exp { base, job }` (for `ExpGain`) or `Chat(text)` (for `Chat`).
  An answer of the wrong kind counts as a failure. JSON stays: encoding is not what the call spends its time on (below).
- **No host calls.** The interceptor instance is built against a linker whose imports all trap, so a call to the host fails.
  Interceptors read their configuration through `script_sdk_2::configured::<T>()`, which the server provides once through
  `plugin_configure`. Anything with a side effect goes back through a hook or a normal run.
- **Synchronous and bounded.** Each plugin keeps one warm instance. The call has an epoch deadline a few ticks away as its hard
  stop (the epoch has millisecond resolution), and its measured time is checked against `scripting.interceptor_deadline_us`
  (default 200 µs). A trap, an overrun or an answer that does not decode is logged and the decision goes on as if that plugin
  had allowed it; the instance is rebuilt for the next call. After `scripting.interceptor_failures` (default 10) failures of
  one kind, that interceptor of that plugin is turned off until restart.
- **Order.** Plugins are consulted in load order; each sees the previous plugin's answer. The first refusal wins, and its
  message is shown to the character (except for `ExpGain`, refused silently).
- **The six of the first set** all landed: `TradeRequest`, `Warp` (portals, scripts, Teleport, Butterfly Wing; not the placement
  at login nor the respawn of a dead character), `ItemDrop`, `ExpGain` (after the server and plugin rates), `ItemUse`, `Chat`
  (public chat, refused or rewritten). Measured on `ExpGain` with the example plugin, in release: mean 2 µs, p99 6 µs,
  max 30 µs per call, including JSON both ways. When no plugin intercepts a kind, the call site costs one bit test.
- **Hot paths take parameters, not calls.** Per-hit damage, regeneration and movement are not intercepted call by call. A
  plugin publishes numbers for them instead, through the rates above, from its init or a hook.

## Out of scope
- **Custom packets.** A plugin that adds client packets would need its ids registered in the packet framing table
  (unregistered headers drop the connection) and a reserved range away from the login extension (`0xC3E0`-`0xC3E3`). A later ADR.
- **Hot reload.** Plugins load at boot; changing one means a restart.
- **Skills.** Skills are native Rust implementing the `Skill` trait in `lib/skills`. A plugin can react to a skill (hook), not
  add one.

# Consequences
- A server can be customised by adding a folder and a line of configuration. The base game stays untouched, and a plugin's NPCs,
  events and variables are attributable to it in logs and in `/db/npcs`.
- The game loop's budget is protected by construction: init and hooks never make it wait, and interceptors are pure,
  deadline-bounded code.
- Epoch interruption applies to every script, not just plugins: a base script that computes for more than 2 s of CPU now traps.
  None does; the trace and smoke tests check it.
- Capability checks put a match on every host call of a plugin, in the runtime; base modules skip it.
- `PluginInfo`, `Hook`, `Interception`, `Verdict` and the new `Function` variants (`PluginConfig`, `SetRate`, `GetRate`, appended)
  are part of the stable SDK surface. Adding them did not change `ABI_VERSION`, so base modules built before keep loading.
  Changing them later means bumping it, which invalidates every plugin built for the old one; the server says which plugin was
  refused and why.
- The trace test (`npc_trace`) covers the base modules only; plugins are tested by their authors (unit tests of the crate, and
  `server/src/server/plugins/tests.rs` for the example).

# Alternatives considered
- **Native plugins (`dylib`, or Hercules' HPM model of pre/post hooks on any function).** The most power, but Rust has no stable
  ABI, so a plugin is tied to one exact server build, a panic or bad pointer takes down the process, and plugins hook into
  internals that change with every refactor. Wasm keeps the sandbox, the budgets and a versioned surface.
- **An embedded interpreter (Lua, Rhai).** Easier for operators who do not write Rust, but a second scripting stack next to the
  Wasm one, with its own API to keep in step. A plugin can be written in any language that compiles to Wasm against the same
  ABI later.
- **Plugins as manifest entries only (phase 1 without hooks or interceptors).** Cheap, but it only covers content; rates,
  rules and reactions would keep requiring forks.
- **Wasmtime fuel for the CPU budget.** Exact and deterministic, but it slows every guest instruction; the epoch costs almost
  nothing and a millisecond resolution is enough for both budgets.
