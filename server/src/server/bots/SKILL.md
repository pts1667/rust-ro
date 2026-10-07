---
name: ro-bot
description: Play a Ragnarok Online character ("bot") on the rust-ro server through its HTTP and WebSocket API. Observe the whole map, walk, talk to NPCs, pick items up and fight monsters. Use when asked to control, test or play the game as a bot.
---

# Controlling a bot on the rust-ro server

A **bot** is a character that your program drives. It lives in the real game world next to players: it can be seen, attacked, and
talk to NPCs. You see the **whole map** it stands on, nothing is hidden by a field of view.

## Connecting

| | |
|---|---|
| Base URL | `http://127.0.0.1:6902` (`bots.host` and `bots.port` of the server configuration) |
| Key | the content of `config/bots_api_key.txt` on the server, created on first start if it is missing |
| Header | `Authorization: Bearer <key>` or `X-API-Key: <key>`, on every request including the WebSocket upgrade |

`GET /bots/SKILL.md` (this file) is the only request that needs no key.

```sh
KEY=$(cat config/bots_api_key.txt)
curl -s -X POST localhost:6902/bots -H "Authorization: Bearer $KEY" -H 'Content-Type: application/json' -d '{"name":"Scout"}'
```

## Concepts

- A bot is named by its character: at most 19 characters, at least `char_server.char_name_min_length` (4 by default), with the characters the server allows for character names. Names are case-insensitive.
  `POST /bots` creates a new character on its own account and refuses a name that is taken (409). To play a bot that exists,
  `connect` it: it keeps its level, items and position across restarts of the server and of your program.
- New bots start in `prontera` at `156,191` as Novices. Their look is chosen at creation and can not be changed afterwards.
- **Coordinates** are cells: `x` grows to the east, `y` grows to the **north**. A cell is walkable or not; see `/map`.
- Everything on a map has a numeric **`id`**: NPCs, monsters, items on the ground, warps and other players. You pass that id as
  the `target` of actions. Ids change when a monster dies and respawns, so read them from a fresh observation.
- Walking takes about 150 ms per cell. Actions that walk wait for the end of the walk (see `journey`).

## Loop to follow

1. `GET /bots/{name}/observation` and decide.
2. Send one action. Most return when the action is done, with a result that says how it went.
3. Observe again. Never reuse ids or positions from an old observation after a map change (`journey: "map_changed"`).

## REST endpoints

| Request | What it does |
|---|---|
| `GET /bots` | The bots created or connected through the API since the server started: `[{name, char_id, connected}]` |
| `POST /bots` | Creates a new character, see below |
| `GET /bots/{name}` | Summary, plus the status of the character when connected |
| `POST /bots/{name}/connect` | Puts a bot in the game, also one created before the server started |
| `POST /bots/{name}/disconnect` | Saves the character and takes it out of the game |
| `GET /bots/{name}/observation` | Everything the bot sees, see below |
| `GET /bots/{name}/map` | Walkable cells of the map. `?x=&y=&radius=` crops the square around a cell, to save space |
| `POST /bots/{name}/actions` | Runs one action, the JSON body is the action |

Errors are `{"error": "..."}` with status 400 (the request cannot be done, the message says why), 404 (unknown bot), 409 (the bot
is not connected, or a limit was reached) or 503 (the game did not answer in time).

### Creating a bot

```json
POST /bots
{"name": "Scout", "sex": "M", "hair_style": 1, "hair_color": 0, "connect": true}
```

Only `name` is required. `sex` is `"M"` or `"F"` (default `"M"`), `hair_style` goes from 0 to 23 (default 1), `hair_color` from 0 to 8
(default 0), `connect` (default true) puts the new character in the game at once. The answer is `{name, char_id, connected}`.
Errors: 400 for a name or look the server refuses, 409 when a bot of this name exists or `bots.max_bots` bots are registered.
Everything else about the character (job, stats) is the same for all bots: a level 1 Novice with the starting items.

### Observation

```json
{
  "ready": true,
  "self": {"name": "Scout", "char_id": 150001, "job": "Novice", "base_level": 1, "job_level": 1, "hp": 40, "max_hp": 40,
           "sp": 11, "max_sp": 11, "zeny": 0, "weight": 0, "status_points": 48, "x": 156, "y": 191,
           "action": "idle", "moving": false, "attack_target": null, "dead": false},
  "map": {"name": "prontera", "instance": 0, "width": 312, "height": 392},
  "players": [{"id": 150002, "name": "Other", "x": 150, "y": 190}],
  "mobs":    [{"id": 400012, "name": "Poring", "mob_id": 1002, "x": 160, "y": 200, "hp": 50, "max_hp": 50}],
  "npcs":    [{"id": 100123, "name": "Kafra Employee", "x": 151, "y": 29}],
  "items":   [{"id": 400301, "item_id": 909, "name": "Jellopy", "amount": 1, "x": 158, "y": 195}],
  "warps":   [{"id": 100500, "x": 156, "y": 20, "half_width": 2, "half_height": 1, "to_map": "prt_fild08", "to_x": 170, "to_y": 380}],
  "inventory": [{"index": 2, "item_id": 1201, "name": "Knife", "amount": 1, "equipped": true}],
  "dialog": null,
  "messages": [{"seq": 1, "text": "..."}]
}
```

`action` is one of `idle`, `moving`, `attacking`, `using_skill`, `sitting`, `dead`. `ready` is false for a moment after connecting.
`dialog` is what the NPC currently shows (see Dialogues). `messages` are the last system messages sent to the bot.

## Actions

Send them as the body of `POST /bots/{name}/actions`, or over the WebSocket. `type` selects the action.

| Action | Parameters | Result |
|---|---|---|
| `move` | `x`, `y`, `wait` (default true) | Walks to the cell. `path_length`, and with `wait` a `journey` |
| `use` | `target` | Walks to the target and uses it, see below |
| `attack` | `target` (a monster), `wait` (default false) | Walks to the monster and attacks it until it is dead. Without `wait` it returns at once and the fight goes on in the background; any other command of the bot ends it. With `wait`, `fight` tells how it ended: `over` (the monster is gone, killed or not), `dead` (the bot died), `stopped` (another command took over), `unreachable` or `timeout` |
| `stop` | | Stops walking, attacking and any fight in the background |
| `respawn` | | Returns to the save point after dying (`self.dead` is true) |
| `dialog_next` | | Continues a dialogue that shows a Next button |
| `dialog_choose` | `option` (1 is the first) | Picks an entry of a menu |
| `dialog_number` | `value` | Answers a number prompt |
| `dialog_text` | `text` | Answers a text prompt |
| `dialog_close` | | Closes the conversation, whatever it waits for |

`journey` tells how a walk ended: `arrived`, `map_changed` (a warp was taken), `stopped` (the path was blocked or cancelled) or
`timeout` (still walking after `bots.action_timeout_secs`, 30 by default, observe to see where the bot is).

### `use`

`use` is the one call to reach something and act on it. What happens depends on the target:

- **NPC**: walks next to it and starts the conversation. The result has `action: "talk"` and the `dialog` it opened.
- **Item on the ground**: walks to it and picks it up (`action: "pick_up"`). Observe the inventory to confirm. Items of a monster that
  another player killed stay locked to that player for a while.
- **Warp**: walks onto it, the result is `journey: "map_changed"` and the bot stands on the other map. There is nothing else to call.
- Monsters are fought with `attack`; players cannot be targeted.

```json
{"type": "use", "target": 100123}
→ {"target": 100123, "kind": "npc", "journey": "arrived", "action": "talk",
   "dialog": {"npc_id": 100123, "lines": ["[Kafra Employee]", "Welcome!"], "prompt": {"kind": "menu", "options": ["Save", "Cancel"]}}}
```

### Dialogues

While a conversation is open the NPC waits for an answer. `dialog.lines` is the text of the page, `dialog.prompt.kind` tells the answer:

| `prompt.kind` | Answer with |
|---|---|
| `next` | `dialog_next` |
| `menu` (with `options`) | `dialog_choose` with a number from 1 to the number of options |
| `number` | `dialog_number` |
| `text` | `dialog_text` |
| `running` | nothing yet, the script is still working: observe again |
| `shop` | shops cannot be driven through the API, use `dialog_close` |

Every answer returns the next `dialog`, or `null` when the conversation ended. A wrong kind of answer is refused with 400 and
leaves the conversation alone. An NPC that never ends its conversation can always be left with `dialog_close`. While a conversation is
open the bot is not blocked, but a new `use` on an NPC replaces it.

## WebSocket: `/bots/ws`

Same key in the same header. Messages are JSON text. One connection controls any number of bots.

```json
→ {"id": 1, "type": "create", "name": "Scout"}
← {"id": 1, "ok": true, "result": {"name": "Scout", "char_id": 150001, "connected": true}}
→ {"id": 2, "type": "move", "bot": "Scout", "x": 160, "y": 200}
← {"id": 2, "ok": true, "result": {"path_length": 9, "journey": "arrived"}}
→ {"id": 3, "type": "subscribe", "bot": "Scout", "interval_ms": 500}
← {"event": "observation", "bot": "Scout", "observation": {...}}
```

- A command carries an `id` of your choice, which the answer repeats: `{"id", "ok": true, "result"}` or `{"id", "ok": false, "error"}`.
- `type` is any action above, or `list`, `create` (the fields of `POST /bots`), `connect`, `disconnect`, `status`, `observe`, `map` (`x`, `y`,
  `radius`), `subscribe`, `unsubscribe`. Every command except `list` and `create` names its bot in `bot`.
- Commands run concurrently: send `stop` while a long `move` is waiting, the `move` is answered with `journey: "stopped"`.
- After `subscribe`, the server pushes events without being asked: `observation` every `interval_ms` (200 to 10000, default 1000), and
  at once `dialog` (`dialog`, `null` when it closed) and `message` (`message: {seq, text}`).

## Limits of this version

Bots can move, use NPCs, pick items up and attack. They cannot yet use skills or items, equip, trade, chat, or join parties.
Monsters fight back and a bot can die: check `self.hp` and `self.dead`, and call `respawn` to come back at the save point with a
few hit points. A novice has 40 HP: stay away from monsters that attack on sight (`Pupa`, `Wild Rose`...) until the bot is stronger.
