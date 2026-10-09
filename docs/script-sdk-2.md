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

Actions are grouped by domain. Each group is a thin typed wrapper over one `Function` variant, and checks the argument order against the server handler:

- `ctx.items()`: `count(item)`, `give(item, amount)`, `take(item, amount)`, `is_equipped(item)`.
- `ctx.quests()`: `start`, `complete`, `erase`, `change(old, new)`, `check(quest)`, `progress(quest)`.
- `ctx.timers()`: `init`, `start`, `stop`, for the current NPC's timer.
- `ctx.npc()`: `emotion(id)`, `special_effect(effect)`, `do_event("NPC::Label")`.
- `ctx.fx()`: `cutin(image, position)`, `special_effect(effect)`.
- `ctx.instance()`: `npc_name(npc, id)`, `map_name(map, id)`, `id()`. The `id` argument is optional: `None` means the player's own instance.
- `ctx.party()`: `is_leader(party_id)`.
- `ctx.warp(map, x, y)`, `ctx.set_npc_visible(npc, visible)`.
- `ctx.monster(map, x, y, name, class, amount, event)`, `ctx.area_monster(map, area, name, class, amount, event)`, `ctx.announce(message, flag)`.
- `ctx.rand(max)` for `0..max - 1`, and `ctx.rand_range(min, max)` for both ends included, as rathena's `rand`.

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

## Not covered yet

- Typed groups for `battleground`, `guild`, the waiting room, and the instance enter, create and warp calls. Those take several arguments, so they need their own pass over the server handlers.
- `KillMonster`, `CheckWeight` and the sound effects. Their argument handling is not yet verified.
- Most of the roughly 250 `Function` variants. Use `ctx.call` for them until they are added.
- Registering scripts by name with a module. The host still calls the generated dispatch.

Known server gaps that affect these calls are listed in [script-server-gaps.md](script-server-gaps.md). The design is recorded in [ADR 4](adr/4-typed-script-sdk.md).
