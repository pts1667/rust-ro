# Writing a plugin

A plugin changes or adds to the game without touching the server or its scripts: new NPCs, events, rewards, rule
changes, rate events, custom windows. It is a small Rust crate compiled to WebAssembly, dropped in a folder and turned on
in `config.json`.

This page is the guided tour: how to start, the common recipes, and where the walls are. The full reference is in
[plugins.md](plugins.md) (packaging, hooks, interceptors, settings), [plugin-ui.md](plugin-ui.md) (windows, overheads,
bots) and [script-sdk-2.md](script-sdk-2.md) (everything a script can call). Three working plugins are in
`scripts/plugins/`:

| Example | Shows |
| --- | --- |
| `double_xp` | settings, rates, an NPC, a clock event, hooks, an interceptor |
| `ui_shop` | a custom window, an overhead, a bot action |
| `alive_towns` | NPCs generated from settings, a timer loop, batched commands |

## Your first plugin

A plugin that greets players at login and places a talking NPC.

**1. Create the crate** in `scripts/plugins/greeter/Cargo.toml`. The package name is the plugin's name:

```toml
[package]
name = "greeter"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
script-sdk-2.workspace = true
serde = { version = "1.0", features = ["derive"] }
```

**2. Describe it** in `scripts/plugins/greeter/info.json`, with every setting and its default:

```json
{
  "title": "Greeter",
  "description": "Welcomes players and points them to the Greeter in Prontera.",
  "config": {
    "welcome": { "default": "Welcome back!", "description": "Message shown at login." }
  }
}
```

**3. Write it** in `src/lib.rs`:

```rust
use script_sdk_2::{Ctx, Hook, Script};

script_sdk_2::plugin_module! {
    info { name: "greeter", version: "0.1.0", capabilities: [Dialogue] }
    init => init,
    npcs { "Greeter" => greeter }
    events {}
    hooks { Login => welcome }
    interceptors {}
}

fn init(_ctx: &Ctx) -> Script {
    Ok(())
}

fn greeter(ctx: &Ctx) -> Script {
    ctx.mes_as("Greeter", "Nice to see you here.")?;
    ctx.close()
}

fn welcome(ctx: &Ctx, _hook: &Hook) -> Script {
    let text: String = ctx.plugin_setting("welcome")?;
    ctx.player().message(&text)
}
```

**4. Build it.** `cargo run --package tools --bin scripts-build --release` writes `config/plugins/greeter/plugin.wasm`.

**5. Place the NPC** in `config/plugins/greeter/npcs.json`:

```json
[{ "map_name": "prontera", "name": "Greeter", "sprite": "4_F_KAFRA1", "x": 150, "y": 180, "dir": 4 }]
```

**6. Turn it on** in `config.json`, then restart the server:

```json
"scripting": { "plugins": ["greeter"] }
```

If something is wrong, such as a missing capability, a bad setting or an unknown NPC, the boot stops and the log says
why. The server never runs without a plugin its operator listed.

## The building blocks

| Block | What it is | Declared in |
| --- | --- | --- |
| `init` | runs once at boot, with no player and no NPC | `init => fn` |
| NPCs | a click runs the handler, which can talk, trade, warp... | `npcs { }` + `npcs.json` |
| Events | named labels: timers, clock times, monster deaths, walk arrivals | `events { }` + `events.json` |
| Hooks | run after something happened to a player; cannot undo it | `hooks { }` |
| Interceptors | asked before the game does something; can allow, refuse or change it | `interceptors { }` |
| Rates | this plugin's share of the experience and drop rates | `ctx.rates()` in `init` |
| Settings | values the operator can change without rebuilding | `info.json` |
| Windows, overheads | custom UI, drawn by nostalro-client | `windows { }` |
| Bot actions | commands that bots can run through the bot API | `bot_actions { }` |
| Generated NPCs | NPCs computed from the settings at boot | `generated_npcs => fn` |
| `@` commands | chat commands, open to permission groups by level | `commands { }` |
| Items | new items, with what using or wearing them does | `items { }`, `item_bonuses { }` + `items.json` |

Every handler is ordinary Rust. NPC, event, hook, window and bot handlers can call into the game through `ctx` and may
take their time: a dialogue that waits for the player blocks nobody. Interceptors are different: they are pure
functions that the game waits for, and they cannot call into the game at all.

## Capabilities

A plugin lists the groups of game functions it calls in `info { capabilities: [...] }`. A call outside them stops the run
with an error in the log. An operator can narrow the list but never widen it.

| Capability | For |
| --- | --- |
| `Dialogue` | dialogue windows, chat messages, cutins, effects, emotes, sounds |
| `World` | warps, monsters, map flags, moving NPCs, instances, battlegrounds, castles |
| `Economy` | items, zeny, shops, storage, refining, mail |
| `Character` | job, levels, experience, skills, statuses, healing, quests |
| `Rates` | experience and drop rates |
| `Accounts` | permission groups and account data |
| `Broadcast` | announcements to a map or the whole server |
| `Interface` | plugin windows and overheads |

Reading things (character info, items, time, who is on a map), the plugin's own variables, timers and events need no
capability.

## Recipes

### React to what players do
Hooks run just after something happened, with the player attached. Available hooks: `Login`, `Logout`, `MapEnter`,
`LevelUp`, `JobChange`, `Kill`, `Die`, `MobKilled`, `ItemObtained`, `ItemUsed`, `TradeCompleted`, `SkillUsed`,
`PartyChanged` and `GuildChanged`.

```rust
hooks { MobKilled => bounty }

fn bounty(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::MobKilled { mob_id: 1038, .. } = hook else { return Ok(()) };  // Osiris
    ctx.items().give(607, 1)?;                                                 // Yggdrasil Berry, needs Economy
    ctx.player().message("The Pharaoh's curse lifts. Take this.")
}
```

A hook nobody declared costs nothing, so declare only the ones you use.

### Do something on a schedule
Name an event after the clock, register it in `events.json`, and point it at a handler:

```rust
events { "Herald::OnClock2000" => evening, "Herald::OnMinute30" => half_hour }
```
```json
{ "Herald::OnClock2000": {}, "Herald::OnMinute30": {} }
```

The labels are rathena's: `OnMinuteMM`, `OnHourHH`, `OnClockHHMM`, `OnMonHHMM` (and the other weekdays), `OnDayMMDD`.

For a shorter, steady beat, use an NPC timer. `OnTimer5000` fires 5 seconds after the timer starts. The handler restarts
the timer to make a loop:

```rust
events { "Herald::OnTimer5000" => beat }

fn beat(ctx: &Ctx) -> Script {
    // ... the periodic work ...
    ctx.timers().init(None)   // start again from 0
}
```

Something has to start the timer once, usually a `MapEnter` hook calling `ctx.timers().init(Some("Herald"))`. Guard that
call with a variable so each player arriving doesn't restart the count.

Timers and clock labels reach only NPCs on **loaded** maps. A map loads the first time a player enters it and then stays
loaded; castles load at boot. A schedule on a quiet map starts once someone visits it.

### Add or replace an NPC
Add a handler in `npcs { }` and its placement in `npcs.json`. To take over an NPC of the base game, use its exact name
with `"replaces": true`. Fields you leave out keep the original's values:

```json
[{ "name": "Kafra Employee#prt", "replaces": true }]
```

An NPC can also look like a player (`appearance`), walk (`ctx.npc().walk_to(..)`), emote and talk overhead. To show a
plugin's own shop, either sell through a dialogue (`ctx.menu`, `ctx.items().give`, `ctx.player().set_zeny`) or open a
custom window (see below).

### Change experience or drops
Two tools, depending on how selective the change is:

- **The same for everyone:** set a rate in `init`. It costs nothing per kill, since the server reads the value without
  calling the plugin.
  ```rust
  ctx.rates().set(Rate::BaseExp, 200)?;   // 200% of the server rate; Rates capability
  ```
  `BaseExp`, `JobExp`, `Drop` and `CardDrop` exist. The shares of all plugins multiply.
- **Depending on the player or the monster:** an `ExpGain` interceptor gets every kill's experience and returns what
  to grant instead. See `double_xp` for an example.

Drop chances can only be changed through rates. There is no interceptor for each drop roll.

### Forbid or rewrite something
Interceptors run before the game acts, and the first plugin to refuse wins. They exist for `TradeRequest`, `Warp`,
`ItemDrop`, `ExpGain`, `ItemUse` and `Chat`:

```rust
interceptors { TradeRequest => no_trade_maps }

fn no_trade_maps(interception: &Interception) -> Verdict {
    let Interception::TradeRequest { map, .. } = interception else { return Verdict::Allow };
    let maps: Vec<String> = script_sdk_2::setting("no_trade_maps").unwrap_or_default();
    if maps.contains(map) { Verdict::Refuse("Trading is not allowed here.".into()) } else { Verdict::Allow }
}
```

Keep them short. The game waits for them, they have a deadline of 200 µs by default, and they see only the decision and
the plugin's settings. An interceptor that crashes or runs late counts as "allow", and after 10 failures it is turned
off until the next restart. A `Chat` interceptor can also return `Verdict::Chat(text)` to rewrite the message.

### Spawn monsters for an event
With `World`, spawn monsters from `init`, a timer or an NPC. To learn when one dies, name an event label:

```rust
events { "Hunt Master::OnKingDead" => king_dead }

fn start_hunt(ctx: &Ctx) -> Script {
    ctx.monster("prontera", 150, 150, "Poring King", 1002, 1, Some("Hunt Master::OnKingDead"))?;
    ctx.announce("The Poring King appeared in Prontera!", BC_ALL)   // Broadcast
}

fn king_dead(ctx: &Ctx) -> Script {
    ctx.items().give(617, 1)?;   // the killer is attached; Economy
    ctx.announce("The Poring King has fallen.", BC_ALL)
}
```

`ctx.kill_monster(map, label)` removes the monsters of a label when the event ends.

### Remember things
Rust `static`s don't survive between runs: each run starts a fresh instance. Keep state in variables:

| Name | Kept | Shared with |
| --- | --- | --- |
| `$count` | permanently, server-wide | only this plugin |
| `$@count` | until restart, server-wide | only this plugin |
| `#count`, `##count` | permanently, per account | only this plugin |
| `count` | permanently, per character | **every script** |
| `@count` | until logout, per character | **every script** |

Character variables are shared with the base scripts and other plugins, so prefix them with your plugin's name
(`greeter_visits`), as `double_xp` does with `double_xp_kills`.

### Let operators configure it
Every value an operator might want to change goes in `info.json` with a default. The operator writes only the keys they
change into `config/plugins/<name>.json`. Read the settings with a struct:

```rust
#[derive(Deserialize)]
struct Settings { welcome: String, reward_item: i32 }

let settings: Settings = ctx.plugin_config()?;
```

To reject settings that make no sense, return an error from `init` (`Err(Stop::from("..."))`) or from `generated_npcs`.
The boot then stops with your message instead of the plugin misbehaving later.

### Custom windows and overheads
With `Interface`, a plugin can open its own windows (text, tables, item icons, images, buttons, number and text inputs)
and handle their clicks, or show an animated image or caption above any unit. Only nostalro-client draws them, and the
stock client never receives them. `ui_shop` is the full example, and [plugin-ui.md](plugin-ui.md) has every widget and
limit.

### Let bots use the plugin
`ctx.bot().observe(key, &value)` shows data in the observation of a bot's character, and `bot_actions { }` declares
commands that bots can run, answered with `ctx.bot().reply(..)`. Windows are usable by bots too, through the same checks
as players. See the Bots section of [plugin-ui.md](plugin-ui.md).

### Add an @ command
Declare it with the lowest permission group level that may use it, and answer with `ctx.reply`:

```rust
commands { "lucky" => (count_tickets, 0, "Tells how many Lucky Tickets you hold") }

fn count_tickets(ctx: &Ctx, _command: &PluginCommand) -> Script {
    let count = ctx.items().count(40001)?;
    ctx.reply(&format!("You hold {count} Lucky Tickets."))
}
```

`command.args` holds the words after the name. The handler runs with the player attached, so it can do anything an
NPC can, within the plugin's capabilities. Level 0 opens it to every player; staff commands take a higher level, such as
60 or 99. A built-in command's name or alias is refused, so plugins never change what `@item` means. Bots can run
plugin commands too.

### Add an item
Write it in `items.json` with an id from 40000 to 59999 and the fields of `config/items.json`, then give it behaviour
in the module:

```json
[{ "id": 40001, "name_aegis": "Lucky_Ticket", "name_english": "Lucky Ticket", "item_type": "Usable",
   "weight": 1, "description": ["Use it for a lucky draw."], "look": 6153, "icon": "ticket.png" }]
```
```rust
items { 40001 => lucky_ticket }              // what using it does
item_bonuses { 40002 => clover_charm }       // what wearing it gives

fn lucky_ticket(item: &ItemUse) -> Script {
    let roll = item.call(Function::Rand, vec![100.into()])?.number()?;
    item.grant(if roll < 5 { 607 } else { 501 }, 1)
}

fn clover_charm(item: &ItemBonus) -> Script {
    item.bonus().apply(Bonus::Luk(3))
}
```

Every usable item needs a use handler, and the boot checks that the file and the module agree. `look` names an item of
the game whose icon and floor sprite clients borrow, and `icon` a PNG of `ui/` to use instead. Turning the plugin off
later keeps the items in players' hands as plain items. nostalro-client shows names, descriptions and icons from the
server's catalog. The stock client shows unknown items unless its GRF is patched. `lucky_charms` is the full example.

### Drive many NPCs
- Generate the NPCs from the settings with `generated_npcs`, instead of writing a long `npcs.json`.
- Control them by unit id (`GetNpcId` answers all ids in one call).
- Group the commands of one beat in `ctx.batch()`: up to 256 calls in one pass of the game loop instead of one round
  trip each.

`alive_towns` moves 100 NPCs per town on a one-second timer this way.

## What plugins cannot do

Some of these are deliberate, and some just aren't there yet. If you need one, it's a server change, not a plugin.

- **Run on every server or map tick.** There is no tick hook. The finest beat is an NPC timer, and seconds are a
  sensible minimum.
- **Stop things other than the six interceptions.** A hook reacts after the fact. A plugin can't block a skill, a
  login, an attack, a purchase or a pickup, and can't change damage.
- **Change the game's formulas or data.** Damage, skills, statuses, the game's item and monster stats, drop lists,
  jobs and maps stay as the server defines them. The only numbers a plugin can scale are the four rates.
- **Add monsters, maps or sprites, or playable skills and jobs.** New items are possible (above). Skills and jobs can
  be declared in `skills.json` and `jobs.json`, which the server checks and publishes to clients, but nobody can learn
  or become them yet ([plugins.md](plugins.md#skills-and-jobs-scaffold) lists what is missing). Item icons and plugin
  windows can use the plugin's own PNG images, and nothing else.
- **Add client packets.** Windows aside, plugins talk to players through the normal script functions and their own
  `@` commands. A `Chat` interceptor can't act on what it reads, because it can't call the game.
- **Reach outside the game.** No files, no network, no system clock, no threads. Time comes from `ctx.time_tick` and
  `ctx.time_field`, and randomness from `ctx.rand`. Crates that need any of these won't run.
- **Load or reload while the server runs.** Adding, removing or changing a plugin, its files or its settings takes a
  restart.
- **Use a lot of resources in one run.** Each run has 1 MiB of memory, 10,000 game calls and 2 s of CPU time by
  default. Waiting for the player or for the game doesn't count toward the CPU time.
- **Fire timers on maps nobody has visited.** See [Do something on a schedule](#do-something-on-a-schedule).

## Testing

- Handlers are plain Rust functions. Test them with `cargo test` from `scripts/`. `script_sdk_2::MockTransport`
  answers the game calls and records them, so a test can check what the plugin asked for. Interceptors are pure
  functions: call them directly after `script_sdk_2::plugin::configure_for_test(include_str!("../info.json"))`. The
  examples all have tests to copy.
- `server/src/server/plugins/tests.rs` loads a plugin into a test server end to end.
- On a running server, a failed run logs a warning with the reason, and a plugin that can't load stops the boot.

## Common mistakes

- **An event never fires.** The label is missing from `events.json`, or the NPC's map has never been loaded.
- **A call fails with a capability error.** Add the capability to `info { capabilities: [...] }`.
- **`plugin_module!` doesn't compile.** NPC and event names must be sorted, and the sections up to `interceptors` must
  all be present, even when empty.
- **Two plugins step on each other.** Both used the same unprefixed character variable, or both claimed an NPC name
  without `replaces`.
- **An interceptor turns itself off.** It called into the game, which traps, or it did too much work for its deadline.
