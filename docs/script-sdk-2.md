# script-sdk-2: writing NPC scripts

`lib/script-sdk-2` is the readable API for NPC, item and event scripts. It sits on top of `lib/script-sdk`, which defines the wire types (`Function`, `Request`, `Value`) and the `rust_ro::invoke` import. Scripts written against `script-sdk-2` compile to WebAssembly like the old generated code, but read like ordinary Rust.

## The shape of a script

A script is a function that takes a `&Ctx` and returns a `Script`:

```rust
use script_sdk_2::{Ctx, Script};

fn prontera_guard(ctx: &Ctx) -> Script {
    ctx.mes_as("Prontera Guard", "Outside the walls, stay alert.\nWant a tip?")?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 0 {
        ctx.mes("Watch the south gate.")?;
    }
    ctx.close()
}
```

- Every `ctx` call returns `Result<_, Stop>`. `?` stops the script early.
- `Stop::End` means the conversation is over, normally. `ctx.close()` and `ctx.end()` return it.
- `Stop::Error(message)` is a real failure. The host logs it. Host errors and cancelled menus become this variant.
- `Script` is `Result<(), Stop>`. Returning `Ok(())` finishes the script the same way `Stop::End` does.

## Dialogue

| Method | Effect |
| --- | --- |
| `ctx.mes(text)` | One dialogue box. `\n` in `text` breaks lines. |
| `ctx.mes_as(speaker, text)` | Same, with a `[speaker]` title line first. |
| `ctx.next()` | Waits for the player to press next. |
| `ctx.menu(&[..])` | Offers options and returns the 0-based index. Cancel is `Stop::Error`. |
| `ctx.close()` | Closes the window and ends the script. |
| `ctx.close_window()` | Closes the window and keeps running (rathena `close2`). |
| `ctx.end()` | Ends the script without touching the window. |

Consecutive lines belong in one `mes` call, separated by `\n`. The host already joins multiple `Mes` arguments with `\n`, so this sends the same dialogue text as before.

## Values

`Val` is a number or a text, as in rathena scripts.

- Build one with `Val::from(..)` from `i32`, `bool`, `&str` or `String`. The `args!` macro builds argument lists: `args![501, 1]`.
- `val == 5` and `val == "5"` compare by text when the types differ, as rathena does.
- `val < 5`, `>=` and friends order numbers only. A text is never less or greater than a number.
- `val + x` adds numbers and joins texts. `try_sub`, `try_mul`, `try_div` and `try_rem` return `Result`, because text operands and division by zero are errors.

## Variables

`ctx.var(name)` takes the rathena name, and the prefix chooses the scope:

| Name | Scope |
| --- | --- |
| `Zeny`, `BaseLevel` | Character |
| `#gold` / `##gold` | Account |
| `$world` | Server |
| `$@event_state` | Server, temporary |
| `@menu` | Character, temporary |
| `.npc_flag` | NPC instance |
| `'instance_flag` | Instance |

A trailing `$` makes a text variable. `ctx.var("name$").set(7)` stores the text `"7"`, and `ctx.var("level").set("12")` stores the number `12`.

Array elements use `get_at(index)` and `set_at(index, value)`.

## Player and input

`ctx.player()` reads the player in the conversation through the character's status variables:

- `zeny()`, `base_level()`, `job_level()`, `class()` return numbers.
- `name()`, `party_name()`, `guild_name()`, `map_name()` return text.
- `set_zeny(amount)` writes zeny. Adding to it is `set_zeny(zeny + cost)`.

`ctx.input_number(min, max)` and `ctx.input_text(min, max)` ask the player for a value and return an `Input<T>`:

```rust
let amount = ctx.input_number(1, 100)?;
if amount.bound == Bound::Above {
    ctx.mes("Limited to 100.")?;
}
let quantity = amount.value;
```

Out-of-range numbers are clamped to the nearest bound. Long text is cut to `max` characters. `Bound` says which bound was crossed (`Below`, `Within` or `Above`), so the script can reproduce rathena's `-1`/`0`/`1` status with `bound.code()`.

## Game actions

Actions are grouped by domain. Each group is a thin typed wrapper over one or more `Function` variants. Argument order is taken from the server handler. The main wrappers have unit tests that assert the arguments sent; none of these calls has been run against a live client yet.

- `ctx.items()`: `count(item)`, `give(item, amount)`, `take(item, amount)`, `is_equipped(item)`, `check_weight(&[(item, amount)])`, `name(item)`, `place(item, amount, map, x, y)` (rathena `makeitem`, dropped on the floor for everyone).
- `ctx.quests()`: `start`, `complete`, `erase`, `change(old, new)`, `check(quest)`, `progress(quest)`.
- `ctx.timers()`: `init`, `start`, `stop`, for the current NPC's timer, and `set_elapsed(ms, npc)` for the timer of `npc` (`None` for the current NPC).
- `ctx.npc()`: `emotion(id)`, `special_effect(effect)`, `do_event("NPC::Label")`, `id(npc)` (`None` for the current NPC), `name()`, `visible_name()`, `hidden_name()`, `map_name()` for the current NPC, `variable(".name", npc, index)` and `set_variable(".name", npc, index, value)` for another NPC's variables.
- `ctx.fx()`: `cutin(image, position)`, `special_effect(effect)`, `sound_effect(file, kind)`, `sound_effect_all(file, kind, map, area)`.
- `ctx.party()`: `is_leader(party_id)`, `name(party_id)`.
- `ctx.player()` also has `char_id()`, `party_id()`, `guild_id()`, `account_id()` for the attached player, `skill_level(skill)`, and `give_experience(base, job)`.
- `ctx.guild()`: `name(guild)`, `master_name(guild)`, `is_master(guild)`, `skill_level(guild, skill)`, `experience(amount)`, `open_storage()`.
- `ctx.instance()`: `create(name, mode)`, `enter(name, position)`, `warp_all(map, x, y, id, flags)`, `destroy(id)`, `announce(id, message)`, `npc_name(npc, id)`, `map_name(map, id)`, `id()`, `check_party(party, amount)`, `check_guild(guild, amount)`, `info(name, kind)`, `info_at(name, index)`, `live_info(kind, id)`, `list(map, mode)`, `var(name, id)`, `set_var(name, value, id)`. An `id` of `None` means the player's own instance. `enter_with(name, EnterOptions)` also picks the character and an explicit instance.
- `ctx.battleground()`: `create(cemetery, events)`, `join(bg, char, destination)`, `leave(char)`, `desert(char)`, `destroy(bg)`, `warp(bg, spot)`, `set_cemetery(bg, x, y)`, `members(bg)`, `member_count(bg)`, `count_in_area(bg, map, area)`, `update_score(map, first, second)`, `reserve(map, ended)`, `unbook(map)`, `info(name, kind)`, `monster(bg, spot, name, class, event)`, `set_monster_team(mob, bg)`. A `char` of `0` means the attached player.
- `ctx.waiting_room()`: `open(title, limit, rules)`, `delete(npc)`, `kick(npc, name)`, `kick_all(npc)`, `enable_event(npc)`, `disable_event(npc)`, `state(kind, npc)`, `title(npc)`, `event(npc)`, `warp(map, x, y, count)`. `npc` of `None` means the current NPC. `title` and `event` return `None` when the NPC has no room.
- `ctx.warp(map, x, y)`, `ctx.set_npc_visible(npc, visible)`.
- `ctx.monster(map, x, y, name, class, amount, event)`, `ctx.area_monster(map, area, name, class, amount, event)`, `ctx.kill_monster(map, label)`, `ctx.announce(message, flag)`.
- `ctx.map_announce(map, message, flag, color)`, `ctx.view_point(action, x, y, number, color)`, `ctx.map_users(map)`, `ctx.mob_count(map, label)`, `ctx.time_string(format, limit)`.
- `ctx.rand(max)` for `0..max - 1`, and `ctx.rand_range(min, max)` for both ends included, as rathena's `rand`.
- `ctx.map_warp(source, destination)`, `ctx.area_warp(source, area, destination)`, `ctx.area_heal(map, area, hp, sp)` (percentages), `ctx.time_field(field)`, `ctx.time_tick(kind)`.
- `ctx.sleep(ms)` and `ctx.progress_bar(color, seconds)`: both pause the script while the game keeps running.
- `ctx.npc().set_display(npc, sprite)`.
- `ctx.set_cell(map, area, cell, enabled)` (`constants::CELL_*`), `ctx.set_map_flag(map, flag, values)` and `ctx.remove_map_flag(map, flag, values)` (`constants::MF_*`; `values` are the flag's own arguments, such as a skill id).
- `ctx.player()` also has `equipped_in(slot)` and `equipped_card(slot, card)` (`constants::EQI_*` slots), `save_point(map, x, y, range, char_id)`, `grant_skill(skill, level, SkillGrant)` (the `SkillGrant` variants are rathena's `skill` flags 0 to 3), `hire_mercenary(class, milliseconds)` and `map_xy()` for the attached player's map and position.
- `ctx.pet()`: `info(PetInfo, char_id)` for the active pet of the attached player (or of `char_id`), and `catch(lure, PetCatch)` to start taming with a lure item or a pet class.
- `ctx.guardian(map, x, y, name, class, event)` spawns a castle guardian. rathena's guardian index is not sent yet, see [script-server-gaps.md](script-server-gaps.md).
- `ctx.player()` also has `has_cart(target)`, `has_falcon(target)`, `is_riding(target)`, `is_mounting(target)`, `read_param(name)`, `change_job(job)`, `end_status(kind, target)`, `set_look(type, value)`, `equipped_item_id(slot)`, `equipped_refine(slot)`, `partner_id(target)`. A `target` of `None` means the attached player.

### Bonuses

`ctx.bonus()` applies item and effect bonuses. Names are enum variants, so a typo is a compile error:

```rust
ctx.bonus().apply(Bonus::Str(5))?;               // bonus bStr, 5;
ctx.bonus().apply(Bonus::NoCastCancel)?;         // bonus bNoCastCancel;  (flags take no value)
ctx.bonus().apply2(Bonus2::AddRace(constants::RC_DemiHuman, 5))?;  // bonus2 bAddRace, race, 5;
ctx.bonus().auto_spell(AutoSpell::new("MC_LOUD", 1, 10))?; // bonus3 bAutoSpell, skill, level, rate;
ctx.bonus().auto_bonus(AutoBonus { program: 1, rate: 10, duration: 5000, battle_flags: None, visual_program: None })?;
```

- `Bonus`, `Bonus2` and `Bonus3` are generated from the names the scripts use (`tools/scripts-import/gen_sdk2_bonus.py`). A name used as a flag takes no value; every other name takes a value (`Bonus`) or a key and a value (`Bonus2`) or three values (`Bonus3`).
- `AutoSpell`, `AutoSpellOnSkill`, `AutoBonus` and `AutoSkillBonus` are hand-written. The auto-spell struct picks `bonus3`, `bonus4` or `bonus5` from which optional fields are set, and `battle_flags` requires `flags`.
- Key arguments of `Bonus2` and `Bonus3` are plain numbers or constants such as `RC_DemiHuman`. Their meaning comes from the bonus name, so check the rathena reference for the name you use.
- `bUnbreakableHelm` is written as a flag. Four converted scripts also pass `0` to it; the server ignores the value, so the typed form drops it too.

Some calls take a positional list of optional values. The wrappers fill any gap with the server's default, so passing only the later options still sends a correct list.

## Constants

`script_sdk_2::constants` holds the server's values for the constants scripts use, such as `constants::JOB_NOVICE` and `constants::BC_ALL`. Numeric constants are `i32`. A few names the server resolves to their own text (`EQI_*`, `SC_*`, `EF_*`, `DT_*`, `ITEMINFO_*`) are `&str`.

The file is generated. After a server change to a constant, run `python tools/scripts-import/gen_sdk2_constants.py`, which reruns the server's own lookup and rewrites the file. `--check` fails instead of writing when the file is stale. The generator prints the names the server does not resolve; those stay on `ctx.constant(name)`.

## Testing a script without a host

`MockTransport` answers requests with a closure and records them:

```rust
use script_sdk_2::{Ctx, Function, MockTransport, Request, Stop, Value};

let transport = MockTransport::new(|request| match request {
    Request::Call { function: Function::Select, .. } => Ok(Value::Number(1)),
    _ => Ok(Value::default()),
});
let result = prontera_guard(&Ctx::new(&transport));
assert_eq!(result, Err(Stop::End));
assert_eq!(transport.calls(Function::Mes).len(), 2);
```

`MockTransport::silent()` answers everything with `0`. `transport.requests()` and `transport.calls(function)` return what the script asked for.

## Item scripts

Item scripts have two typed contexts, as [ADR 5](adr/5-item-script-api.md) explains. `ItemBonus` runs when the item is worn or refined and can only read the wearer and describe bonuses. `ItemUse` runs when the item is consumed and also has effects and dialogue:

```rust
fn red_potion(item: &ItemUse) -> Script {
    item.heal(45, 0)?;                        // queued, applied when the script returns Ok
    if item.menu(&["Keep it", "Use it now"])? == 1 {
        item.mes("Your wounds close.")?;
    }
    item.close()
}
```

- Effects are queued. A cancelled conversation applies none of them and does not consume the item.
- `ItemUse::read` and `write` take character variables only. Scoped names (`#`, `$`, `@`, `'`, `.`) are refused.
- `read_any` and `write_any` take the name as the script spells it, scope prefix included. The host answers a scoped read from the value it loaded before the script ran, and every name a script reads is listed in its manifest entry, so the generator writes these calls for converted items.
- `.@` names are Rust locals.

The item layer is `lib/script-sdk-2/src/item.rs`. Every item is in the items module, `scripts/items`, which `scripts/items/src/generated.rs` generates from the item sources with `tools/scripts-import/import_items.py`. A body without effects is an `ItemBonus` function and the rest are `ItemUse` functions. `item_module!` registers them by item id, and the server runs an item through `Entry::Item(id)`, which falls back to the bonus table for passive items.

## Not covered yet

- `Function` variants that only an item script can use. The item script queues them, and the server applies them when the item is used. An NPC script that calls one gets `Unsupported game request`, so they have no wrapper here:
  - `PercentHeal`, `ItemHeal`, `Heal`, `ItemSkill`, `SkillEffect`, `Produce`, `Cooking`.
  - `GetRefine` and `RandomGroupItem`, which read the item being used.
- Text fields of the waiting room state other than `title` and `event` (kind `5`), which the server leaves empty.

Known server gaps that affect these calls are listed in [script-server-gaps.md](script-server-gaps.md). The design is recorded in [ADR 4](adr/4-typed-script-sdk.md).
