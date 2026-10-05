"""Convert the pre-renewal rAthena mob_skill_db.txt into config/mob_skills.json.

Usage: python import_mob_skills.py [path/to/mob_skill_db.txt] [output.json]
"""
import json
import sys
from pathlib import Path

DEFAULT_SOURCE = Path(r"C:\Users\Thoma\Documents\sources\rathena\db\pre-re\mob_skill_db.txt")
DEFAULT_OUTPUT = Path(__file__).resolve().parents[2] / "config" / "mob_skills.json"

FIELDS = [
    "mob_id", "label", "state", "skill_id", "level", "rate", "cast_time", "delay",
    "cancelable", "target", "condition", "condition_value",
]


def parse_line(line):
    parts = [part.strip() for part in line.split(",")]
    if len(parts) < 12:
        return None
    values = dict(zip(FIELDS, parts))
    extra = parts[12:17] + [""] * (5 - len(parts[12:17]))
    return {
        "mob_id": int(values["mob_id"]),
        "state": values["state"].lower(),
        "skill_id": int(values["skill_id"]),
        "level": int(values["level"]),
        "rate": int(values["rate"]),
        "cast_time": int(values["cast_time"] or 0),
        "delay": int(values["delay"] or 0),
        "cancelable": values["cancelable"].lower() == "yes",
        "target": values["target"].lower(),
        "condition": values["condition"].lower(),
        "condition_value": values["condition_value"].lower(),
        "values": extra[:5],
    }


def convert(source):
    entries = []
    for raw in source.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.split("//", 1)[0].strip()
        if not line:
            continue
        entry = parse_line(line)
        if entry is not None:
            entries.append(entry)
    return entries


def main():
    source = Path(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT_SOURCE
    output = Path(sys.argv[2]) if len(sys.argv) > 2 else DEFAULT_OUTPUT
    entries = convert(source)
    output.write_text(json.dumps(entries, separators=(",", ":")), encoding="utf-8")
    print(f"Wrote {len(entries)} mob skill entries to {output}")


if __name__ == "__main__":
    main()
