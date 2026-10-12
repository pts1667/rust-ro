"""List where the NPCs of the Wasm script modules send characters, into config/npc_warps.json.

An NPC warps a character with `ctx.warp("map", x, y)` or `ctx.call(Function::Warp, ...)` in its function, the module's own
helpers it calls, or its events (the `"<npc>::<label>"` keys of the module's `events` table, `OnTouch` being the one a
character triggers by walking into the NPC's area). Only literal destinations are found: a warp to a computed map (the
warper, Kafra teleports through the shared library) is not listed.

`checks` names what the code that leads to the warps reads, a hint that the warp has a condition: `zeny`, `item`, `quest`,
`level`, `job` or `variable` (a character or account variable such as a quest step). It is a static scan: a listed warp
may sit behind a condition that is not named, and a named check may guard something else.

Output: { "<module>": { "<npc entry>": { "warps": [{"map", "x", "y"}], "touch": bool, "checks": [...] } } }, keyed by the
names of the module's `npcs` table (the `entry` of config/wasm/npcs.json).

Run as: python -I -X utf8 tools/scripts-import/extract_npc_warps.py
"""

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SCRIPTS = REPO / "scripts"
OUT = REPO / "config" / "npc_warps.json"

TABLE = re.compile(r"\b(npcs|events)\s*\{([^}]*)\}")
TABLE_ENTRY = re.compile(r'"((?:[^"\\]|\\.)*)"\s*=>\s*([A-Za-z_][\w:]*)')
FN = re.compile(r"\bfn\s+(\w+)\b[^{;]*\{")
CALL = re.compile(r"\b(\w+)\s*\(")
STRING = re.compile(r'"((?:[^"\\\n]|\\.)*)"')
# Strings are replaced by "S<index>" before matching, see `code_only`
NUMBER = r"(?:Val::from\(\s*)?(-?\d+)\s*\)?"
MAP = r'(?:Val::from\(\s*)?"S(\d+)"\s*\)?'
WARP = re.compile(r"\.warp\s*\(\s*" + MAP + r"\s*,\s*" + NUMBER + r"\s*,\s*" + NUMBER + r"\s*\)")
WARP_CALL = re.compile(r"Function::Warp\s*,\s*(?:vec!|args!)\s*\[\s*" + MAP + r"\s*,\s*" + NUMBER + r"\s*,\s*" + NUMBER + r"\s*\]")
VAR = re.compile(r'\bvar\s*\(\s*"S(\d+)"\s*\)')
CHECKS = {
    "item": re.compile(r"Function::(?:CountItem|Delitem)\b|\bcount_item\s*\(|\bdelete_item\s*\("),
    "quest": re.compile(r"\bquests\s*\(\s*\)|Function::Checkquest\b"),
}
VARIABLE_CHECKS = {"zeny": {"zeny"}, "level": {"baselevel", "joblevel"}, "job": {"class", "basejob", "baseclass", "upper"}}


def code_only(text, strings):
    """Source without comments, its strings replaced by `"S<index>"` into `strings`, so braces are only code's own."""
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    text = re.sub(r"//[^\n]*", " ", text)

    def keep(match):
        strings.append(match.group(1))
        return f'"S{len(strings) - 1}"'

    text = STRING.sub(keep, text)
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


def scan(body, strings):
    warps = [(strings[int(m.group(1))], int(m.group(2)), int(m.group(3))) for m in WARP.finditer(body)]
    warps += [(strings[int(m.group(1))], int(m.group(2)), int(m.group(3))) for m in WARP_CALL.finditer(body)]
    checks = {name for name, pattern in CHECKS.items() if pattern.search(body)}
    for match in VAR.finditer(body):
        name = strings[int(match.group(1))]
        # `@` and `.` variables are the script's own scratch space, `$` ones belong to the server
        if not name or name[0] in "@.$'":
            continue
        kind = next((kind for kind, names in VARIABLE_CHECKS.items() if name.lower() in names), "variable")
        checks.add(kind)
    return warps, checks


def reachable(root, bodies, strings):
    seen, todo, warps, checks = set(), [root], [], set()
    while todo:
        name = todo.pop()
        if name in seen or name not in bodies:
            continue
        seen.add(name)
        for body in bodies[name]:
            found, read = scan(body, strings)
            warps += found
            checks |= read
            todo.extend(call.group(1) for call in CALL.finditer(body) if call.group(1) in bodies)
    return warps, checks


def module_warps(module):
    lib = (module / "src" / "lib.rs").read_text(encoding="utf-8")
    tables = {name: [(json.loads(f'"{key}"'), target.split("::")[-1]) for key, target in TABLE_ENTRY.findall(body)] for name, body in TABLE.findall(lib)}
    strings, bodies = [], {}
    for path in sorted((module / "src").rglob("*.rs")):
        for name, found in functions(code_only(path.read_text(encoding="utf-8"), strings)).items():
            bodies.setdefault(name, []).extend(found)

    npc_names = {key for key, _ in tables.get("npcs", [])}
    result = {}

    def add(name, target, touch):
        warps, checks = reachable(target, bodies, strings)
        if not warps:
            return
        entry = result.setdefault(name, {"warps": [], "touch": False, "checks": set()})
        for map_name, x, y in warps:
            warp = {"map": map_name, "x": x, "y": y}
            if warp not in entry["warps"]:
                entry["warps"].append(warp)
        entry["touch"] |= touch
        entry["checks"] |= checks

    for key, target in tables.get("npcs", []):
        add(key, target, False)
    for key, target in tables.get("events", []):
        name, _, label = key.rpartition("::")
        if name in npc_names:
            add(name, target, label.startswith("OnTouch"))
    for entry in result.values():
        entry["checks"] = sorted(entry["checks"])
    return result


def main():
    result = {}
    for group in sorted(path for path in SCRIPTS.iterdir() if path.is_dir()):
        for module in sorted(path for path in group.iterdir() if (path / "src" / "lib.rs").exists()):
            found = module_warps(module)
            if found:
                result[module.name] = found

    OUT.write_text(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    npcs = sum(len(entries) for entries in result.values())
    warps = sum(len(entry["warps"]) for entries in result.values() for entry in entries.values())
    touch = sum(entry["touch"] for entries in result.values() for entry in entries.values())
    print(f"wrote {OUT.relative_to(REPO)}: {npcs} NPC entries ({touch} by touch) warp to {warps} places in {len(result)} modules")
    return 0


if __name__ == "__main__":
    sys.exit(main())
