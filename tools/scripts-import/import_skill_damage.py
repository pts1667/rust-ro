import argparse
import json
import pathlib

from import_mapflags import CASTERS, ROOT

OUTPUT = ROOT / "server/src/server/script/skill_damage_adjustments.json"


def import_adjustments(source_root):
    metadata = json.loads((ROOT / "server/src/server/script/skill_metadata.json").read_text(encoding="utf-8"))
    skills = {skill["Name"]: skill["Id"] for skill in metadata}
    identifiers = set(skills.values())
    entries = {}
    skipped = 0
    paths = [source_root / "db/skill_damage_db.txt", source_root / "db/import/skill_damage_db.txt"]
    for path in paths:
        if not path.is_file():
            if path == paths[0]:
                raise ValueError(f"Missing skill damage database: {path}")
            continue
        for line_number, line in enumerate(path.read_text(encoding="utf-8-sig").splitlines(), 1):
            line = line.split("//", 1)[0].strip()
            if not line:
                continue
            fields = [field.strip() for field in line.split(",")]
            skill = skills.get(fields[0])
            if skill is None and fields[0].isdigit():
                skill = int(fields[0])
            if skill not in identifiers:
                skipped += 1
                continue
            try:
                if not 4 <= len(fields) <= 7:
                    raise ValueError("expected skill, caster, map mask, and one to four rates")
                caster = CASTERS.get(fields[1])
                if caster is None:
                    caster = int(fields[1])
                maps = int(fields[2])
                if not 0 <= caster <= CASTERS["BL_ALL"] or not 0 <= maps <= 4294967295:
                    raise ValueError("caster or map mask is out of range")
                rates = [max(-100, min(100000, int(value))) for value in fields[3:]]
                entries[skill] = {"skill_id": skill, "caster": caster, "maps": maps,
                                  "rates": [*rates, *([0] * (4 - len(rates)))]}
            except ValueError as error:
                raise ValueError(f"{path}:{line_number}: {error}") from error
    return [entries[key] for key in sorted(entries)], skipped


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-root", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, default=OUTPUT)
    args = parser.parse_args()
    entries, skipped = import_adjustments(args.source_root.resolve())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(entries, indent=2) + "\n", encoding="utf-8")
    print(f"Imported {len(entries)} classic skill damage adjustments; skipped {skipped} non-classic entries")


if __name__ == "__main__":
    main()
