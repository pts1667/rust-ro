# Plugin UI

Builds on [server plugins](7-plugins.md). Author guide: [docs/plugin-ui.md](../plugin-ui.md).

**Date**: 2026-10-18

**Status**: Accepted

# Context
Plugins reach players only through the stock dialogue: `mes`, `menu`, cutins, effects and the NPC shop with its fixed
layout. A plugin that sells from a changing stock, shows a leaderboard or tracks an event has to squeeze it into a dialogue
box that can neither scroll a table nor show an icon next to a price. We want plugins to declare their own UI. That means
windows with text, tables, images (animated ones too), item icons, buttons and inputs, enough for a shop menu. It also
means notifications drawn above units, such as "LEVEL UP" rising and fading.

What constrains the design:
- **We own the client.** nostalro-client is ours and draws its UI in immediate mode from plain data. The stock client cannot
  be extended, and stock packets cannot carry this. ADR 7 left custom packets as a later ADR, and this is that ADR.
- **Scripts run off the game loop** (ADR 7). A click has to reach the plugin as a run, and the plugin's answer comes back as
  host calls.
- **Bots are players too.** CLAUDE.md asks that every bot action answer with what the game did. A plugin window that a bot
  cannot read or click would split the game in two.
- **Plugins are untrusted.** Whatever a plugin sends goes out on many connections, and whatever a client sends back reaches
  a plugin.

# Decision

## A shared schema crate
`lib/plugin-ui` holds the whole vocabulary: `Window` and its `Node` tree, `UiEvent`, `Overhead` and its motions, validation and
the wire encoding. It depends only on serde, so the guest (through `script-sdk`), the server and nostalro-client (as a git
dependency, like `packets`) compile the same types. The server and the client cannot drift apart on a field name, and the
client evaluates motions with the same `Overhead::transform` the tests check.

## Persistent windows, replaced whole
- A window is a tree of nodes that a plugin opens with `ctx.ui().open(&window)` from any run that has a player attached. It
  stays open until closed. Its events go to a handler the plugin declares for its id in `plugin_module!` (`windows { }`), run
  as `Entry::UiEvent` with the player attached.
- After an event the plugin sends the whole window again rather than a diff. Windows are small (32 KiB at most), JSON
  compresses nothing worth a patch format, and the plugin keeps no UI state between runs. To make full replacement
  painless, the client keeps the window's position, its scroll offsets, its row selection and what the player typed,
  unless the plugin itself changed an input's value.
- The server stores the last window per (character, plugin, id) in `PluginUiStore`, a cloneable handle with its own lock
  like `Server::duels()`, and clears it at logout. The store is what events are checked against. It is also what bots
  observe, and it lets a stateless handler know what the player was looking at.
- Each event carries the value of every input of the window, so a Buy click brings the amount. A plugin never asks the
  client for state.

## Events are checked, then serialized
- The server accepts an event only for an open window, an existing widget of the right kind (an enabled button, an input,
  a selectable table and one of its rows), and inputs clamped to their bounds (`UiEvent::accepted_by`). A forged packet
  cannot click a disabled Buy button or invent a row.
- Runs are serialized per (character, plugin): one in flight, 4 waiting, the rest refused. `GameEvent::PluginJobDone` starts
  the next. This stops two fast clicks from racing on the player's zeny, and it is the rate limit. The queue is shared with
  bot actions.

## Overheads are fire-and-forget
An overhead is an image and/or a caption with a duration and a list of motions (rise, fade in, fade out, scale, pop, shake,
bounce, each with a start, a length and an easing). The server validates it and sends it to the players around the unit, or
only to the attached player, and stores nothing. The client plays it from the moment it arrives. Motions are data, not
code, so a plugin cannot make the client compute anything unbounded.

## Custom packets on the map connection
`0xC3E4`-`0xC3E7` continue the custom range of the login packets: `CZ_CLIENT_FEATURES`, `ZC_PLUGIN_WINDOW`,
`CZ_PLUGIN_UI_EVENT` and `ZC_PLUGIN_OVERHEAD`. Variable packets carry JSON bodies.
- The client announces that it draws plugin UI after each map enter, and the server sends the `ZC_*` packets only to sessions
  that did. The stock client, which would drop the connection on an unknown header, never receives one.
- Both directions are registered in the framing tables (`request_handler/framing.rs` on the server, `connection.rs` on the
  client), because an unregistered header drops the connection.
- JSON rather than a binary layout: the tree is recursive and optional-heavy. The same JSON is what bots read, and the
  encoding cost is small next to the run that produced the window.

## Assets travel with the plugin
Images are PNG files in `config/plugins/<name>/ui/`. A frame path written `@/file` becomes `plugin/<name>/file`. The client
data endpoint serves that folder at `/data/plugin/<name>/` and lists its files in the `/data/check` it builds at boot,
so the client downloads and caches them like any other file. A plugin may not name another plugin's folder or climb out of
`data/`. Item icons come from the client's own tables by item id.

## Bots get the same reach, plus what the plugin chooses
- The bot observation lists each plugin's open windows, exactly as the client receives them.
- The `plugin_ui` action runs the same checks and handler as a click and answers with the window after the handler ran.
- A plugin may also declare bot actions (`bot_actions { "buy" => (fn, "description") }`, run as `Entry::BotAction`) and
  publish observations (`ctx.bot().observe(key, value)`). Then a bot does not have to scrape a table to know the stock.
  `ctx.bot().reply(value)` is the answer to the action.
- The bot's reply travels with the queued job, so it answers only when the plugin has run, within the bot API's 5 seconds.

## Capability and ownership
`UiOpen`, `UiClose` and `UiOverhead` need the new `Capability::Interface`. `BotObserve` and `BotReply` only touch the
plugin's own data and need none. The runtime appends the calling plugin's name to these calls, and refuses them from base
modules: only a plugin has handlers to receive the events. A plugin can therefore neither close nor observe as another plugin.

# Consequences
- A plugin can build a shop, a board or a tracker without server changes. `ui_shop` is the example: an animated coin, a
  scrolling stock table, an amount input, a level-up overhead and a bot `buy` action.
- The server sends more on the map connection, but only to clients that asked for it, and only when a plugin acts.
- `plugin-ui` is part of the SDK surface. New node kinds or fields with serde defaults keep old plugins loading. A plugin
  that uses a node an older client does not know gets an error in that client's log, and the window does not open.
- `PluginInfo` gained `windows` and `bot_actions` with serde defaults, and `plugin_module!` takes both sections as optional.
  `ABI_VERSION` did not change, so `double_xp` built before this keeps loading. The plan had foreseen a bump; it turned out
  unnecessary.
- nostalro-client depends on rust-ro for one more crate. Until rust-ro publishes it, the client uses a path dependency.

# Alternatives considered
- **Extending NPC dialogue** (more `mes` codes, a richer `callshop`). It stays within stock packets, but it cannot express
  tables, icons or free layout, and it keeps the window modal to an NPC conversation.
- **HTML in a web view.** It is maximally expressive, but it needs a browser engine in the client and opens a script channel
  from untrusted plugins to the client. Its look also matches nothing else in the UI.
- **Diff updates.** They mean smaller packets for big tables, but also a patch format, versioned state on both sides and
  resync on loss. They can be added later without changing the model, because a full window is always a valid update.
- **Windows that live only as long as the run that opened them** (like a dialogue). They are simpler to clean up, but a shop
  would have to keep a script suspended while the player browses, holding a runtime slot and a CPU budget for minutes.
- **Binary layouts per node.** They are smaller, but every new field becomes a protocol change, and bots would need their
  own decoder.
