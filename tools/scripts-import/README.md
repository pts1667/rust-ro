# Importing the initial game content

The server loads compiled Wasm. These optional Python tools convert the repository's original pre-renewal content offline:

```shell
python tools/scripts-import/import_items.py
python tools/scripts-import/import_npcs.py
cargo run --package tools --bin scripts-build
```

`import_items.py` reads `config/items.json` and emits `scripts/src/items.rs` plus `config/wasm/items.json`. It supports the expressions, conditionals, assignments, and calls present in the repository item catalog. It fails without writing outputs if a source cannot be converted. Each item becomes a separate Rust function to keep Wasmtime compilation efficient. The compiled guest exports the metadata fingerprint, and startup also checks each item's source hash.

`import_npcs.py` reads the enabled entries in `config/npc/scripts_custom.conf`. It emits `config/wasm/npcs.json` and the pre-renewal warp destinations embedded by the Rust warper. It recognizes the initially migrated NPC implementations and shop definitions, and rejects unrecognized NPC behavior. Inactive NPC source files are not included in the runtime manifest.

`config/wasm/map_flags.json` supplies the shared and pre-renewal map rules at startup. To regenerate it from the local reference checkout, run `python tools/scripts-import/import_mapflags.py --source-root ../rathena`. The importer follows the enabled shared and `pre-re` map-flag configuration entries in order, including custom save points, nightmare drops, skill duration and damage rates. It rejects unsupported flags and reads no renewal sources. The generated manifest is a repository asset; running the server or building the guest does not require the reference checkout.

New executable behavior should be written in Rust using `script-sdk`. These converters are migration helpers, not a general rAthena compiler. Reimporting regenerates its output files, so retain manual Rust changes separately or update the importer when necessary.
