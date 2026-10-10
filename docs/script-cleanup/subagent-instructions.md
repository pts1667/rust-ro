# Cleaning up one generated NPC script file

You clean up one generated NPC script file so it reads like hand-written Rust. The behaviour must not change. You have no other context, and you do not see the project's CLAUDE.md, so this file is everything you need. Read it fully before you touch the file.

## The project, in four sentences

This is a Ragnarok Online game server written in Rust (the pre-renewal game version). Its NPCs (shopkeepers, quest givers, job masters, warpers) used to be written in rAthena's NPC script language, and a converter turned them into Rust. Each NPC or event is a function that the server runs inside a WebAssembly sandbox, and the function talks to the server through a `ctx: &Ctx` value (show a dialogue box, ask a menu, read a variable, give an item, warp the player). The Rust API for that is `script-sdk-2`, in `lib/script-sdk-2/src/`, described in `docs/script-sdk-2.md`; you can read them if a call is unclear, but you do not need to.

## Your task and its limits

- You are given the path of one file, for example `scripts/towns/src/airports/einbroch.rs`. Clean up that file only.
- **You cannot compile, and you must not try.** The orchestrator compiles each module after all agents have finished, and sends back any file that fails. So every edit has to be obviously type-correct: if you are not sure a rewrite compiles, do not make it.
- **There is no test either.** Nothing will tell you that you broke the behaviour, so be conservative. A less pretty file that behaves the same is a success. A prettier file that behaves differently is a failure.
- Other agents are editing other files in the same working tree at the same time. Touch nothing but your file.
- If the file has more than 3,000 lines, do not start: report that it is too large.

The machine runs Windows. Every path in this document is relative to the repository root, the directory that holds `Cargo.toml`, `scripts/` and `lib/`; your working directory is that root. Use the Read, Edit and Grep tools where you can. If you use a shell, use Bash with forward slashes in paths.

## What the generated code looks like

The converter's output is correct but hard to read:

- Long chains of nested `if`/`else` on one variable, each level wrapped in `{ ... }`.
- `ctx.call(Function::DelItem, vec![Val::from(535), Val::from(10)])?` for every call, when `args![535, 10]` is the same thing.
- Local variables as `l_name` (an rAthena `.@name` local), cloned at every comparison (`l_x.clone() == 1`).
- Dead locals and unused `mut` (`let mut l_arg = Val::from(0);` that is never read).
- Multi-screen dialogues written as deeply nested calls, not as a sequence of steps.
- Wrapper functions, `switch` blocks, menus and endings written in a mechanical form (see "Generated patterns").

Each NPC is a `pub fn name(ctx: &Ctx) -> Script`, and the file usually holds several. Some have a private `*_run(ctx, step, args)` state machine next to them, which the converter emits for scripts that jump between labels.

## Hard rules

1. **Same behaviour.** The same host calls happen in the same order with the same arguments (the one exception: a `ctx.constant(..)` lookup may become a `constants::NAME`, see "More examples of good cleanup"). The same dialogue text is shown, character for character, including typos, spacing and colour codes such as `^3355FF`. The same variables are written. Each script ends the same way (a `ctx.close_window()?` followed by `Err(Stop::End)` is a close, `Err(Stop::End)` alone is an end without touching the window).
2. **Only the named file.** Edit only the file you were given. Do not edit `lib.rs`, `mod.rs`, `shared`, other NPC files, `Cargo.toml`, anything under `config/`, anything under `tools/`, or any document. Do not run the generator, and do not copy or build any `.wasm`.
3. **Keep the public surface.** Keep every `pub fn` name and signature. The module's `lib.rs` registry refers to them by path, and keys like `"Charles Orleans#cook_"` must still resolve. Do not rename anything that is `pub`.
4. **Do not reorder reads.** Moving a call past dialogue is a behaviour change, with one exception: `StrCharInfo`, `GetItemName`, `GetItemInfo` and `GetAreaUsers` are read-only lookups. You may compute those before a dialogue line, but only if you keep their results in the same text.
5. **Do not swap a helper for a typed wrapper unless it is in the table below or in "More examples of good cleanup"**, and then only in the cases those allow. Every other `ctx.call(Function::X, ..)` stays a `ctx.call`.
6. **Comments.** Write new comments only where the code cannot say what it does. Keep comments short and useful to a future developer. Do not write comments that repeat the code, and do not comment obvious steps. Do not delete existing comments.
7. **Stop when unsure.** If a rewrite would need a guess about what the original does, leave that part as it was and say so in your report.
8. **No compiling.** Do not run `cargo` (not `check`, not `build`, not `test`). Do not remove the `#![allow(..)]` lines or any name from the `use script_sdk_2::{..}` line, since you cannot tell whether they are still needed. Do add a name to the `use` line when your edit starts using it (`args` after you replace `vec![Val::from(..)]`, `constants` after you use a constant).
9. **No git that changes anything.** You may run `git diff` and `git log` to read. Never run `git checkout`, `restore`, `stash`, `reset`, `add` or `commit`: other agents' unsaved work is in the same tree, and the orchestrator decides what is kept.

## How to work

- Read the file in pieces of about 400 lines (the Read tool's `offset` and `limit`), and clean one NPC function at a time, from the top. Finish a function (and its `_body` or `_run` helpers) before you start the next.
- Make changes with the Edit tool, one function or one block at a time. Do not rewrite the whole file with Write: you would have to retype every dialogue line, and one slipped character changes what players read.
- For a rewrite that repeats hundreds of times and is purely mechanical (`vec![Val::from(a), Val::from(b)]` to `args![a, b]`), you may use a short script. Write it as a file in your scratch directory and run it, because quotes in a shell one-liner break. Run `git diff --stat -- <file>` afterwards and read the diff. Never use a script to change dialogue text.
- When you are done, run `rustfmt --edition 2021 <your file>`; it only formats, and the warnings it prints about nightly options are normal.

## Finishing the file

The first line of a generated file is a `// Generated by tools/scripts-import/convert_sdk2.py from ...` comment. The generator rewrites every file that starts with it, and skips every other file. When you have finished cleaning the file, delete that first line, so the generator never overwrites your work. Delete only that line, and put nothing in its place.

- Delete it only if you changed the file. If you left the file untouched, or stopped before you made any edit, leave the line.
- The line covers the whole file, not just one NPC: once it is gone, every NPC in the file is considered hand-maintained.
- If you find that the generated code for an NPC needs a change to the generator, do not hand-edit around it. Leave that NPC as it is and report it.

## Files you may read

Read them only when a call is unclear; you do not need them for the common cases below. Never edit any of them.

| File | What is in it |
| --- | --- |
| `docs/script-sdk-2.md` | Guide to the `script-sdk-2` API. |
| `lib/script-sdk-2/src/dialogue.rs` | `mes`, `lines`, `lines_as`, `next`, `menu`, `close`, `close_window`, `end`. |
| `lib/script-sdk-2/src/value.rs` | `Val`: `number`, `text`, `is_true`, `loosely_equals`, `==`, `+`, `try_sub`, `try_mul`, `try_div`, `try_rem`. |
| `lib/script-sdk-2/src/vars.rs` | `ctx.var(name)`: `get`, `set`, `add`, arrays. |
| `lib/script-sdk-2/src/runtime.rs` | `runtime::op`, `select_values`, `menu_options`, `local_get`, `local_set`, `index`, `arg` and the string helpers the generator calls. |
| `lib/script-sdk-2/src/constants.rs` | `constants::NAME`: which names are `i32` and which are `&str`. |
| `lib/script-sdk-2/src/items.rs`, `quests.rs`, `npc.rs`, `fx.rs` | `ctx.items()`, `ctx.quests()`, `ctx.npc()`, `ctx.fx()`. |
| `lib/script-sdk-2/src/player.rs`, `world.rs`, `jobs.rs`, `random.rs` | `ctx.player()`, `ctx.warp`, `ctx.set_npc_visible`, `ctx.time_field`, `ctx.ea_class`, `ctx.rand`, `ctx.rand_range`. |
| `lib/script-sdk-2/src/flow.rs` | `Script` and `Stop`. |
| `scripts/systems/src/npcs.rs` | Hand-written NPC scripts in the target style. |
| `tools/scripts-import/convert_sdk2.py` | The generator, to see why a pattern looks as it does. Read only. |

## The API you will use

| Call | Meaning |
| --- | --- |
| `ctx.mes(text)` | One dialogue box. `\n` breaks lines. |
| `ctx.mes_as(speaker, text)` | Same, with a `[speaker]` title line first. |
| `ctx.lines(args![..])` / `ctx.lines_as(speaker, args![..])` | One dialogue box from one line per value. This is what the converter emits. Keep it as it is: its line breaks are part of the text. |
| `ctx.next()` | Waits for the player to press next. |
| `ctx.menu(&["A", "B"])?` | Shows options and returns the 0-based index as `usize`. Cancel is an error that propagates with `?`. |
| `ctx.close()` / `ctx.end()` | `close()` is `close_window()?` then `Err(Stop::End)`. `end()` is `Err(Stop::End)`. Both return a `Script`. |
| `ctx.close_window()?` | Closes the window and keeps running. |
| `ctx.var("name").get()?` / `.set(v)?` | Reads and writes a rAthena variable (`Zeny`, a quest counter, ...). |
| `ctx.call(Function::X, args![a, b])?` | Calls a host function. |
| `args![a, b]` | Builds a `Vec<Val>` from numbers and strings. |
| `Val::from(..)` | Builds a number or text value from `i32`, `bool`, `&str`, `String`. |
| `val.number()?` | The value as `i32`. Errors if it is text. |
| `val.text()` | The value as text. |
| `val.is_true()` | Truthiness, as rAthena uses it. |

Comparisons: `val == 5` and `val == "5"` compare by text when the types differ, as rAthena does. `val < 5` orders numbers only, and a text is never less than or greater than a number. `val.loosely_equals(&other)` is the same loose equality as `==`.

### Typed wrappers you may use

Use one only when the original call has exactly the arguments shown (no more, no fewer), every argument is a literal (a number or a string written in the code, not a variable or an expression), and the original either discards the result or ends in `.number()?` (so the result is a number). Each of these sends exactly the request shown.

| Original | Replacement |
| --- | --- |
| `ctx.call(Function::CountItem, vec![Val::from(7311)])?.number()?` | `ctx.items().count(7311)?` |
| `ctx.call(Function::DelItem, vec![Val::from(7311), Val::from(1)])?;` | `ctx.items().take(7311, 1)?;` |
| `ctx.call(Function::GetItem, vec![Val::from(7311), Val::from(1)])?;` | `ctx.items().give(7311, 1)?;` |
| `ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(150), Val::from(150)])?;` | `ctx.warp("prontera", 150, 150)?;` |
| `ctx.call(Function::EnableNpc, vec![Val::from("Name")])?;` | `ctx.set_npc_visible("Name", true)?;` |
| `ctx.call(Function::DisableNpc, vec![Val::from("Name")])?;` | `ctx.set_npc_visible("Name", false)?;` |
| `ctx.call(Function::Cutin, vec![Val::from("image"), Val::from(2)])?;` | `ctx.fx().cutin("image", 2)?;` |
| `ctx.call(Function::SpecialEffect, vec![Val::from(10)])?;` | `ctx.fx().special_effect(10)?;` |
| `ctx.call(Function::Emotion, vec![Val::from(3)])?;` | `ctx.npc().emotion(3)?;` |
| `ctx.call(Function::NpcSpecialEffect, vec![Val::from(10)])?;` | `ctx.npc().special_effect(10)?;` |
| `ctx.call(Function::DoNpcEvent, vec![Val::from("Npc::OnLabel")])?;` | `ctx.npc().do_event("Npc::OnLabel")?;` |
| `ctx.call(Function::SetQuest, vec![Val::from(1000)])?;` | `ctx.quests().start(1000)?;` |
| `ctx.call(Function::CompleteQuest, vec![Val::from(1000)])?;` | `ctx.quests().complete(1000)?;` |
| `ctx.call(Function::EraseQuest, vec![Val::from(1000)])?;` | `ctx.quests().erase(1000)?;` |
| `ctx.call(Function::ChangeQuest, vec![Val::from(1000), Val::from(1001)])?;` | `ctx.quests().change(1000, 1001)?;` |
| `ctx.call(Function::CheckQuest, vec![Val::from(1000)])?.number()?` | `ctx.quests().check(1000)?` |
| `ctx.call(Function::Rand, vec![Val::from(10)])?.number()?` | `ctx.rand(10)?` |

If the original compares the `Val` directly, as in `ctx.call(Function::CountItem, ..)? == 0`, there is no `.number()?`, so keep the `ctx.call`.

## Generated patterns

These forms are in nearly every file. How to treat each one:

**`args!` and clones.** `vec![Val::from(a), Val::from(b)]` as a call argument list becomes `args![a, b]` when each item is a literal or a `Val` expression. `l_x.clone()` can lose its `.clone()` when it is only compared (`l_x.clone() == 1` becomes `l_x == 1`), and keeps it where the value is passed on by value and used again afterwards.

**Endings.** In a function that returns `Script`, `ctx.close_window()?; return Err(Stop::End);` becomes `return ctx.close();`, and a lone `return Err(Stop::End);` becomes `return ctx.end();`. In a function that returns `Result<Val, Stop>` (the `*_body` and `*_run` functions) keep `Err(Stop::End)`, because `ctx.close()` and `ctx.end()` have another return type there.

**`*_body` wrappers.** An NPC is often a private `fn x_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop>` plus `pub fn x(ctx: &Ctx) -> Script { x_body(ctx, Vec::new()).map(|_| ()) }`. Merge them into one `pub fn x(ctx: &Ctx) -> Script` only when all of these hold: the body never reads `args`, nothing else in the file calls `x_body`, and every `return` and the last expression of the body is either `Err(..)` or `Ok(` followed by a literal. Then change each `Ok(<literal>)` to `Ok(())`. If any of this is unclear, keep both functions.

**Menus.** `Val::from(runtime::select_values(ctx, &[Val::from("A:B:C")])?)` shows the options split on `:` and returns a 1-based number. The equivalent is `ctx.menu(&["A", "B", "C"])?`, which is 0-based. Do this only when all of these hold: the option text is one string literal, no part of it between `:` is empty, the result is used only inside the same function, and you can change every comparison of it (`== 1` becomes `== 0`, `== 2` becomes `== 1`, and so on, including `<`, `>` and `switch` cases) without a doubt. Otherwise keep `select_values`. Never mix the two numberings in one function. A menu whose options are computed (not one literal string) stays as it is.

**`switch` blocks.** A `switch` is lowered to a labelled block `'bN: { ... }` with `subjectN`, `matchedN` and `no_caseN`. Each case is `if !matchedN && subjectN == value { matchedN = true; }` followed by `if matchedN { body }`. Cases fall through: once a case has matched, every following case body runs too, until a `break 'bN;`. `no_caseN` is the `default:` test. You may rewrite a `switch` as `if`/`else if`, or `match`, only when every case body ends with `break 'bN;`, `return` or `continue`, and the default case, if any, is the last one. If a case body does not end that way, the cases fall through: leave the whole switch as it is.

**Local variables.** `l_x` is an rAthena local. You may rename a local to a plainer name (`l_gems` to `gems`) inside one function, if the new name cannot clash with another binding in that function. `runtime::local_set`, `runtime::local_get`, `runtime::index` and `runtime::arg` implement rAthena's arrays and `getarg`: leave them as they are. `runtime::op` is the generic operator: see "More examples of good cleanup" for when it can go.

**State machines.** A `*_run(ctx, step, args)` function with an `enum` and `'machine: loop` is the lowering of labels and `goto`. Keep the machine. You may rename the enum variants to say what each screen does, and clean up the code inside each screen.

## Cleanup steps, in order

Work through these in order and skip any that do not apply.

1. **Flatten nested `if`/`else`.** Replace `else { if c { .. } else { .. } }` with `else if`. Chains on one variable can become `match`, but see the warning below.
2. **Use `args!`.** Replace `vec![Val::from(a), Val::from(b)]` with `args![a, b]`.
3. **Remove clones in comparisons.** `l_x.clone() == 1` can become `l_x == 1` when `l_x` is `Val` and the comparison is by value. Check the type first.
4. **Remove dead code.** Delete locals that are never read, and `mut` that is not needed. Keep the `#![allow(..)]` lines (rule 8).
5. **Turn step-by-step dialogue into state.** When a conversation moves through several menus and screens, and the screens lead back to each other, write one `enum` variant per screen (named for what it does, such as `AskJob`, `Confirm`) and a `loop { match state { .. } }`. Keep it only when the flow really loops or jumps. A straight-line conversation stays as plain code.
6. **Use typed wrappers**, `ctx.menu`, `ctx.close()` and the other forms above, under their conditions.
7. **Look for the same kind of change** as in "More examples of good cleanup" below.
8. **Other** after steps 1-7, cleanup how you think would be best. Prioritise clean, readable code.

**Warning on `match`.** `Val` compares loosely. In rAthena, a text value is never equal to a number, and `.number()?` raises an error on text. So `match x.number()? { 1 => .. }` changes behaviour when `x` could hold text. Use `match` or `.number()?` only when the value is always a number, for example a local that only ever gets numeric literals or arithmetic results. When in doubt, keep `== 1` on the `Val`.

## More examples of good cleanup

These show the kind of change that makes a file read well. They are examples, not a checklist: when you meet similar code, apply the same idea if you are sure the result does exactly what the old code did (same host requests, same values, same errors). If you are not sure, leave the code alone and say so in your report. You cannot compile, so for every change look at the type of each name you touch and at every other place that uses it.

**Numbers do not need `runtime::op`.** `runtime::op(&a, "<", &b)?.is_true()` is the general rAthena operator: it works on text as well, and fails on a text next to a number. When both sides are always numbers it is just `<`. A variable whose name does not end in `$` is always a number (the server refuses to store text in it), so `.get()?.number()?` on it cannot fail and gives an `i32`. The results of `ctx.call(Function::CountItem, ..)`, `Function::EaClass`, `Function::GetTime` and `x.len() as i32` are numbers too. A local (`l_x`) is a number only if every assignment to it is a number.

```rust
// before
if (runtime::op(&ctx.var("$god2").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
    && runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
{
// after
if ctx.var("$god2").get()?.number()? >= ctx.var("$@god_check1").get()?.number()?
    && ctx.var("$god3").get()?.number()? < ctx.var("$@god_check2").get()?.number()?
{
```

The same applies to `|`, `&` and the other operators on numbers, and to comparisons with a number written in the code: `runtime::op(&ctx.var("x").get()?, "<", &Val::from(5))?.is_true()` becomes `ctx.var("x").get()?.number()? < 5`. Keep `runtime::op` when either side is a text, a `$`-named variable, a local you cannot show to be a number, or the result is used as a value that you cannot show to be an `i32`.

**Plain `==` on a `Val`.** `val.loosely_equals(&Val::from(1))` is the same as `val == 1`, and `val.loosely_equals(&Val::from("a"))` the same as `val == "a"`. `subject1.loosely_equals(&Val::from(2))` in a `switch` becomes `subject1 == 2`. When both sides are `Val`, keep `loosely_equals`: `==` between two `Val` is a stricter comparison.

**Constants that are plain numbers.** `ctx.constant("BC_MAP")?` asks the host for a number each time. `script_sdk_2::constants::BC_MAP` is the same number, known when the module is built. Use it only for names declared as `pub const NAME: i32` in `lib/script-sdk-2/src/constants.rs`: search that file for the name. The names declared `&str` (such as `EQI_ACC_L`) are text, not the same thing: keep `ctx.constant(..)` for those. Add `constants` to the `use script_sdk_2::{..}` line.

```rust
// before
runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?
// after
Val::from(constants::BC_MAP | constants::BC_NPC)

// before
ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?)
// after
ctx.var("Sex").get()? == constants::SEX_MALE
```

**Named getters in place of the same request.** Where the old code sends exactly the request the getter sends, and the result is used as a number or text, use the getter:

| Original | Replacement |
| --- | --- |
| `ctx.var("Zeny").get()?.number()?` | `ctx.player().zeny()?` (also `base_level()`, `job_level()`, `class()` for `BaseLevel`, `JobLevel`, `Class`) |
| `ctx.call(Function::StrCharInfo, vec![Val::from(0)])?` | `ctx.player().name()?`, a `String`: fine inside `Val + ..`, `args![..]`, `.set(..)` and `Val::from(..)` |
| `ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()?` | `ctx.time_field(constants::DT_HOUR)?` |
| `ctx.call(Function::EaClass, vec![])?.number()?` | `ctx.ea_class(None)?` |
| `ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?.number()?` | `ctx.rand_range(1, 4)?` |

These return `i32`, so a comparison reads `ctx.player().zeny()? < price` once `price` is a number. If the old result was kept as a `Val` (a `let subject = ..` that later calls `subject.loosely_equals(..)`, or a value passed on as a `Val`), either change every use of it or keep the old call.

**Arithmetic on numbers.** `a.try_sub(b)?`, `try_mul`, `try_div` and `try_rem` on `Val` are rAthena's operators. On two numbers they are `-`, `*`, `/` and `%`, with one difference: division or remainder by zero is an error that ends the script, where Rust's `/` and `%` panic. Overflow wraps in both, and it does not occur in practice, so do not worry about it. So when both operands are numbers (the rules of "Numbers do not need `runtime::op`") write them with Rust operators on `i32`, and wrap the result in `Val::from(..)` where a `Val` is needed. Divide or take a remainder this way only when the divisor is a non-zero number written in the code. `+` is different: on a `Val` it joins texts, so change it only when both sides are numbers. Do not use `ctx.var(..).add(n)` in place of a read and a write: it is another host request.

```rust
// before
ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_pay.clone())?))?;
if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 10000 {
l_pay = (l_input.clone().try_mul(Val::from(15))?);
l_sec = (l_start_time.clone().try_rem(Val::from(100))?);
// after, when `l_pay`, `l_input` and `l_start_time` are known to be numbers
ctx.player().set_zeny(ctx.player().zeny()? - l_pay.number()?)?;
if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 10000 {
l_pay = Val::from(l_input.number()? * 15);
l_sec = Val::from(l_start_time.number()? % 100);
```

Keep the type of a local as it is unless you can see every use of it: turning `l_pay` from `Val` into `i32` is only safe when each later use accepts an `i32`. Wrapping the new value back in `Val::from(..)` is the safe choice.

**Dead parentheses and temporaries.** After the changes above, drop `( .. )` that no longer groups anything, and inline a `let` that is used once right after it. Keep a `let` that names a value the reader would otherwise have to work out.

## Example 1: a chain of `if`

Before:

```rust
if l_new_book.clone() == 1 {
    ctx.call(Function::DelItem, vec![Val::from(535), Val::from(10)])?;
    ctx.call(Function::GetItem, vec![Val::from(7472), Val::from(1)])?;
} else {
    if l_new_book.clone() == 2 {
        ctx.call(Function::DelItem, vec![Val::from(538), Val::from(5)])?;
        ctx.call(Function::GetItem, vec![Val::from(7473), Val::from(1)])?;
    } else {
        if l_new_book.clone() == 3 {
            ctx.call(Function::DelItem, vec![Val::from(551), Val::from(5)])?;
            ctx.call(Function::GetItem, vec![Val::from(7474), Val::from(1)])?;
        }
    }
}
```

After, if `l_new_book` is always a number (rule above):

```rust
match l_new_book.number()? {
    1 => {
        ctx.items().take(535, 10)?;
        ctx.items().give(7472, 1)?;
    }
    2 => {
        ctx.items().take(538, 5)?;
        ctx.items().give(7473, 1)?;
    }
    3 => {
        ctx.items().take(551, 5)?;
        ctx.items().give(7474, 1)?;
    }
    _ => {}
}
```

If you cannot show `l_new_book` is always a number, use `if`/`else if` on `l_new_book == 1` instead. That is still flat and keeps the loose comparison.

## Example 2: a whole NPC (wrapper, menus, endings)

Before (the `ctx.lines_as` texts are shortened here; yours stay whole):

```rust
fn airport_staff_airport1a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Airport Staff", args!["Welcome to the", "Einbroch Airport,"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Board the Airship:Cancel")])?) == 1 {
        ctx.lines_as("Airport Staff", args!["The Airship boarding fee", "is 1,200 zeny."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            if ctx.call(Function::CountItem, vec![Val::from(7311)])?.number()? > 0 {
                ctx.call(Function::DelItem, vec![Val::from(7311), Val::from(1)])?;
                ctx.call(Function::Warp, vec![Val::from("airport"), Val::from(148), Val::from(51)])?;
                return Err(Stop::End);
            }
            if ctx.var("Zeny").get()?.number()? >= 1200 {
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1200))?))?;
                ctx.call(Function::Warp, vec![Val::from("airport"), Val::from(148), Val::from(51)])?;
                return Err(Stop::End);
            }
            ctx.lines_as("Airport Staff", args!["I'm sorry, but you don't", "have enough zeny."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Airport Staff", args!["Thank you and", "have a nice day."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn airport_staff_airport1a(ctx: &Ctx) -> Script {
    airport_staff_airport1a_body(ctx, Vec::new()).map(|_| ())
}
```

After: the body never reads `args` and only ends in `Err(Stop::End)`, so the two functions merge. Both menus are one literal each, used once, so they become `ctx.menu` and `== 1` becomes `== 0`. The three warps and item calls have literal arguments.

```rust
pub fn airport_staff_airport1a(ctx: &Ctx) -> Script {
    ctx.lines_as("Airport Staff", args!["Welcome to the", "Einbroch Airport,"])?;
    ctx.next()?;
    if ctx.menu(&["Board the Airship", "Cancel"])? == 0 {
        ctx.lines_as("Airport Staff", args!["The Airship boarding fee", "is 1,200 zeny."])?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.items().count(7311)? > 0 {
                ctx.items().take(7311, 1)?;
                ctx.warp("airport", 148, 51)?;
                return ctx.end();
            }
            if ctx.player().zeny()? >= 1200 {
                ctx.player().set_zeny(ctx.player().zeny()? - 1200)?;
                ctx.warp("airport", 148, 51)?;
                return ctx.end();
            }
            ctx.lines_as("Airport Staff", args!["I'm sorry, but you don't", "have enough zeny."])?;
            return ctx.close();
        }
    }
    ctx.lines_as("Airport Staff", args!["Thank you and", "have a nice day."])?;
    ctx.close()
}
```

Note what did not change: the `ctx.lines_as` texts, the order of every call, and the variable write.

## Checking your work

Run `rustfmt --edition 2021 <your file>`. Then run `git diff -U0 -- <your file>` and read it once, looking for: a changed string (the same text must appear on both sides, except where a menu string such as `"Yes:No"` became `"Yes", "No"`), a changed number, a call that moved or disappeared, a comparison whose numbering you shifted, and a brace that no longer matches. Do not compile or run any test.

## Report

End with a short report, no more than 15 lines:

- The file you changed, and the NPC or event names in it.
- The cleanup steps you applied, with counts.
- What you left as it was, and why (for example, "`l_new_book` may be text, so kept `== 1`").
- Whether you deleted the `// Generated by` line.
- Anything you are unsure about, with the function name. Say what would settle it.

Do not claim the code compiles or that the behaviour is unchanged: say "not compiled" and "not run".
