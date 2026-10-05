#!/usr/bin/env python3
"""Import the First Edition guild castle layout used by the native siege service."""

import argparse
import json
import pathlib
import re

import yaml


def castle_names(reference):
    body = yaml.safe_load((reference / "db/pre-re/castle_db.yml").read_text(encoding="utf-8"))["Body"]
    return {entry["Map"]: (entry["Id"], entry["Name"]) for entry in body if entry.get("Type", "First_Edition") == "First_Edition" and entry["Map"].split("_")[0] in ("aldeg", "gefg", "payg", "prtg")}


def import_castles(root, reference):
    source = (reference / "npc/guild/agit_main.txt").read_text(encoding="utf-8")
    names = castle_names(reference)
    castles = {map_name: {"id": ident, "map": map_name, "name": name} for map_name, (ident, name) in names.items()}

    for map_name, x, y in re.findall(r'strnpcinfo\(2\) == "(\w+)"\) \{ setarray \.@emproom\[0\],(\d+),(\d+); \}', source.split("OnAgitBreak:")[0]):
        castles[map_name]["emperium"] = [int(x), int(y)]

    recv = source[source.index("OnRecvCastle:"):source.index("// WoE : Guild Kafras")]
    regions = {}
    for region, body in re.findall(r'compare\(strnpcinfo\(2\),"(\w+)"\)\) \{(.*?)\n\t\t\}\n\t\t(?:else|// Add)', recv, re.S):
        before, _, after = body.partition("// Set Emperium room spawn coordinates and spawn monsters.")
        spawn = lambda text: [{"name": name, "id": int(ident), "count": int(count)}
                              for name, ident, count in re.findall(r'monster strnpcinfo\(2\),(?:0,0|\.@emproom\[0\],\.@emproom\[1\]),"([^"]+)",(\d+),(\d+);', text)]
        regions[region] = {"field": spawn(before), "emperium_room": spawn(after.split("}\n")[-1] if False else after)}
    for castle in castles.values():
        region = castle["map"].split("_")[0]
        castle["spawns"] = regions[region]

    guard = source[source.index("OnSpawnGuardians:"):source.index("OnGuardianDied:")]
    for map_name, types, xs, ys in re.findall(r'"(\w+)"\) \{\s*setarray \.@guardiantype\[0\],([\d,]+);\s*setarray \.@guardianposx\[0\],([\d,]+);\s*setarray \.@guardianposy\[0\],([\d,]+);', guard):
        castles[map_name]["guardians"] = [{"type": int(t), "x": int(x), "y": int(y)} for t, x, y in zip(types.split(","), xs.split(","), ys.split(","))]

    steward = source[source.index("-\tscript\tCastle Manager#cm::cm"):source.index("-\tscript\tGld_Guard_Template")]
    for map_name, room in re.findall(r'"(\w+)"\) \{[^}]*?setarray \.@masterroom\[0\],(\d+,\d+);', steward, re.S):
        castles[map_name]["master_room"] = [int(v) for v in room.split(",")]

    treasure = source[source.index("-\tscript\tGld_Trea_Spawn"):]
    for map_name, box, xs, ys in re.findall(r'"(\w+)"\) \{\s*set \.@treasurebox,(\d+);\s*setarray \.@treasurex\[0\],([\d,]+);\s*setarray \.@treasurey\[0\],([\d,]+);', treasure):
        castles[map_name]["treasure"] = {"box": int(box), "cells": [[int(x), int(y)] for x, y in zip(xs.split(","), ys.split(","))]}

    for map_name, castle in castles.items():
        text = (reference / f"npc/guild/{map_name}.txt").read_text(encoding="utf-8")
        steward = re.search(rf"^{map_name},(\d+),(\d+),(\d+)	duplicate\(cm\)	([^	]+)	", text, re.M)
        kafra = re.search(rf"^{map_name},(\d+),(\d+),(\d+)	duplicate\(guildkafra\)	([^	]+)	", text, re.M)
        dungeon = re.search(rf"^{map_name},(\d+),(\d+),(\d+)	duplicate\(gdlever\)	([^	]+)	", text, re.M)
        exit_lever = re.search(rf"^{map_name},(\d+),(\d+),(\d+)	script	(#[Ll]ever_\w+)	\d+,\{{.*?warp \"{map_name}\",(\d+),(\d+);", text, re.M | re.S)
        for key, match in (("steward", steward), ("kafra", kafra), ("dungeon_lever", dungeon)):
            if not match:
                raise ValueError(f"{map_name} has no {key}")
            castle[key] = {"x": int(match[1]), "y": int(match[2]), "dir": int(match[3]), "name": match[4]}
        if not exit_lever:
            raise ValueError(f"{map_name} has no treasure room exit lever")
        flag_pattern = lambda kind: re.findall(rf"^(\w+),(\d+),(\d+),(\d+)	duplicate\({kind}Flags\w+\)	([^	]+)	722", text, re.M)
        flag = lambda match: {"map": match[0], "x": int(match[1]), "y": int(match[2]), "dir": int(match[3]), "name": match[4]}
        flag_return = re.search(rf'warp "{map_name}",(\d+),(\d+);', text)
        castle["flags"] = {
            "return": [int(flag_return[1]), int(flag_return[2])],
            "outside": [flag(match) for match in flag_pattern("Outside")],
            "inside": [flag(match) for match in flag_pattern("Inside")],
        }
        castle["treasure_lever"] = {"x": int(exit_lever[1]), "y": int(exit_lever[2]), "dir": int(exit_lever[3]), "name": exit_lever[4], "exit": [int(exit_lever[5]), int(exit_lever[6])]}
    lever = source[source.index("-	script	Lever#gd::gdlever"):source.index("// Guardian Spawner Template")]
    dungeon_maps = {"aldeg": "gld_dun02", "gefg": "gld_dun04", "payg": "gld_dun01", "prtg": "gld_dun03"}
    for region, body in re.findall(r'compare\(strnpcinfo\(2\),"(\w+)"\)\) \{(.*?)\n\t\}', lever, re.S):
        for suffix, x, y in re.findall(r'compare\(strnpcinfo\(2\),"(cas\d+)"\)\) setarray \.@coordinates\[0\],(\d+),(\d+);', body):
            castles[f"{region}_{suffix}"]["dungeon_lever"]["destination"] = [dungeon_maps[region], int(x), int(y)]
    kafra_regions = re.findall(r'compare\(strnpcinfo\(2\),"(\w+)"\)\) \{\s*setarray \.@destination\$\[0\],"([^"]+)","(\w+)";\s*setarray \.@coordinates\[0\],(\d+),(\d+);', source)
    for region, label, map_name, x, y in kafra_regions:
        for castle in castles.values():
            if castle["map"].startswith(region + "_"):
                castle["kafra"]["destination"] = [label, map_name, int(x), int(y)]

    ordered = sorted(castles.values(), key=lambda castle: castle["id"])
    for castle in ordered:
        missing = {"emperium", "spawns", "guardians", "master_room", "treasure", "flags"} - castle.keys()
        if missing:
            raise ValueError(f"{castle['map']} is missing {sorted(missing)}")
        if not castle["spawns"]["field"] or not castle["spawns"]["emperium_room"]:
            raise ValueError(f"{castle['map']} has empty spawn tables")
    out = root / "server/src/server/script/castles.json"
    out.write_text(json.dumps(ordered, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {len(ordered)} castles")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", type=pathlib.Path, default=pathlib.Path("../rathena"))
    args = parser.parse_args()
    import_castles(pathlib.Path(__file__).resolve().parents[2], args.rathena.resolve())
