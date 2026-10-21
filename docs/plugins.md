# Plugins

A plugin adds NPCs, events, reactions and rule changes to a server without editing the base game. It is a Wasm script module
in its own folder, turned on by one line of configuration. Design and rationale: [ADR 7](adr/7-plugins.md). The examples are
`double_xp` (`scripts/plugins/double_xp`, installed in `config/plugins/double_xp`) and `ui_shop`, which opens its own shop
window ([plugin UI](plugin-ui.md)).

New to plugins? Start with the [plugin guide](plugin-guide.md): a first plugin, the common recipes, and what plugins
cannot do. This page is the reference.

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
- Changing the list, a plugin's files or its settings file takes a restart.
- A plugin that cannot load stops the boot with the reason (`Plugin x is refused: ...`): a server never runs without a plugin its
  operator listed. Set `"enabled": false` to boot without it.

`config.template.json` ships `double_xp` listed and disabled. To try it, set `"enabled": true` (or write `"plugins": ["double_xp"]`)
and restart: the server announces the event, the Rate Clerk stands in Prontera (161, 186), monsters give 200% experience and
150% drop chances by default. `ui_shop` ships listed and disabled too: its Curio Trader stands in Prontera (158, 186) and needs
nostalro-client to draw its window. `alive_towns`, also listed and disabled, puts 100 walking townsfolk in each of 22 towns
(see `scripts/plugins/alive_towns/README.md`); `{ "towns": ["prontera"] }` in `config/plugins/alive_towns.json` narrows it.

### Plugin settings
Every plugin declares its settings with a default and a description (its `info.json`, compiled into `plugin.wasm`; the boot
log names the plugin's title). To change some, write `config/plugins/<name>.json` next to the plugin's folder, with only the
keys to change:
```json
{ "exp_percent": 300 }
```
- Keys left out keep their default. Arrays and objects are replaced whole, not merged.
- A key the plugin does not declare, or a value of another JSON type than the default, refuses the plugin, so a typo stops the
  boot instead of being ignored. The refusal lists the keys the plugin knows.
- It is the plugin's only settings file. A `config.json` left inside the plugin's folder (the former location) refuses the
  plugin until it is moved.
- These files are local to the server, like `config.json`: git ignores `config/plugins/*.json`.

### Package layout
```
config/plugins/<name>/
  plugin.wasm     the module (with its info.json inside); the folder name must be the plugin's name
  npcs.json       optional: NPCs to place
  events.json     optional: event labels to register
  items.json      optional: its items (see Items)
  skills.json     optional: its skills, jobs.json its jobs (a scaffold, see Skills and jobs)
  ui/             optional: images of its windows, overheads and item icons, served to clients at /data/plugin/<name>/
config/plugins/<name>.json   optional: the operator's overrides of the plugin's settings
```

### Capabilities
A plugin declares the groups of host functions it calls; a call outside them is refused and the run stops with an error in the
log.

| Capability | What it allows |
| --- | --- |
| `Dialogue` | dialogue windows, messages, cutins, effects and sounds shown to the attached player |
| `World` | warps, monsters, map flags, NPCs (moving, walking, look), instances, battlegrounds, castles, waiting rooms |
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
Next to `Cargo.toml`, `info.json` describes the plugin and every setting it reads, with its default:
```json
{
  "title": "Double XP",
  "description": "An experience and drop rate event.",
  "config": {
    "exp_percent": { "default": 200, "description": "Percent of the experience a monster gives." },
    "milestone": { "default": 100, "description": "Kills between two messages to the player." }
  }
}
```
`title` and `description` are required, `config` may be empty. `plugin_module!` compiles the file into the module, so a built
plugin always carries its defaults, and `scripts-build` refuses to build a plugin whose `info.json` does not decode.

`cargo run --package tools --bin scripts-build --release` builds it to `config/plugins/<name>/plugin.wasm`; the JSON files of
the folder are written by hand. A plugin can also be built outside this repository, against the same `script-sdk-2`, and dropped
in a folder.

`src/lib.rs` declares the plugin with `plugin_module!`. Every section up to `interceptors` is required, empty ones included;
`windows` and `bot_actions` ([plugin UI](plugin-ui.md)), then `commands`, `items` and `item_bonuses` (below) are optional,
in that order. NPC and event names and item ids must be sorted.
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
- `"speed"` (20 to 1000 ms per cell) sets its walking speed. `"appearance"` draws a job sprite like a player:
  `{ "sex": 1, "hair": 5, "hair_color": 2, "clothes_color": 1, "head_top": 17, "head_mid": 0, "head_bottom": 0, "weapon": 0,
  "shield": 0 }` (head gear and weapons by view id, every field optional).

NPCs can also come from the plugin's code, computed from its settings at boot: `generated_npcs => my_npcs` in
`plugin_module!`, where `fn my_npcs(index: usize) -> Result<Option<PluginNpc>, String>` reads `script_sdk_2::configured()` and
returns the NPC at `index` (`None` past the last), an entry of the same shape as `npcs.json`. The server asks page by page, so
a guest never holds them all in its 1 MiB. They are placed after those of `npcs.json`, at most 5,000 per plugin; an error refuses the
plugin with its message, which is how a plugin rejects settings it cannot use.

`events.json` maps labels to options, like `config/wasm/events.json`: `{ "Greeter::OnClock1200": {} }`. `entry` and `replaces`
work as for NPCs. Labels follow the rathena names (`OnInit`, `OnMinute00`, `OnClock1200`, `OnTimer<ms>`...).

### NPC routines
An NPC's events can drive it around its map: `ctx.npc().walk_to_then(None, x, y, "Baker::OnAtOven")` walks it there and
runs the label on arrival with the walker's unit id as its argument (`ctx.arguments()`), where it can emote, talk
(`Function::NpcTalk`), turn or start its next walk. `OnInit` or an `OnTimer<ms>`/`OnMinute` label starts the loop. A map is
only loaded when a player first enters it (castles at boot), and `OnInit`, timers and clock labels reach only NPCs on loaded
maps, so an NPC elsewhere starts its loop from a `MapEnter` hook (`InitNpcTimer` on its name). Walking needs `World`; emotes and effects need `Dialogue`. A walk may cross the whole map: the server splits it into legs. A click
stops the NPC where it is. `ctx.unit_walk(..)` walks players and monsters the same way (see `docs/script-server-gaps.md`).

A plugin driving many NPCs should name them by unit id (`GetNpcId(0, [names])` answers all ids in one call) and send its
commands together: `ctx.batch()` collects calls and `send()` runs them in one game loop pass, instead of one round trip each
(see `docs/script-sdk-2.md`). `alive_towns` drives 100 NPCs per town from one timer per town this way.

### Initialisation
`init` runs once at boot, after the base scripts' `OnInit`, in plugin order. No player is attached and there is no NPC:
dialogue windows are refused, announcements go to the whole server. It is where a plugin sets its rates or spawns its monsters.

### Configuration
The plugin reads its settings as a JSON object holding every key of its `info.json`: the operator's value from
`config/plugins/<name>.json`, else the default.
- `ctx.plugin_setting::<u32>("exp_percent")` reads one key (a key `info.json` does not declare is an error);
  `ctx.plugin_config::<T>()` decodes them all into a struct, which needs no `Default`: every key is always present.
  `ctx.plugin_config_json()` gives the JSON text.
- Interceptors use `script_sdk_2::setting::<T>(key)` and `script_sdk_2::configured::<T>()`.
- `script_sdk_2::plugin_manifest()` returns the `info.json` itself: title, description, settings with defaults and descriptions.
- In the plugin's unit tests, `script_sdk_2::plugin::configure_for_test(include_str!("../info.json"))` gives the interceptor
  functions the defaults, as the server would.

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

Hooks are queued by the game loop and run in the next tick; a hook nobody declared costs nothing. Maps are named without
`.gat` (`prontera`).

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

### `@` commands
`commands { "name" => (handler, level, "usage") }` adds `@name`. The handler is a `fn(&Ctx, &PluginCommand) -> Script`
that runs with the player attached, like an NPC, in the plugin's job queue for that player; `command.args` holds the
words after the name. `ctx.reply(text)` writes a line in the player's chat window (a line break starts another line,
at most 50 lines per reply).
```rust
commands { "lucky" => (count_tickets, 0, "Tells how many Lucky Tickets you hold") }

fn count_tickets(ctx: &Ctx, _command: &PluginCommand) -> Script {
    let count = ctx.items().count(40001)?;
    ctx.reply(&format!("You hold {count} Lucky Tickets."))
}
```
- Names are 1 to 32 lower case letters, digits or `_`, typed in any case. A name or alias of a built-in command
  (`config/groups.json`'s `atcommands`), or one another plugin added, refuses the plugin.
- A group may use the command when its `level` is at least the command's level, when it lists the command in its
  `commands`, or when it has `all_commands`. `@commands` lists the plugin commands a player may use. Only `@` commands:
  `#` (on another character) is not offered.
- Bots run them with the `command` action, under `bots.command_group`, and get `{"command": name, "replies": [...]}`.
- A player who types a command while the previous one of the same plugin still runs waits in its queue (4 deep);
  past that they read "@name is busy". A failed run tells them "@name failed." and logs the error.

### Items
A plugin adds items in `items.json`, with ids from 40000 to 59999. An entry takes the fields of `config/items.json`
(`id`, `name_aegis`, `name_english`, `item_type`, `price_buy`, `weight`, `location`, `slots`, `equip_level_min`...;
`weight`, `job_flags` (every job), `class_flags`, `location`, `flags` and `trade_flags` default), but no `script`, plus
what only plugin items have:

| Field | |
| --- | --- |
| `description` | text shown by clients, a string with line breaks or a list of lines |
| `look` | an item of the game whose icon and floor sprite clients borrow |
| `icon` | a lower case `.png` of the plugin's `ui/` folder, preferred over `look` for the inventory icon |
| `dialogue` | the use script talks (`mes`, menus): it runs like an NPC conversation |
| `dynamic_bonuses` | the bonus script reads the wearer (level, refine, other gear) and runs on every status computation instead of once at boot |

```json
[{ "id": 40001, "name_aegis": "Lucky_Ticket", "name_english": "Lucky Ticket", "item_type": "Usable", "weight": 1,
   "description": ["A ticket for the lucky draw."], "look": 6153, "icon": "ticket.png" }]
```
Behaviour is Rust in the module, keyed by id: `items { 40001 => lucky_ticket }` for what using it does (an
`fn(&ItemUse) -> Script`) and `item_bonuses { 40002 => clover_charm }` for what wearing it or the card gives (an
`fn(&ItemBonus) -> Script`), written as in `scripts/items` ([the script SDK](script-sdk-2.md)). Item scripts get no `Ctx`:
they read the settings with `item.call(Function::PluginConfig, vec![])`, and their calls need the plugin's capabilities.

The boot refuses a plugin when:
- an id is outside the range, defined twice, or by two plugins; a `name_aegis` is taken by a game item or another plugin;
- a usable item (`Healing`, `Usable`, `DelayConsume`, `Cash`) has no `items` handler, or another type has one;
- an `item_bonuses` handler is on an item that is neither worn nor a card, or a handler has no item;
- `look` is not an item of the game, `icon` is not a file of `ui/`, or a worn item has no `location`.

The database keeps every plugin item's definition. A plugin turned off leaves its items in inventories and storages as
plain items without scripts (a warning lists them), and they come back to life when it returns; a later plugin reusing
the name of such an item renames the old one `<name>_<id>`.

Clients need the name, description and icon, which the stock client reads from its GRF: see the client catalog below.

### Skills and jobs (scaffold)
`skills.json` (ids 20000 to 20999) and `jobs.json` (ids 4500 to 4599) are read, checked and published in the client
catalog, but **no character can learn, use or become them yet**:
```json
[{ "id": 20000, "name": "RF_RUNE_STRIKE", "display_name": "Rune Strike", "max_level": 5, "target": "enemy",
   "sp_cost": [10, 12, 14, 16, 18], "range": 1, "look": 5, "description": ["..."], "icon": "rune.png" }]
```
```json
[{ "id": 4500, "name": "Rune_Fencer", "display_name": "Rune Fencer", "base_job": "Swordsman",
   "skills": [{ "skill": "RF_RUNE_STRIKE", "max_level": 5, "requires": [{ "skill": "SM_BASH", "level": 5 }] }] }]
```
`target` is `passive`, `self`, `enemy`, `ally` or `ground`; a tree names skills of the game or of any loaded plugin.
What remains before they play:
- characters hold skills as `SkillEnum`, generated from the game's skills and with no room for other ids; plugin skills
  need a skill store and a `Skill` implementation (hooks or a script entry) beside it;
- jobs are the `JobName` enum, used by stats, equipment and skill trees, so a plugin job needs its own record (based on
  `base_job`) in each of them;
- skill and job packets carrying plugin ids must go only to clients that announced they read the catalog: the stock
  client does not know them.

### Client catalog
At boot the server writes what clients need to draw plugin content into `/data/plugin-catalog.json` on the client data
port (`client_data`), listed in `/data/check` before the plugins' `ui/` files. Its types are `plugin_ui::catalog`
(`PluginCatalog { items, skills, jobs }`), shared with nostalro-client, which reads it at start: names, descriptions,
slots, `look` and `icon` of every plugin item, including retired ones (`plugin: null`). Without plugin content the
file is not served. The stock client shows plugin items as unknown items unless its GRF data is patched by hand.

### Limits
- 1 MiB of linear memory, 10,000 host calls and `cpu_budget_ms` of CPU per run (waiting for the host is free). Each request
  of a batch counts as a host call, and a batch holds at most 256.
- Interceptor messages are JSON in a 16 KiB buffer.
- No filesystem, network or clock other than through host calls.

### Testing
Script and interceptor functions are plain Rust: test them in the crate with `cargo test` from `scripts/`, using
`script_sdk_2::MockTransport` for the host (see the example's tests). `server/src/server/plugins/tests.rs` loads the example
in a test server and checks its requests, hooks and interceptor end to end; a new plugin can follow it.
