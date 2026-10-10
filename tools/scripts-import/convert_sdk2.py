#!/usr/bin/env python3
"""Lowers rathena NPC scripts to the readable script-sdk-2 modules under `scripts/`.

    python tools/scripts-import/convert_sdk2.py --survey     # what lowers, and why the rest does not

An NPC converts only when every construct in it lowers (see rathena_script/sdk2.py). `convert_npcs.py --write` calls
`lower_modules` and `write_modules`, so converted NPCs are routed to their module by name and the rest keep the legacy code.
"""

import argparse
import collections
import pathlib
import re
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from rathena_script.codegen import Context  # noqa: E402
from rathena_script.names import load_names  # noqa: E402
from rathena_script.parser import parse_file  # noqa: E402
from rathena_script.sdk2 import Unsupported, fn_name, lower_function, lower_script  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parents[2]
# sdk-2 module name -> rathena sources it covers (directories below the rathena `npc` tree). Together they are every active source.
MODULES = {
    "towns": ["npc/cities", "npc/airports", "npc/merchants", "npc/kafras", "npc/pre-re/cities", "npc/pre-re/merchants", "npc/pre-re/airports", "npc/pre-re/kafras"],
    "misc": ["npc/other", "npc/pre-re/other", "npc/pre-re/guides", "npc/instances", "npc/events"],
    "jobs": ["npc/jobs", "npc/pre-re/jobs"],
    "quests": ["npc/quests", "npc/pre-re/quests"],
}
MODULE_ROOTS = {module: ROOT / "scripts" / module for module in MODULES}
SHARED_ROOT = ROOT / "scripts" / "shared"
# Same rule as convert_npcs.py: `function script`s outside the converted sources that `callfunc` can still reach
LIBRARY_EXCLUDED = ("npc/re/", "npc/test/", "npc/custom/", "npc/quests/")


def source_files(rathena, sources):
    for source in sources:
        base = rathena / source
        for path in [base] if base.is_file() else sorted(base.rglob("*.txt")):
            yield path


def relative_path(rathena, path):
    return str(path.relative_to(rathena)).replace("\\", "/")


def flat_name(relative):
    """The shared-crate module of a rathena source: its path below `npc/`, with `/` and `-` as `_`."""
    return fn_name(relative[len("npc/"):-len(".txt")].replace("/", "_"))


def library_files(rathena, converted):
    for path in sorted((rathena / "npc").rglob("*.txt")):
        relative = relative_path(rathena, path)
        if not relative.startswith(LIBRARY_EXCLUDED) and relative not in converted:
            yield path, relative


def lower_library(sources, constants, parameters, exnames):
    """Lowers the `function script`s every module can call, into the shared crate.

    A function that does not lower is dropped, and so is every caller of it, until nothing else drops.
    Returns `(shared, callable)`: the lowered functions by source file, and each lower-case name that stays callable with its source.
    """
    library = {}
    for relative, definition in sources:
        if definition.kind == "function" and not definition.error and definition.name:
            library.setdefault(definition.name.lower(), (relative, definition))
    live = set(library)
    while True:
        world = Context(constants, parameters, {name: f"crate::{flat_name(library[name][0])}::{fn_name(name)}" for name in live}, exnames)
        lowered, failed = {}, set()
        for name in sorted(live):
            relative, definition = library[name]
            try:
                lowered[name] = (relative, lower_function(definition, world, fn_name(name)))
            except Unsupported:
                failed.add(name)
        if not failed:
            break
        live -= failed
    shared = collections.defaultdict(list)
    for name in sorted(lowered):
        relative, function = lowered[name]
        shared[relative].append(function)
    return shared, {name: library[name][0] for name in live}


def lower_modules(rathena):
    """Lowers every script of every sdk-2 module, and the shared functions they call.

    Returns `(routes, modules, blocked, shared)`. `routes` maps the `(file, line)` of each converted script definition to
    `(module, key, labels)`, where `key` is the name the guest dispatches it by and `labels` its event labels. `modules` maps a module to its scripts as
    `(relative source, key, Lowered)`, `blocked` counts the reasons scripts were left on the legacy code, and `shared` maps
    each `npc/` source to its lowered `function script`s.
    """
    constants, parameters = load_names(rathena)
    parsed, converted = [], set()
    for module, sources in MODULES.items():
        for path in source_files(rathena, sources):
            relative = relative_path(rathena, path)
            converted.add(relative)
            parsed += [(module, relative, definition) for definition in parse_file(path, relative)]
    library = []
    for path, relative in library_files(rathena, converted):
        library += [(relative, definition) for definition in parse_file(path, relative)]
    exnames = {d.exname: d.name for _, _, d in parsed if d.exname and d.place is not None and d.kind in ("script", "duplicate")}
    shared, callable = lower_library([(relative, d) for _, relative, d in parsed] + library, constants, parameters, exnames)
    world = Context(constants, parameters, {name: f"shared::{flat_name(relative)}::{fn_name(name)}" for name, relative in callable.items()}, exnames)
    routes, modules, blocked = {}, {}, collections.defaultdict(list)
    taken, bases = collections.defaultdict(set), collections.defaultdict(set)
    for module, relative, definition in parsed:
        # A template placed only by `duplicate` has no display name, just its `::` global name
        script_name = definition.name or definition.exname
        if definition.kind != "script" or definition.error or not script_name:
            continue
        # A name used by two scripts keeps its first key and function name; a later one is keyed and named by its line
        base = fn_name(script_name)
        if base in bases[(module, relative)]:
            base = f"{base}_l{definition.line}"
        bases[(module, relative)].add(base)
        try:
            lowered = lower_script(definition, world, base)
        except Unsupported as reason:
            blocked[str(reason)].append(f"{relative}:{definition.line} {definition.name}")
            continue
        key = script_name if script_name not in taken[module] else f"{script_name}#{flat_name(relative)}_{definition.line}"
        taken[module].add(key)
        routes[(definition.file, definition.line)] = (module, key, [label for label, _ in lowered.events])
        modules.setdefault(module, []).append((relative, key, lowered))
    return routes, modules, blocked, shared


GENERATED = "// Generated by tools/scripts-import/convert_sdk2.py"


def file_header(text, origin):
    """The `use` line and lint allowances of a generated file, with the script-sdk-2 items its text needs."""
    imports = ["args"] if "args!" in text else []
    imports += ["Ctx", "Function"] if "Function::" in text else ["Ctx"]
    imports += ["Script", "Stop"] if "Stop" in text else ["Script"]
    imports += ["Val"] if re.search(r"\bVal\b", text) else []
    imports += ["runtime"] if "runtime::" in text else []
    return (
        f"{GENERATED} from {origin}. Delete this line to stop the generator from overwriting the file.\n"
        "#![allow(unused_imports, unused_labels, unused_assignments, unused_mut, unused_parens, unused_variables, unreachable_code)]\n\n"
        f"use script_sdk_2::{{{', '.join(sorted(imports, key=str.lower))}}};\n\n"
    )


def render_shared(shared):
    """Rust sources of the shared crate as {path under src/: text}: one module per rathena source, and the crate root."""
    files, roots = {}, []
    for relative, functions in sorted(shared.items()):
        text = "\n\n".join(item for function in functions for item in function.items)
        files[f"{flat_name(relative)}.rs"] = file_header(text, relative) + text + "\n"
        roots.append(flat_name(relative))
    files["lib.rs"] = f"{GENERATED}: the `function script`s modules call through `callfunc`. Delete this line to stop the generator from overwriting the file.\n" + "".join(
        f"pub mod {root};\n" for root in roots
    )
    return files


def render_module(scripts):
    """Rust sources of one module as {path under src/: text}, and the `(key, path)` exports the crate root registers."""
    tree = collections.defaultdict(set)
    sources, exports = collections.OrderedDict(), []
    for relative, key, lowered in sorted(scripts, key=lambda item: item[0]):
        parts = relative[len("npc/"):-len(".txt")].split("/")
        directory, stem = [fn_name(part) for part in parts[:-1]], fn_name(parts[-1])
        for depth in range(len(directory)):
            tree["/".join(directory[:depth])].add(directory[depth])
        tree["/".join(directory)].add(stem)
        source = sources.setdefault("/".join(directory + [stem]), {"relative": relative, "functions": [], "used": set()})
        for name in (name for item in lowered.items for name in re.findall(r"^(?:pub )?fn (\w+)", item, re.MULTILINE)):
            if name in source["used"]:
                raise ValueError(f"{relative}: function {name} is generated twice")
            source["used"].add(name)
        source["functions"].append("\n\n".join(lowered.items))
        exports.append((key, "::".join(directory + [stem, lowered.npc])))
        for label, event_name in lowered.events:
            exports.append((f"{key}::{label}", "::".join(directory + [stem, event_name])))
    files = {}
    for path, source in sources.items():
        text = "\n\n".join(source["functions"])
        files[f"{path}.rs"] = file_header(text, source["relative"]) + text + "\n"
    for directory, children in tree.items():
        if directory:
            files[f"{directory}/mod.rs"] = f"{GENERATED}.\n" + "".join(f"pub mod {child};\n" for child in sorted(children))
    roots = sorted(child for child in tree[""])
    files["lib.rs"] = render_root(roots, exports)
    return files


def render_root(roots, exports):
    """The crate root: the modules, and the registry of NPC and event names the host dispatches by."""
    npcs = sorted((key, path) for key, path in exports if "::" not in key)
    events = sorted((key, path) for key, path in exports if "::" in key)
    lines = [f"{GENERATED}. Delete this line to stop the generator from overwriting the file."]
    lines += [f"pub mod {root};" for root in roots]
    lines += ["", "script_sdk_2::script_module! {", "    npcs {"]
    lines += [f'        "{key}" => {path},' for key, path in npcs]
    lines += ["    }", "    events {"]
    lines += [f'        "{key}" => {path},' for key, path in events]
    lines += ["    }", "}", ""]
    return "\n".join(lines)


CARGO_TEMPLATE = """[package]
name = "game-{module}"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
name = "{module}"
crate-type = ["cdylib"]

[dependencies]
script-sdk-2 = {{ path = "../../lib/script-sdk-2" }}
shared = {{ path = "../shared" }}

[profile.release]
opt-level = "s"
lto = true
panic = "abort"
strip = "debuginfo"
"""

SHARED_CARGO = """[package]
name = "shared"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
name = "shared"

[dependencies]
script-sdk-2 = { path = "../../lib/script-sdk-2" }
"""


def is_hand_written(path):
    """A file whose first line is not the generated header was edited by hand, and the generator leaves it alone."""
    with path.open(encoding="utf-8") as source:
        return not source.readline().startswith(GENERATED)


def write_tree(root, files):
    """Replaces the generated sources under `root/src` with `files`, formatted. Files without the generated header are kept as they are."""
    (root / "src").mkdir(parents=True, exist_ok=True)
    kept = {path for path in (root / "src").rglob("*.rs") if is_hand_written(path)}
    for stale in (root / "src").rglob("*.rs"):
        if stale not in kept:
            stale.unlink()
    skipped = []
    for relative, text in files.items():
        target = root / "src" / relative
        if target in kept:
            skipped.append(target)
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")
    generated = sorted(str(path) for path in (root / "src").rglob("*.rs") if path not in kept)
    if generated:
        subprocess.run(["rustfmt", "--edition", "2021", *generated], check=True)
    orphans = kept.difference(skipped)
    print(f"{root.name}: skipped {len(skipped)} hand-written files, kept {len(orphans)} more that nothing generates any more")
    for path in sorted(orphans):
        print(f"  not generated any more: {path.relative_to(root)}")


def write_modules(modules):
    """Writes the Rust sources of every module under `scripts/<module>/` and its Cargo manifest."""
    for module, scripts in modules.items():
        root = MODULE_ROOTS[module]
        write_tree(root, render_module(scripts))
        (root / "Cargo.toml").write_text(CARGO_TEMPLATE.format(module=module), encoding="utf-8")
        print(f"wrote scripts/{module}: {len(scripts)} scripts")


def write_shared(shared):
    """Writes the shared `function script` crate under `scripts/shared/`."""
    write_tree(SHARED_ROOT, render_shared(shared))
    (SHARED_ROOT / "Cargo.toml").write_text(SHARED_CARGO, encoding="utf-8")
    print(f"wrote scripts/shared: {sum(len(functions) for functions in shared.values())} functions")


def survey(rathena):
    _, modules, blocked, _ = lower_modules(rathena)
    converted = sum(len(scripts) for scripts in modules.values())
    print(f"converted: {converted}  blocked: {sum(len(names) for names in blocked.values())}")
    for reason, names in sorted(blocked.items(), key=lambda item: -len(item[1]))[:40]:
        print(f"  {len(names):4}  {reason}")
        for name in names[:12]:
            print(f"        {name}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", default=str(ROOT.parent / "rathena"))
    parser.add_argument("--survey", action="store_true")
    parser.add_argument("--write", action="store_true")
    options = parser.parse_args()
    if options.survey:
        survey(pathlib.Path(options.rathena))
    if options.write:
        _, modules, _, shared = lower_modules(pathlib.Path(options.rathena))
        write_shared(shared)
        write_modules(modules)


if __name__ == "__main__":
    main()
