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

## Game actions

Actions are grouped by domain. Each group is a thin typed wrapper over one or more `Function` variants:

- `ctx.items()`: `count(item)`, `give(item, amount)`, `take(item, amount)`.
- `ctx.quests()`: `start`, `complete`, `erase`, `change(old, new)`, `check(quest)`, `progress(quest)`.
- `ctx.timers()`: `init`, `start`, `stop`, for the current NPC's timer.
- `ctx.warp(map, x, y)`, `ctx.set_npc_visible(npc, visible)`.

Anything without a typed wrapper goes through `ctx.call(Function::X, args![..])`, and `ctx.request(Request::..)` covers the rest of the protocol.

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

- Typed server constants (`JOB_NOVICE` and similar). `ctx.constant(name)` still looks them up at run time.
- Input prompts (`InputNumber`, `InputString`) and the typed group for them.
- Registering scripts with a module and name. Scripts are not yet reachable from the host; the current host still calls the generated `run_npc` / `run_event` dispatch.
- Most of the roughly 250 `Function` variants have no typed wrapper yet. Use `ctx.call` for them until they are added.
