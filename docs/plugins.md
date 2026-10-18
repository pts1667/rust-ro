# Plugins

A plugin adds NPCs, events, reactions and rule changes to a server without editing the base game. It is a Wasm script module
in its own folder, turned on by one line of configuration. Design and rationale: [ADR 7](adr/7-plugins.md). The examples are
`double_xp` (`scripts/plugins/double_xp`, installed in `config/plugins/double_xp`) and `ui_shop`, which opens its own shop
window ([plugin UI](plugin-ui.md)).

## For operators

### Turning a plugin on or off
Plugins are listed under `scripting` in `config.json`, and only listed, enabled plugins load:
```json
"scripting": {
  "plugins_dir": "config/plugins",
  "plugins": [
    "double_xp",
    { "name": "event_shop", "enabled": false },
    { "name": "pvp_rules", "capabilities": ["World", "Broadcast"] }
  ]
}
```
- An entry is a folder name under `plugins_dir`, or an object with `name`, `enabled` (default `true`) and `capabilities`.
- Plugins load, initialise and intercept in the order of the list. A plugin that `requires` another must come after it.
- `capabilities` narrows what the plugin may do (see below). It can only remove capabilities the plugin declared, never add.
- Changing the list, a plugin's files or its `config.json` takes a restart.
- A plugin that cannot load stops the boot with the reason (`Plugin x is refused: ...`): a server never runs without a plugin its
  operator listed. Set `"enabled": false` to boot without it.

`config.template.json` ships `double_xp` listed and disabled. To try it, set `"enabled": true` (or write `"plugins": ["double_xp"]`)
and restart: the server announces the event, the Rate Clerk stands in Prontera (161, 186), monsters give 200% experience and
150% drop chances. Its settings are in `config/plugins/double_xp/config.json`. `ui_shop` ships listed and disabled too: its Curio
Trader stands in Prontera (158, 186) and needs nostalro-client to draw its window.

### Package layout
```
config/plugins/<name>/
  plugin.wasm     the module; the folder name must be the plugin's name
  npcs.json       optional: NPCs to place
  events.json     optional: event labels to register
  config.json     optional: the plugin's settings, any JSON
  ui/             optional: images of its windows and overheads, served to clients at /data/plugin/<name>/
```

### Capabilities
A plugin declares the groups of host functions it calls; a call outside them is refused and the run stops with an error in the
log.

| Capability | What it allows |
| --- | --- |
| `Dialogue` | dialogue windows, messages, cutins, effects and sounds shown to the attached player |
| `World` | warps, monsters, map flags, NPCs, instances, battlegrounds, castles, waiting rooms |
| `Economy` | items, zeny, shops, storage, refining, mail |
| `Character` | job, levels, experience, skills, statuses, healing, quests, save point |
| `Rates` | the server's experience and drop rates |
| `Accounts` | permission groups and account data |
| `Broadcast` | announcements to a map or the whole server |
| `Interface` | plugin windows and overhead notifications ([plugin UI](plugin-ui.md)) |

Reads (character info, items, time, map users...), the plugin's own variables, timers, events and what it shows bots need no
capability.

### Settings
| Setting | Default | |
| --- | --- | --- |
| `scripting.plugins_dir` | `config/plugins` | where plugin folders are |
| `scripting.plugins` | `[]` | the plugins to load, in order |
| `scripting.cpu_budget_ms` | 2000 | CPU time a script run may spend, for every script, not just plugins |
| `scripting.interceptor_deadline_us` | 200 | time an interceptor call may take |
| `scripting.interceptor_failures` | 10 | failures after which an interceptor is turned off until restart |

## For authors

### A new plugin
A plugin is a crate under `scripts/plugins/<name>` (the scripts workspace picks it up), with the package name as plugin name:
```toml
[package]
name = "my_plugin"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
script-sdk-2.workspace = true
serde = { version = "1.0", features = ["derive"] }
```
`cargo run --package tools --bin scripts-build --release` builds it to `config/plugins/<name>/plugin.wasm`; the JSON files of
the folder are written by hand. A plugin can also be built outside this repository, against the same `script-sdk-2`, and dropped
in a folder.

`src/lib.rs` declares the plugin with `plugin_module!`. Every section up to `interceptors` is required, empty ones included;
`windows` and `bot_actions` are optional ([plugin UI](plugin-ui.md)). NPC and event names must be sorted.
```rust
use script_sdk_2::{Ctx, Hook, Interception, Script, Verdict};

script_sdk_2::plugin_module! {
    info { name: "my_plugin", version: "0.1.0", requires: [], capabilities: [Dialogue, Broadcast] }
    init => init,
    npcs { "Greeter" => greeter }
    events { "Greeter::OnClock1200" => noon }
    hooks { Login => welcome }
    interceptors { Chat => no_shouting }
}
```
Script bodies are written as in any module ([the script SDK](script-sdk-2.md)): NPC and event handlers are `fn(&Ctx) -> Script`.

### Placing NPCs and events
`npcs.json` has the shape of `config/wasm/npcs.json` without `module`:
```json
[{ "map_name": "prontera", "name": "Greeter", "sprite": "4_F_KAFRA1", "x": 150, "y": 180, "dir": 4 }]
```
- `entry` names the handler when it differs from `name`.
- A name the base game or an earlier plugin already placed is a load error, unless the entry has `"replaces": true`. A
  replacing entry takes over that NPC; the fields it leaves out (map, cell, sprite, direction, size) are the original's, so
  `{ "name": "Kafra Employee#prt", "replaces": true }` swaps the behaviour and keeps the NPC where it was.
- `"test": true` places the NPC only when `scripting.test_npcs` is on.

`events.json` maps labels to options, like `config/wasm/events.json`: `{ "Greeter::OnClock1200": {} }`. `entry` and `replaces`
work as for NPCs. Labels follow the rathena names (`OnInit`, `OnMinute00`, `OnClock1200`, `OnTimer<ms>`...).

### Initialisation
`init` runs once at boot, after the base scripts' `OnInit`, in plugin order. No player is attached and there is no NPC:
dialogue windows are refused, announcements go to the whole server. It is where a plugin sets its rates or spawns its monsters.

### Configuration
`ctx.plugin_config::<T>()` decodes `config.json` with serde (`{}` when there is none); `ctx.plugin_config_json()` gives the text.
Give the settings struct `#[serde(default)]` so a missing file or field falls back to defaults.

### Variables
Server and account variables (`$x`, `$@x`, `#x`, `##x`) are the plugin's own: they are stored as `$my_plugin::x`, so they cannot
collide with the base scripts or other plugins. Character variables (`x`, `@x`) are shared with every script.

### Rates
With the `Rates` capability, `ctx.rates().set(Rate::BaseExp, 150)` sets this plugin's share of a rate (`BaseExp`, `JobExp`,
`Drop`, `CardDrop`), in percent of the configured rate. Shares of all plugins multiply; setting 100 removes this plugin's
effect. `ctx.rates().get(rate)` reads the effective percent. Rates are the way to change per-hit, per-kill or per-drop numbers
cheaply: the server reads them without calling the plugin.

### Hooks
A hook is a `fn(&Ctx, &Hook) -> Script` that runs after something happened to a character, with that character attached while
they are online. It cannot cancel what happened; it reacts through ordinary calls.

| Hook | Fires when |
| --- | --- |
| `Login` | the character enters the game |
| `Logout` | the character leaves (runs without the player attached) |
| `MapEnter` | the character arrives on a map, including at login |
| `LevelUp` | base or job level up (`kind`, `level`) |
| `JobChange` | the job changes |
| `Kill` / `Die` | the character killed another player / died |
| `MobKilled` | the character killed a monster (`mob_id`, map, cell) |
| `ItemObtained` / `ItemUsed` | an item enters the inventory / is used |
| `TradeCompleted` | a trade went through (fires for both sides) |
| `SkillUsed` | the character used a skill |
| `PartyChanged` / `GuildChanged` | joined (id) or left (0) a party / guild |

Hooks are queued by the game loop and run in the next tick; a hook nobody declared costs nothing.

### Interceptors
An interceptor is a `fn(&Interception) -> Verdict` the game loop waits for before it does something. It runs in a separate
instance with no host calls at all (any call traps) and a deadline of `interceptor_deadline_us`, so it only computes from the
decision and its settings, read with `script_sdk_2::configured::<T>()`.

| Interception | Possible verdicts |
| --- | --- |
| `TradeRequest` | `Allow`, `Refuse(message)` |
| `Warp` (portals, scripts, Teleport, Butterfly Wing; not login placement or respawn) | `Allow`, `Refuse(message)` |
| `ItemDrop` (to the floor) | `Allow`, `Refuse(message)` |
| `ExpGain` (per monster, after rates) | `Allow`, `Refuse(message)` (no exp), `Exp { base, job }` |
| `ItemUse` | `Allow`, `Refuse(message)` |
| `Chat` (public chat) | `Allow`, `Refuse(message)`, `Chat(text)` |

Plugins are asked in order, each seeing the previous answer; the first refusal wins and its message (if not empty) is shown to
the player, except for `ExpGain`, which happens on every kill and stays silent. A verdict of the wrong kind, a trap or an overrun counts as a failure: the decision goes on as if the plugin had
allowed it, and after `interceptor_failures` failures that interceptor is turned off until restart. Static state in the module
does not survive a failure: the instance is rebuilt.

### Windows, overheads and bots
With the `Interface` capability a plugin opens its own windows (text, tables, images, item icons, buttons, inputs) and handles
their events in `windows { "id" => handler }`, and draws notifications above units with `ctx.ui().overhead(..)`. It also chooses
what bots observe (`ctx.bot().observe`) and which actions they may run (`bot_actions { }`). All of it is in
[plugin UI](plugin-ui.md).

### Limits
- 1 MiB of linear memory, 10,000 host calls and `cpu_budget_ms` of CPU per run (waiting for the host is free).
- Interceptor messages are JSON in a 16 KiB buffer.
- No filesystem, network or clock other than through host calls.

### Testing
Script and interceptor functions are plain Rust: test them in the crate with `cargo test` from `scripts/`, using
`script_sdk_2::MockTransport` for the host (see the example's tests). `server/src/server/plugins/tests.rs` loads the example
in a test server and checks its requests, hooks and interceptor end to end; a new plugin can follow it.
