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

Query parameters on listings: `q` (part of the name, case-insensitive), `map` (NPCs only), `offset`, `limit` (default 50, at most 200).

Categories and keys:

| Category | Key | Entry holds |
| --- | --- | --- |
| `items` | id or name | The item, its client `description`, its script metadata, `dropped_by`, `sold_by`, `recipes` (`made_by`, `used_in`), `arrow_crafting`, `groups` (boxes), `combo_sets`, `pet_items`. |
| `mobs` | id or name | The monster, `drops`, `mvp_drops`, `skills` (own and generic ones), `spawns`, `quests` that hunt it, `summoned_by`. |
| `quests` | id (number) | The quest database entry with linked objectives and drops, and its text: `description`, `guide`, `recommended_level` (the Base Level, or `"Unknown"`), `level_basis`, `level_override` (a BaseLevel gate, with its script line), `sources`, `prerequisites` (quests that must be started or finished first, with the checking line), `generated`. Rewards are not in it; the NPCs that start the quest are on the maps (see below). |
| `skills` | id or name | `config/skill.json` entry, `metadata` (route, cast time, cooldown, flags), `learned_by` (jobs), `cast_by` (monsters). |
| `maps` | name | Flags, `warps` out (with destination links), `arrivals` (warp cells on other maps that land here), `connected_maps` (the other maps it links with, both ways), mob spawns, placed NPCs, `quests` (the quests those NPCs start, see below). |
| `warps` | `<map>/<x>,<y>` | A warp cell of a loaded map (the tile that takes a character to another map): its size and where it leads. Listed by name, `q` filters. |
| `destinations` | `<map>/<x>,<y>` | A place on the warper's menu (`config/wasm/warps.json`, the list every warper NPC offers): category, group and map. |
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

### Quest offers

A map's `quests` lists the quests its placed NPCs start: a `quests().start(id)` or the new id of a `quests().change(old, new)`
in the NPC's function, its helpers and its events (`<npc>::<label>`). `config/quest_offers.json`, `{ module: { npc: [ids] } }`,
is written by `tools/scripts-import/extract_quest_offers.py` from the scripts. It is a static scan, so a quest started by a
shared helper that no NPC calls by its own name is not listed (quest 18060 is the one case today). Rewards are not in it.

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
- Shop items are read from the `systems` `shop` NPCs only; other scripts that sell items are compiled Wasm and are not inspected.
- `constructor_args` of an NPC are the raw script values (`{"Number": 0}`).
