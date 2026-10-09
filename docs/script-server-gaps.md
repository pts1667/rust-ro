# Script server behaviour gaps

The converted NPC scripts (`scripts/`, generated from rathena) compile and pass the parity and unit gates, but some commands are only partly implemented on the server. This page lists what is missing or unverified, so a reader knows where to look before relying on a script.

Status as of the blocker-clearing pass: the converter reports 4133 converted and 0 blocked. "Converted" means the script lowers to SDK calls. It does not mean every command has full server behaviour.

## Partial or stubbed commands

| Command | Current behaviour | rathena behaviour | Where |
| --- | --- | --- | --- |
| `setitemscript` | Runtime error: `setitemscript is not supported: item bonus scripts are compiled into the item modules`. | Replaces an item's bonus script at run time. | `server/src/server/service/script_npc_commands.rs` |
| `requestguildinfo` | No-op without a callback. With a callback it returns an error. | Sends guild info to the callback label. | `script_npc_commands.rs` |
| `maprespawnguildid` clone removal | Flag `4` removes script mobs but spares guardians and Emperium. Clone-class removal is written (`RESPAWN_REMOVES_CLONES = false`, which matches rathena) but has no spawn-level test, because the test mob database has no class in 3999–20020. | Same. | `server/src/server/service/script_map_commands.rs`, `server/src/server/service/map_instance_service.rs` |
| `getgdskilllv` | Returns `-1` for a guild skill the guild does not have. Not checked against rathena; whether `-1` is right is still open. | To confirm. | `server/src/server/script/game_api.rs` |

## Guardians and castles

- **Guardian index is not implemented.** Guardians are marked `Mob::guardian` (`server/src/server/state/mob.rs`). They have no index, so a script cannot address a specific guardian.
- **Castle ownership is not checked by guardians.** Guardian spawns from `Function::Guardian` set `is_guardian`, but nothing ties a guardian to the castle owner.
- **Clear-slot-on-death is not implemented.** rathena clears the guardian slot when a guardian dies. Here the slot stays occupied, so a respawn command can't reuse it.
- **Agit scripts are not active.** The nguild and `guild2/agit_*` scripts convert, but all nguild lines are commented out in `npc/scripts_athena.conf`, and the castle definitions are in `db/pre-re/castle_db.yml`. They are untested in play.

## Constant names

- **Constant lookup is case-sensitive, rathena's is not.** rathena matches identifiers with `strcasecmp`, so `Ele_fire` and `Ele_Fire` are the same constant. The server's table matches the exact spelling, except the element names, which match in any case (`element_constant` in `server/src/server/script/constant.rs`). Any other spelling mismatch fails at run time with `Unknown script constant`, which stops the script. `gen_sdk2_constants.py` reports the names it cannot resolve, so run it after changing scripts.
- **Enchant cards are not modelled.** `ITEMINFO_SUBTYPE` reports weapon or ammo type only, so no item has the `CARD_ENCHANT` subtype. Scripts that skip enchant cards behave as if none exist.

## Attached players (`attachrid` / `detachrid`)

- `attachrid` makes character commands (`set`, `getitem`, `equip`, `delequip`, `warpchar`, `isloggedin`) act on the attached player. The attached player is stored in `NpcScriptHost::attached` (`server/src/server/script/host.rs`).
- **Dialogue to a non-session attached player is refused.** `mes`, `menu` and `next` aimed at an attached player other than the session's player return `Dialogue with an attached player is not supported`. The NPC can't talk to a second player yet.
- **Not tested in game.** The paths are covered by code review and unit tests, not by playing through them.

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
- Full server unit suite passes: 920 passed, 0 failed, 8 ignored (release mode).
- Integration tests were not run for this pass.
- The clone-class range and `maprespawnguildid` on clones are covered only by a direct unit test of `is_clone_class`.

## Not yet decided

- Whether `setitemscript` needs a runtime mechanism for item bonuses, or stays a stub. Only the inactive `valentinesday_2012` event uses it.
- Whether guardians need an index and castle ownership before the agit scripts are enabled.
- Whether nguild and agit scripts should be left out of blocker counts while they are inactive.
