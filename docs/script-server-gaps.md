# Script server behaviour gaps

The converted NPC scripts (`scripts/`, generated from rathena) compile and pass the parity and unit gates, but some commands are only partly implemented on the server. This page lists what is missing or unverified, so a reader knows where to look before relying on a script.

Status as of the blocker-clearing pass: the converter reports 4133 converted and 0 blocked. "Converted" means the script lowers to SDK calls. It does not mean every command has full server behaviour.

Every row below was checked against the server code and `../rathena/src/map/script.cpp` in the verification pass that followed.

## Partial or stubbed commands

| Command | Current behaviour | rathena behaviour | Where |
| --- | --- | --- | --- |
| `setitemscript` | Runtime error: `setitemscript is not supported: item bonus scripts are compiled into the item modules`. Unsupported by design: rathena parses the replacement text with its script interpreter (`if`, `skill`, `itemheal` and the rest), and the server has no interpreter, because scripts are compiled ahead of time to Wasm. | Replaces an item's bonus script at run time. | `server/src/server/service/script_npc_commands.rs` |
| `requestguildinfo` | Guild records are read from the repository on demand, so without a callback it does nothing, as the data is always available. With an event label it queues that NPC event like `donpcevent`. More than a guild and a label is an error. | Loads the guild into the cache and runs the event when it is there. | `script_npc_commands.rs` |
| `maprespawnguildid` clone removal | Flag `4` removes script mobs but spares guardians and Emperium. Clone-class removal (`RESPAWN_REMOVES_CLONES = false`, which matches rathena's `guild_maprespawn_clones: no`) is covered by a spawn-level test that retags a spawned mob as a clone class, because the test mob database has none in 3999–20020. The range is inclusive on both ends, as in `mob_is_clone`. | Same. | `server/src/server/service/script_map_commands.rs`, `server/src/server/service/map_instance_service.rs` |
| `getgdskilllv` | Returns `-1` when the guild does not exist and `0` when the guild lacks the skill, as rathena does. It accepts a numeric skill id or a guild skill name such as `"GD_GUARDUP"`. An unknown name reads as level `0`, as in rathena. | Same. | `server/src/server/service/castle_service.rs` |

## Guardians and castles

Castles that the server runs itself (`server/src/server/script/castles.json`, `castle_service.rs`) are separate from the `guardian` script command, but both give a guardian the same castle binding.

- **Native castle guardians work.** They are spawned per slot with the owner's `GD_GUARDUP` bonus, defence investment and allied guilds, and carry `Mob::castle_owner` and `Mob::castle_slot`. A guardian that dies clears its `CD_ENABLED_GUARDIANxx` flag, as rathena does in `mob_dead`, so the next castle refresh does not bring it back. A castle refresh removes guardians without clearing the flag.
- **The `guardian` script command is bound to its castle.** It takes the castle owner, allies, defence investment and `GD_GUARDUP` level from the castle of the map, accepts the optional guardian index, and returns the mob id. An index outside 0–7 and a map that is not a castle are errors, and so is a slot that already holds a living guardian, as in rathena's `mob_spawn_guardian`. The Agit castles (`nguild_*`) count as castles for this.
- **`guardianinfo` is not implemented, on purpose.** rathena uses it only in `agit_main.txt`, which the native castle service replaces. No converted script calls it.
- **Agit scripts are not active.** The nguild scripts convert, but rathena comments out every nguild line in `npc/scripts_athena.conf`, and the castle definitions are in `db/pre-re/castle_db.yml`. They are untested in play.

## Constant names

- **Constant lookup is case-sensitive underneath, rathena's is not.** rathena matches identifiers with `strcasecmp` (`script.cpp`, `search_str`), so `Ele_fire` and `Ele_Fire` are the same constant. The server's tables hold one spelling each. When a name is not found, `constant` in `item_script_handler.rs` retries it as upper case, `Title_Case`, `PREFIX_Title_Case` and lower case, so `job_novice`, `Ele_fire` and `et_huk` resolve. A spelling outside those forms still fails with `Unknown script constant`, which stops the script.
- **No current script depends on the retry.** `gen_sdk2_constants.py` resolves all 804 names it finds. It only sees names written as `ctx.constant("NAME")` or `constants::NAME`. Names kept in arrays or passed as arguments are invisible to it. A scan of every `SC_`, `ET_`, `EQI_`, `EFST_` and similar string literal in `scripts/` found 425 more names, and all resolve. Rerun that kind of scan after changing scripts.
- **Enchant cards are not modelled.** `ITEMINFO_SUBTYPE` reports weapon or ammo type only, and `CARD_ENCHANT` resolves to `1` while no item has that subtype. The pre-renewal item database has no enchant cards, so scripts that skip them behave correctly.

## Attached players (`attachrid` / `detachrid`)

- `attachrid` makes character commands (`set`, `getitem`, `equip`, `delequip`, `warpchar`, `isloggedin`) act on the attached player. The attached player is stored in `NpcScriptHost::attached` (`server/src/server/script/host.rs`).
- **Dialogue goes to the attached player.** `mes`, `close`, `message`, `dispbottom` and `cutin` are sent to their client. `next`, `menu` and the input commands register an input channel on their session (`remote_dialogue.rs`), so their client's replies reach the script. The channel is released, and any window left open is closed, on `detachrid`, on a new `attachrid` and when the script ends. Attaching to a player who is in another conversation, or offline, makes the dialogue command fail.
- **Background scripts can talk to an attached player.** The wedding ceremony in `marriage.txt` is a timer script that attaches the bride and talks to her.
- **Not tested in game.** The channel routing and release are unit tested. The wedding flow has not been played through.

## Walking (`npcwalkto`, `npcstop`, `npcspeed`, `unitwalk`, `unitwalkto`, `unitstopwalk`)

- **Implemented, not played against a client.** The script call is checked in the game loop (`service/script_npc_walk.rs`); the walk is stepped by the map's movement thread (`step_npc_walks` in `service/map_unit_walk.rs`), which moves the NPC one cell per step delay and turns it the way it goes. Nearby clients get `ZC_NOTIFY_MOVE` when the walk starts, and players who come into view of a walking NPC get it after its entry packet. Clicks, range checks and `getmapxy` see the cell the NPC has reached.
- **Arrival label, an addition.** `npcwalkto x, y, "<npc>", "<Npc::OnLabel>"` runs the label without a player when the NPC arrives, like the label of rathena's `unitwalk`. rathena's `npcwalkto` has no label. Walking to the cell the NPC stands on runs it at once.
- **`npcwalkto` answers 1 or 0.** 0 when the NPC is hidden or dead, or no path leads to the cell. rathena answers nothing. Paths are searched like a click on the ground (a few dozen cells), so a long route is a chain of walks, each started from the previous arrival label.
- **`npcstop` takes no flags.** It behaves like rathena's default (`USW_FIXPOS | USW_MOVE_FULL_CELL | USW_FORCE_STOP`): the NPC finishes the cell it is entering, nearby clients get `ZC_STOPMOVE` for that cell, and the arrival label is dropped. Clicking a walking NPC stops it the same way, as in rathena.
- **What ends a walk.** Another `npcwalkto` replaces it. `movenpc`, a `setunitdata` move, knockback, a map change, `disablenpc` and death end it without running the arrival label.
- **`npcspeed`** takes 20 to 1000 ms per cell (refused outside that range, as in rathena); NPCs start at 200. A walking NPC is drawn again with the new speed and sent on from where it is.
- **`unitwalk` and `unitwalkto`** (`service/script_unit_walk.rs`) take a player (account or character id), a monster or an NPC. They answer 1 when the walk starts and 0 when no path leads there, the unit cannot walk (dead, frozen, trading, vending, a monster without `CanMove`), or an earlier `unitwalk` still moves it, as rathena refuses a second forced walk. Pets, homunculi and mercenaries answer 0: they follow their owner.
  - A player walks through the movement loop like a click on the ground. The label runs attached to them when their path ends on the destination; a player who clicks elsewhere, warps or changes maps drops it. rathena runs the label at the end of whatever walk comes next.
  - A monster leaves its target alone: its AI and skills wait until the walk ends, as in rathena. A walk cut short (knockback, a chase started by damage) gives it back to its AI without the label.
  - An NPC walks as with `npcwalkto`; `npcwalkto` may replace a forced walk.
  - `unitwalkto` walks to the cell the target stands on when called. It does not follow a target that moves, and walks onto the cell rather than next to it.
- **`unitstopwalk`** stops a player where they are (the half-way rule of a cancelled click), a monster on its current cell, and an NPC on the cell it is entering. The flag is accepted and ignored; nearby clients always get `ZC_STOPMOVE`.
- **Not done:** moving touch areas. The area of an `OnTouch` NPC follows it, but a walking NPC that reaches a standing player does not trigger it; only the player's own steps do.
- **Sprites:** the client animates the walk only for sprites that have walking frames (player job sprites, monster sprites). Most NPC sprites slide.

## Character commands without an in-game check

These are implemented and reviewed, but nothing has been run against a live client:

- `equip` / `delequip` on an attached player. `equip` picks the first matching inventory item with `equip == 0`. `delequip` takes off the equipment in the named slot and removes the item from inventory.
- `set Hp`. Clamps to `1..=max_hp`, as rathena does. Requires an attached player.
- `warpchar`. The fourth argument is the character id.
- `playbgm`. Sends the BGM packet to the context character only.
- `wedding`. Sends the congratulation packet to the attached player, or to the NPC's area when there is none.
- `isloggedin`. Checks the account, and the character when one is given, against the online sessions.

## Test coverage

- `generated_npc_trace` (release) passes. It compares the generated scripts against the expected host call trace for the converted NPCs.
- Full server unit suite passes: 913 passed, 0 failed, 8 ignored (release mode).
- Integration tests were not run for this pass.
- Covered by unit tests: clone removal on spawned mobs, guardian slot reuse, the slot-clearing event from `mob_die` and its castle handler, the castle binding of a script `guardian`, guild skill lookup by id or name, constant spelling retries and the remote dialogue channel.
- `requestguildinfo` with a callback is covered by compilation only. No converted script passes a callback.

## Not yet decided

- Whether `setitemscript` is worth an item-script interpreter. Only the inactive `valentinesday_2012` event and unloaded custom events use it. A narrower option is to convert such calls at import time.
- Whether nguild and agit scripts should be left out of blocker counts while they are inactive.
