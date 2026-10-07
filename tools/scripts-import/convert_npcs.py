#!/usr/bin/env python3
"""Convert rathena NPC scripts to Rust modules of the `scripts` crate.

    python tools/scripts-import/convert_npcs.py --survey                 # what converts, what blocks
    python tools/scripts-import/convert_npcs.py --write npc/quests       # regenerate scripts/src/generated and the registries

Anything the converter does not understand blocks the NPC that uses it; blocked NPCs are listed, never approximated.
"""

import argparse
import collections
import json
import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from rathena_script.codegen import Context, generate_function, rust_string  # noqa: E402
from rathena_script.names import load_names  # noqa: E402
from rathena_script.parser import parse_file  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_SOURCES = ["npc/quests", "npc/instances", "npc/merchants/refine.txt", "npc/merchants/advanced_refiner.txt"]
LIBRARY_SOURCES = ["npc"]
LIBRARY_EXCLUDED = ("npc/re/", "npc/test/", "npc/custom/", "npc/quests/")
FIRST_NPC_ENTRY = 10_000
FIRST_EVENT_ENTRY = 100_000


def module_name(path, rathena):
    relative = path.relative_to(rathena / "npc").with_suffix("")
    return "m_" + re.sub(r"[^a-z0-9]+", "_", str(relative).lower()).strip("_")


def collect(rathena, sources):
    definitions = []
    for source in sources:
        base = rathena / source
        files = [base] if base.is_file() else sorted(base.rglob("*.txt"))
        for path in files:
            for definition in parse_file(path, str(path.relative_to(rathena)).replace("\\", "/")):
                definition.module = module_name(path, rathena)
                definitions.append(definition)
    return definitions


def collect_library(rathena, excluded_files):
    """`function script` definitions of every other NPC file, for `callfunc` targets outside the converted sources."""
    definitions = []
    for path in sorted((rathena / "npc").rglob("*.txt")):
        relative = str(path.relative_to(rathena)).replace("\\", "/")
        if relative.startswith(LIBRARY_EXCLUDED) or relative in excluded_files:
            continue
        if "function	script	" not in path.read_text(encoding="utf-8", errors="replace"):
            continue
        for definition in parse_file(path, relative):
            if definition.kind == "script" and not definition.error:
                continue
            definition.module = module_name(path, rathena)
            definitions.append(definition)
    return definitions


def convert(rathena, sources, library_sources):
    constants, parameters = load_names(rathena)
    definitions = collect(rathena, sources)
    library = collect_library(rathena, set()) if library_sources else []
    functions = [d for d in definitions + library if d.kind == "function" and not d.error]
    exnames = {d.exname: d.name for d in definitions if d.exname and d.place is not None and d.kind in ("script", "duplicate")}
    world = Context(constants, parameters, {}, exnames)
    for index, function in enumerate(functions):
        function.rust_name = f"fn_{index}_{re.sub(r'[^a-z0-9]+', '_', function.name.lower()).strip('_')}"
        function.convertible = True
    seen = {}
    for function in functions:
        seen.setdefault(function.name.lower(), function)
    scripts = [d for d in definitions if d.kind == "script" and not d.error]
    for index, script in enumerate(scripts):
        script.rust_name = f"npc_{FIRST_NPC_ENTRY + index}"
        script.entry_id = FIRST_NPC_ENTRY + index

    for _ in range(10):
        world.functions = {name: f"super::{function.module}::{function.rust_name}" for name, function in seen.items() if function.convertible}
        changed = False
        for function in functions:
            function.source, function.blockers, function.labels = generate_function(world, function.rust_name, function.body)
            if function.blockers and function.convertible:
                function.convertible = False
                changed = True
        if not changed:
            break
    world.functions = {name: f"super::{function.module}::{function.rust_name}" for name, function in seen.items() if function.convertible}
    for script in scripts:
        script.source, script.blockers, script.labels = generate_function(world, script.rust_name, script.body)
    return definitions, library, scripts, functions


def unique_names(scripts):
    """Scripts by the name `duplicate()` refers to: the unique name when there is one."""
    return {script.exname or script.name: script for script in scripts}


def report(definitions, scripts, functions):
    kinds = collections.Counter(d.kind for d in definitions)
    print("definitions:", dict(kinds))
    parse_errors = [d for d in definitions if d.error]
    print(f"parse errors: {len(parse_errors)}")
    for definition in parse_errors:
        print(f"  {definition.file}:{definition.line} {definition.name}: {definition.error}")
    good = [s for s in scripts if not s.blockers]
    print(f"scripts: {len(scripts)}  convertible: {len(good)}")
    by_name = unique_names(scripts)
    duplicates = [d for d in definitions if d.kind == "duplicate"]
    duplicate_ok = sum(1 for d in duplicates if d.source in by_name and not by_name[d.source].blockers)
    print(f"duplicates: {len(duplicates)}  convertible: {duplicate_ok}")
    print(f"functions: {len([f for f in functions])}  convertible: {len([f for f in functions if f.convertible])}")
    counts = collections.Counter()
    sole = collections.Counter()
    for script in scripts:
        copies = 1 + sum(1 for d in duplicates if d.source == (script.exname or script.name))
        for blocker in script.blockers:
            counts[blocker] += copies
        if len(script.blockers) == 1:
            sole[next(iter(script.blockers))] += copies
    for function in functions:
        if function.blockers and function.file.startswith("npc/quests"):
            print(f"blocked function {function.name}: {sorted(function.blockers)}")
    print("\nblockers (NPCs affected, duplicates included) / (NPCs that only this blocks):")
    for blocker, count in counts.most_common():
        print(f"  {count:5}  {sole[blocker]:5}  {blocker}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sources", nargs="*", default=DEFAULT_SOURCES)
    parser.add_argument("--rathena", default=str(ROOT.parent / "rathena"))
    parser.add_argument("--survey", action="store_true")
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--no-library", action="store_true")
    options = parser.parse_args()
    rathena = pathlib.Path(options.rathena)
    definitions, library, scripts, functions = convert(rathena, options.sources, [] if options.no_library else LIBRARY_SOURCES)
    report(definitions, scripts, functions)
    if options.write:
        from rathena_script.emit import write_outputs
        write_outputs(ROOT, definitions, scripts, functions)


if __name__ == "__main__":
    main()
