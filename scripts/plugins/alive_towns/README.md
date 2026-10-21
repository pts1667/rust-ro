# Alive Towns

About a hundred procedural townsfolk per town. Each one repeats a small routine in a loop: walking up to a shop NPC and
asking about its wares with emotes, going into a building and coming back out a while later, strolling back and forth between
two places, loitering and looking around. Every 150 seconds some of them meet in groups of three to five on the plazas and
chat with emotes. Guards patrol. Clicking a townsperson tells what they are up to. Some of them have short quests for the
players walking by.

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

## Mini-quests
- A player walking within `quest_offer_range` cells (14 by default, 70% of the 20 cells of sight, counted |dx| + |dy| as
  the server does) of a townsperson with a quest for them sees it in a bubble above that
  townsperson, with Accept and Decline. The townsperson stops, turns to the player and waits until the player answers or
  walks away, then goes back to their routine. After an answer, no other offer shows until the player moves on.
- Offers are per player: whether a townsperson has one, and what it is, is drawn from (seed, town, townsperson, character,
  hour), so `quest_offer_percent` of the townsfolk have one for each player, a new set every hour. A declined or accepted
  offer does not come back that hour.
- A quest is a hunt (kill a number of one monster) or a gathering (bring a number of one etc. item): guards hunt, children
  gather, others gather 40% of the time. The monster is one of the town's prey (see Town data) within 10 levels below and
  3 above the player, or the 3 closest in level when fewer fit. Rewards grow with the count and the monster's experience and
  level, scaled by `quest_reward_percent`; the time limit is drawn from `quest_minutes`.
- A player holds `quest_max_active` quests at most. Expired ones are dropped silently. A finished quest is rewarded as soon
  as the player is in one of the plugin's towns: at once when it finishes there, otherwise on the next town entry or login.
- The tracker is a frameless, collapsible window under the minimap with each quest, its progress and the time left. Bots
  read the same under `plugins.alive_towns.data.quests` and answer offers through `plugin_ui` clicks on the bubble.
- State: the journal is JSON in the character variable `alive_towns_quests$`, written only on an answer, a town entry, a
  login or a reward. Kills are atomic counters (`alive_towns_kills_<n>`), so concurrent hooks cannot lose one, and a reward
  is paid by the one run that takes the claim counter `alive_towns_claim_<n>` from 0 to 1. Gathering progress is the
  inventory count, and the items are taken when the reward is paid.
- The controller drives the offers: in its once-a-second run it also reads the town's players and where the townsfolk
  really stand (`ctx.npc_cells`, since walks around buildings drift from the plan), opens and closes bubbles,
  and keeps who stands by whom in `$@near_<map>$`. Townsfolk waiting for an answer skip their routine steps.

## Town data
`src/towns_data.rs` is generated: rerun `cargo run --package tools --bin alive-towns-data --release` from the repository root
after changing maps, city warps (`config/npc/pre-re/warps/cities`) or placed NPCs. For every town of `config/wasm/warps.json`
it lists, within the walkable region of the town's arrival cell:
- doors: warps into buildings (`*_in*`, churches, temples, towers), with the cell where someone coming out stands;
- counters: placed NPCs with a person sprite and a shown name, with the cell in front of them;
- spots about 8 cells apart, and plazas: open 7×7 areas about 16 cells apart, so a ring of radius 2 around them is walkable;
- prey: up to 40 normal monsters spawning at least 5 at a time on the fields within 3 warps of the town (not through another
  town or a building), each on its closest map, with its most common etc. drop (8% or more) and whether it stands still
  (speed 1000: Pupa, eggs, plants), which changes the hunt texts. Towns with no fields around (jawaii, mid_camp, moscovia,
  thor_camp) have none and offer no quests.

The looks use what the 2012 client has: hair styles 1 to 27, hair colours 0 to 8, clothes palettes 0 to 3 (Crusaders and
mounted sprites are left out), head gear from the classic view ids of `config/items.json`.
