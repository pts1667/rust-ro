#!/usr/bin/env python3
"""Lowers the active rathena NPC scripts to the script-sdk-2 modules under `scripts/` and writes the NPC and event manifests.

    python tools/scripts-import/convert_npcs.py --survey   # what lowers, and why the rest does not
    python tools/scripts-import/convert_npcs.py --write    # regenerate scripts/<module>/src, scripts/shared and the manifests

The scripts are hand-maintained after a `--write`: it replaces the generated sources, so run it only to import new rathena data.
Anything the lowering does not understand blocks the NPC that uses it; blocked NPCs are listed, never approximated.
"""

import argparse
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from convert_sdk2 import lower_modules, survey, write_modules, write_shared  # noqa: E402
from rathena_script.emit import write_registries  # noqa: E402
from rathena_script.parser import parse_file  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parents[2]
PRE_RE_CONF = "npc/pre-re/scripts_main.conf"
# Parts of the active pre-renewal chain that are Rust by hand (battlegrounds, castles: `castle_service` replaces the War of Emperium scripts of
# `npc/guild`) or have their own importers (spawns, warps, map flags).
NOT_CONVERTED = ("npc/battleground/", "npc/guild/", "npc/guild2/", "npc/mobs/", "npc/pre-re/mobs/", "npc/warps/", "npc/pre-re/warps/", "npc/mapflag/", "npc/pre-re/mapflag/", "npc/other/marriage.txt", "npc/other/divorce.txt")


def conf_files(rathena, conf=PRE_RE_CONF):
    """Script files of a rathena `scripts_*.conf` chain in load order: `npc:` adds a file, `import:` follows another conf, `delnpc:` removes a file again."""
    files = []
    for line in (rathena / conf).read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("//"):
            continue
        kind, _, path = line.partition(":")
        path = path.strip()
        if kind == "import" and "scripts_custom" not in path:
            files += [file for file in conf_files(rathena, path) if file not in files]
        elif kind == "npc" and path not in files:
            files.append(path)
        elif kind == "delnpc" and path in files:
            files.remove(path)
    return files


def active_sources(rathena):
    return [path for path in conf_files(rathena) if not path.startswith(NOT_CONVERTED)]


def collect(rathena, sources):
    definitions = []
    for source in sources:
        base = rathena / source
        files = [base] if base.is_file() else sorted(base.rglob("*.txt"))
        for path in files:
            definitions += parse_file(path, str(path.relative_to(rathena)).replace("\\", "/"))
    return definitions


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", default=str(ROOT.parent / "rathena"))
    parser.add_argument("--survey", action="store_true")
    parser.add_argument("--write", action="store_true")
    options = parser.parse_args()
    rathena = pathlib.Path(options.rathena)
    if options.survey or not options.write:
        survey(rathena)
    if options.write:
        routes, modules, _, shared = lower_modules(rathena)
        write_shared(shared)
        write_modules(modules)
        write_registries(ROOT, collect(rathena, active_sources(rathena)), routes)


if __name__ == "__main__":
    main()
