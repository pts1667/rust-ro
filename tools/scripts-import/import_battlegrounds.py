#!/usr/bin/env python3
"""Import the shared battleground queue database used by the native battleground service."""

import argparse
import json
import pathlib

import yaml

MAX_BG_MEMBERS = 30
JOB_IDS = {
    "Novice": 0,
    "SuperNovice": 23,
    "Novice_High": 4001,
    "Baby": 4023,
    "Super_Baby": 4045,
    "Super_Novice_E": 4190,
    "Super_Baby_E": 4191,
}


def team(node):
    return {
        "x": int(node["RespawnX"]),
        "y": int(node["RespawnY"]),
        "death_event": node.get("DeathEvent", ""),
        "quit_event": node.get("QuitEvent", ""),
        "active_event": node.get("ActiveEvent", ""),
        "variable": node.get("Variable", ""),
    }


def import_battlegrounds(root, reference):
    cache = root / "config/maps/pre-re"
    body = yaml.safe_load((reference / "db/battleground_db.yml").read_text(encoding="utf-8"))["Body"]
    result = []
    for entry in body:
        locations = [location for location in entry["Locations"] if (cache / f"{location['Map']}.mcache").exists()]
        if not locations:
            print(f"skipped {entry['Name']}: no arena map in the map cache")
            continue
        join = entry.get("Join", {})
        jobs = [JOB_IDS[name] for name, active in entry.get("JobRestrictions", {}).items() if active]
        result.append({
            "id": entry["Id"],
            "name": entry["Name"][:23],
            "required_players": min(max(entry.get("MinPlayers", 1), 1), MAX_BG_MEMBERS // 2),
            "max_players": min(max(entry.get("MaxPlayers", MAX_BG_MEMBERS // 2), 1), MAX_BG_MEMBERS // 2),
            "min_level": entry.get("MinLevel", 0),
            "max_level": entry.get("MaxLevel", 0),
            "deserter_seconds": entry.get("Deserter", 600),
            "start_delay_seconds": entry.get("StartDelay", 0),
            "solo": join.get("Solo", True),
            "party": join.get("Party", True),
            "guild": join.get("Guild", True),
            "job_restrictions": jobs,
            "maps": [{
                "map": location["Map"],
                "start_event": location["StartEvent"],
                "team_a": team(location["TeamA"]),
                "team_b": team(location["TeamB"]),
            } for location in locations],
        })
    out = root / "server/src/server/script/battlegrounds.json"
    out.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {len(result)} battlegrounds")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", type=pathlib.Path, default=pathlib.Path("../rathena"))
    args = parser.parse_args()
    import_battlegrounds(pathlib.Path(__file__).resolve().parents[2], args.rathena.resolve())
