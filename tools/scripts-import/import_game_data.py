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
ITEM_USE_GROUPS = {"MF_NOTELEPORT", "MF_NORETURN", "GIANT_FLY_WING"}


def load(path):
    return yaml.safe_load(path.read_text(encoding="utf-8")) or {}


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


def main():
    parser = argparse.ArgumentParser(description="Import pre-renewal script reward, summon and crafting data.")
    parser.add_argument("--rathena", type=pathlib.Path, default=ROOT.parent / "rathena")
    args = parser.parse_args()
    reference = args.rathena / "db/pre-re"
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
        if row[1] == 0 or not (row[2] in {1, 2, 3} or 11 <= row[2] <= 23) or row[3] >= 1000:
            continue
        recipes.append({"id": row[0], "item_id": row[1], "level": row[2], "skill_id": row[3], "skill_level": row[4],
                        "materials": [{"item_id": row[index], "amount": row[index + 1]} for index in range(5, len(row), 2)]})
    required_ids = {recipe["item_id"] for recipe in recipes}
    required_ids.update(material["item_id"] for recipe in recipes for material in recipe["materials"])
    for group in group_sources:
        if group["Group"] in needed:
            required_ids.update(aliases[entry["Item"]] for subgroup in group.get("SubGroups", []) for entry in subgroup.get("List", []))
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
    mob_catalog_path = ROOT / "config/mobs.json"
    mob_catalog = json.loads(mob_catalog_path.read_text(encoding="utf-8"))
    mobs = {mob["id"]: mob for mob in mob_catalog["mobs"]}
    mob_aliases = {mob["name"]: mob["id"] for mob in mobs.values()}
    for mob in load(reference / "mob_db.yml").get("Body", []):
        mob_aliases[mob["AegisName"]] = mob["Id"]
        if mob["Id"] in mobs:
            mobs[mob["Id"]]["monster_class"] = mob.get("Class", "Normal")
            groups_for_mob = [name for name, enabled in mob.get("RaceGroups", {}).items() if enabled]
            if groups_for_mob:
                mobs[mob["Id"]]["race_groups"] = groups_for_mob
    mob_catalog_path.write_text(json.dumps(mob_catalog, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    summons = []
    for group in load(reference / "mob_summon.yml").get("Body", []):
        name = group["Group"].upper()
        if name not in SUMMON_GROUPS:
            continue
        entries = [{"mob_id": mob_aliases[entry["Mob"]], "rate": entry["Rate"]} for entry in group.get("Summon", [])
                   if mob_aliases.get(entry["Mob"]) in mobs]
        summons.append({"id": SUMMON_GROUPS.index(name), "name": name, "default": mob_aliases[group["Default"]], "entries": entries})
    data = {"source": "rAthena db/pre-re", "groups": groups, "summons": summons, "recipes": recipes,
            "item_use_groups": item_use_groups(group_sources, aliases),
            "item_aliases": {name: item_id for name, item_id in aliases.items() if item_id in items}}
    (ROOT / "server/src/server/script/game_data.json").write_text(json.dumps(data, separators=(",", ":"), ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Imported {len(groups)} reward groups, {len(summons)} summon groups and {len(recipes)} recipes; added {len(added)} missing pre-renewal items")


if __name__ == "__main__":
    main()
