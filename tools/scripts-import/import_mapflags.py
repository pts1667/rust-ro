import argparse
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_SOURCE = ROOT / "config"
DEFAULT_OUTPUT = ROOT / "config/wasm/map_flags.json"
CASTERS = {"BL_PC": 1, "BL_MOB": 2, "BL_PET": 4, "BL_HOM": 8, "BL_MER": 16,
           "BL_ITEM": 32, "BL_SKILL": 64, "BL_NPC": 128, "BL_CHAT": 256, "BL_ELEM": 512,
           "BL_ALL": 4095, "BL_CHAR": 1 | 2 | 8 | 16 | 512}
DROP_LOCATIONS = {"inventory": 1, "equip": 2, "all": 3}


def parse_line(line, skills, known_flags):
    match = re.fullmatch(r"(\S+)\s+mapflag\s+(\S+)(?:\s+(.*))?", line.strip())
    if not match:
        return None
    map_name, flag, data = match.groups()
    flag = flag.lower()
    if flag not in known_flags:
        raise ValueError(f"Unsupported map flag {flag}")
    entry = {"map": map_name.removesuffix(".gat").lower(), "flag": flag}
    data = (data or "").strip()
    if data.lower() == "off":
        entry["enabled"] = False
        return entry
    fields = [field.strip() for field in data.split(",")]
    if flag == "nosave":
        if not data or data.lower() == "savepoint":
            entry["save"] = ["SavePoint", 0, 0]
        elif len(fields) == 3:
            entry["save"] = [fields[0].removesuffix(".gat"), int(fields[1]), int(fields[2])]
        else:
            raise ValueError("Invalid nosave destination")
    elif flag == "pvp_nightmaredrop":
        if len(fields) != 3:
            raise ValueError("Invalid nightmare drop")
        entry["arguments"] = [-1 if fields[0].lower() == "random" else int(fields[0]),
                              DROP_LOCATIONS[fields[1].lower()], int(fields[2])]
    elif flag == "skill_duration":
        if len(fields) != 2:
            raise ValueError("Invalid skill duration")
        entry["arguments"] = [skills[fields[0]], int(fields[1])]
    elif flag == "skill_damage":
        if not data:
            entry["arguments"] = [0, CASTERS["BL_ALL"], 0, 0, 0, 0]
        else:
            if not 3 <= len(fields) <= 6:
                raise ValueError("Invalid skill damage adjustment")
            skill = 0 if fields[0].lower() == "all" else skills[fields[0]]
            caster = CASTERS.get(fields[1])
            if caster is None:
                caster = int(fields[1])
            rates = [int(value) for value in fields[2:]]
            entry["arguments"] = [skill, caster or CASTERS["BL_ALL"], *rates, *([0] * (4 - len(rates)))]
    elif flag in {"bexp", "jexp", "nocommand", "restricted", "battleground", "invincible_time", "specialpopup"}:
        default = 1 if flag == "battleground" else 100 if flag == "nocommand" else 0
        if flag == "restricted" and not data:
            raise ValueError("Restricted map requires a zone")
        entry["arguments"] = [int(data) if data else default]
    elif data:
        raise ValueError(f"Unexpected arguments for {flag}: {data}")
    return entry


def import_manifest(source_root):
    source_root = pathlib.Path(source_root).resolve()
    skills = {row["Name"]: row["Id"] for row in json.loads(
        (ROOT / "server/src/server/script/skill_metadata.json").read_text(encoding="utf-8"))}
    known_flags = set(re.findall(r'=>\s*"([^"]+)"',
        (ROOT / "server/src/server/model/map_flags.rs").read_text(encoding="utf-8")))
    configurations = [source_root / "npc/scripts_mapflags.conf", source_root / "npc/pre-re/scripts_mapflags.conf"]
    entries = []
    for configuration in configurations:
        for declaration in configuration.read_text(encoding="utf-8-sig").splitlines():
            declaration = declaration.split("//", 1)[0].strip()
            if not declaration.startswith("npc:"):
                continue
            relative = pathlib.Path(declaration.split(":", 1)[1].strip())
            if relative.is_absolute() or ".." in relative.parts or "re" in relative.parts:
                raise ValueError(f"Invalid pre-renewal map flag source {relative}")
            path = source_root / relative
            text = re.sub(r"/\*.*?\*/", "", path.read_text(encoding="utf-8-sig"), flags=re.S)
            for line_number, line in enumerate(text.splitlines(), 1):
                line = line.split("//", 1)[0].strip()
                try:
                    entry = parse_line(line, skills, known_flags)
                except (ValueError, KeyError) as error:
                    raise ValueError(f"{path}:{line_number}: {error}") from error
                if entry is not None:
                    entries.append(entry)
    return entries


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-root", type=pathlib.Path, default=DEFAULT_SOURCE)
    parser.add_argument("--output", type=pathlib.Path, default=DEFAULT_OUTPUT)
    arguments = parser.parse_args()
    entries = import_manifest(arguments.source_root)
    arguments.output.parent.mkdir(parents=True, exist_ok=True)
    arguments.output.write_text(json.dumps(entries, indent=2) + "\n", encoding="utf-8")
    print(f"Imported {len(entries)} flags for {len({entry['map'] for entry in entries})} maps")


if __name__ == "__main__":
    main()
