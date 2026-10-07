#!/usr/bin/env python3
"""Import the pre-renewal memorial dungeon database used by the native instance service."""

import argparse
import json
import pathlib

import yaml

DEFAULT_TIME_LIMIT = 3600
DEFAULT_IDLE_TIMEOUT = 300


def convert(entry):
    enter = entry["Enter"]
    maps = [name for name, active in (entry.get("AdditionalMaps") or {}).items() if active and name != enter["Map"]]
    return {
        "id": entry["Id"],
        "name": entry["Name"],
        "time_limit": entry.get("TimeLimit", DEFAULT_TIME_LIMIT),
        "idle_timeout": entry.get("IdleTimeOut", DEFAULT_IDLE_TIMEOUT),
        "no_npc": entry.get("NoNpc", False),
        "no_map_flag": entry.get("NoMapFlag", False),
        "destroyable": entry.get("Destroyable", True),
        "enter_map": enter["Map"],
        "enter_x": enter["X"],
        "enter_y": enter["Y"],
        "maps": maps,
    }


def main():
    root = pathlib.Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", default=str(root.parent / "rathena"))
    parser.add_argument("--output", default=str(root / "server/src/server/model/instances.json"))
    options = parser.parse_args()
    source = pathlib.Path(options.rathena) / "db/pre-re/instance_db.yml"
    entries = yaml.safe_load(source.read_text(encoding="utf-8"))["Body"]
    names = [entry["Name"] for entry in entries]
    if len(set(names)) != len(names):
        raise SystemExit("duplicate instance names")
    pathlib.Path(options.output).write_text(json.dumps([convert(entry) for entry in entries], indent=2) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} instances to {options.output}")


if __name__ == "__main__":
    main()
