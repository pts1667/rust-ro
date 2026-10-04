import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
NPC_ROOT = ROOT / "config/npc"
KINDS = {"Test": 1, "Global Variable Test": 2, "Warper": 3, "Stylist#custom_stylist": 4, "Job Master": 5, "ShopTemplate": 6}


def import_manifest():
    paths = [line.split(":", 1)[1].strip() for line in (NPC_ROOT / "scripts_custom.conf").read_text().splitlines() if line.startswith("npc:")]
    npcs = []
    for path in paths:
        for line in (NPC_ROOT / path).read_text(encoding="utf-8").splitlines():
            match = re.match(r"^([^\t]+)\t+(script|duplicate\([^)]+\)|shop)\t+([^\t]+)\t+(.+)", line)
            if not match:
                continue
            placement, form, name, details = match.groups()
            if "," not in placement:
                continue
            location = placement.split(",")
            sprite, _, remaining = details.partition(",")
            source_kind = form[10:-1] if form.startswith("duplicate(") else name
            kind = 6 if form == "shop" else KINDS.get(source_kind)
            if not kind:
                raise ValueError(f"NPC {name} has no Rust implementation")
            arguments = []
            if form == "shop":
                arguments.append({"Number": 0})
                for specification in remaining.split(","):
                    pairs = specification.split(":")
                    if len(pairs) % 2:
                        raise ValueError(f"Invalid shop item {specification}")
                    for index in range(0, len(pairs), 2):
                        arguments += [{"Number": int(pairs[index])}, {"Number": int(pairs[index + 1])}]
            npcs.append(dict(map_name=location[0], x=int(location[1]), y=int(location[2]), dir=int(location[3]), name=name.strip(), sprite=sprite.strip().split("//")[0].strip(), entry_id=kind, constructor_args=arguments))
    return npcs


def import_warps():
    source = (NPC_ROOT / "pre-re/custom/warper.txt").read_text(encoding="utf-8")
    source = re.sub(r"//[^\n]*", "", source)
    labels = list(re.finditer(r"^([TFDCGIS]\d+):", source, re.M))
    groups = {"T": "Towns", "F": "Fields", "D": "Dungeons", "C": "Guild Castles", "G": "Guild Dungeons", "I": "Instances", "S": "Special Areas"}
    menu_names = {label: name for name, label in re.findall(r'"([^"\n]+)"\s*,\s*([TFDCGIS]\d+)', source)}
    destinations = []
    for position, match in enumerate(labels):
        label = match[1]
        block = source[match.end():labels[position+1].start() if position+1 < len(labels) else source.index("}\n", match.end())]
        restriction = re.search(r'Restrict\("(RE|Pre-RE)"([^)]*)\)', block)
        restricted = set()
        if restriction and restriction[1] == "RE":
            restricted = {int(n) for n in re.findall(r"\d+", restriction[2])}
            if not restricted:
                continue
        go = re.search(r'Go\("([^"\n]+)",(\d+),([^;]+)\);', block)
        if go:
            y = 114 if "RENEWAL" in go[3] else int(go[3])
            destinations.append(dict(category=groups[label[0]], group=menu_names[label], name=menu_names[label], map=go[1], x=int(go[2]), y=y))
            continue
        coordinates = re.search(r"setarray @c\[(\d+)\],([^;]+);", block)
        display = re.search(r'Disp\("([^"\n]+)"([^)]*)\)', block)
        pick = re.search(r'Pick\(([^)]*)\)', block)
        if not coordinates or not display or not pick:
            raise ValueError(f"Cannot convert warp {label}")
        values = [0] * int(coordinates[1]) + [int(v.strip()) for v in coordinates[2].split(",")]
        range_arguments = [int(v) for v in re.findall(r"\d+", display[2])]
        names = [f"{display[1]} {i}" for i in range(range_arguments[0], range_arguments[1]+1)] if range_arguments else display[1].split(":")
        maps = re.findall(r'"([^"\n]*)"', pick[1])
        offset_match = re.search(r",(\d+)\s*$", pick[1])
        offset = int(offset_match[1]) if offset_match else 0
        for index, name in enumerate(names, 1):
            if index in restricted:
                continue
            map_index = index - offset if maps[0] else index
            map_name = f"{maps[0]}{map_index:02}" if maps[0] else maps[index]
            destinations.append(dict(category=groups[label[0]], group=menu_names[label], name=name, map=map_name, x=values[map_index*2], y=values[map_index*2+1]))
    return destinations


def main():
    npcs = import_manifest()
    warps = import_warps()
    destination = ROOT / "config/wasm"
    destination.mkdir(exist_ok=True)
    for name, value in [("npcs", npcs), ("warps", warps)]:
        (destination / f"{name}.json").write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    print(f"Converted {len(npcs)} NPC placements and {len(warps)} pre-renewal destinations")


if __name__ == "__main__":
    main()
