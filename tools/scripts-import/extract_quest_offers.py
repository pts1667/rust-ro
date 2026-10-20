"""List the quests each NPC of the Wasm script modules starts, into config/quest_offers.json.

A quest is offered by the NPC whose function starts it (`quests().start(id)`, or the raw
`ctx.call(Function::SetQuest, vec![Val::from(id)])` / `args![id]`) or hands it over by replacing another
(`quests().change(old, new)`, the new id). The function's helpers are followed, and the events of an NPC (the
`"<npc>::<label>"` keys of the module's `events` table) count for that NPC. Raw `Function::ChangeQuest` calls are left out:
the converted scripts use them for every step of a chain, which would mark 317 more NPCs (doors, warps, examiners) as givers. A function that starts the quest
named by one of its arguments (`runtime::arg(&args, n, ..)`, the Merchant's serial step) offers the literal its callers pass
at that place of their `args![..]`. Output:
{ "<module>": { "<npc name>": [quest ids] } }, the names the module's `npcs` table uses.

Run as: python -I -X utf8 tools/scripts-import/extract_quest_offers.py
"""

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SCRIPTS = REPO / "scripts"
CATALOG = REPO / "server" / "src" / "server" / "script" / "quests.json"
OUT = REPO / "config" / "quest_offers.json"

TABLE = re.compile(r"\b(npcs|events)\s*\{([^}]*)\}")
TABLE_ENTRY = re.compile(r'"((?:[^"\\]|\\.)*)"\s*=>\s*([A-Za-z_][\w:]*)')
FN = re.compile(r"\bfn\s+(\w+)\b[^{;]*\{")
CALL = re.compile(r"\b(\w+)\s*\(")
START = re.compile(r"quests\(\)\s*\.\s*start\s*\(\s*(\d+)\s*\)")
CHANGE = re.compile(r"quests\(\)\s*\.\s*change\s*\(\s*\d+\s*,\s*(\d+)\s*\)")
LITERAL = r"(?:Val::from\(\s*(\d+)\s*\)|(\d+))"
RAW_START = re.compile(rf"Function::SetQuest\s*,\s*(?:vec|args)!\[\s*{LITERAL}\s*\]")
ARG_START = re.compile(r"Function::SetQuest\s*,\s*(?:vec|args)!\[\s*runtime::arg\(\s*&args\s*,\s*(\d+)\s*,")


def code_only(text):
    """Source without comments, string contents or char literals, so braces and parentheses are only code's own."""
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    text = re.sub(r"//[^\n]*", " ", text)
    text = re.sub(r'"(?:[^"\\\n]|\\.)*"', '""', text)
    return re.sub(r"'(?:[^'\\\n]|\\.)'", "' '", text)


def functions(code):
    """Each function's body, by name, found by matching its braces."""
    bodies = {}
    for match in FN.finditer(code):
        depth, end = 0, None
        for index in range(match.end() - 1, len(code)):
            if code[index] == "{":
                depth += 1
            elif code[index] == "}":
                depth -= 1
                if depth == 0:
                    end = index
                    break
        if end is not None:
            bodies.setdefault(match.group(1), []).append(code[match.end() : end])
    return bodies


def quest_ids(body, arg_starts):
    ids = {int(match.group(1)) for match in START.finditer(body)}
    ids |= {int(match.group(1)) for match in CHANGE.finditer(body)}
    ids |= {int(match.group(1) or match.group(2)) for match in RAW_START.finditer(body)}
    for name, places in arg_starts.items():
        for match in re.finditer(rf"\b{name}\s*\([^;]*?args!\[([^\]]*)\]", body):
            values = [value.strip() for value in match.group(1).split(",")]
            ids |= {int(values[place]) for place in places if place < len(values) and values[place].isdigit()}
    return ids


def reachable(root, bodies, arg_starts):
    seen, todo, found = set(), [root], set()
    while todo:
        name = todo.pop()
        if name in seen or name not in bodies:
            continue
        seen.add(name)
        for body in bodies[name]:
            found |= quest_ids(body, arg_starts)
            todo.extend(call.group(1) for call in CALL.finditer(body) if call.group(1) in bodies)
    return found


def module_offers(module):
    lib = (module / "src" / "lib.rs").read_text(encoding="utf-8")
    tables = {name: [(json.loads(f'"{key}"'), target.split("::")[-1]) for key, target in TABLE_ENTRY.findall(body)] for name, body in TABLE.findall(lib)}
    bodies = {}
    for path in sorted((module / "src").rglob("*.rs")):
        for name, found in functions(code_only(path.read_text(encoding="utf-8"))).items():
            bodies.setdefault(name, []).extend(found)

    arg_starts = {name: {int(match.group(1)) for body in found for match in ARG_START.finditer(body)} for name, found in bodies.items()}
    arg_starts = {name: places for name, places in arg_starts.items() if places}

    npc_names = {key for key, _ in tables.get("npcs", [])}
    offers = {}
    for key, target in tables.get("npcs", []):
        ids = reachable(target, bodies, arg_starts)
        if ids:
            offers.setdefault(key, set()).update(ids)
    for key, target in tables.get("events", []):
        name, _, _ = key.rpartition("::")
        ids = reachable(target, bodies, arg_starts)
        if ids and name in npc_names:
            offers.setdefault(name, set()).update(ids)
    return {name: sorted(ids) for name, ids in offers.items()}, npc_names


def main():
    catalog = {quest["id"] for quest in json.loads(CATALOG.read_text(encoding="utf-8"))}
    result, offered, total_npcs, unknown = {}, set(), 0, set()
    for group in sorted(path for path in SCRIPTS.iterdir() if path.is_dir()):
        for module in sorted(path for path in group.iterdir() if (path / "src" / "lib.rs").exists()):
            offers, npc_names = module_offers(module)
            total_npcs += len(npc_names)
            if offers:
                result[module.name] = offers
            for ids in offers.values():
                offered.update(ids)
                unknown.update(i for i in ids if i not in catalog)

    OUT.write_text(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    npc_count = sum(len(names) for names in result.values())
    print(f"wrote {OUT.relative_to(REPO)}: {npc_count} NPC names offer {len(offered)} quests in {len(result)} modules (of {total_npcs} NPC names)")
    if unknown:
        print(f"warning: {len(unknown)} offered quest ids are not in the quest catalog: {sorted(unknown)[:20]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
