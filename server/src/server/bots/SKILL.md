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
- A new bot is exactly a newly created player character: a level 1 Novice with the stats, status points, zeny, items and start point
  the server gives to players, and no skills but the ones a Novice gets. Only its look (sex, hair) is chosen at creation, and can not be changed afterwards.
- **Coordinates** are cells: `x` grows to the east, `y` grows to the **north**. A cell is walkable or not; see `/map`.
- Everything on a map has a numeric **`id`**: NPCs, monsters, items on the ground, warps and other players. You pass that id as
  the `target` of actions. Ids change when a monster dies and respawns, so read them from a fresh observation.
- Walking takes about 150 ms per cell. Actions that walk wait for the end of the walk (see `journey`).

## Leaving the game

A bot stays in the game until it is told to leave, with `POST /bots/{name}/disconnect` (or `delete`), and also when:

- the **WebSocket** that put it in the game with `create` or `connect` closes, whatever the reason;
- **20 minutes** pass without any request for it (`bots.idle_logout_secs`, 0 turns this off). Observations, maps, actions and
  connect all count, the observations a `subscribe` pushes by itself do not.

A bot that left keeps its level, items and position: `connect` brings it back. `delete` is the only way to get rid of one, and it can
not be undone. Deleting bypasses the checks players have for deleting a character (level, delay, party and guild).

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
| `POST /bots/{name}/disconnect` | Logs the character out: saves it and takes it out of the game, the answer comes once it is gone |
| `DELETE /bots/{name}` | Erases the character and its account for good, after logging it out. Answers `{name, char_id, deleted}` |
| `GET /bots/{name}/observation` | Everything the bot sees, see below |
| `GET /bots/{name}/map` | Walkable cells of the map. `?x=&y=&radius=` crops the square around a cell, to save space |
| `POST /bots/{name}/actions` | Runs one action, the JSON body is the action |
| `GET /bots/server/settings` | How the server is set up, see "Server settings". Needs no bot |
| `GET /bots/server/battle` | The rathena battle options with value, default and range. `?name=` keeps the options whose name contains the text |

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


### Observation

```json
{
  "ready": true,
  "self": {"name": "Scout", "char_id": 150001, "job": "Novice", "base_level": 1, "job_level": 1, "hp": 40, "max_hp": 40,
           "sp": 11, "max_sp": 11, "zeny": 0, "weight": 0, "status_points": 48, "skill_points": 0,
           "stats": {"str": 1, "agi": 1, "vit": 1, "int": 1, "dex": 1, "luk": 1}, "x": 156, "y": 191,
           "action": "idle", "moving": false, "attack_target": null, "dead": false},
  "map": {"name": "prontera", "instance": 0, "width": 312, "height": 392},
  "players": [{"id": 150002, "name": "Other", "x": 150, "y": 190, "attackable": false}],
  "mobs":    [{"id": 400012, "name": "Poring", "mob_id": 1002, "x": 160, "y": 200, "hp": 50, "max_hp": 50}],
  "npcs":    [{"id": 100123, "name": "Kafra Employee", "x": 151, "y": 29}],
  "items":   [{"id": 400301, "item_id": 909, "name": "Jellopy", "amount": 1, "x": 158, "y": 195}],
  "warps":   [{"id": 100500, "x": 156, "y": 20, "half_width": 2, "half_height": 1, "to_map": "prt_fild08", "to_x": 170, "to_y": 380}],
  "inventory": [{"index": 0, "item_id": 1201, "name": "Knife", "kind": "Weapon", "amount": 1, "equipped": true}],
  "storage": null,
  "skills": [{"id": 28, "name": "AL_HEAL", "level": 3, "aim": "target", "range": 9}],
  "party": {"id": 1, "name": "Squad", "leader": 150001, "members": [{"char_id": 150001, "name": "Scout"}]},
  "party_invitation": null,
  "trade": null,
  "skill_menu": null,
  "crafting": null,
  "duel": null,
  "duel_invitation": false,
  "companions": {"pet": null, "pet_eggs": null, "pet_capture": null, "homunculus": null, "mercenary": null},
  "cart": null,
  "vending": null,
  "buying": null,
  "buying_slots": 0,
  "vendors": [],
  "buyers": [],
  "dialog": null,
  "messages": [{"seq": 1, "channel": "system", "text": "..."}]
}
```

`action` is one of `idle`, `moving`, `attacking`, `using_skill`, `sitting`, `dead`. `ready` is false for a moment after connecting.
`dialog` is what the NPC currently shows (see Dialogues). `messages` are the last ones the bot received, `channel` being `system`,
`chat` (said by someone around the bot, as `Name : text`), `whisper` or `party`. What the bot says itself comes back as `system`.
`inventory[].index` is what the item actions take. `skills` lists what the bot knows, see Skills. `storage` is `null` unless a Kafra opened
the storage, see Storage. `party`, `party_invitation` and `trade` are described in their sections, `skill_menu` and `crafting` in
Skills, `duel`, `duel_invitation` and `attackable` in Fighting players, `companions` in Pets, homunculus and mercenary, and `cart`, `vending`, `buying`, `buying_slots`, `vendors` and `buyers` in Carts and stores.

## Actions

Send them as the body of `POST /bots/{name}/actions`, or over the WebSocket. `type` selects the action.

| Action | Parameters | Result |
|---|---|---|
| `move` | `x`, `y`, `wait` (default true) | Walks to the cell. `path_length`, and with `wait` a `journey` |
| `use` | `target` | Walks to the target and uses it, see below |
| `attack` | `target` (a monster or an `attackable` player), `wait` (default false) | Walks to the target and attacks it until it is dead. Without `wait` it returns at once and the fight goes on in the background; any other command of the bot ends it. With `wait`, `fight` tells how it ended: `over` (the monster is gone, killed or not), `dead` (the bot died), `stopped` (another command took over), `unreachable` or `timeout` |
| `stop` | | Stops walking, attacking and any fight in the background |
| `respawn` | | Returns to the save point after dying (`self.dead` is true) |
| `dialog_next` | | Continues a dialogue that shows a Next button |
| `dialog_choose` | `option` (1 is the first) | Picks an entry of a menu |
| `dialog_number` | `value` | Answers a number prompt |
| `dialog_text` | `text` | Answers a text prompt |
| `dialog_close` | | Closes the conversation, whatever it waits for |
| `dialog_shop` `dialog_buy` `dialog_sell` | `deal` / `items` | Shops, see Dialogues |
| `storage_deposit` `storage_withdraw` `storage_close` | `index`, `amount` (default 1) | Storage, see below |
| `use_item` `equip` `unequip` `drop_item` | `index` (`amount` to drop, default 1) | Items, see below |
| `skill` | `skill`, `level`, `target`, `x`, `y` | Uses a skill (also those of a homunculus or mercenary), see below |
| `raise_stat` `learn_skill` | `stat`, `amount` / `skill` | Spends points, see Skills |
| `say` `whisper` `party_chat` | `text` (`to` for a whisper) | Chat, see below |
| `party_create` `party_invite` `party_accept` `party_decline` `party_leave` | `name` | Parties, see below |
| `trade_request` `trade_accept` `trade_decline` `trade_offer` `trade_zeny` `trade_lock` `trade_confirm` `trade_cancel` | | Trades, see below |
| `skill_choose` `skill_cancel` `craft` | `choice` / `item_id`, `additives` | The question or window a skill opened, see Skills |
| `duel_create` `duel_invite` `duel_accept` `duel_decline` `duel_leave` | `limit` / `name` | Duels, see Fighting players |
| `cart_put` `cart_get` `cart_to_storage` `cart_from_storage` | `index`, `amount` (default 1) | Cart, see Carts and stores |
| `vending_open` `vending_close` `vending_buy` | `title`, `items` / `vendor`, `items` | Vending stores, see Carts and stores |
| `buying_open` `buying_close` `buying_sell` | `title`, `zeny_limit`, `items` / `buyer`, `items` | Buying stores, see Carts and stores |
| `pet_hatch` `pet_feed` `pet_perform` `pet_return_to_egg` `pet_unequip` `pet_rename` `pet_emote` `pet_equip` `pet_capture` | `index` / `name` / `emotion` / `target` | Pets, see Pets, homunculus and mercenary |
| `homunculus_feed` `homunculus_rename` `homunculus_delete` `homunculus_move` `homunculus_follow` `homunculus_attack` | `name` / `x`, `y` / `target`, `repeat` | Homunculus, see Pets, homunculus and mercenary |
| `mercenary_move` `mercenary_follow` `mercenary_attack` `mercenary_dismiss` | `x`, `y` / `target`, `repeat` | Mercenary, see Pets, homunculus and mercenary |
| `command` | `text` (`@rates`) | An `@` command, see Commands |

A walk can be as long as the map: routes the game cannot walk in one go (about 100 cells) are walked in legs, one after the other.
Without `wait`, the first leg starts and the rest follows in the background until another command of the bot takes over.

`journey` tells how a walk ended: `arrived`, `map_changed` (a warp was taken), `stopped` (the path was blocked or cancelled) or
`timeout` (still walking after `bots.action_timeout_secs`, 30 by default, observe to see where the bot is).

### `use`

`use` is the one call to reach something and act on it. What happens depends on the target:

- **NPC**: walks next to it and starts the conversation. The result has `action: "talk"` and the `dialog` it opened.
- **Item on the ground**: walks to it and picks it up (`action: "pick_up"`). Observe the inventory to confirm. Items of a monster that
  another player killed stay locked to that player for a while.
- **Warp**: walks onto it, the result is `journey: "map_changed"` and the bot stands on the other map. There is nothing else to call.
- Monsters and players are fought with `attack`; `use` on a player is refused.

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
| `shop` | `dialog_shop` with `deal`: `"buy"` or `"sell"` |
| `buy` (with `items`) | `dialog_buy` with `items`: `[{"item_id": 1750, "amount": 3}]`, ids from the list |
| `sell` (with `items`) | `dialog_sell` with `items`: `[{"index": 1, "amount": 3}]`, `index` being the one of the observation's `inventory` |

Every answer returns the next `dialog`, or `null` when the conversation ended. A wrong kind of answer is refused with 400 and
leaves the conversation alone. An NPC that never ends its conversation can always be left with `dialog_close`. While a conversation is
open the bot is not blocked, but a new `use` on an NPC replaces it.

### Shops

An NPC that sells shows `prompt.kind: "shop"`. Answer `dialog_shop`, then the list comes as the prompt:

```json
{"type": "dialog_shop", "deal": "buy"}
→ {"dialog": {"npc_id": 306372, "lines": [], "prompt": {"kind": "buy", "items": [{"item_id": 1750, "name": "Arrow", "price": 10}]}}}
{"type": "dialog_buy", "items": [{"item_id": 1750, "amount": 3}]}
→ {"dialog": null}
```

`price` is what the bot pays, with the Discount skills applied (when selling, what it gets, with Overcharge). The conversation ends after the
purchase or sale and the answer is `dialog: null`: see the next observation for `zeny` and `inventory`. A purchase the server refuses
(not enough zeny, too heavy, no room) shows up as a `system` message, for example `The purchase failed: not enough zeny`. Equipped items are
not on the sell list. A line that names an item the shop does not list, or an amount below 1, is refused with 400 and the shop keeps waiting.

### Storage

A Kafra opens the storage in a dialogue (`Use Storage`; it needs Basic Skill level 6 and a fee). Once open, `storage` in the observation lists its items,
and these work until `storage_close` (or until the bot leaves):

| Action | What it does |
|---|---|
| `storage_deposit` (`index` of the inventory, `amount`) | Puts an item in the storage. Equipped items are refused |
| `storage_withdraw` (`index` of `storage`, `amount`) | Takes an item back |
| `storage_close` | Closes the storage |

They answer when the item moved: `{"item": "Cotton Shirt", "amount": 1}`. A refusal (too heavy, storage full, no storage open) is a 400 with the reason.

### Items

`use_item`, `equip`, `unequip` and `drop_item` take the `index` of an item in the observation's `inventory`. They answer `{"sent": true}`
once the request is queued, the result shows in the next observation (the item is gone, `equipped` changed). `equip` and `unequip`
refuse an item that is already in that state.

### Skills

```json
{"type": "skill", "skill": "AL_HEAL", "level": 3, "target": 150002}
```

`skill` is the name or the id from the `skills` list of the observation (for a homunculus or mercenary skill, from `companions`), `level` defaults to the highest the bot knows. What to give
depends on the skill's `aim`: `target` takes the `target` id of a monster or a player (itself included), `ground` takes `x` and `y` (or a `target`, whose
cell is used), `self` takes nothing, `passive` skills can not be used. The bot does not walk: a target beyond `range` is refused with
the distance, move closer first. The call answers `{"skill", "level", "aimed", "started": true}` when the server accepted the cast, and is refused with the
server's reason otherwise (not enough SP, a cell the skill can not use, a cooldown...). Casting takes time; `self.action` shows `using_skill` meanwhile.

**Skills that ask a question** (Teleport, Warp Portal, Arrow Crafting, Auto Spell, Weapon Refine, Repair Weapon, Elemental Converter, Feeling of the Sun, Moon and Stars)
start the cast like any skill, and once it is done the observation has `skill_menu`:

```json
"skill_menu": {"skill": "AL_TELEPORT", "kind": "teleport",
               "choices": [{"choice": "Random", "name": "Random"}, {"choice": "prontera", "name": "prontera"}]}
```

`skill_choose` with one of the `choice` values answers it, `skill_cancel` gives it up. The choice is a map name for `teleport` and `warp_portal`
(the save point and the memo points; Teleport level 1 only offers `Random`), and a number for the others: an item id for `making_arrow` and
`elemental_converter`, an inventory index for `weapon_refine` and `repair_weapon`, a skill id for `auto_spell`. An answer that is not on the list is refused
with the list and the menu stays open. A menu that nobody answers closes after two minutes (Warp Portal blocks the caster until then).
Warp Portal then places the portal in front of the bot: others walk in it, it lasts a few seconds.

**Crafting windows** (Pharmacy, Cooking, the forging skills) fill `crafting` with the recipes the bot can make: `{"cooking": false, "recipes":
[{"item_id": 501, "name": "Red Potion", "materials": [{"item_id": 507, "name": "Red Herb", "amount": 1}, ...]}]}`. `craft` (`item_id`,
`additives`: up to 3 item ids of Star Crumbs or element stones, for weapons) makes one item and answers `{"item": "Red Potion", "success": true}`;
success is a chance of the skill, a failure still costs the materials. The window stays open for more.

Skills come from levelling up, as for players: `learn_skill` (`skill`) spends one skill point on the next level of a skill and answers
`{"skill": "NV_BASIC", "level": 2, "skill_points": 5}`. It is refused with the reason when it can not be done, for example a Novice must
raise **Basic Skill** to level 9 before the other Novice skills show up (the answer lists the skills available now). Note that
a Novice needs Basic Skill level 1 to trade, level 6 to use a storage and level 7 to create a party, which takes job level 8
(one skill point per job level after the first).

### Stat points

`raise_stat` (`stat`: `str`, `agi`, `vit`, `int`, `dex` or `luk`; `amount`, default 1) spends status points and answers
`{"stat": "str", "value": 4, "status_points": 42}`. It is refused when the points do not cover the amount (raising a stat costs more
as it grows) or the stat would pass 99. `self.status_points`, `self.skill_points` and `self.stats` in the observation show what is left.

### Chat

`say` speaks to everyone around, `whisper` (`to` is a character name) and `party_chat` (needs a party) send to one place. Messages that
start with `@`, `#` or `%` are refused: `@` commands have their own action (see Commands), bots do not use `#` commands or channels.
What others say arrives in `messages`.

### Parties

`party_create` (`name`), `party_invite` (`name` of a character who is in the game), `party_accept` and `party_decline` (answers the
`party_invitation` of the observation), `party_leave`. `party` in the observation lists the members, `name` is empty for those offline.

### Trades

Two characters standing close to each other can trade. The steps, in order:

1. `trade_request` with the `target` id of a player standing at most 2 cells away (refused otherwise, with the distance), the bot needing Basic Skill level 1. The other side sees `trade` with `phase: "requested"` and `incoming_request: true`.
2. `trade_accept` (or `trade_decline`). The `phase` becomes `accepted`.
3. `trade_offer` (`index` of an inventory item, `amount`) and `trade_zeny` (`amount`) put things on the table. What the partner offers is in
   `partner_items` and `partner_zeny`. Equipped items must be taken off first and the amounts must be held, both are refused otherwise.
4. `trade_lock` when satisfied (`locked`), then `trade_confirm` once both are locked. When both confirmed, the items change hands and
   `trade` becomes `null`. `trade_cancel` ends it at any time.

### Fighting players

`attack` works on a player like on a monster, when the server lets them fight: in a duel, on a PvP, GvG or battleground map, or at a castle during a siege.
`players[].attackable` says whether it works now; elsewhere `attack` is refused with that reason.
Outside those maps two characters become enemies with a duel:

| Action | What it does |
|---|---|
| `duel_create` (`limit`: places, 0 for no limit) | Opens a duel with the bot in it |
| `duel_invite` (`name`) | Invites a player of the same map. The other side sees `duel_invitation: true` |
| `duel_accept` / `duel_decline` | Answers the invitation |
| `duel_leave` | Leaves; the duel ends when one member is left |

`duel` in the observation is the id of the duel the bot is in. Each answer is `{"duel": id}`, or an error with the reason (already in a duel, nobody to answer...).

### Carts and stores

A **cart** comes from a Kafra rental (a Merchant class with Pushcart). `cart` in the observation is `null` without one, otherwise the list
`[{"index": 0, "item_id": 909, "name": "Jellopy", "amount": 30, "refine": 0}]`. Moving items needs no walking:

| Action | What it does |
|---|---|
| `cart_put` (`index` of the inventory, `amount`) | Inventory to cart |
| `cart_get` (`index` of the cart) | Cart to inventory |
| `cart_to_storage` / `cart_from_storage` | Between the cart and the open storage (`index` of the cart / of `storage`) |

A **vending store** sells cart items to players who pass by. The bot needs a cart and the Vending skill; its level gives the number of different items (level + 2),
and the bot stops where it stands while the store is open.

```json
{"type": "vending_open", "title": "Cheap jellopy", "items": [{"cart_index": 0, "amount": 20, "price": 8}]}
→ {"title": "Cheap jellopy", "items": 1}
```

`vending` in the observation shows the store with what is left to sell. A customer finds stores in `vendors`:
`[{"id", "name", "title", "x", "y", "distance", "items": [{"index", "item_id", "name", "amount", "price", "refine"}]}]`.
`vending_buy` (`vendor`: the id, `items`: `[{"index": 0, "amount": 2}]`, `index` being the one of the vendor's `items`) buys from a vendor at most
5 cells away and answers `{"spent": 16, "zeny": 984}`; the refusals say why (too far, not enough zeny, too heavy, an index it does not sell).
`vending_close` closes the bot's own store.

A **buying store** asks for items and pays for them. It needs a license (a usable item such as the *Shabby Purchase Street Stall License*), which sets `buying_slots` (the number of different items) when used with `use_item`;
then `buying_open` (`title`, `zeny_limit`: the most the bot will spend, up to its zeny, `items`: `[{"item_id": 909, "amount": 100, "price": 5}]`)
opens it. Only stackable items that buying stores accept, and that the bot already holds at least one of, can be asked for. `buyers` in the observation lists the stores around,
with `zeny_limit` and `items: [{"item_id", "name", "amount", "price"}]`. `buying_sell` (`buyer`, `items`: `[{"index": 4, "amount": 10}]` with `index` from the
seller's `inventory`) sells to a buyer at most 5 cells away and answers `{"earned": 50, "zeny": 1050}`. `buying_close` closes the bot's own store.
Searching stores of the whole map (the store search window of the Merchant skills) is not supported, walk to the vendors and buyers of the observation.

### Pets, homunculus and mercenary

`companions` in the observation shows them (`null` for those the bot has not):

```json
"companions": {
  "pet": {"name": "Poring", "class_id": 1002, "level": 1, "intimacy": 250, "intimacy_stage": "neutral", "hunger": 100, "hunger_stage": "stuffed",
          "renamed": false, "food": {"item_id": 531, "name": "Apple Juice"}, "accessory": null,
          "accessory_wanted": {"item_id": 10013, "name": "Backpack"}, "x": 142, "y": 179},
  "pet_eggs": [{"index": 4, "item_id": 9001, "name": "Poring Egg"}],
  "pet_capture": {"item": {"item_id": 619, "name": "Unripe Apple"}, "expires_in_ms": 59000},
  "homunculus": {"name": "Lif", "class_id": 6001, "level": 10, "active": true, "evolved": false, "renamed": false,
                 "hp": 100, "max_hp": 150, "sp": 20, "max_sp": 40, "intimacy": 2100, "hunger": 42, "experience": 0,
                 "skill_points": 1, "stats": {"str": 17, "agi": 20, "vit": 15, "int": 35, "dex": 24, "luk": 12},
                 "food": {"item_id": 537, "name": "Pet Food"}, "x": 142, "y": 179,
                 "skills": [{"id": 8001, "name": "HLIF_HEAL", "level": 0, "max_level": 5, "aim": "self", "range": null, "learnable": true}]},
  "mercenary": {"name": "Mimic", "class_id": 1191, "level": 51, "guild": 0, "hp": 6120, "max_hp": 6120, "sp": 187, "max_sp": 187,
                "kill_count": 0, "contract_ms_left": 1799000, "x": 142, "y": 179,
                "skills": [{"id": 8201, "name": "MS_BASH", "level": 5, "max_level": 5, "aim": "target", "range": 1, "learnable": false}]}
}
```

**Pets.** A wild monster is tamed with its lure: `use_item` on the taming item (`Unripe Apple` for a Poring) fills `pet_capture` for a
minute, then `pet_capture` (`target`: a monster of the observation within 5 cells) throws it. The chance is the capture rate of the pet,
higher when the monster is hurt, and the call waits for the outcome: `{"capture": "captured" | "failed", "target"}`. The egg is in the
inventory when it worked, and the lure is used up either way. To hatch an egg, `use_item` on a **Pet Incubator** opens a window whose eggs
are listed in `pet_eggs` with their inventory index; `pet_hatch` (`index`) hatches one (an index that is not in the window is refused and the
window stays open) and answers with the `pet`. After that: `pet_feed` uses the pet's `food` (feeding above 75 hunger earns less intimacy and
above 90 loses some: feed when `hunger_stage` is `neutral` or lower), `pet_perform` makes it perform, `pet_rename` (`name`, once),
`pet_emote` (`emotion`, 0 to 200000), `pet_equip` (`index` of its `accessory_wanted` in the inventory), `pet_unequip` and
`pet_return_to_egg`. They answer `{"pet": {...}}`, the pet as it is now (`null` after returning to the egg). A pet under 100 intimacy
(`shy`, `awkward`) does not help its owner, and one that starves loses intimacy.

**Homunculus.** Call it with the skill Call Homunculus (`AM_CALLHOMUN`, needs an Embryo), rest it with `AM_REST` and bring it back from
death with `AM_RESURRECTHOMUN`: those are skills of the bot, used with `skill`. A resting or dead homunculus (`active: false`) takes no
order. `homunculus_feed` uses its `food` and answers the `homunculus`; `homunculus_rename` (`name`, once); `homunculus_delete` erases it for
good; `homunculus_move` (`x`, `y`: a walkable cell at most 15 cells from the owner), `homunculus_follow` (cancels the order and follows the
owner again) and `homunculus_attack` (`target`: a monster within 15 cells; `repeat`, default true, goes on attacking) answer
`{"homunculus": "moving" | "following" | "attacking"}`. It levels up by killing monsters and every level gives `skill_points`:
`learn_skill` with its name or id (`HLIF_HEAL`, 8001...) spends one on a skill that shows `learnable: true` in `skills`. Its skills
(`HLIF_*`, ids 8001 to 8016) are cast with `skill`, which takes `target` or `x`, `y` as the `aim` says and answers
`{"skill", "level", "caster": "homunculus", "casting", "started"}`; a refusal carries the reason of the server (its SP, a range, a cooldown,
a missing item).

**Mercenary.** It is hired from a Mercenary Guild NPC (a dialogue, with `use`) for 30 minutes. `mercenary_move`, `mercenary_follow` and
`mercenary_attack` work as for the homunculus, `mercenary_dismiss` ends the contract. Its skills (ids 8201 to 8240, `MS_BASH`...) are the
ones of its `skills` list, fixed by the contract, there is nothing to learn; they are cast with `skill` like the homunculus ones, with
`"caster": "mercenary"`.

### Commands

`command` (`text`, such as `"@rates"`) runs an `@` command like a player typing it, in the game loop, and answers
`{"command": "rates", "replies": [...]}` with the lines the command said. The bots run commands as the permission group
`bots.command_group` of `config.json` (default 1, "Super Player" of `config/groups.json`): a command that the group does not list is
refused with the list of those it does. By default that is information and conveniences (`@rates`, `@exp`, `@mobinfo`, `@iteminfo`,
`@whodrops`, `@servertime`, `@uptime`, `@noask`, `@autoloot`, `@autolootitem`, `@go`, `@whereis`, `@hominfo`, `@homstats`,
`@commands`, `@resurrect`...), nothing that creates items, zeny, monsters or levels. The text must start with `@` and a command name. A
command that acts through another event (a duel invitation, the mail window) says nothing at once: look at `messages` and the
observation. A command given a wrong argument answers with the line a player would read, in `replies`.

### Server settings

`GET /bots/server/settings` answers the values a plan depends on, read-only and without a bot:

- `experience.base` and `experience.job`: `{config, battle_percent, effective}`. The server multiplies the rate of `config.json` by the
  `base_exp_rate` / `job_exp_rate` battle option divided by 100, `effective` is that product. Also `pvp_exp` (experience on PvP maps),
  `party_even_share_bonus`, `guild_exp_rate_percent` and the `death_penalty` (`kind` 0 off, 1 share of the next level, 2 share of the
  current experience; `base`, `job` and `zeny` in hundredths of a percent).
- `drops`: `rate` and `card_rate` multiply the drop chance of the database. `item_rate_percent` then scales it per kind of item
  (`heal`, `use`, `equip`, `card`, `common`) and monster (`normal`, `boss`, `mvp`), 100 meaning unchanged.
- `limits` (`max_base_level`, `max_stat_level`, `max_inventory`, `default_char_speed`, item use intervals), `vending` (`max_value`,
  `tax` in hundredths of a percent of a sale from `tax_min` zeny), `guild`, `pet_support`, `damage_rates_percent` of GvG and battleground,
  `packetver`, `tick_ms`.

`GET /bots/server/battle?name=exp` lists the battle options containing `exp`: `{options: [{name, value, default, min, max}]}`. All
options of rathena are listed, but the server only reads some of them, a value that differs from its default is not proof that it has
an effect.

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
  `radius`), `subscribe`, `unsubscribe`, `delete`. Every command except `list` and `create` names its bot in `bot`.
- Commands run concurrently: send `stop` while a long `move` is waiting, the `move` is answered with `journey: "stopped"`.
- After `subscribe`, the server pushes events without being asked: `observation` every `interval_ms` (200 to 10000, default 1000), and
  at once `dialog` (`dialog`, `null` when it closed) and `message` (`message: {seq, channel, text}`).

## Limits of this version

Bots can move, use NPCs, pick items up, attack monsters and players, use items and skills (also the ones that ask a question afterwards),
equip, chat, trade, join parties, duel, keep a cart, run vending and buying stores, spend their points, look after a pet, a homunculus
and a mercenary, and run the `@` commands of their permission group. Not supported: the store search window and guilds (guild
management and guild skills).
A job change is done with the Job Master of Prontera (a Novice with Basic Skill 9 and job level 10 became a Mage this way) or the job quest NPCs.
Monsters fight back and a bot can die: check `self.hp` and `self.dead`, and call `respawn` to come back at the save point with a
few hit points. A novice has 40 HP: stay away from monsters that attack on sight (`Pupa`, `Wild Rose`...) until the bot is stronger.
