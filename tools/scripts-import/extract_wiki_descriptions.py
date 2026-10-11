"""Copy the "## Description" section of each iRO Wiki page that matches a game entry into config/wiki_descriptions.json.

Items, monsters and statuses match a page by their English name, skills by the English name in config/skill.json
(its `description` field). A page without a "## Description" section is skipped. Relative links in the copied text
are rewritten to /wiki/<page>, the path the server serves the wiki under.

Run as: python -I -X utf8 tools/scripts-import/extract_wiki_descriptions.py
"""

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
WIKI = REPO / "wiki"
OUT = REPO / "config" / "wiki_descriptions.json"
HEADER = re.compile(r"^## Description[ \t]*$", re.M)
SECTION_END = re.compile(r"^(## |\*\*Related pages|---)", re.M)
LINK = re.compile(r"\]\((?!https?:|/|#|mailto:)([^)\s]+)\)")


def page_name(name):
    return name.strip().replace(" ", "_")


def description_of(path):
    text = path.read_text(encoding="utf-8", errors="replace")
    header = HEADER.search(text)
    if not header:
        return None
    rest = text[header.end():]
    end = SECTION_END.search(rest)
    body = (rest[: end.start()] if end else rest).strip()
    if not body:
        return None
    return LINK.sub(lambda match: f"](/wiki/{match.group(1)})", body)


def main():
    pages = {}
    for path in WIKI.glob("*.md"):
        text = description_of(path)
        if text is not None:
            pages[path.stem] = text

    data = json.loads((REPO / "config" / "items.json").read_text(encoding="utf-8"))
    items = {str(item["id"]): item["name_english"] for item in data["items"] if item.get("name_english")}
    data = json.loads((REPO / "config" / "mobs.json").read_text(encoding="utf-8"))
    mobs = {str(mob["id"]): mob["name_english"] for mob in data["mobs"] if mob.get("name_english")}
    data = json.loads((REPO / "config" / "skill.json").read_text(encoding="utf-8"))
    skills = {str(skill["id"]): skill["description"] for skill in data["skills"] if skill.get("description")}
    statuses = {}
    for status in json.loads((REPO / "config" / "status_effect.json").read_text(encoding="utf-8")):
        if status.get("Status"):
            statuses[status["Status"]] = status["Status"]

    result = {}
    for category, names in (("items", items), ("mobs", mobs), ("skills", skills), ("statuses", statuses)):
        matched = {}
        for key, name in names.items():
            page = page_name(name)
            if page in pages:
                matched[key] = {"page": f"{page}.md", "description": pages[page]}
        result[category] = dict(sorted(matched.items(), key=lambda pair: pair[0]))

    OUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    counts = ", ".join(f"{category} {len(entries)}" for category, entries in result.items())
    print(f"wrote {OUT.relative_to(REPO)}: {counts}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
