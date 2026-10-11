#!/usr/bin/env python3
"""Generate config/quest_text.json: description, guide and recommended level for each quest.

For every quest the rathena pre-re chain references, the prompt holds the quest facts, the NPC blocks that
reference it (verbatim), and the monsters and items behind its level. The llama.cpp server writes the text.
Levels from script gates, objectives and script monsters come from code; the model picks a level only when
the only source is item droppers. Results are cached per quest, so an interrupted run resumes.

Run as: python -I -X utf8 tools/scripts-import/gen_quest_text.py --help
"""

import argparse
import concurrent.futures
import json
import re
import time
import urllib.request
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
MAIN_CONF = "npc/pre-re/scripts_main.conf"
EXCERPT_CAP = 24_000
MAX_ATTEMPTS = 3
HAND_WRITTEN = {1009, 1010, 1011, 1012}
DROPPER_CAP = 10
WIDE_RANGE = 20

# The second number is the new quest of changequest old,new.
QUEST_REF = re.compile(
    r"\b(?:setquest|completequest|erasequest|changequest|checkquest|questprogress|isbegin_quest|questinfo)\b\s*\(?\s*(\d+)(?:\s*,\s*(\d+))?"
)
HEADER = re.compile(r"^(?:-|function|[^\t,]+,\d+,\d+(?:,\d+)?)\t")
POSITION = re.compile(r"^([^,]+),(\d+),(\d+)")
SPEAKER = re.compile(r'\bmes\s+"\[([^\]"]+)\]"')
LINES_AS = re.compile(r'\blines_as\(\s*"([^"]+)"')
NPC_CALL = re.compile(r'\b(?:donpcevent|enablenpc|disablenpc|hideonnpc|hideoffnpc|setnpcdisplay)\s*\(?\s*"([^"]+)"')
BASE_GATE = re.compile(r"\bBaseLevel\s*(>=|>)\s*(\d+)")
MONSTER_CALL = re.compile(r'\bmonster\s*"[^"\n]*"\s*,\s*-?\d+\s*,\s*-?\d+\s*,\s*"([^"\n]+)"\s*,\s*(\d+)')
AREA_CALL = re.compile(
    r'\bareamonster\s*"[^"\n]*"\s*,\s*-?\d+\s*,\s*-?\d+\s*,\s*-?\d+\s*,\s*-?\d+\s*,\s*"([^"\n]+)"\s*,\s*(\d+)'
)
MONSTER_LINE = re.compile(r"\tmonster\t([^\t\n]+)\t(\d+)")
ITEM_CALL = re.compile(r"\b(?:countitem2|countitem|delitem2|delitem)\b\s*\(?\s*([A-Za-z_]\w*|\d+)")

REQUIRED_KEYS = ("description", "guide", "recommended_level", "level_basis", "level_override")

SYSTEM_PROMPT = (
    "You write short, accurate player guides for a Ragnarok Online pre-renewal quest database. "
    "Use only the facts and script excerpts you are given. Reply with one JSON object and nothing else."
)

RULES = """- Use only the quest facts, the monsters and items above, and the script excerpts. Do not invent names, places, items, rewards or numbers.
- If the excerpts do not show what the player must do, say so in description and give fewer steps.
- description: one or two sentences saying what the quest is about.
- guide: one to six short steps. Each step names the NPC and the map from the NPC list where it applies. A step says only what the excerpt shows the player doing or being told. Do not say the player must collect or hunt an item or monster unless the excerpt shows it.
- Item and monster names in description and guide must appear in the facts or script excerpts. Do not write names with "#" or question marks.
- recommended_level is the Base Level a player should have for this quest. It is not the Job Level. Follow the Recommended level section exactly.
- level_basis: one sentence. When recommended_level is a number, it names the monster, item or gate that number came from, using only names listed above. When it is "Unknown", it says the sentence given in the Recommended level section.
- level_override: null in almost every case. Set it only when a script condition checks BaseLevel, such as "BaseLevel >= 40", and give {"level": that number, "quote": that exact script line}. A Job Level check and a dialogue line are never a level_override. You may mention a Job Level requirement in description.

Reply with one JSON object in this shape and nothing else:
{"description": "...", "guide": ["...", "..."], "recommended_level": 60, "level_basis": "...", "level_override": null}

Three examples of the level fields. The names are made up; use only the names in the quest you are given.
- Gate: the Recommended level section says "fixed by the level gate BaseLevel > 59". Then "recommended_level": 60, "level_basis": "The level gate in the NPC script requires BaseLevel > 59.", "level_override": null
- Item droppers: the section lists the levels 25, 31 and 40. Pick one listed level, for example 31, and name the item and monster. Then "recommended_level": 31, "level_basis": "Thorn Shell (item 2001) is dropped by Example Crawler (level 31).", "level_override": null
- Unknown: the section says "Unknown". Then "recommended_level": "Unknown", "level_basis": "No monster, item or level gate in the facts sets a level.", "level_override": null. A Job Level requirement does not change this."""


def read_text(path):
    raw = path.read_bytes()
    for codec in ("utf-8-sig", "cp949"):
        try:
            return raw.decode(codec)
        except UnicodeDecodeError:
            pass
    return raw.decode("latin-1")


def chain_files(source):
    files, seen, queue = [], set(), [MAIN_CONF]
    while queue:
        conf = queue.pop(0)
        if conf in seen:
            continue
        seen.add(conf)
        for line in read_text(source / conf).splitlines():
            text = line.split("//")[0].strip()
            if ":" not in text:
                continue
            key, _, value = text.partition(":")
            value = value.strip()
            if key == "import":
                queue.append(value)
            elif key == "npc" and value not in files:
                files.append(value)
    return files


class Block:
    def __init__(self, rel, header):
        self.rel = rel
        self.lines = [header]
        fields = header.split("\t")
        self.kind = fields[1] if len(fields) > 1 else ""
        self.name = fields[2].split("#")[0].strip() if len(fields) > 2 else ""
        position = POSITION.match(fields[0])
        self.where = f"{position[1]} {position[2]},{position[3]}" if position else None
        self.is_npc = fields[0] != "function" and bool(self.name) and (
            self.kind.startswith(("script", "duplicate(", "shop", "cashshop"))
        )
        self.text = ""

    def finish(self):
        self.text = "\n".join(self.lines).rstrip()


def parse_blocks(source, files):
    blocks = []
    for rel in files:
        path = source / rel
        if not path.is_file():
            continue
        current = None
        for line in read_text(path).splitlines():
            if HEADER.match(line):
                current = Block(rel, line)
                blocks.append(current)
            elif current is not None:
                current.lines.append(line)
    for block in blocks:
        block.finish()
    return blocks


def name_of(mob):
    return mob.get("name_english") or mob.get("name") or f"monster {mob['id']}"


def split_sentences(text):
    return [part for part in re.split(r"(?<=[.!?])\s+", text.strip()) if part]


def normalise(text):
    return " ".join(text.split())


def unique(values):
    result = []
    for value in values:
        if value and value not in result:
            result.append(value)
    return result


class Catalog:
    def __init__(self, repo, source_root):
        self.quests = {q["id"]: q for q in json.loads((repo / "server/src/server/script/quests.json").read_text(encoding="utf-8"))}
        self.mobs = {m["id"]: m for m in json.loads((repo / "config/mobs.json").read_text(encoding="utf-8"))["mobs"]}
        items = json.loads((repo / "config/items.json").read_text(encoding="utf-8"))["items"]
        self.items = {i["id"]: i for i in items}
        self.item_by_aegis = {i["name_aegis"].lower(): i["id"] for i in items if i.get("name_aegis")}
        self.mob_by_name = {}
        for mob in self.mobs.values():
            for key in {mob.get("name_english", ""), mob.get("name", "")}:
                if key:
                    self.mob_by_name.setdefault(key.lower(), []).append(mob["id"])
        self.droppers = {}
        for mob in self.mobs.values():
            for drop in (mob.get("drops") or []) + (mob.get("mvp_drops") or []):
                self.droppers.setdefault(drop["item_id"], set()).add(mob["id"])
        self.known_names = sorted(
            {n for n in list(self.item_name_list()) + [name_of(m) for m in self.mobs.values()] if len(n) >= 5}
        )
        self.source = source_root

    def item_name_list(self):
        return [i.get("name_english") or i.get("name_aegis") for i in self.items.values()]

    def resolve_item(self, token):
        if token.isdigit():
            return int(token) if int(token) in self.items else None
        return self.item_by_aegis.get(token.lower())

    def resolve_script_mob(self, mob_id, name):
        if mob_id in self.mobs:
            return self.mobs[mob_id]
        ids = self.mob_by_name.get(name.lower(), [])
        return self.mobs[ids[0]] if len(ids) == 1 else None


class Level:
    def __init__(self):
        self.level = None
        self.source = None
        self.gate_min = None
        self.low = None
        self.high = None
        self.allowed = []
        self.wide = False
        self.lines = []
        self.text = ""


def build_level(quest, catalog, primary_lines):
    level = Level()
    gates = []
    for line in primary_lines:
        if "getexp" in line:
            continue
        for match in BASE_GATE.finditer(line):
            value = int(match[2]) + (1 if match[1] == ">" else 0)
            gates.append((value, line.strip()))
    objective_mobs = []
    for obj in quest["objectives"]:
        mob = catalog.mobs.get(obj["mob"]) if obj["mob"] else None
        if mob and mob.get("level", 0) > 0:
            objective_mobs.append((mob, obj["count"]))
    script_mobs = []
    seen = set()
    for line in primary_lines:
        for pattern in (MONSTER_CALL, AREA_CALL):
            for match in pattern.finditer(line):
                mob = catalog.resolve_script_mob(int(match[2]), match[1])
                if mob and mob["id"] not in seen and mob.get("level", 0) > 0:
                    seen.add(mob["id"])
                    script_mobs.append(mob)
        for match in MONSTER_LINE.finditer(line):
            mob = catalog.resolve_script_mob(int(match[2]), match[1])
            if mob and mob["id"] not in seen and mob.get("level", 0) > 0:
                seen.add(mob["id"])
                script_mobs.append(mob)
    item_ids = []
    for line in primary_lines:
        for match in ITEM_CALL.finditer(line):
            item_id = catalog.resolve_item(match[1])
            if item_id is not None and item_id not in item_ids:
                item_ids.append(item_id)

    if gates:
        level.gate_min, gate_line = max(gates)
        level.level = level.gate_min
        level.source = "gate"
        level.text = f"a level gate in the script ({gate_line})"
    elif objective_mobs:
        top = max(objective_mobs, key=lambda pair: pair[0]["level"])[0]
        level.level = top["level"]
        level.source = "objectives"
        level.text = f"the quest objective monster {name_of(top)}"
    elif script_mobs:
        top = max(script_mobs, key=lambda mob: mob["level"])
        level.level = top["level"]
        level.source = "script monsters"
        level.text = f"the monster {name_of(top)} named in the script"

    for mob, count in objective_mobs:
        level.lines.append(f"- {name_of(mob)} (monster {mob['id']}, level {mob['level']}): quest objective, {count} to defeat")
    for mob in script_mobs:
        level.lines.append(f"- {name_of(mob)} (monster {mob['id']}, level {mob['level']}): named by a spawn line in the script")

    listed_levels = []
    for item_id in item_ids:
        item = catalog.items[item_id]
        label = f"{item.get('name_english') or item['name_aegis']} (item {item_id})"
        droppers = sorted((catalog.mobs[m] for m in catalog.droppers.get(item_id, ()) if m in catalog.mobs), key=lambda m: (m["level"], m["id"]))
        if not droppers:
            level.lines.append(f"- {label}: no monster drops it.")
        elif len(droppers) > DROPPER_CAP:
            level.lines.append(
                f"- {label}: a common item, dropped by {len(droppers)} monsters. Assume it is easily available; it sets no level requirement."
            )
        else:
            names = ", ".join(f"{name_of(m)} (level {m['level']})" for m in droppers)
            level.lines.append(f"- {label}: dropped by {names}.")
            listed_levels.extend(m["level"] for m in droppers if m["level"] > 0)
    if level.level is None and listed_levels:
        level.allowed = sorted(set(listed_levels))
        level.low, level.high = min(listed_levels), max(listed_levels)
        if level.high - level.low > WIDE_RANGE:
            level.wide = True
        else:
            level.source = "item droppers"
    return level, item_ids


def npc_names(texts, primary_npcs):
    names = [block.name for block in primary_npcs]
    for text in texts:
        names += [m[1].split("#")[0].strip() for m in SPEAKER.finditer(text)]
        names += [m[1].split("#")[0].strip() for m in LINES_AS.finditer(text)]
        names += [m[1].split("::")[0].split("#")[0].strip() for m in NPC_CALL.finditer(text)]
    return [name for name in unique(names) if "?" not in name]


def npc_lines(names, npc_index):
    lines, sources = [], []
    for name in names:
        places = npc_index.get(name)
        if not places:
            lines.append(f"- {name}: not an NPC name in the pre-re scripts; name only")
            sources.append(name)
            continue
        shown = [place or "no fixed position" for place in places[:3]]
        more = f" (and {len(places) - 3} more with this name)" if len(places) > 3 else ""
        lines.append(f"- {name}: {'; '.join(shown)}{more}")
        sources.append(f"{name} ({shown[0]})")
    return lines, sources


def excerpt_of(blocks):
    shown, total = [], 0
    for block in blocks:
        room = EXCERPT_CAP - total
        if room <= 0:
            break
        text = block.text if len(block.text) <= room else block.text[:room] + "\n[block cut: excerpt limit reached]"
        shown.append(block)
        total += min(len(block.text), room)
        if len(block.text) > room:
            break
    omitted = len(blocks) - len(shown)
    return shown, omitted


class Context:
    pass


def build_context(quest, blocks, catalog, npc_index):
    shown, omitted = excerpt_of(blocks)
    shown_texts = [block.text[:EXCERPT_CAP] for block in shown]
    primary_lines = [line for text in shown_texts for line in text.splitlines()]
    primary_npcs = [block for block in shown if block.is_npc]
    names = npc_names(shown_texts, primary_npcs)
    name_lines, sources = npc_lines(names, npc_index)
    for block in primary_npcs:
        spoken = unique(m[1].strip() for m in SPEAKER.finditer(block.text))
        if spoken:
            name_lines.insert(0, f"- {block.name} ({block.where or 'no fixed position'}) is the NPC of the script block; its dialogue speaks as {', '.join(spoken)}.")
    level, item_ids = build_level(quest, catalog, primary_lines)

    facts = [f"Quest {quest['id']}: {quest['title']}."]
    if quest.get("time"):
        facts.append(f"Time limit: {quest['time']} seconds.")
    for obj in quest["objectives"]:
        if not obj["mob"]:
            facts.append(f"Objective: {obj['count']} unspecified; see the script.")
            continue
        mob = catalog.mobs.get(obj["mob"])
        target = f"{name_of(mob)} (monster {mob['id']})" if mob else f"monster id {obj['mob']} (not in mobs.json)"
        where = f" in {obj['map_name']}" if obj.get("map_name") else ""
        facts.append(f"Objective: defeat {obj['count']} x {target}{where}.")

    if level.level is not None:
        level_text = (
            f"Recommended level is {level.level}, fixed by {level.text}. "
            f"Set recommended_level to {level.level}, and name that source in level_basis."
        )
    elif level.source == "item droppers":
        allowed = ", ".join(str(value) for value in level.allowed)
        level_text = (
            "No gate and no monster level apply. Set recommended_level to one of the item dropper levels listed "
            f"below ({allowed}), and say in level_basis which item and monster that level is from."
        )
    elif level.wide:
        level_text = (
            f"No gate and no monster level apply. The item droppers listed below range from level {level.low} to "
            f"{level.high}, too wide to pick one level. Set recommended_level to \"Unknown\". Set level_basis to "
            "\"The item droppers range too widely to pick one level.\""
        )
    else:
        level_text = (
            'No level source applies. Set recommended_level to "Unknown". Set level_basis to '
            '"No monster, item or level gate in the facts sets a level."'
        )

    context = Context()
    context.quest = quest
    context.level = level
    context.sources = sources
    context.omitted = omitted
    context.blocks = shown
    context.excerpt = "\n\n".join(f"--- script block from {b.rel} ---\n{t}" for b, t in zip(shown, shown_texts))
    if omitted:
        context.excerpt += f"\n\n[{omitted} more block(s) referencing this quest were left out: excerpt limit reached]"
    context.excerpt_norm = normalise(" ".join(shown_texts))
    context.prompt = "\n\n".join(
        [
            "== Quest facts ==\n" + "\n".join(facts),
            "== Recommended level ==\n" + level_text
            + "\nSet level_override only if a script line checks BaseLevel, as the Rules describe.",
            "== Monsters and items behind the level ==\n" + ("\n".join(level.lines) or "none"),
            "== NPCs named in the excerpts ==\n" + ("\n".join(name_lines) or "none"),
            "== Script excerpts (verbatim rathena source) ==\n" + context.excerpt,
            "== Rules ==\n" + RULES,
        ]
    )
    context.context_lower = context.prompt.lower()
    return context


def parse_reply(content):
    text = re.sub(r"^```(?:json)?\s*|\s*```$", "", content.strip())
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        start, end = text.find("{"), text.rfind("}")
        if start < 0 or end < start:
            return None
        try:
            return json.loads(text[start : end + 1])
        except json.JSONDecodeError:
            return None


def names_missing_from(text, catalog, reference_lower):
    return [
        name
        for name in catalog.known_names
        if re.search(rf"(?<!\w){re.escape(name)}(?!\w)", text) and name.lower() not in reference_lower
    ]


def override_problem(override, context):
    quote = normalise(override["quote"])
    if not quote:
        return "quote is empty"
    if quote not in context.excerpt_norm:
        return "quote is not a line of the excerpt"
    if "baselevel" not in override["quote"].lower():
        return "quote is not a BaseLevel check"
    level = override["level"]
    if not re.search(rf"(?<!\d){level}(?!\d)", override["quote"]):
        return "quote does not contain the level"
    if context.level.gate_min is not None and level < context.level.gate_min:
        return f"level is below the script gate {context.level.gate_min}"
    return None


def resolve_level(reply_level, override, context):
    note = None
    if override is not None:
        problem = override_problem(override, context)
        if problem is None:
            if reply_level != override["level"]:
                return None, "recommended_level must equal level_override.level", None
            return override["level"], None, None
        note = problem
    level = context.level
    if level.level is not None:
        if note is None and reply_level != level.level:
            return None, f"recommended_level must be {level.level} ({level.source})", note
        return level.level, None, note
    if level.source == "item droppers":
        if isinstance(reply_level, int) and not isinstance(reply_level, bool) and reply_level in level.allowed:
            return reply_level, None, note
        return None, f"recommended_level must be one of the item dropper levels: {', '.join(str(v) for v in level.allowed)}", note
    if reply_level != "Unknown":
        return None, 'recommended_level must be "Unknown" because no level source applies', note
    return "Unknown", None, note


def validate(content, context, catalog, model):
    data = parse_reply(content)
    if not isinstance(data, dict):
        return None, ["reply is not one JSON object"], None
    missing = [key for key in REQUIRED_KEYS if key not in data]
    if missing:
        return None, [f"missing keys: {', '.join(missing)}"], None

    errors = []
    description, guide, basis = data["description"], data["guide"], data["level_basis"]
    written = " ".join([description] + [step for step in guide if isinstance(step, str)])
    if isinstance(description, str) and isinstance(guide, list):
        if "#" in written or re.search(r"\?{2,}", written):
            errors.append('description and guide must not contain "#" or runs of question marks')
        missing = names_missing_from(written, catalog, context.context_lower)
        if missing:
            errors.append(f"description or guide names {', '.join(missing)}, which is not in the facts or excerpt")
    if not isinstance(description, str) or not description.strip():
        errors.append("description must be a non-empty string")
    elif len(split_sentences(description)) > 2:
        errors.append("description has more than two sentences")
    if not (isinstance(guide, list) and 1 <= len(guide) <= 6 and all(isinstance(s, str) and s.strip() for s in guide)):
        errors.append("guide must be one to six non-empty strings")
    if not isinstance(basis, str) or not basis.strip():
        errors.append("level_basis must be a non-empty string")
    else:
        unknown = names_missing_from(basis, catalog, context.context_lower)
        if unknown:
            errors.append(f"level_basis names {', '.join(unknown)}, which is not in the facts or excerpt")
    reply_level = data["recommended_level"]
    if not (reply_level == "Unknown" or (isinstance(reply_level, int) and not isinstance(reply_level, bool))):
        errors.append('recommended_level must be an integer or "Unknown"')
    override = data["level_override"]
    if override is not None and not (
        isinstance(override, dict)
        and isinstance(override.get("level"), int)
        and not isinstance(override.get("level"), bool)
        and isinstance(override.get("quote"), str)
    ):
        errors.append('level_override must be null or {"level": integer, "quote": string}')
        override = None
    if errors:
        return None, errors, None

    final, level_error, note = resolve_level(reply_level, override, context)
    if level_error:
        return None, [level_error], note
    entry = {
        "description": description.strip(),
        "guide": [step.strip() for step in guide],
        "recommended_level": final,
        "level_basis": basis.strip(),
        "sources": context.sources,
        "level_source": context.level.source,
        "generated": True,
        "model": model,
    }
    return entry, [], note


def chat(messages, args):
    body = {"model": args.model, "temperature": 0.2, "max_tokens": 16384, "messages": messages}
    if args.no_think:
        body["chat_template_kwargs"] = {"enable_thinking": False}
    request = urllib.request.Request(
        args.url, data=json.dumps(body).encode("utf-8"), headers={"Content-Type": "application/json"}
    )
    with urllib.request.urlopen(request, timeout=1800) as response:
        data = json.load(response)
    choice = data["choices"][0]
    message = choice["message"]
    return message.get("content") or "", message.get("reasoning_content") or "", data.get("usage", {}), choice.get("finish_reason")


def generate(quest_id, context, catalog, args):
    messages = [{"role": "system", "content": SYSTEM_PROMPT}, {"role": "user", "content": context.prompt}]
    attempts, content, override_note = [], "", None
    for _ in range(MAX_ATTEMPTS):
        started = time.monotonic()
        try:
            content, reasoning, usage, finish = chat(messages, args)
        except Exception as error:
            return {"id": quest_id, "status": "error", "reason": str(error), "attempts": attempts}
        entry, errors, override_note = validate(content, context, catalog, args.model)
        attempts.append(
            {
                "errors": errors,
                "seconds": round(time.monotonic() - started, 1),
                "reasoning_chars": len(reasoning),
                "completion_tokens": usage.get("completion_tokens"),
                "finish": finish,
            }
        )
        if not errors:
            return {
                "id": quest_id,
                "status": "ok",
                "entry": entry,
                "override_note": override_note,
                "attempts": attempts,
                "raw": content,
                "think": not args.no_think,
            }
        messages += [
            {"role": "assistant", "content": content},
            {"role": "user", "content": "Rejected: " + "; ".join(errors) + ". Reply again with the JSON object only, fixing these problems."},
        ]
    return {"id": quest_id, "status": "failed", "attempts": attempts, "raw": content, "think": not args.no_think}


def load_cache(path):
    cache = {}
    if path.is_file():
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.strip():
                record = json.loads(line)
                cache[record["id"]] = record
    return cache


def select(targets, args):
    if args.ids:
        wanted = [int(token) for token in args.ids.split(",")]
        return [q for q in wanted if q in targets]
    if args.sample:
        if args.sample == 1:
            return targets[:1]
        step = (len(targets) - 1) / (args.sample - 1)
        return [targets[round(i * step)] for i in range(args.sample)]
    return targets


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--source-root", default=str(REPO.parent / "rathena"))
    parser.add_argument("--ids", help="comma-separated quest ids")
    parser.add_argument("--sample", type=int, help="generate N quests spread evenly over the set")
    parser.add_argument("--dry-run", action="store_true", help="build prompts and level sources; call no model")
    parser.add_argument("--prompt-dir", help="write each prompt to <dir>/<id>.txt")
    parser.add_argument("--cache", default=str(Path(__file__).with_name("quest_text.cache.jsonl")))
    parser.add_argument("--output", default=str(REPO / "config/quest_text.json"))
    parser.add_argument("--url", default="http://127.0.0.1:8080/v1/chat/completions")
    parser.add_argument("--model", default="tiel-coder")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--no-think", action="store_true", help="turn off the model's reasoning for these requests")
    parser.add_argument("--retry-failed", action="store_true")
    args = parser.parse_args()

    source = Path(args.source_root).resolve()
    catalog = Catalog(REPO, source)
    files = chain_files(source)
    blocks = parse_blocks(source, files)
    npc_index = {}
    for block in blocks:
        if block.is_npc:
            npc_index.setdefault(block.name, []).append(block.where)

    refs = {}
    for block in blocks:
        numbers = {int(raw) for match in QUEST_REF.finditer(block.text) for raw in match.groups() if raw}
        for quest_id in numbers:
            if quest_id in catalog.quests:
                refs.setdefault(quest_id, []).append(block)
    targets = sorted(q for q in refs if q not in HAND_WRITTEN)
    print(f"chain files {len(files)}, blocks {len(blocks)}, referenced quests {len(refs)}, generated set {len(targets)}")

    selected = select(targets, args)
    cache = load_cache(Path(args.cache))
    todo = [q for q in selected if q not in cache or (args.retry_failed and cache[q]["status"] != "ok")]
    contexts = {q: build_context(catalog.quests[q], refs[q], catalog, npc_index) for q in todo}

    if args.prompt_dir:
        Path(args.prompt_dir).mkdir(parents=True, exist_ok=True)
        for q, context in contexts.items():
            (Path(args.prompt_dir) / f"{q}.txt").write_text(context.prompt, encoding="utf-8")

    if args.dry_run:
        counts = {}
        for context in contexts.values():
            key = context.level.source or "none"
            counts[key] = counts.get(key, 0) + 1
        print("dry run, level sources", counts, "prompts", len(contexts))
        return

    print(f"selected {len(selected)}, to generate {len(todo)}, cached {len(selected) - len(todo)}")
    cache_file = Path(args.cache)
    with cache_file.open("a", encoding="utf-8") as out, concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futures = {pool.submit(generate, q, contexts[q], catalog, args): q for q in todo}
        for future in concurrent.futures.as_completed(futures):
            q = futures[future]
            record = future.result()
            record["level_source"] = contexts[q].level.source
            cache[q] = record
            out.write(json.dumps(record, ensure_ascii=False) + "\n")
            out.flush()
            last = record["attempts"][-1] if record["attempts"] else {}
            print(
                f"{q:>6} {record['status']:<7} attempts={len(record['attempts'])} "
                f"seconds={last.get('seconds', '-')} reasoning={last.get('reasoning_chars', '-')} "
                f"level={record['entry']['recommended_level'] if record['status'] == 'ok' else '-'} "
                f"{'; '.join(record['attempts'][-1]['errors']) if record['status'] == 'failed' else ''}",
                flush=True,
            )

    output = {str(q): cache[q]["entry"] for q in sorted(targets) if q in cache and cache[q]["status"] == "ok"}
    Path(args.output).write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    statuses = {}
    for q in targets:
        if q in cache:
            statuses[cache[q]["status"]] = statuses.get(cache[q]["status"], 0) + 1
    print("status counts", statuses, "wrote", len(output), "entries to", args.output)


if __name__ == "__main__":
    main()
