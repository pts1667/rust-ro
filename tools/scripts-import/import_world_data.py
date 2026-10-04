"""Import companion and guild data from an explicitly pre-renewal rAthena checkout."""
import argparse
import json
import re
from pathlib import Path

import yaml
from import_items import Parser


def body(path):
    return yaml.safe_load(path.read_text(encoding="utf-8")).get("Body", [])


def client_frames(reference, packetver):
    definitions = {"PACKETVER": packetver, "PACKETVER_MAIN_NUM": packetver, "PACKETVER_RE_NUM": 0,
                   "PACKETVER_ZERO_NUM": 0, "PACKETVER_SAK_NUM": 0}
    active = True
    stack = []
    frames = {}

    def condition(expression):
        expression = expression.split("//", 1)[0].strip()
        expression = re.sub(r"defined\s*\(?\s*(\w+)\s*\)?", lambda match: str(int(match[1] in definitions)), expression)
        expression = re.sub(r"\b[A-Za-z_]\w*\b", lambda match: str(definitions.get(match[0], 0)), expression)
        if re.search(r"[^0-9\s()<>=!&|+*/.-]", expression):
            raise ValueError(f"Unsupported packet-version expression: {expression}")
        expression = expression.replace("&&", " and ").replace("||", " or ")
        expression = re.sub(r"!(?!=)", " not ", expression)
        return bool(eval(expression, {"__builtins__": {}}, {}))

    for line in (reference / "src/map/clif_packetdb.hpp").read_text().splitlines():
        stripped = line.strip()
        if stripped.startswith(("#ifdef ", "#ifndef ", "#if ")):
            if stripped.startswith("#ifdef "):
                accepted = stripped.split()[1] in definitions
            elif stripped.startswith("#ifndef "):
                accepted = stripped.split()[1] not in definitions
            else:
                accepted = condition(stripped[4:])
            stack.append((active, accepted))
            active = active and accepted
        elif stripped.startswith("#elif "):
            parent, previous = stack[-1]
            accepted = not previous and condition(stripped[6:])
            stack[-1] = (parent, previous or accepted)
            active = parent and accepted
        elif stripped == "#else":
            parent, previous = stack[-1]
            active = parent and not previous
            stack[-1] = (parent, True)
        elif stripped.startswith("#endif"):
            active = stack.pop()[0]
        elif active and stripped.startswith("#define "):
            name = stripped.split()[1]
            if "(" not in name:
                definitions.setdefault(name, 1)
        elif active:
            match = re.match(r"(?:parseable_packet|packet)\s*\(\s*(0x[0-9a-fA-F]+)\s*,\s*(-?\d+)\s*[,)]", stripped)
            if match:
                frames[int(match[1], 16)] = int(match[2])
    if stack:
        raise ValueError("Unbalanced primary packet database conditions")
    return frames


def import_data(repository, reference, pet_overrides=None):
    items = {item["name_aegis"]: item["id"] for item in json.loads((repository / "config/items.json").read_text())["items"]}
    mobs = {mob["name"]: mob for mob in json.loads((repository / "config/mobs.json").read_text())["mobs"]}
    for category in ("etc", "usable", "equip"):
        for item in body(reference / f"db/pre-re/item_db_{category}.yml"):
            items.setdefault(item["AegisName"], item["Id"])
    pet_sources = {source["Mob"]: source for source in body(reference / "db/pre-re/pet_db.yml")}
    if pet_overrides is not None:
        for override in body(pet_overrides):
            if override["Mob"] not in pet_sources:
                raise ValueError(f"Pet override is not a pre-renewal pet: {override['Mob']}")
            pet_sources[override["Mob"]].update(override)
    pets = []
    for source in pet_sources.values():
        mob = mobs[source["Mob"]]
        pets.append({
            "class_id": mob["id"], "name": mob["name_english"], "level": mob["level"],
            "tame_item": items.get(source.get("TameItem"), 0), "egg_item": items[source["EggItem"]],
            "food_item": items.get(source.get("FoodItem"), 0), "equip_item": items.get(source.get("EquipItem"), 0),
            "capture_rate": source.get("CaptureRate", 0), "fullness": source.get("Fullness", 0),
            "hungry_delay": source.get("HungryDelay", 60) * 1000, "hunger_increase": source.get("HungerIncrease", 20),
            "intimacy_start": source.get("IntimacyStart", 250), "intimacy_fed": source.get("IntimacyFed", 50),
            "intimacy_overfed": source.get("IntimacyOverfed", -100), "intimacy_hungry": source.get("IntimacyHungry", -5),
            "intimacy_owner_die": source.get("IntimacyOwnerDie", -20),
            "has_bonus_script": bool(source.get("Script")), "has_support_script": bool(source.get("SupportScript")),
            "attack_rate": min(source["AttackRate"], 10000) if "AttackRate" in source else 10001,
            "retaliation_rate": min(source["RetaliateRate"], 10000) if "RetaliateRate" in source else 10001,
            "change_target_rate": min(source["ChangeTargetRate"], 10000) if "ChangeTargetRate" in source else 10001,
        })
    compile_pet_scripts(repository, pet_sources.values(), mobs)
    mercenaries = body(reference / "db/mercenary_db.yml")
    mercenaries.extend(body(reference / "db/pre-re/mercenary_db.yml"))
    by_id = {row["Id"]: row for row in mercenaries}
    mercenaries = []
    for source in by_id.values():
        mercenaries.append({
            "class_id": source["Id"], "name": source["Name"], "level": source.get("Level", 1),
            "hp": source.get("Hp", 1), "sp": source.get("Sp", 1), "attack": source.get("Attack", 0),
            "attack2": source.get("Attack2", 0), "defense": source.get("Defense", 0), "magic_defense": source.get("MagicDefense", 0),
            "stats": [source.get(name, 1) for name in ("Str", "Agi", "Vit", "Int", "Dex", "Luk")],
            "range": source.get("AttackRange", 1), "walk_speed": source.get("WalkSpeed", 150),
            "attack_delay": source.get("AttackDelay", 4000), "attack_motion": source.get("AttackMotion", 2000),
            "size": source.get("Size", "Small"), "race": source.get("Race", "Formless"),
            "element": source.get("Element", "Neutral"), "element_level": source.get("ElementLevel", 1),
            "skills": [{"name": skill["Name"], "level": skill["MaxLevel"]} for skill in source.get("Skills", [])],
        })
    classes = {name: 6001 + index for index, name in enumerate((
        "Lif", "Amistr", "Filir", "Vanilmirth", "Lif2", "Amistr2", "Filir2", "Vanilmirth2",
        "Lif_H", "Amistr_H", "Filir_H", "Vanilmirth_H", "Lif_H2", "Amistr_H2", "Filir_H2", "Vanilmirth_H2"))}
    homunculi = []
    for source in body(reference / "db/pre-re/homunculus_db.yml"):
        stats = {row["Type"]: row for row in source["Status"]}
        homunculi.append({
            "class_id": classes[source["Class"]], "evolution_class": classes[source["EvolutionClass"]], "name": source["Name"],
            "food_item": items[source.get("Food", "Pet_Food")], "hungry_delay": source.get("HungryDelay", 60000),
            "attack_delay": source.get("AttackDelay", 700), "race": source.get("Race", "Demihuman"),
            "element": source.get("Element", "Neutral"), "size": source.get("Size", "Small"),
            "evolution_size": source.get("EvolutionSize", "Medium"),
            "base": [stats.get(name, {}).get("Base", 1) for name in ("Hp", "Sp", "Str", "Agi", "Vit", "Int", "Dex", "Luk")],
            "evolution_min": [stats.get(name, {}).get("EvolutionMinimum", 0) for name in ("Hp", "Sp", "Str", "Agi", "Vit", "Int", "Dex", "Luk")],
            "evolution_max": [stats.get(name, {}).get("EvolutionMaximum", 0) for name in ("Hp", "Sp", "Str", "Agi", "Vit", "Int", "Dex", "Luk")],
            "growth_min": [stats.get(name, {}).get("GrowthMinimum", 0) for name in ("Hp", "Sp", "Str", "Agi", "Vit", "Int", "Dex", "Luk")],
            "growth_max": [stats.get(name, {}).get("GrowthMaximum", 0) for name in ("Hp", "Sp", "Str", "Agi", "Vit", "Int", "Dex", "Luk")],
            "skills": [{"name": skill["Skill"], "max_level": skill["MaxLevel"],
                        "required_level": skill.get("RequiredLevel", 0), "required_intimacy": skill.get("RequiredIntimacy", 0) * 100,
                        "evolution": skill.get("RequireEvolution", False),
                        "required": [{"name": row["Skill"], "level": row["Level"]} for row in skill.get("Required", [])]}
                       for skill in source.get("SkillTree", [])],
        })
    guild_exp = [row["Exp"] for row in sorted(body(reference / "db/pre-re/exp_guild.yml"), key=lambda row: row["Level"])]
    hom_exp = [row["Exp"] for row in sorted(body(reference / "db/pre-re/exp_homun.yml"), key=lambda row: row["Level"])]
    return {"pets": pets, "mercenaries": mercenaries, "homunculi": homunculi, "guild_experience": guild_exp,
            "homunculus_experience": hom_exp, "client_frames": client_frames(reference, 20120229)}


def compile_pet_scripts(repository, sources, mobs):
    import hashlib
    dispatch = ["use script_sdk::{Context, Function, Value};"]
    definitions = []
    metadata = []
    compiled = {"bonus": [], "support": []}
    for source in sources:
        class_id = mobs[source["Mob"]]["id"]
        for key, kind in (("Script", "bonus"), ("SupportScript", "support")):
            text = source.get(key, "") or ""
            converter = Parser(text)
            code = converter.compile()
            if converter.programs:
                raise ValueError("Pet automatic bonus programs require an explicit pet program catalog")
            compiled[kind].append(f"{class_id} => pet_{kind}_{class_id}(ctx),")
            definitions.append(f"#[inline(never)]\n#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]\nfn pet_{kind}_{class_id}(ctx: &Context) -> Result<(), String> {{ {code}\nOk(()) }}")
            metadata.append({"class_id": class_id, "kind": kind, "source_hash": hashlib.md5(text.encode()).hexdigest(), "calls": sorted(converter.calls)})
    for kind in compiled:
        dispatch.append(f"pub fn run_{kind}(ctx: &Context, id: u32) -> Result<(), String> {{ match id {{")
        dispatch.extend(compiled[kind])
        dispatch.extend(['_ => Err(format!("Unknown pre-renewal pet class {id}")),', "} }"])
    (repository / "scripts/src/pets.rs").write_text("\n".join(dispatch + definitions) + "\n", encoding="utf-8")
    destination = repository / "config/wasm"
    destination.mkdir(exist_ok=True)
    (destination / "pets.json").write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--pet-overrides", type=Path)
    arguments = parser.parse_args()
    repository = Path(__file__).resolve().parents[2]
    output = repository / "server/src/server/service/script_world_data.json"
    data = import_data(repository, arguments.reference.resolve(), arguments.pet_overrides)
    output.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    print(f"Imported {len(data['pets'])} pets, {len(data['mercenaries'])} mercenaries and {len(data['homunculi'])} homunculus families")
