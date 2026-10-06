#!/usr/bin/env python3
"""Import player groups, their permissions and the atcommand alias table into config/groups.json."""

import argparse
import json
import pathlib

import yaml

PERMISSIONS = [
    "can_trade", "can_party", "all_skill", "all_equipment", "skill_unconditional", "join_chat", "kick_chat", "hide_session",
    "who_display_aid", "hack_info", "any_warp", "view_hpmeter", "view_equipment", "use_check", "use_changemaptype", "all_commands",
    "receive_requests", "show_bossmobs", "disable_pvm", "disable_pvp", "disable_commands_when_dead", "channel_admin",
    "can_trade_bounded", "item_unconditional", "command_enable", "bypass_stat_onclone", "bypass_max_stat", "attendance",
    "macro_detect", "macro_register", "trade_unconditional",
]
MAX_LEVEL = 99


def own_commands(entries, canonical):
    enabled = []
    for name, allowed in (entries or {}).items():
        name = canonical.get(name.lower())
        if name is None:
            raise ValueError(f"unknown atcommand {name}")
        if allowed and name not in enabled:
            enabled.append(name)
        elif not allowed and name in enabled:
            enabled.remove(name)
    return enabled


def own_permissions(entries):
    enabled = set()
    for name, allowed in (entries or {}).items():
        name = name.lower()
        if name == "all_permission":
            enabled = set(PERMISSIONS) if allowed else set()
        elif name in PERMISSIONS:
            enabled.add(name) if allowed else enabled.discard(name)
        else:
            raise ValueError(f"unknown permission {name}")
    return enabled


def import_groups(groups_path, atcommands_path):
    command_entries = yaml.safe_load(atcommands_path.read_text(encoding="utf-8"))["Body"]
    canonical = {}
    for entry in command_entries:
        canonical[entry["Command"].lower()] = entry["Command"].lower()
        for alias in entry.get("Aliases", []):
            canonical[alias.lower()] = entry["Command"].lower()
    body = yaml.safe_load(groups_path.read_text(encoding="utf-8"))["Body"]
    groups = {}
    for entry in body:
        groups[entry["Id"]] = {
            "id": entry["Id"],
            "name": entry["Name"],
            "level": min(int(entry.get("Level", 0)), MAX_LEVEL),
            "log_commands": bool(entry.get("LogCommands", False)),
            "commands": own_commands(entry.get("Commands"), canonical),
            "char_commands": own_commands(entry.get("CharCommands"), canonical),
            "permissions": own_permissions(entry.get("Permissions")),
            "inherit": [name.lower() for name, enabled in (entry.get("Inherit") or {}).items() if enabled],
        }
    by_name = {group["name"].lower(): group for group in groups.values()}
    resolved = {}

    def resolve(group, trail=()):
        if group["id"] in resolved:
            return resolved[group["id"]]
        if group["id"] in trail:
            raise ValueError(f"group {group['name']} inherits itself")
        commands, char_commands, permissions = list(group["commands"]), list(group["char_commands"]), set(group["permissions"])
        for parent_name in group["inherit"]:
            parent = resolve(by_name[parent_name], trail + (group["id"],))
            commands += [name for name in parent["commands"] if name not in commands]
            char_commands += [name for name in parent["char_commands"] if name not in char_commands]
            permissions |= set(parent["permissions"])
        resolved[group["id"]] = {**group, "commands": commands, "char_commands": char_commands, "permissions": sorted(permissions)}
        return resolved[group["id"]]

    result = []
    for group_id in sorted(groups):
        group = resolve(groups[group_id])
        del group["inherit"]
        result.append(group)

    aliases = {entry["Command"].lower(): [alias.lower() for alias in entry.get("Aliases", [])] for entry in command_entries}
    return {"groups": result, "atcommands": dict(sorted(aliases.items()))}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", type=pathlib.Path, default=pathlib.Path("../rathena"))
    args = parser.parse_args()
    root = pathlib.Path(__file__).resolve().parents[2]
    data = import_groups(args.rathena / "conf/groups.yml", args.rathena / "conf/atcommands.yml")
    (root / "config/groups.json").write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {len(data['groups'])} groups and {len(data['atcommands'])} atcommands")
