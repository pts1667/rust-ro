#!/usr/bin/env python3
"""Import the pre-renewal item combos (`db/pre-re/item_combos.yml`) to `config/item_combos.json`.

Each set gets a virtual script id (`FIRST_COMBO_SCRIPT_ID` + position) that `import_items.py` compiles like an item script.
Combos naming an item the catalog does not have are dropped, as rathena does.
"""

import argparse
import json
import pathlib

import yaml

FIRST_COMBO_SCRIPT_ID = 1_000_000


def convert(body, item_ids):
    sets = []
    for entry in body:
        combos = []
        for combo in entry["Combos"]:
            ids = [item_ids.get(name) for name in combo["Combo"]]
            if None not in ids and len(ids) >= 2:
                combos.append(ids)
        if combos:
            sets.append({"id": FIRST_COMBO_SCRIPT_ID + len(sets), "combos": combos, "script": entry["Script"]})
    return sets


def main():
    root = pathlib.Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", default=str(root.parent / "rathena"))
    parser.add_argument("--output", default=str(root / "config/item_combos.json"))
    options = parser.parse_args()
    items = json.loads((root / "config/items.json").read_text(encoding="utf-8"))["items"]
    item_ids = {item["name_aegis"]: item["id"] for item in items}
    source = pathlib.Path(options.rathena) / "db/pre-re/item_combos.yml"
    body = yaml.safe_load(source.read_text(encoding="utf-8"))["Body"]
    sets = convert(body, item_ids)
    pathlib.Path(options.output).write_text(json.dumps(sets, indent=1) + "\n", encoding="utf-8")
    print(f"wrote {len(sets)} of {len(body)} combo sets to {options.output}")


if __name__ == "__main__":
    main()
