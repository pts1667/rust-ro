# Reference API (`/db` and `/wiki`)

A read-only HTTP server on its own port (default `127.0.0.1:6904`, the `reference` block of `config.json`) that publishes what the
server knows about the game as JSON (`/db`) and the iRO Wiki Classic copy as markdown (`/wiki`). It needs no key and no character,
and it never touches the game state: everything comes from the configuration and script catalogs loaded at boot, so it can run
while the game runs. Code: `server/src/server/reference`.

```json
"reference": { "enabled": true, "host": "127.0.0.1", "port": 6904, "wiki_root": "wiki" }
```

## `/db`

| URL | Answers |
| --- | --- |
| `/db` | The categories, with their counts and the wiki index. |
| `/db/<category>` | A page of the category: `{ total, offset, limit, results: [link...] }`. |
| `/db/<category>/<key>` | One entry with everything linked to it. |
| `/db/route?from=<map>&to=<map>&x=<x>&y=<y>` | The fewest hops from one map to another (see Routes). |

Query parameters on listings: `q` (part of the name, case-insensitive; maps match their name or `display_name`), `map` (NPCs only),
`category` (destinations only, the warper menu category such as `Towns`, case-insensitive), `offset`, `limit` (default 50, at most 200).

Categories and keys:

| Category | Key | Entry holds |
| --- | --- | --- |
| `items` | id or name | The item, its client `description`, its script metadata, `dropped_by`, `sold_by`, `recipes` (`made_by`, `used_in`), `arrow_crafting`, `groups` (boxes), `combo_sets`, `pet_items`. |
| `mobs` | id or name | The monster, `drops`, `mvp_drops`, `skills` (own and generic ones), `spawns`, `quests` that hunt it, `summoned_by`. |
| `quests` | id (number) | The quest database entry with linked objectives and drops, and its text: `description`, `guide`, `recommended_level` (the Base Level, or `"Unknown"`), `level_basis`, `level_override` (a BaseLevel gate, with its script line), `sources`, `prerequisites` (quests that must be started or finished first, with the checking line), `generated`. Rewards are not in it; the NPCs that start the quest are on the maps (see below). |
| `skills` | id or name | `config/skill.json` entry, `metadata` (route, cast time, cooldown, flags), `learned_by` (jobs), `cast_by` (monsters). |
| `maps` | name | `display_name` (see Map names), flags, `regions` (the walkable areas of the map: cells joined without a warp, numbered from 1, with their size), `warps` out (with destination links, the `region` the warp is used from and the `to.region` it lands in), `arrivals` (warp cells on other maps that land here, with both regions), `transports` (NPCs that send characters to other maps: guards, sewer entrances, the warpers, with `touch`, `checks` and destinations), `transport_arrivals` (NPCs of other maps that send characters here), `connected_maps` (the other maps it links with, both ways), mob spawns, placed NPCs (with their `region`), `quests` (the quests those NPCs start, see below). |
| `warps` | `<map>/<x>,<y>` | A warp cell of a loaded map (the tile that takes a character to another map): its size and where it leads. Listed by name, `q` filters. |
| `destinations` | `<map>/<x>,<y>` | A place on the warper's menu (`config/wasm/warps.json`, the list every warper NPC offers): category, group and map. Listed even when no warper is placed: warpers are test NPCs, placed only with `scripting.test_npcs`. |
| `npcs` | `<map>/<name>` | The placement, the script module and entry, and for shops the `shop` offers (`price` resolved from `-1`). |
| `pets` | class id or name | The pet with its taming, egg, food and equip items. |
| `jobs` | id or name | `config/job.json` entry with its `skill_tree`. |
| `statuses` | name | The status effect of `config/status_effect.json`. |

Names: where a name is shared by several entries (monsters and items can share English names), the lowest id answers, so a key
always gives the same entry.

### Links

Entries name what they relate to as links: `{ "id": 1002, "name": "Poring", "href": "/db/mobs/1002" }`. Following `href` walks
the data (an item to the monsters that drop it, a monster to its spawn maps, a map to its warps and NPCs).

Keys in a URL are percent-encoded per `/`-separated part, so `/db/npcs/prontera/Job%20Master%231` is the NPC `Job Master#1`.

### Wiki text

Items, monsters, skills and statuses carry `wiki`: `{ "page": "/wiki/<page>.md", "description": "..." }`, the Description section
of the iRO Wiki page with the same English name, or `null` when no page has one. The text is copied into
`config/wiki_descriptions.json` by `tools/scripts-import/extract_wiki_descriptions.py` and its links point into `/wiki/`.

Items also carry `description`: `{ "identified": [...], "unidentified": [...] }`, the lines the client shows for the item
(English, pre-renewal) or `null` when the client has none. The lines come from the ROenglishPRE translation's
`data/idnum2itemdesctable.txt` and `data/num2itemdesctable.txt`, copied into `config/item_descriptions.json` by
`tools/scripts-import/extract_item_descriptions.py <ROenglishPRE checkout>`. Colour codes are removed. The translation is
a fan project that its author says is for educational use only.

### Map names

A map's links name it as the client does in English (`{ "id": "prt_fild08", "name": "Prontera Field area 8", ... }`) and its
entry has the same text in `display_name`; both fall back to the map name (`null` for `display_name`) when the client has none.
The names come from the ROenglishRE translation's `Ragnarok/data/mapnametable.txt` (ROenglishPRE has no map name table, and a
map keeps its name across episodes), copied into `config/map_names.json` by
`tools/scripts-import/extract_map_names.py <ROenglishRE checkout>`. 988 of the 1090 loaded maps have one; the others are
mostly renewal-era instances and event maps. The first line of a map wins, as later duplicates are generic.

### Quest offers

A map's `quests` lists the quests its placed NPCs start: a `quests().start(id)`, a raw `Function::SetQuest` with a literal id
(or the literal its callers pass when the id is an argument, as for the Merchant) or the new id of a `quests().change(old, new)`
in the NPC's function, its helpers and its events (`<npc>::<label>`). Raw `Function::ChangeQuest` calls are not counted: the
converted scripts use them for every step of a chain. `config/quest_offers.json`, `{ module: { npc: [ids] } }`,
is written by `tools/scripts-import/extract_quest_offers.py` from the scripts. It is a static scan, so a quest started by a
shared helper that no NPC calls by its own name is not listed (quest 18060 is the one case today). Rewards are not in it.
The game reads the same file for the "!" above a giver whose script sets no `questinfo` of its own, shown until the player has
one of its quests in the log (`show_quest_icons` in `service/quest_service.rs`).

### Routes

`/db/route?from=<map>&to=<map>` answers `{ from, to, region, hops: [...] }`, the hops in order and at most 20 of them; `from`
equal to `to` gives no hops. With the player's `x` and `y` on `from` (both or neither), the route starts in the walkable region
of that cell (`region`); without them it may start in any region. Maps are named by their id (`prt_fild08`).

- A warp hop: `{ kind: "warp", map, x, y, x_size, y_size, to_map, to_x, to_y, region, to_region, checks: [] }`, the warp cell
  to step on (`x_size`, `y_size` are half its width and height). A warp that lands on a warp of the next map names where that
  one leads.
- An NPC hop: `{ kind: "npc", map, x, y, npc, destination, menu, to_map, ..., checks: [] }`, a warper to talk to and the
  entries of `menu` to pick in order (category, group, then the place unless the group holds one). Warpers are test NPCs, so
  these hops exist only with `scripting.test_npcs`.
- Each hop also has `path: [[x, y], ...]`, the cells to walk to on its map in order: from the player's `x`, `y` for the first
  hop and from the previous hop's `to_x`, `to_y` for the others, to the closest cell of the warp's area or within 2 cells of
  the warper (the start cell left out, `[]` when it is one already). Each leg is one the client's own A* walks, which gives up
  after 100 iterations, so a client goes from cell to cell. `path` is absent when no walk is known: no `x` and `y`, an
  unreachable warp, or a map missing from the map cache.

Only ways the data states are taken: no guessed way back of a warp, no random warp, and no NPC without a known menu (guards,
Kafras, the Culvert Guardian, touch NPCs). Without the warpers, Morocc (its Continental Guards), the sewers, `pay_dun04`
(`payd04r` lands on a random cell) and the training grounds cannot be reached. Unknown maps and maps with no route answer 404,
a missing `from` or `to` 400. Code: `server/src/server/reference/db/route.rs`, after rust-ro-bot's `knowledge/graph.rs`.

### Errors

`{ "error": "..." }` with 404 for an unknown category, entry or key, 400 for a key of the wrong shape (quests need their number,
NPCs need `<map>/<name>`).

## `/wiki`

`/wiki/<page>` serves the files of `wiki_root` (default `wiki/`, the repository's copy of the iRO Wiki Classic dump, GFDL 1.3+).
Markdown is served as `text/markdown; charset=utf-8`, `/wiki` redirects to `/wiki/index.md`, and nothing outside the folder is
served.

The copy was made from the dump's `parsed/` folder. Eight image files had `%40` and `%27` in their names where the pages link to
`@` and `'`, so they were renamed to the decoded names (`1%40orcs.png` became `1@orcs.png`); the links in the pages are URL-encoded
and resolve to them.

## Known gaps

- Quests: the server has the title, time limit and objectives of each quest (from rathena `quest_db.yml`), not the rewards.
  Those live in the compiled NPC scripts under `scripts/quests`. The giver NPCs come from `config/quest_offers.json` (see
  Quest offers). The text fields and `prerequisites` come from `config/quest_text.json`: generated by
  `tools/scripts-import/gen_quest_text.py` (`generated` true) or written by hand (`generated` false). A quest not listed there has
  null text fields.
- Monster skills: `skills` lists the entries of `config/mob_skills.json` that apply to the monster, including the generic
  ones for every boss or every normal monster. The file is read from the working directory, as the server does.
- Spawns and NPCs come from the maps the server loads (`config/maps`, `config/wasm/npcs.json`); NPCs spawned by scripts are not
  listed.
- Warper destinations are one list, not one per warper NPC: every warper offers the same places, as the warper script does.
- Regions come from the map cache (`config/maps/pre-re`). Walking never cuts a corner, so cells are in one region when
  orthogonal steps join them. A landing on a cell that is not walkable counts for the nearest walkable cell.
- `transports` come from `config/npc_warps.json`, a static scan of the scripts by `tools/scripts-import/extract_npc_warps.py`.
  It finds literal destinations only: Kafra teleports (a shared function with variable destinations) are not listed, and
  `checks` (`zeny`, `item`, `quest`, `level`, `job`, `variable`) only say what the script reads on the way to the warp, not the
  condition. A destination of `x` 0 lets the server pick a random cell.
- Shop items are read from the `systems` `shop` NPCs only; other scripts that sell items are compiled Wasm and are not inspected.
- `constructor_args` of an NPC are the raw script values (`{"Number": 0}`).
