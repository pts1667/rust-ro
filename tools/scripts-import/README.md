# Importing the initial game content

The server loads compiled Wasm. These optional Python tools convert the repository's original pre-renewal content offline:

```shell
python tools/scripts-import/import_items.py
python tools/scripts-import/import_npcs.py
cargo run --package tools --bin scripts-build
```

`import_items.py` reads `config/items.json` and emits `scripts/src/items.rs` plus `config/wasm/items.json`. It supports the expressions, conditionals, assignments, and calls present in the repository item catalog. It fails without writing outputs if a source cannot be converted. Each item becomes a separate Rust function to keep Wasmtime compilation efficient. The compiled guest exports the metadata fingerprint, and startup also checks each item's source hash.

`import_npcs.py` reads the enabled entries in `config/npc/scripts_custom.conf`. It emits `config/wasm/npcs.json` and the pre-renewal warp destinations embedded by the Rust warper. It recognizes the initially migrated NPC implementations and shop definitions, and rejects unrecognized NPC behavior. Inactive NPC source files are not included in the runtime manifest.

`import_world_data.py ../rathena` imports shared and pre-renewal companion data and compiles pet bonus/support scripts. `--pet-overrides path/to/pets.yml` merges script overrides for existing pre-renewal pet classes. Literal `petautobonus`, `petautobonus2`, and `petautobonus3` bodies become Rust functions in `scripts/src/pets.rs`; their bonus and visual programs are listed in `config/wasm/pet_bonus_programs.json`. The guest exports `run_pet_auto_bonus` separately from item bonus programs. Rebuild the guest and server after changing these assets.

`config/wasm/map_flags.json` supplies the shared and pre-renewal map rules at startup. To regenerate it from the local reference checkout, run `python tools/scripts-import/import_mapflags.py --source-root ../rathena`. The importer follows the enabled shared and `pre-re` map-flag configuration entries in order, including custom save points, nightmare drops, skill duration and damage rates. It rejects unsupported flags and reads no renewal sources. The generated manifest is a repository asset; running the server or building the guest does not require the reference checkout.

`python tools/scripts-import/import_skill_damage.py --source-root ../rathena` imports the shared `db/skill_damage_db.txt` and optional `db/import/skill_damage_db.txt` overrides for skills in the embedded classic metadata. It writes `server/src/server/script/skill_damage_adjustments.json`; rebuild the server after changing it. These adjustments combine with map-wide and per-skill `skill_damage` rules and select normal, PvP, GvG, battleground, flagged, or restricted-zone maps. The reference checkout currently supplies no active adjustment rows, so the bundled catalog is empty. Runtime map rules still apply without any database rows.

New executable behavior should be written in Rust using `script-sdk`. These converters are migration helpers, not a general rAthena compiler. Reimporting regenerates its output files, so retain manual Rust changes separately or update the importer when necessary.

`python tools/scripts-import/import_talkie_packets.py ../rathena` regenerates `server/src/server/request_handler/talkie_box_packets.json` from the shared main-client packet registry and shuffle headers. Talkie Box and Graffiti share this input packet. The importer preserves version-specific lengths, padding, field offsets, shortened modern messages, and packet-ID redefinitions. The server embeds this catalog; runtime and normal builds do not require the reference checkout. Text ground skills send and receive message bytes without imposing UTF-8 on legacy clients. Regenerate the catalog and rebuild the server when changing supported client layouts.

`python tools/scripts-import/import_battlegrounds.py --rathena ../rathena` regenerates `server/src/server/script/battlegrounds.json` from the shared `db/battleground_db.yml` (queue sizes, level range, job restrictions, deserter and start delays, arena maps with team spawn points and NPC event labels). Arenas whose map is missing from `config/maps/pre-re` are skipped. The server embeds the catalog; rebuild it after regenerating. `import_npcs.py` also emits the Flavius, KvM and Tierra Gorge arena placements (`battleground_arena_npcs`, `kvm_arena_npcs`, `tierra_npcs`) and merges their compiled events (ids from 1000, 2000 and 3000, matching `scripts/src/battleground_arena.rs`, `battleground_kvm.rs` and `battleground_tierra.rs`) into `config/wasm/events.json`; rebuild the guest after changing either side.

Compiled NPC timer callbacks use entries such as `"NPC Name::OnTimer5000": 12` in `config/wasm/events.json`, with matching Rust dispatch in `scripts/src/events.rs`. Timer labels use positive millisecond counts and run through `run_event`; registering a label does not generate its handler body. The stock event registry currently contains no NPC timer handlers. The SDK exposes NPC timer controls, player timer attachment, and countdown calls through `Function`.
