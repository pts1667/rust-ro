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


BREEDERS = [
    ("prontera", 55, 350, 5, "Peco Peco Breeder#knt", "Peco Peco Breeder", "riding", 7, 2500, "Knight"),
    ("prontera", 232, 318, 3, "Peco Peco Breeder#cru", "Peco Peco Breeder", "riding", 14, 3500, "Crusader"),
    ("hu_in01", 381, 304, 5, "Falcon Breeder#hnt", "Falcon Breeder", "falcon", 11, 2500, "Hunter"),
]


def builtin_npcs():
    npcs = [dict(map_name="prontera", x=151, y=193, dir=6, name="Mount Master", sprite="123", entry_id=7, constructor_args=[])]
    npcs += [
        dict(map_name=map_name, x=x, y=y, dir=direction, name=name, sprite="105", entry_id=14,
             constructor_args=[{"String": title}, {"String": kind}, {"Number": job}, {"Number": price}, {"String": job_name}])
        for map_name, x, y, direction, name, title, kind, job, price, job_name in BREEDERS
    ]
    npcs += [
        dict(map_name="prt_church", x=97, y=100, dir=4, name="Wedding Staff#w", sprite="71", entry_id=11, constructor_args=[]),
        dict(map_name="prt_church", x=100, y=128, dir=4, name="Bishop#w", sprite="60", entry_id=12, constructor_args=[]),
        dict(map_name="nif_in", x=190, y=112, dir=5, name="Deviruchi#divorce", sprite="738", entry_id=13, constructor_args=[]),
    ]
    castles = json.loads((ROOT / "server/src/server/script/castles.json").read_text(encoding="utf-8"))
    text = lambda value: {"String": value}
    number = lambda value: {"Number": value}
    for castle in castles:
        map_name = castle["map"]
        steward, kafra, dungeon, treasure = castle["steward"], castle["kafra"], castle["dungeon_lever"], castle["treasure_lever"]
        slots = [number(value) for guardian in castle["guardians"] for value in (guardian["type"], guardian["x"], guardian["y"])]
        npcs.append(dict(map_name=map_name, x=steward["x"], y=steward["y"], dir=steward["dir"], name=steward["name"], sprite="55", entry_id=8,
                         constructor_args=[text(map_name), text(steward["name"]), number(castle["master_room"][0]), number(castle["master_room"][1])] + slots))
        npcs.append(dict(map_name=map_name, x=kafra["x"], y=kafra["y"], dir=kafra["dir"], name=kafra["name"], sprite="117", entry_id=10,
                         constructor_args=[text(map_name), text(kafra["destination"][0]), text(kafra["destination"][1]), number(kafra["destination"][2]), number(kafra["destination"][3])]))
        npcs.append(dict(map_name=map_name, x=dungeon["x"], y=dungeon["y"], dir=dungeon["dir"], name=dungeon["name"], sprite="111", entry_id=9,
                         constructor_args=[text(map_name), text(dungeon["destination"][0]), number(dungeon["destination"][1]), number(dungeon["destination"][2]), number(1)]))
        npcs.append(dict(map_name=map_name, x=treasure["x"], y=treasure["y"], dir=treasure["dir"], name=treasure["name"], sprite="111", entry_id=9,
                         constructor_args=[text(map_name), text(map_name), number(treasure["exit"][0]), number(treasure["exit"][1]), number(0)]))
        flags = castle["flags"]
        for kind, group in enumerate(("outside", "inside")):
            for flag in flags[group]:
                npcs.append(dict(map_name=flag["map"], x=flag["x"], y=flag["y"], dir=flag["dir"], name=flag["name"], sprite="722", entry_id=19,
                                 constructor_args=[text(map_name), number(flags["return"][0]), number(flags["return"][1]), number(kind)]))
    return npcs


ARENA_EVENT_BASE = 1000
ARENAS = [dict(map="bat_b01", short="b01")]
ARENA_FLAGS = [
    ("Guillaume Camp", 973, [(81, 83), (94, 83), (81, 66), (94, 66), (139, 142), (139, 158), (110, 161), (110, 137), (63, 135), (63, 165), (10, 296)]),
    ("Croix Camp", 974, [(306, 233), (317, 233), (306, 216), (317, 216), (257, 158), (257, 141), (297, 164), (297, 136), (336, 161), (336, 139), (389, 16)]),
]
ROLE_DECORATION, ROLE_THERAPIST, ROLE_VINTENAR, ROLE_VINTENAR_OVER = 0, 1, 2, 4


def arena_events(position, arena):
    """Event ids must match `scripts/src/battleground_arena.rs::event`."""
    m, s = arena["map"], arena["short"]
    names = {
        0: f"start#{m}::OnInit", 1: f"start#{m}::OnReadyCheck", 2: f"start#{m}::OnReset",
        3: f"start#{m}::OnGuillaumeActive", 4: f"start#{m}::OnCroixActive",
        10: f"OBJ#{m}_a::OnMyMobDead", 11: f"OBJ#{m}_b::OnMyMobDead",
        12: f"guardian#{m}_a::OnMyMobDead", 13: f"guardian#{m}_b::OnMyMobDead",
        20: f"Battle Therapist#{s}_a::OnTimer25000", 21: f"Battle Therapist#{s}_a::OnTimer26500",
        22: f"Battle Therapist#{s}_b::OnTimer25000", 23: f"Battle Therapist#{s}_b::OnTimer26500",
        30: f"Guillaume Vintenar#{s}_a::OnInit", 31: f"Croix Vintenar#{s}_b::OnInit",
        32: f"Vintenar#{m}_aover::OnInit", 33: f"Vintenar#{m}_bover::OnInit",
        50: f"#{m}_timer::OnTimer1000", 51: f"start#{m}::OnTimer10000",
    }
    for kind, delay in enumerate([7000, 8000, 1800000, 1803000, 1808000, 1822000, 1825000, 1830000, 1900000], 40):
        names[kind] = f"countdown#{m}::OnTimer{delay}"
    return {name: ARENA_EVENT_BASE + 100 * position + kind for kind, name in names.items()}


def battleground_arena_npcs():
    number = lambda value: {"Number": value}
    npcs = []
    for position, arena in enumerate(ARENAS):
        m, s = arena["map"], arena["short"]
        place = lambda map_name, x, y, name, sprite, role=ROLE_DECORATION, direction=3: npcs.append(dict(
            map_name=map_name, x=x, y=y, dir=direction, name=name, sprite=str(sprite), entry_id=15,
            constructor_args=[number(position), number(role)]))
        place(m, 15, 15, f"start#{m}", 844)
        place(m, 1, 1, f"OBJ#{m}_a", 844)
        place(m, 1, 2, f"OBJ#{m}_b", 844)
        place(m, 1, 3, f"guardian#{m}_a", 844)
        place(m, 1, 3, f"guardian#{m}_b", 844)
        place(m, 1, 5, f"countdown#{m}", 844)
        place("bat_room", 2, 151, f"#{m}_timer", 844)
        place(m, 10, 294, f"Battle Therapist#{s}_a", 95, ROLE_THERAPIST)
        place(m, 389, 14, f"Battle Therapist#{s}_b", 95, ROLE_THERAPIST)
        place(m, 10, 294, f"Guillaume Vintenar#{s}_a", 934, ROLE_VINTENAR)
        place(m, 389, 14, f"Croix Vintenar#{s}_b", 934, ROLE_VINTENAR + 1)
        place(m, 10, 294, f"Vintenar#{m}_aover", 419, ROLE_VINTENAR_OVER)
        place(m, 389, 14, f"Vintenar#{m}_bover", 415, ROLE_VINTENAR_OVER + 1)
        for team, sprite, points in ARENA_FLAGS:
            for number_suffix, (x, y) in enumerate(points, 21):
                place(m, x, y, f"{team}#flag{number_suffix}", sprite)
    return npcs


KVM_EVENT_BASE = 2000
KVM_ARENAS = [dict(map="bat_c01", short="KvM01", officer="KVM01", officer_places=[(51, 130, 5, 419), (148, 53, 1, 415)])]
KVM_HOST_TIMERS = [1000, 3000, 6000, 30000, 45000, 50000, 55000, 59000, 61000, 300000, 330000, 345000, 350000, 355000, 360000]
KVM_OUT_TIMERS = [1000, 3000, 5000, 55000, 60000]


def kvm_events(position, arena):
    """Event ids must match `scripts/src/battleground_kvm.rs::event`."""
    host, out = f"{arena['short']}_BG", f"{arena['short']}_BG_Out"
    names = {
        0: f"{host}::OnInit",
        1: f"{host}::OnGuillaumeDie", 2: f"{host}::OnCroixDie",
        3: f"{host}::OnGuillaumeActive", 4: f"{host}::OnCroixActive",
        5: f"{host}::OnStart", 30: f"{out}::OnBegin",
    }
    aliases = {f"{host}::OnGuillaumeQuit": 1, f"{host}::OnCroixQuit": 2}
    for kind, delay in enumerate(KVM_HOST_TIMERS, 10):
        names[kind] = f"{host}::OnTimer{delay}"
    for kind, delay in enumerate(KVM_OUT_TIMERS, 31):
        names[kind] = f"{out}::OnTimer{delay}"
    events = {name: KVM_EVENT_BASE + 100 * position + kind for kind, name in names.items()}
    events.update({name: KVM_EVENT_BASE + 100 * position + kind for name, kind in aliases.items()})
    return events


def kvm_arena_npcs():
    number = lambda value: {"Number": value}
    npcs = []
    for position, arena in enumerate(KVM_ARENAS):
        place = lambda map_name, x, y, name, sprite, role, direction=3: npcs.append(dict(
            map_name=map_name, x=x, y=y, dir=direction, name=name, sprite=str(sprite), entry_id=16,
            constructor_args=[number(position), number(role)]))
        place(arena["map"], 52, 131, f"{arena['short']}_BG", 844, 0)
        place(arena["map"], 52, 132, f"{arena['short']}_BG_Out", 844, 0)
        for camp, (x, y, direction, sprite) in enumerate(arena["officer_places"]):
            place(arena["map"], x, y, f"KVM Officer#{arena['officer']}{'AB'[camp]}", sprite, 1 + camp, direction)
    return npcs


TIERRA_EVENT_BASE = 3000
TIERRA_MAP = "bat_a01"
TIERRA_FLAGS = [
    ("Guillaume Camp", 973, [(171, 309), (149, 310), (119, 336), (118, 357), (150, 380), (173, 380), (210, 344), (350, 325), (358, 325)]),
    ("Croix Camp", 974, [(138, 12), (108, 36), (108, 63), (136, 87), (167, 86), (199, 49), (168, 16), (357, 74), (348, 74)]),
]
TIERRA_COUNTDOWN_TIMERS = [7000, 8000, 1800000, 1803000, 1808000, 1822000, 1825000, 1830000]


def tierra_events():
    """Event ids must match `scripts/src/battleground_tierra.rs::event`."""
    m = TIERRA_MAP
    names = {
        0: f"start#{m}::OnInit", 1: f"start#{m}::OnReadyCheck",
        2: f"start#{m}::OnGuillaumeActive", 3: f"start#{m}::OnCroixActive", 5: f"start#{m}::OnTimer10000",
        10: f"OBJ#{m}_a::OnMyMobDead", 11: f"OBJ#{m}_b::OnMyMobDead", 12: f"OBJ#{m}_n::OnMyMobDead",
        14: f"barricade#{m}_a::OnMyMobDead", 15: f"barricade#{m}_b::OnMyMobDead",
        20: "Battle Therapist#a01_a::OnTimer25000", 21: "Battle Therapist#a01_a::OnTimer26500",
        22: "Battle Therapist#a01_b::OnTimer25000", 23: "Battle Therapist#a01_b::OnTimer26500",
        24: "Valley Ghost#bat_a01_n::OnTimer25000", 25: "Valley Ghost#bat_a01_n::OnTimer26500",
        30: "Guillaume Vintenar#a01_a::OnInit", 31: "Croix Vintenar#a01_b::OnInit",
        32: "Guillaume Blacksmith#a01::OnInit", 33: "Croix Blacksmith#bat_a01::OnInit",
        50: f"#{m}_timer::OnTimer1000",
        60: f"barri_warp_up#{m}_a::OnTouch", 61: f"barri_warp_down#{m}a::OnTouch",
        62: f"barri_warp_up#{m}_b::OnTouch", 63: f"barri_warp_down#{m}b::OnTouch",
    }
    for kind, delay in enumerate(TIERRA_COUNTDOWN_TIMERS, 40):
        names[kind] = f"countdown#{m}::OnTimer{delay}"
    return {name: TIERRA_EVENT_BASE + kind for kind, name in names.items()}


def tierra_npcs():
    number = lambda value: {"Number": value}
    npcs = []
    m = TIERRA_MAP
    def place(map_name, x, y, name, sprite, role=0, direction=3, touch=None):
        npc = dict(map_name=map_name, x=x, y=y, dir=direction, name=name, sprite=str(sprite), entry_id=17,
                   constructor_args=[number(0), number(role)])
        if touch:
            npc["x_size"], npc["y_size"] = touch
        npcs.append(npc)
    for offset, name in enumerate([f"start#{m}", f"OBJ#{m}_a", f"OBJ#{m}_b", f"barricade#{m}_a", f"barricade#{m}_b", f"OBJ#{m}_n"]):
        place(m, 15, 15 + offset, name, 844)
    place(m, 1, 5, f"countdown#{m}", 844)
    place("bat_room", 1, 151, f"#{m}_timer", 844)
    place(m, 53, 377, "Battle Therapist#a01_a", 95, 1)
    place(m, 45, 19, "Battle Therapist#a01_b", 95, 1)
    place(m, 60, 216, "Valley Ghost#bat_a01_n", 950, 2)
    place(m, 53, 377, "Guillaume Vintenar#a01_a", 419, 3)
    place(m, 45, 19, "Croix Vintenar#a01_b", 415, 4)
    place(m, 185, 270, "Guillaume Blacksmith#a01", 851, 5, 1)
    place(m, 170, 121, "Croix Blacksmith#bat_a01", 851, 6, 5)
    for x, y, name in [(194, 267, f"barri_warp_up#{m}_a"), (194, 265, f"barri_warp_down#{m}a"),
                       (177, 130, f"barri_warp_up#{m}_b"), (177, 128, f"barri_warp_down#{m}b")]:
        place(m, x, y, name, 111, touch=(7, 0))
    for team, sprite, points in TIERRA_FLAGS:
        for index, (x, y) in enumerate(points, 1):
            place(m, x, y, f"{team}#flag{index}", sprite)
    return npcs


BG_RECRUITERS = [
    ("prontera", 123, 83, 3), ("moc_ruins", 75, 162, 3), ("aldebaran", 146, 109, 3), ("geffen", 109, 66, 3),
    ("payon", 189, 105, 3), ("lighthalzen", 153, 86, 5), ("rachel", 149, 138, 3),
]


def battleground_common_npcs():
    number = lambda value: {"Number": value}
    npcs = []
    place = lambda map_name, x, y, name, sprite, role, code=0, direction=3: npcs.append(dict(
        map_name=map_name, x=x, y=y, dir=direction, name=name, sprite=str(sprite), entry_id=18,
        constructor_args=[number(role), number(code)]))
    for code, (map_name, x, y, direction) in enumerate(BG_RECRUITERS, 1):
        place(map_name, x, y, f"Maroll Battle Recruiter::BatRecruit{code}", 728, 0, code, direction)
    place("bat_room", 148, 150, "Teleporter#Battlefield", 124, 1, 0, 5)
    place("bat_room", 160, 141, "Prince Croix", 416, 2)
    place("bat_room", 160, 159, "General Guillaume", 420, 3)
    place("bat_room", 160, 150, "Erundek", 109, 4)
    for x, y, name, sprite in [(161, 158, "Gen. Guillaume's Aide#01", 419), (161, 160, "Gen. Guillaume's Aide#03", 419),
                               (161, 140, "Prince Croix's Aide#01", 415), (161, 142, "Prince Croix's Aide#02", 415)]:
        place("bat_room", x, y, name, sprite, 5)
    return npcs


def merge_arena_events():
    path = ROOT / "config/wasm/events.json"
    events = {name: id for name, id in json.loads(path.read_text(encoding="utf-8")).items() if id < ARENA_EVENT_BASE}
    for position, arena in enumerate(ARENAS):
        events.update(arena_events(position, arena))
    for position, arena in enumerate(KVM_ARENAS):
        events.update(kvm_events(position, arena))
    events.update(tierra_events())
    path.write_text(json.dumps(events, indent=2) + "\n", encoding="utf-8")


def main():
    npcs = import_manifest() + builtin_npcs() + battleground_arena_npcs() + kvm_arena_npcs() + tierra_npcs() + battleground_common_npcs()
    merge_arena_events()
    warps = import_warps()
    destination = ROOT / "config/wasm"
    destination.mkdir(exist_ok=True)
    for name, value in [("npcs", npcs), ("warps", warps)]:
        (destination / f"{name}.json").write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    print(f"Converted {len(npcs)} NPC placements and {len(warps)} pre-renewal destinations")


if __name__ == "__main__":
    main()
