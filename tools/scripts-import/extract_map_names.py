"""Copy the English map names of the client into config/map_names.json.

The source is a checkout of zackdreaver/ROenglishRE: its Ragnarok/data/mapnametable.txt (ROenglishPRE has no map name
table, and a map keeps its name across episodes). Each line is `<map>.rsw#<name>#`. The first line of a map wins: the
later duplicates are generic ("Prontera Field" for every prt_fild). Names the translation left in Korean are skipped.
Only maps of config/maps/pre-re are written.

Run as: python -I -X utf8 tools/scripts-import/extract_map_names.py <ROenglishRE checkout>
"""

import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
OUT = REPO / "config" / "map_names.json"


def parse(path):
    table = {}
    for line in path.read_bytes().splitlines():
        line = line.strip()
        if not line or line.startswith(b"//"):
            continue
        fields = line.split(b"#")
        if len(fields) < 2 or not fields[0] or not fields[1].strip():
            continue
        name = fields[1].strip()
        if not name.isascii():
            continue
        map_name = fields[0].decode("ascii").lower().removesuffix(".rsw").removesuffix(".gat")
        table.setdefault(map_name, name.decode("ascii"))
    return table


def main(argv):
    if len(argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    table = parse(Path(argv[1]) / "Ragnarok" / "data" / "mapnametable.txt")

    maps = sorted(path.stem for path in (REPO / "config" / "maps" / "pre-re").glob("*.mcache"))
    result = {map_name: table[map_name] for map_name in maps if map_name in table}

    OUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}: {len(result)} of {len(maps)} maps")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
