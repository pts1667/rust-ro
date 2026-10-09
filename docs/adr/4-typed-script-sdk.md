# Typed script SDK

Refines [compiled WebAssembly scripts](3-wasmtime.md). It replaces the single-module layout and the generated `ctx.call(Function::X, vec![..])` style with a readable, typed API.

**Date**: 2026-10-09

# Context
The NPC scripts are generated from rathena `.txt` files. They compile into one module, and each script is a numbered function (`npc_10506`). Conditions are `op(n(select(ctx, &[..])?), "==", n(1))?.truthy()`, constants are looked up at run time (`constant(ctx, "JOB_NOVICE")?`), and each dialogue line is its own `mes` call. The code is hard to read, and a script's number shifts whenever the source list changes.

`lib/script-sdk` already holds the wire protocol (`Function`, `Request`, `Value`) and the `rust_ro::invoke` import. Those stay.

# Decision
- `lib/script-sdk-2` is the authoring API. It wraps the wire types in typed groups (`ctx.items()`, `ctx.quests()`, `ctx.player()`, ...), dialogue helpers (`ctx.mes` takes multi-line text, `ctx.menu` returns a 0-based index), and `Stop`-based control flow (`?` ends a script early).
- Constants are typed `pub const`s in `script_sdk_2::constants`. Their values come from the server's own lookup, through `tools/scripts-import/gen_sdk2_constants.py`, so they cannot drift from the server.
- `ctx.call(Function::X, args![..])` stays as the escape hatch for any function without a typed wrapper.
- Generated scripts are hand-maintained once the generator has produced them. The generator is run once per migration, not on every build.
- Tests run the same scripts against `MockTransport`, a recording transport with scripted replies. No game host is needed.

The module split and name-based dispatch are in [ADR 6](6-script-modules.md). This record covers only the authoring API and constants.

# Consequences
- Scripts read like ordinary Rust, and the mock transport tests them natively.
- Typed wrappers are thin: each names one `Function` variant and its argument order, so a wrong argument order is caught by the wrapper's unit test.
- Anything without a wrapper stays on `ctx.call`, so coverage can grow one group at a time.
- The constants file must be regenerated after a server change to a constant's value. The generator needs a release build of the server test suite, so it is slow to run.
