# Plugin UI

A plugin can open its own windows for a player, holding text, images, item icons, buttons, inputs and tables, and it handles
their clicks. It can draw notifications above units, such as an image rising over a player who levelled up. It can also
choose what bots see of it and which actions they may run. This page covers windows, overheads, bots, assets, limits
and the packets. Design and rationale are in [ADR 8](adr/8-plugin-ui.md), and the general plugin guide is
[plugins.md](plugins.md).

The example is `ui_shop` (`scripts/plugins/ui_shop`). Its Curio Trader in Prontera opens a shop with an animated coin,
a scrolling stock table and an amount input. A level up shows a star above the player, and bots can read the stock and buy.
To try it, enable it in `config.json` (`{ "name": "ui_shop", "enabled": true }`) and restart.

Windows and overheads are drawn by [nostalro-client](https://github.com/nmeylan/nostalro-client). The stock client knows none of
this: the server sends these packets only to a client that announced it draws them, and a stock client never sees a plugin
window.

## Windows

A window needs the `Interface` capability and a handler for its id in the `windows { }` section of `plugin_module!`:

```rust
use script_sdk_2::ui::{Align, EventKind, ImageRef, Node, Table, UiEvent, Window};

script_sdk_2::plugin_module! {
    info { name: "my_shop", version: "1.0.0", capabilities: [Interface, Economy] }
    init => init,
    npcs { "Trader" => trader }
    events {}
    hooks {}
    interceptors {}
    windows { "shop" => shop_event }
    bot_actions {}
}

fn trader(ctx: &Ctx) -> Script {
    let stock = Table::new("stock", 6)
        .column("", 32, Align::Center)
        .column("Item", 150, Align::Start)
        .row("501", [Node::item(501), Node::text("Red Potion")]);
    ctx.ui().open(&Window::new("shop", "Trader", 300, Node::column([
        Node::row([Node::image(ImageRef::plugin("coin.png"), 24, 24), Node::title("Welcome")]),
        stock.into(),
        Node::row([Node::number_input("amount", 1, 1, 99), Node::button("buy", "Buy")]),
    ])))
}

fn shop_event(ctx: &Ctx, event: &UiEvent) -> Script {
    match (event.kind, event.widget.as_str()) {
        (EventKind::Click, "buy") => {
            let amount = event.number("amount").unwrap_or(1);
            // ... buy, then reopen the window with the new state
            Ok(())
        }
        _ => Ok(()),
    }
}
```

- `ctx.ui().open(&window)` can be called from any run with a player attached: an NPC click, a hook, a window event or a bot
  action. `init` has no player, so the call is refused there.
- A window stays open after the script ends, until the player closes it or the plugin calls `ctx.ui().close(id)`.
- Opening an id that is already open replaces its content. The window keeps where the player moved it, its scroll position
  and what the player typed, except in an input whose `value` the plugin changed. A handler that changes something re-opens
  the window with the new state.
- A plugin can have 8 windows open per player. The server keeps them until the character logs out, across map changes,
  except windows anchored above a unit (see [Placement and frame](#placement-and-frame)).
- When the client announces itself again (`CZ_CLIENT_FEATURES`, after each map enter), the server sends it every window
  still open, with their countdowns shortened by the time since they were opened.
- Only plugins can open windows. Base script modules use the stock dialogue (`mes`, `menu`, `callshop`).

### Nodes

Nodes are tagged by `type` in JSON. The SDK builders are on `Node`, `Table` and `ImageRef` (`plugin_ui`, re-exported in
`script_sdk_2::ui`).

| Node | Builder | Fields |
| --- | --- | --- |
| `column` | `Node::column([..])` | `gap` (4), `align` of the children (`start`, `center`, `end`), `children` |
| `row` | `Node::row([..])` | `gap` (4), `align` vertical (`center` from the builder), `children` |
| `text` | `Node::text`, `wrapped`, `title` (bold) | `text`, `color` `RRGGBB`, `bold`, `align`, `wrap`, `shadow`, `outline` (`.outlined()` sets both). `^RRGGBB` codes work inside the text, except in bold text |
| `countdown` | `Node::countdown(seconds)` | time left, counted down by the client as `2h 05m`, `42m 10s`, `9s` (`format_countdown`); `color`, `bold`, `align`, `shadow`, `outline` like text |
| `image` | `Node::image(ImageRef, w, h)` | an image, animated when it has several frames, see [Assets](#assets) |
| `item_icon` | `Node::item(id)` | the item's icon, with its name as tooltip, and an optional `amount` |
| `button` | `Node::button(id, label)`, `.disabled()` | `id`, `label`, `enabled` |
| `number_input` | `Node::number_input(id, value, min, max)` | `id`, `value`, `min`, `max`, `width` (60) |
| `text_input` | `Node::text_input(id, value, max_len)` | `id`, `value`, `max_len`, `width` (60) |
| `table` | `Table::new(id, visible_rows)` | `columns` (`title`, `width`, `align`), `rows` (`id`, `cells`), `visible_rows`, `selectable`, `row_height` (24) |
| `gauge` | `Node::gauge(value, max, width)` | a bar, `color` `RRGGBB` |
| `spacer` | `Node::spacer(px)` | vertical space in a column, horizontal in a row |
| `separator` | | a line across a column, or down a row |

Layout:
- A column stacks its children. Text and separators take its full width; other children are placed by the column's `align`.
- A row lines its children up from the left and aligns them vertically.
- Wrapped text wraps at the width the column offers. A window without `height` grows with its content, and `width` is
  always set by the plugin.
- A table shows `visible_rows` rows and scrolls when it has more. Each cell is centred vertically in its row and placed in its
  column by the column's `align`. A cell can hold any node, a button included.

### Events

The handler gets a `UiEvent`:

| `kind` | When | `widget` | `row` |
| --- | --- | --- | --- |
| `click` | an enabled button is clicked | the button's id | |
| `submit` | Enter in an input | the input's id | |
| `select` | a row of a `selectable` table is clicked | the table's id | the row's id |
| `closed` | the player closed the window (title bar or Escape) | empty | |

- `inputs` carries the value of every input of the window: `event.number("amount")`, `event.text("name")`.
- The server checks every event against the window it last sent. A widget that is not in it, a disabled button or a window
  that is not open are all refused before the plugin runs.
- Numbers are clamped to the input's `min..=max`.
- A `closed` event removes the window before its handler runs. A `fixed()` window (`closable: false`) cannot be closed by
  the player.
- One event or bot action runs at a time per player and plugin, and up to 4 more wait their turn; anything beyond that is
  refused. Two fast clicks on Buy therefore cannot race on the player's zeny.

### Placement and frame

A window floats by default: the player moves it, and the title bar closes it. Two other placements exist:

```rust
// A speech bubble above an NPC, 4 px over its head, with buttons the player can click
Window::new("offer", "Dalia", 230, content).anchored(npc_id, 4);
// A tracker under the minimap, text over the game with no background, that the player folds to its title
Window::new("quests", "Quests", 180, lines).hud(HudSlot::BelowMinimap).frameless().collapsible();
```

| Placement | Builder | Drawn |
| --- | --- | --- |
| `floating` | (default) | where the player moves it |
| `anchored` | `.anchored(unit_id, offset_y)` | above the unit, following it, hidden while the unit is out of view. `unit_id` is the unit's id in scripts (a character id or an NPC id); the server sends the client the id it knows the unit by (the account id for players) |
| `hud` | `.hud(HudSlot::BelowMinimap)` | right-aligned under the minimap; the windows of a slot stack in the order they were opened |

- Only floating windows can be closed or moved by the player: the others are always `closable: false`.
- Anchored windows close when the character changes map, since their unit is not there anymore.
- `.frameless()` (`frame: "none"`) draws no background, border or title bar: only the content. Use `.outlined()` text so it
  stays readable over the game.
- `.collapsible()` adds a `-`/`+` toggle by the title; a frameless collapsible window shows its title as a header line.
  Re-opening the same id, as a handler or the re-send at map enter does, keeps it folded or unfolded; closing the window or
  restarting the client forgets it.
- A countdown is sent as seconds left and counted down by the client; the server does not resend it each second.

## Overheads

An overhead is an image, a caption under it, or both, drawn above a unit for `duration_ms` and moved by its motions:

```rust
let overhead = Overhead::image(ImageRef::plugin("star.png"), 48, 48, 2200)
    .caption("LEVEL 10!", Some("ffd040"))
    .offset(10)
    .pop(350)
    .rise(36.0, 1600)
    .easing(Easing::EaseOut)
    .fade_in(150)
    .fade_out(600);
ctx.ui().overhead(Above::Player, &overhead)?;
```

- `Above::Player` is the attached player, and `Above::Unit(id)` any unit: a character id, an NPC or a monster.
- `audience` is `area` by default (players within 14 cells of the unit) or `self_only` (`.only_self()`, the attached player alone).
- Nothing is stored server side, so an overhead is sent once and the client plays it.

| Motion | Builder | Effect |
| --- | --- | --- |
| `rise` | `.rise(px, ms)` | moves up by `px` |
| `fade_in` | `.fade_in(ms)` | from transparent |
| `fade_out` | `.fade_out(ms)` | to transparent; without `.at()` it ends with the overhead |
| `scale` | `.scale(from, to, ms)` | grows or shrinks |
| `pop` | `.pop(ms)` | grows from nothing past its size, then settles |
| `shake` | `.shake(px, ms)` | sideways, calming down |
| `bounce` | `.bounce(px, ms)` | two hops, the second lower |

Motions run at the same time from the moment the overhead appears. `.at(ms)` delays the last one, and `.easing(..)` sets its
curve: `linear`, `ease_in`, `ease_out` (default) or `ease_in_out`. `Overhead::transform(elapsed_ms)` is the shared
evaluation that the client draws with.

## Bots

Bots reach plugins through the bot API (`server/src/server/bots/SKILL.md`, "Plugins"):

- Their observation has a `plugins` section listing each plugin's open windows (the same JSON the client draws), its `data`
  and its `actions`.
- `{"type": "plugin_ui", ...}` does what a player does in a window. It goes through the same checks and the same handler,
  and answers with the window as the handler left it.
- `{"type": "plugin", "plugin", "name", "args"}` runs one of the plugin's bot actions. It answers with what the action
  passed to `ctx.bot().reply(..)`.
- Overheads near a bot come as `plugin_overhead` events.

What bots see and do is up to the plugin:

```rust
script_sdk_2::plugin_module! {
    // ...
    bot_actions { "buy" => (bot_buy, "Buys from the trader: {item_id, amount}") }
}

fn trader(ctx: &Ctx) -> Script {
    ctx.bot().observe("stock", &stock)?;   // any Serialize, shown under plugins.my_shop.data.stock
    // ...
}

fn bot_buy(ctx: &Ctx, action: &BotAction) -> Script {
    let args: BuyArgs = bot_args(action)?;  // the bot's `args`, through serde
    // ...
    ctx.bot().reply(&json!({ "bought": amount }))
}
```

- `observe(key, value)` stays until it is replaced, until `ctx.bot().forget(key)`, or until the character logs out. A plugin
  can keep 32 keys of up to 16 KiB each per character.
- A bot action runs with the bot's character attached, and needs no capability of its own (its calls do). The last `reply`
  wins; an action that never replies answers `null`.
- The answer must come within the bot API's 5 seconds.

## Assets

Images are PNG files in the plugin's `ui/` folder, `config/plugins/<name>/ui/`. In a frame path, `@/coin.png`
(`ImageRef::plugin("coin.png")`) is a file of that folder. The server turns it into `plugin/<name>/coin.png`, serves it at
`/data/plugin/<name>/coin.png` on the client data port, and lists it in `/data/check` with its size and md5. The client
downloads and caches it like any other file.

- Any other path is a client texture relative to `data/`, so a window can use the game's own art (`texture/...`).
- A path may not climb (`..`), start with `/` or `\`, or name another plugin's folder.
- Name files in lower case: the client asks for paths in lower case.
- `ImageRef::plugin_animated(["coin_0.png", "coin_1.png"], 8.0)` plays the frames at 8 per second, in a loop; `.once()` holds the
  last frame.
- New or changed assets take a server restart, which rebuilds `/data/check`.

## Limits

The server checks every window and overhead (`plugin_ui::validate_window`, `validate_overhead`) and refuses the call with
the reason:

| | |
| --- | --- |
| window JSON | 32 KiB |
| window width / height | 80..=1024 / up to 768 |
| nodes / nesting depth | 512 / 16 |
| table rows / columns | 200 / 12, at least 1 visible row, unique row ids, no more cells than columns |
| text / labels and ids | 1024 / 64 characters, no control characters (use several nodes instead of newlines) |
| widget ids | unique in the window |
| colours | `RRGGBB` |
| anchored window | a unit id other than 0 |
| image frames / path / side | 64 / 128 characters / 512 px, fps 0..=60 |
| overhead | an image or a text, 1..=10000 ms, 16 motions |

## Packets

These packets are on the map connection. They continue the custom range of the [login packets](custom-login-packets.md).
Variable packets are `id (u16), total length (u16), body`, little endian, and their bodies are JSON. Encoding and decoding
live in `lib/plugin-ui/src/wire.rs`, which the client shares.

| Id | Name | Direction | Body |
| --- | --- | --- | --- |
| `0xC3E4` | `CZ_CLIENT_FEATURES` | client → map | 6 bytes: id, `u32` flags. Flag `1` means "draws plugin UI". Sent after each map enter |
| `0xC3E5` | `ZC_PLUGIN_WINDOW` | map → client | variable: `u8` op, then JSON. Op `0` opens or replaces (`{plugin, window}`); op `1` closes (`{plugin, id}`) |
| `0xC3E6` | `CZ_PLUGIN_UI_EVENT` | client → map | variable: JSON `{plugin, event}` |
| `0xC3E7` | `ZC_PLUGIN_OVERHEAD` | map → client | variable: `u32` unit id (account id for players), JSON `{plugin, overhead}` |

The `ZC_*` packets go only to a session that sent `CZ_CLIENT_FEATURES` with flag `1`.
