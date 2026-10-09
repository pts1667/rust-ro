# Item script API

Refines [the typed script SDK](4-typed-script-sdk.md). It gives item scripts their own typed contexts, in place of the generated `run_item` and `run_bonus` functions of the retired numeric-ABI item module.

**Date**: 2026-10-09

# Context
An item script runs in one of two modes, and the server already enforces the split at run time:

- **Passive** (`ItemScriptHost::bonuses`): runs when the item is worn or refined. It can read the wearer's status and describe bonuses. It cannot write variables, queue effects or open a dialogue.
- **Use** (`ItemScriptHost::consumable`): runs when the item is consumed. Each effect (`Heal`, `Grant`, `Write`, `Call`) is queued in `ItemEffect` and applied after the script returns `Ok`.

A use script that calls a dialogue function (`mes`, `next`, `close`, `select`, the input calls, `cutin`, `message`, `dispbottom`, `callfunc`) runs as a conversation. `start_item_dialog` gives it the same dialogue channel an NPC has, through a fake NPC (id 4,000,000,000). Its effects are applied when the conversation completes.

Four facts from the server shape the design:

1. **Effects are all or nothing.** A script that errors, times out or is cancelled (the player closes the window) applies no effects and does not consume the item. Completion also checks that the item is still the same inventory entry and that the character is alive.
2. **Reads see some of the script's own changes.** A write goes into the item's local variable map, and that map is checked before status variables. Item counts track the script's own `GetItem` and `DelItem`. Status reads otherwise come from a snapshot taken when the script starts.
3. **Variable routing depends on the name.** In a conversation, a read goes to the item host only when the name is already in the item's variable map or is a status variable. Any other read goes to the NPC host.
3a. **Scoped variables work only in conversations.** The item host rejects `VariableRead` and scoped writes with `Operation is unavailable in an item script`, so `#account`, `$server`, `@temporary`, `'instance` and `.npc` variables fail in a direct use. In a conversation the same requests go to the NPC host and succeed. Legacy item code sends plain `Read` for these names, which the item host answers with the default value, so `ctx.read("@megaphone$")` returns empty text with no error. Only one converted item reads a scoped name (`@megaphone$`), and only one writes one.
3b. **`.@` names are item locals.** The converter already emits them as Rust locals (`local_*`), so they never reach the host.
4. **Item-only functions are rejected elsewhere.** `Heal`, `PercentHeal`, `ItemHeal`, `ItemSkill`, `SkillEffect`, `Produce`, `Cooking`, `GetRefine` and `RandomGroupItem` have handlers only on the item path. An NPC script that calls one gets `Unsupported game request`.

# Decision
- **Two contexts.** `ItemBonus` offers bonus methods and read-only wearer queries (`refine`, `wearer().base_level()`, ...). `ItemUse` offers those plus effects and dialogue. A passive script therefore cannot queue an effect, and a use script must say so by taking an `ItemUse`. The compiler enforces the split.
- **Dialogue is a use-context capability, not a separate mode.** `ItemUse` has the same methods as `Ctx` for dialogue (`mes`, `mes_as`, `next`, `menu`, `close`, `close_window`, `input_number`, `input_text`). The server chooses the conversation path from the calls, so the registry derives `interactive` from the script. Authors do not declare it.
- **Effects are queued, and the docs say so.** Each effect method returns after queueing, and nothing is applied until the script returns `Ok`. A script that wants partial results must run its effects after its last dialogue step. A cancelled menu discards everything queued.
- **Locals are Rust locals; typed variables are character-only.** `.@` names are plain `let` bindings, as the converter already emits them. `ItemUse::read` and `write` accept character-scope names only and return `Stop::Error` for a scoped one. `read_any` and `write_any` take the name as the script spells it, so generated code can use scoped names. The host answers a scoped read from the values it loaded before the script ran (every name the script reads is listed in `items.json`), and a scoped write is queued like any other effect. The NPC API keeps `ctx.var` for every scope.
- **Entries are keyed by item id.** `item_module!` lists the item functions by id: `ItemUse` bodies in one table, and `ItemBonus` bodies (passive items with no effects) in another. The server runs an item with `Entry::Item(id)`, which tries the use table and then the bonus table, so the generator's split decides the context. Automatic bonus and visual programs use `Entry::Program(id)`.

Alternatives considered:
- **One context with a mode flag.** Rejected: the mode is only checked at run time, and the mistake shows up as an error during play rather than at compile time.
- **Deref `ItemUse` to `Ctx`.** Rejected: an item script would then reach NPC-only calls such as `warp` or `npc().emotion`, which the server rejects.

# Consequences
- `lib/script-sdk-2/src/item.rs` implements the two contexts. `item_module!` registers the functions by id, `ItemVm` routes items and programs to the items module (`config/wasm/items.wasm`), and `import_items.py` generates the module.
- Name-based routing in `ItemDialogHost` is unchanged. Scoped variables are served through `read_any` and `write_any`, which use the values the host pre-loads from each script's read list, so `ItemDialogHost` does not serve `VariableRead` to item scripts. The typed `read` and `write` still refuse scoped names.
- A scoped read the host did not pre-load still returns the default without an error, as the legacy item module did. Pre-loading ignores a failed repository read (`prepare_host`), so that case stays silent until the host has an error path for it.
- Generated code calls every host function through `item.call(Function::X, ...)` with the arguments the rathena script gives, so the port changes no request. Typed wrappers can replace those calls one function at a time, with the parity check as the guard.
- The `callfunc` helpers (`F_Rand`, `F_CashStore`, ...) are ported to `scripts/items/src/helpers.rs`.
- The legacy item module (`scripts/src/items.rs`) is removed. Before removal, every item and bonus program ran with the same host in the use and passive modes against the legacy module, and queued the same effects, bonuses and error: 2,687 items in two hosts each, no mismatches. That check was deleted with the legacy module.
