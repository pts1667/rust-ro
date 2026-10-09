import argparse
import json
import pathlib
import re

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]
LEGACY_GROUPS = [
    "BLUEBOX", "VIOLETBOX", "CARDALBUM", "GIFTBOX", "SCROLLBOX", "FINDINGORE", "COOKIEBAG", "FIRSTAID",
    "HERB", "FRUIT", "MEAT", "CANDY", "JUICE", "FISH", "BOX", "GEMSTONE", "RESIST", "ORE", "FOOD", "RECOVERY",
    "MINERAL", "TAMING", "SCROLL", "QUIVER", "MASK", "ACCESORY", "JEWEL", "POTION",
]
SUMMON_GROUPS = ["BRANCH_OF_DEAD_TREE", "PORING_BOX", "BLOODY_DEAD_BRANCH", "RED_POUCH_OF_SURPRISE", "CLASSCHANGE", "TAEKWON_MISSION"]
SA_CREATECON = 1007
ABRA_MAX_LEVEL = 10
ITEM_USE_GROUPS = {"MF_NOTELEPORT", "MF_NORETURN", "GIANT_FLY_WING", "CASH_FOOD"}
MOB_CAPABILITIES = {
    "Detector": "Detector", "StatusImmune": "StatusImmune", "SkillImmune": "SkillImmune",
    "KnockbackImmune": "KnockbackImmune", "KnockBackImmune": "KnockbackImmune",
    "NoCast": "NoCast", "NoRandomWalk": "NoRandomWalk", "TeleportBlock": "TeleportBlocked",
    "FixedItemDrop": "FixedItemDrop", "Mvp": "Mvp",
}


def load(path):
    return yaml.safe_load(path.read_text(encoding="utf-8")) or {}


def annotate_mobs(reference):
    path = ROOT / "config/mobs.json"
    catalog = json.loads(path.read_text(encoding="utf-8"))
    mobs = {mob["id"]: mob for mob in catalog["mobs"]}
    aliases = {mob["name"]: mob["id"] for mob in mobs.values()}
    matched = 0
    for source in load(reference / "mob_db.yml").get("Body", []):
        aliases[source["AegisName"]] = source["Id"]
        if source["Id"] not in mobs:
            continue
        target = mobs[source["Id"]]
        target["monster_class"] = source.get("Class", "Normal")
        modes = source.get("Modes", {})
        target["capabilities"] = sorted({capability for name, capability in MOB_CAPABILITIES.items() if modes.get(name)})
        target["damage_modes"] = [name for name in ("IgnoreMelee", "IgnoreRanged", "IgnoreMagic", "IgnoreMisc") if modes.get(name)]
        groups = [name for name, enabled in source.get("RaceGroups", {}).items() if enabled]
        if groups:
            target["race_groups"] = groups
        matched += 1
    path.write_text(json.dumps(catalog, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return mobs, aliases, matched


def item_use_groups(groups, aliases):
    memberships = {
        group["Group"]: sorted({aliases[entry["Item"]]
                                for subgroup in group.get("SubGroups", [])
                                for entry in subgroup.get("List", [])})
        for group in groups if group["Group"] in ITEM_USE_GROUPS
    }
    missing = ITEM_USE_GROUPS - memberships.keys()
    if missing:
        raise ValueError(f"Missing pre-renewal item use groups: {sorted(missing)}")
    return memberships


def missing_item(source):
    kind = source["Type"]
    item = {
        "id": source["Id"], "name_aegis": source["AegisName"], "name_english": source["Name"],
        "item_type": kind, "price_buy": source.get("Buy", source.get("Sell", 0) * 2), "weight": source.get("Weight", 0),
        "job_flags": (1 << 64) - 1, "class_flags": 0, "location": 0, "flags": 0, "trade_flags": 0,
    }
    if "Sell" in source:
        item["price_sell"] = source["Sell"]
    if kind == "Ammo":
        item["ammo_type"] = source["SubType"]
    elif kind == "Weapon":
        item["weapon_type"] = source["SubType"]
    for key, target in [("Attack", "attack"), ("Defense", "defense"), ("Range", "range"), ("Slots", "slots"),
                        ("View", "view"), ("WeaponLevel", "weapon_level"), ("EquipLevelMin", "equip_level_min"),
                        ("EquipLevelMax", "equip_level_max")]:
        if key in source:
            item[target] = source[key]
    if source.get("Script"):
        item["script"] = source["Script"].strip().removeprefix("{").removesuffix("}").strip()
    flags = source.get("Flags", {})
    for index, name in enumerate(["BuyingStore", "DeadBranch", "Container", "UniqueId", "BindOnEquip", "DropAnnounce", "NoConsume", "DropEffect"]):
        if flags.get(name):
            item["flags"] |= 1 << index
    return item


NPC_ITEM_CALL = re.compile(r"Function::(?:GetItem2?|CountItem2?|DelItem2?|MakeItem|ConsumeItem)(?![A-Za-z0-9]),\s*vec!\[\s*Val::from\((\d+)\)")
NPC_MODULES = ("towns", "misc", "jobs", "quests", "shared")


def npc_item_ids():
    """Items the NPCs converted from rathena name: shop stock (`config/wasm/npcs.json`) and the first argument of the item commands."""
    ids = set()
    for npc in json.loads((ROOT / "config/wasm/npcs.json").read_text(encoding="utf-8")):
        if npc.get("module") == "systems" and npc["entry"] == "shop":
            ids.update(argument["Number"] for argument in npc["constructor_args"][1::2] if argument["Number"] > 0)
    for module in NPC_MODULES:
        for path in (ROOT / "scripts" / module / "src").rglob("*.rs"):
            ids.update(int(match) for match in NPC_ITEM_CALL.findall(path.read_text(encoding="utf-8")))
    return ids


def main():
    parser = argparse.ArgumentParser(description="Import pre-renewal script reward, summon and crafting data.")
    parser.add_argument("--rathena", type=pathlib.Path, default=ROOT.parent / "rathena")
    parser.add_argument("--mobs-only", action="store_true", help="Refresh only pre-renewal monster class, capability and damage-mode metadata.")
    args = parser.parse_args()
    reference = args.rathena / "db/pre-re"
    if args.mobs_only:
        _, _, matched = annotate_mobs(reference)
        print(f"Annotated {matched} pre-renewal monsters; unmatched legacy rows retain their fallback metadata")
        return
    catalog_path = ROOT / "config/items.json"
    catalog = json.loads(catalog_path.read_text(encoding="utf-8"))
    items = {item["id"]: item for item in catalog["items"]}
    aliases = {item["name_aegis"]: item["id"] for item in items.values()}
    source_items = {}
    for path in sorted(reference.glob("item_db_*.yml")):
        for item in load(path).get("Body", []):
            source_items[item["Id"]] = item
            aliases[item["AegisName"]] = item["Id"]
    needed = {name.upper() for name in re.findall(r"IG_([A-Za-z_0-9]+)", "\n".join(item.get("script", "") for item in items.values()))}
    group_sources = load(reference / "item_group_db.yml")["Body"]
    needed.update(name for name in LEGACY_GROUPS if any(group["Group"] == name for group in group_sources))
    recipes = []
    for line in (reference / "produce_db.txt").read_text(encoding="utf-8").splitlines():
        line = line.split("//", 1)[0].strip()
        if not line:
            continue
        row = [int(value.strip()) for value in line.split(",")]
        if row[1] == 0 or not (row[2] in {1, 2, 3} or 11 <= row[2] <= 23) or row[3] >= 1000 and row[3] != SA_CREATECON:
            continue
        recipes.append({"id": row[0], "item_id": row[1], "level": row[2], "skill_id": row[3], "skill_level": row[4],
                        "materials": [{"item_id": row[index], "amount": row[index + 1]} for index in range(5, len(row), 2)]})
    arrows = [{"source": entry["Source"], "make": [{"item": make["Item"], "amount": make["Amount"]} for make in entry["Make"] if make.get("Amount", 0) > 0]}
              for entry in load(args.rathena / "db/create_arrow_db.yml")["Body"]]
    arrows = [{"source": aliases[arrow["source"]], "make": [{"item_id": aliases[make["item"]], "amount": make["amount"]} for make in arrow["make"]]}
              for arrow in arrows if arrow["make"] and arrow["source"] in aliases and all(make["item"] in aliases for make in arrow["make"])]
    skill_ids = {skill["Name"]: skill["Id"] for skill in json.loads((ROOT / "server/src/server/script/skill_metadata.json").read_text(encoding="utf-8"))}
    abra = []
    for entry in load(args.rathena / "db/abra_db.yml")["Body"]:
        if entry["Skill"] not in skill_ids:
            continue
        chance = entry.get("Probability", 500)
        per = [0] * ABRA_MAX_LEVEL
        for level in range(ABRA_MAX_LEVEL):
            per[level] = chance if not isinstance(chance, list) else 0
        for level_entry in chance if isinstance(chance, list) else []:
            if level_entry["Level"] <= ABRA_MAX_LEVEL:
                per[level_entry["Level"] - 1] = level_entry["Probability"]
        abra.append({"skill_id": skill_ids[entry["Skill"]], "per": per})
    required_ids = {recipe["item_id"] for recipe in recipes}
    required_ids.update(arrow["source"] for arrow in arrows)
    required_ids.update(make["item_id"] for arrow in arrows for make in arrow["make"])
    required_ids.update(material["item_id"] for recipe in recipes for material in recipe["materials"])
    for group in group_sources:
        if group["Group"] in needed:
            required_ids.update(aliases[entry["Item"]] for subgroup in group.get("SubGroups", []) for entry in subgroup.get("List", []))
    required_ids.update(npc_item_ids() & source_items.keys())
    added = []
    for item_id in sorted(required_ids - items.keys()):
        source = source_items.get(item_id)
        if source is None:
            raise ValueError(f"Pre-renewal item {item_id} is required by script data but has no source record")
        item = missing_item(source)
        items[item_id] = item
        added.append(item_id)
    needed.update(name.upper() for name in re.findall(r"IG_([A-Za-z_0-9]+)", "\n".join(item.get("script", "") for item in items.values())))
    if added:
        catalog["items"] = sorted(items.values(), key=lambda item: item["id"])
        catalog_path.write_text(json.dumps(catalog, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    groups = []
    next_group = len(LEGACY_GROUPS)
    for group in group_sources:
        name = group["Group"]
        if name not in needed:
            continue
        group_id = LEGACY_GROUPS.index(name) if name in LEGACY_GROUPS else next_group
        if name not in LEGACY_GROUPS:
            next_group += 1
        subgroups = []
        for subgroup in group.get("SubGroups", []):
            entries = [{"item_id": aliases[entry["Item"]], "rate": entry.get("Rate", 0), "amount": entry.get("Amount", 1)}
                       for entry in subgroup.get("List", [])]
            subgroups.append({"id": subgroup["SubGroup"], "algorithm": subgroup.get("Algorithm", "SharedPool"), "entries": entries})
        groups.append({"id": group_id, "name": name, "subgroups": subgroups})
    mobs, mob_aliases, _ = annotate_mobs(reference)
    summons = []
    for group in load(reference / "mob_summon.yml").get("Body", []):
        name = group["Group"].upper()
        if name not in SUMMON_GROUPS:
            continue
        entries = [{"mob_id": mob_aliases[entry["Mob"]], "rate": entry["Rate"]} for entry in group.get("Summon", [])
                   if mob_aliases.get(entry["Mob"]) in mobs]
        summons.append({"id": SUMMON_GROUPS.index(name), "name": name, "default": mob_aliases[group["Default"]], "entries": entries})
    data = {"source": "rAthena db/pre-re", "groups": groups, "summons": summons, "recipes": recipes, "arrows": arrows, "abra": abra,
            "item_use_groups": item_use_groups(group_sources, aliases),
            "item_aliases": {name: item_id for name, item_id in aliases.items() if item_id in items}}
    (ROOT / "server/src/server/script/game_data.json").write_text(json.dumps(data, separators=(",", ":"), ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Imported {len(groups)} reward groups, {len(summons)} summon groups and {len(recipes)} recipes; added {len(added)} missing pre-renewal items")


if __name__ == "__main__":
    main()
