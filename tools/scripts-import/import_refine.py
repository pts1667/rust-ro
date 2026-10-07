#!/usr/bin/env python3
"""Import the pre-renewal refine database (rates, prices and ores per weapon or armor level) for the native refine commands."""

import argparse
import json
import pathlib

import yaml

# Shadow equipment does not exist at PACKETVER 20120307.
GROUPS = {"Armor": "armor", "Weapon": "weapon"}
COST_TYPES = {"Normal": "normal", "Enriched": "enriched", "HD": "hd"}


def convert(body, item_ids):
    levels = []
    for group in body:
        if group["Group"] not in GROUPS:
            continue
        for level in group.get("Levels", []):
            for refine in level["RefineLevels"]:
                costs = {}
                for chance in refine.get("Chances", []):
                    costs[COST_TYPES[chance["Type"]]] = {
                        "rate": chance.get("Rate", 0),
                        "price": chance.get("Price", 0),
                        "material": item_ids[chance["Material"]],
                    }
                levels.append({
                    "group": GROUPS[group["Group"]],
                    "level": level["Level"],
                    "refine": refine["Level"],
                    "bonus": refine.get("Bonus", 0),
                    "costs": costs,
                })
    return levels


def main():
    root = pathlib.Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", default=str(root.parent / "rathena"))
    parser.add_argument("--output", default=str(root / "server/src/server/model/refine.json"))
    options = parser.parse_args()
    items = json.loads((root / "config/items.json").read_text(encoding="utf-8"))["items"]
    item_ids = {item["name_aegis"]: item["id"] for item in items}
    source = pathlib.Path(options.rathena) / "db/pre-re/refine.yml"
    body = yaml.safe_load(source.read_text(encoding="utf-8"))["Body"]
    levels = convert(body, item_ids)
    pathlib.Path(options.output).write_text(json.dumps(levels, separators=(",", ":")) + "\n", encoding="utf-8")
    print(f"wrote {len(levels)} refine levels to {options.output}")


if __name__ == "__main__":
    main()
