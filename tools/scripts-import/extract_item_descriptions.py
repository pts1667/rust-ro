"""Copy the English item descriptions of the pre-renewal client into config/item_descriptions.json.

The source is a checkout of zackdreaver/ROenglishPRE: its data/idnum2itemdesctable.txt (identified) and
data/num2itemdesctable.txt (unidentified). The tables are parsed the way the client reads them: the text is split
on '#', a number token is an item id and the token after it holds the description lines. Color codes (^RRGGBB,
^000000) and the divider lines are removed. Only ids that exist in config/items.json are written.

Run as: python -I -X utf8 tools/scripts-import/extract_item_descriptions.py <ROenglishPRE checkout>
"""

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
OUT = REPO / "config" / "item_descriptions.json"
COLOR = re.compile(r"\^[0-9A-Fa-f]{6}|\^000000")


def parse(path):
    table = {}
    current = None
    for token in path.read_text(encoding="utf-8").split("#"):
        trimmed = token.strip()
        if not trimmed:
            continue
        if trimmed.isdigit():
            current = int(trimmed)
        elif current is not None:
            for line in token.splitlines():
                text = COLOR.sub("", line).strip()
                if text and text != "_":
                    table.setdefault(current, []).append(text)
    return table


def main(argv):
    if len(argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    data = Path(argv[1]) / "data"
    identified = parse(data / "idnum2itemdesctable.txt")
    unidentified = parse(data / "num2itemdesctable.txt")

    items = json.loads((REPO / "config" / "items.json").read_text(encoding="utf-8"))["items"]
    result = {}
    for item in sorted(items, key=lambda item: item["id"]):
        item_id = item["id"]
        if item_id not in identified:
            continue
        result[str(item_id)] = {"identified": identified[item_id], "unidentified": unidentified.get(item_id, [])}

    OUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}: {len(result)} of {len(items)} items")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
