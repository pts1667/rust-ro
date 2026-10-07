#!/usr/bin/env python3
"""Import the shared pre-renewal quest database used by the native quest service."""

import argparse
import json
import pathlib
import re

import yaml

MAX_QUEST_OBJECTIVES = 3
RACES = {"Formless", "Undead", "Brute", "Plant", "Insect", "Fish", "Demon", "Demihuman", "Angel", "Dragon", "All"}
SIZES = {"Small", "Medium", "Large", "All"}
ELEMENTS = {"Neutral", "Water", "Earth", "Fire", "Wind", "Poison", "Holy", "Dark", "Ghost", "Undead", "All"}
UNITS = {"d": 86400, "h": 3600, "mn": 60, "s": 1}
WEEKDAYS = ["SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"]


def relative_seconds(text):
    """`+1d12h` style delays of rathena's `solve_time`; None when the text is not one."""
    parts = re.findall(r"(\d+)(d|h|mn|s)", text)
    if not text.startswith("+") or not parts or "".join(f"{n}{u}" for n, u in parts) != text[1:]:
        return None
    return sum(int(amount) * UNITS[unit] for amount, unit in parts)


def exact_time(text):
    """`HHh` / `MONDAY18h` style fixed moment: (seconds from midnight, weekday or -1, days offset)."""
    week = -1
    upper = text.upper()
    for index, name in enumerate(WEEKDAYS):
        if name in upper:
            week = index
    values = {unit: int(amount) for amount, unit in re.findall(r"(\d+)(mn|h|d|j|s)", text)}
    if "h" not in values or values["h"] > 23 or values.get("mn", 0) > 59 or values.get("s", 0) > 59:
        return None
    seconds = values["h"] * 3600 + values.get("mn", 0) * 60 + values.get("s", 0)
    days = values.get("d", values.get("j", 0))
    return seconds if week > 0 else days * 86400 + seconds, week


def objective(node, mobs):
    target = {"mob": 0, "count": int(node.get("Count", 0)), "min_level": 0, "max_level": 0, "race": "All",
              "size": "All", "element": "All", "location": "", "map_name": "", "mobs_allowed": []}
    if "Mob" in node:
        if node["Mob"] not in mobs:
            return None
        target["mob"] = mobs[node["Mob"]]
    else:
        target["min_level"] = int(node.get("MinLevel", 0))
        target["max_level"] = int(node.get("MaxLevel", 0))
        if target["max_level"] and not target["min_level"]:
            target["min_level"] = 1
        for key, allowed in (("Race", RACES), ("Size", SIZES), ("Element", ELEMENTS)):
            value = str(node.get(key, "All"))
            if value not in allowed:
                return None
            target[key.lower()] = value
        location = str(node.get("Location", ""))
        target["location"] = "" if location.lower() == "all" else location
        target["map_name"] = str(node.get("MapName", ""))
        target["mobs_allowed"] = sorted(mobs[name] for name, enabled in (node.get("MapMobTargets") or {}).items()
                                        if enabled and name in mobs)
    return target


def import_quests(root, reference):
    mobs = {mob["name"]: mob["id"] for mob in json.loads((root / "config/mobs.json").read_text(encoding="utf-8"))["mobs"]}
    items = {item["name_aegis"]: item["id"] for item in json.loads((root / "config/items.json").read_text(encoding="utf-8"))["items"]}
    body = yaml.safe_load((reference / "db/pre-re/quest_db.yml").read_text(encoding="utf-8"))["Body"]
    quests, skipped = [], []
    for node in body:
        quest = {"id": int(node["Id"]), "title": node["Title"], "time": 0, "time_at": False, "time_week": -1,
                 "objectives": [], "drops": []}
        if "TimeLimit" in node:
            limit = str(node["TimeLimit"])
            seconds = relative_seconds(limit)
            if seconds is not None:
                quest["time"] = seconds
            elif (fixed := exact_time(limit)) is not None:
                quest["time"], quest["time_week"] = fixed
                quest["time_at"] = True
            else:
                skipped.append((quest["id"], f"TimeLimit {limit}"))
                continue
        failure = None
        for target in node.get("Targets") or []:
            parsed = objective(target, mobs)
            if parsed is None or len(quest["objectives"]) >= MAX_QUEST_OBJECTIVES:
                failure = f"target {target}"
                break
            quest["objectives"].append(parsed)
        for drop in node.get("Drops") or []:
            if drop.get("Item") not in items or (drop.get("Mob") and drop["Mob"] not in mobs):
                continue
            quest["drops"].append({"mob": mobs.get(drop.get("Mob"), 0), "item": items[drop["Item"]],
                                   "count": int(drop.get("Count", 1)), "rate": int(drop["Rate"])})
        if failure:
            skipped.append((quest["id"], failure))
            continue
        quests.append(quest)
    return quests, skipped


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rathena", default="../rathena", type=pathlib.Path)
    root = pathlib.Path(__file__).resolve().parents[2]
    parser.add_argument("--output", default=root / "server/src/server/script/quests.json", type=pathlib.Path)
    arguments = parser.parse_args()
    quests, skipped = import_quests(root, arguments.rathena)
    for quest_id, reason in skipped:
        print(f"skipped quest {quest_id}: {reason}")
    arguments.output.write_text(json.dumps(quests, separators=(",", ":"), ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"{len(quests)} quests written to {arguments.output}")


if __name__ == "__main__":
    main()
