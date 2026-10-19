# Alive Towns

About a hundred procedural townsfolk per town. Each one repeats a small routine in a loop: walking up to a shop NPC and
asking about its wares with emotes, going into a building and coming back out a while later, strolling back and forth between
two places, loitering and looking around. Every 150 seconds some of them meet in groups of three to five on the plazas and
chat with emotes. Guards patrol. Clicking a townsperson tells what they are up to.

Settings are in `info.json`, with their defaults; override them in `config/plugins/alive_towns.json`, for example
`{ "towns": ["prontera", "payon"], "npcs_per_town": 60 }`. Enable the plugin in `config.json` under `scripting.plugins`.

## How it works
- `generated_npcs` places the townsfolk at boot. A persona (name, job sprite or walking NPC sprite, hair, colours, head gear,
  speed) is drawn from (seed, town, index), and so is the routine; the name ends in `#atTTIII` (town and index), which is how
  the dialogue finds them again.
- `schedule.rs` turns a persona into a loop of timed steps (walk, face, emote, come out of a door) and overlays the gatherings
  of each 150 s slot. Everything is a function of the time, so nothing is stored per NPC.
- Each town has a hidden controller, `Alive Towns_<n>` (`director.rs`). The server only loads a map when a player first
  enters it, and an NPC timer needs a loaded map, so a player entering a town starts its controller's `OnTimer1000`
  (`MapEnter` hook). Every second it reads the time and the town's player count in one batch, then sends every step due
  since its previous run in another. A town without players lets its timer lapse (`idle_when_empty`); its townsfolk are put
  back where their routines say when someone arrives, as they are after a gap of more than 5 seconds. A walk into a building
  runs `Alive Towns::OnDoor` on arrival, which hides the walker.

## Town data
`src/towns_data.rs` is generated: rerun `cargo run --package tools --bin alive-towns-data --release` from the repository root
after changing maps, city warps (`config/npc/pre-re/warps/cities`) or placed NPCs. For every town of `config/wasm/warps.json`
it lists, within the walkable region of the town's arrival cell:
- doors: warps into buildings (`*_in*`, churches, temples, towers), with the cell where someone coming out stands;
- counters: placed NPCs with a person sprite and a shown name, with the cell in front of them;
- spots about 8 cells apart, and plazas: open 7×7 areas about 16 cells apart, so a ring of radius 2 around them is walkable.

The looks use what the 2012 client has: hair styles 1 to 27, hair colours 0 to 8, clothes palettes 0 to 3 (Crusaders and
mounted sprites are left out), head gear from the classic view ids of `config/items.json`.
