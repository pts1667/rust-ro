import argparse
import json
import hashlib
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
FUNCTIONS = {
    "warpportal": "WarpPortal",
    "getmapflag": "GetMapFlag", "setmapflag": "SetMapFlag", "removemapflag": "RemoveMapFlag",
    "setmapflagnosave": "SetMapFlagNoSave", "savepoint": "SavePoint", "save": "SavePoint", "getsavepoint": "GetSavePoint",
    "pvpon": "PvpOn", "pvpoff": "PvpOff", "gvgon": "GvgOn", "gvgoff": "GvgOff",
    "bonus": "Bonus", "bonus2": "Bonus2", "bonus3": "Bonus3", "bonus4": "Bonus4", "bonus5": "Bonus5",
    "skill": "Skill", "itemskill": "ItemSkill", "getrefine": "GetRefine", "readparam": "ReadParam",
    "getskilllv": "GetSkillLv", "isequipped": "IsEquipped", "getequipid": "GetEquipId",
    "getequiprefinerycnt": "GetEquipRefineryCnt", "getiteminfo": "GetItemInfo", "rand": "Rand", "max": "Max", "min": "Min", "pow": "Pow",
    "gettime": "GetTime", "vip_status": "VipStatus", "getpartnerid": "GetPartnerId", "checkmadogear": "CheckMadogear",
    "itemheal": "ItemHeal", "percentheal": "PercentHeal", "heal": "Heal", "getitem": "GetItem", "delitem": "DelItem", "countitem": "CountItem",
    "specialeffect2": "SpecialEffect", "skilleffect": "SkillEffect", "sc_start": "StartStatus", "sc_start2": "StartStatus2", "sc_start4": "StartStatus4", "sc_end": "EndStatus",
    "autobonus": "AutoBonus", "autobonus2": "AutoBonus2", "autobonus3": "AutoBonus3", "getrandgroupitem": "RandomGroupItem", "getgroupitem": "GetGroupItem",
    "monster": "Monster", "produce": "Produce", "pet": "Pet", "bpet": "BirthPet", "birthpet": "BirthPet", "guildgetexp": "GuildExperience", "cooking": "Cooking",
    "callfunc": "CallFunction", "mercenary_create": "MercenaryCreate", "mercenary_sc_start": "MercenaryStartStatus", "mercenary_heal": "MercenaryHeal",
    "setfont": "SetFont", "searchstores": "SearchStores", "homevolution": "Homevolution", "announce": "Announce", "buyingstore": "BuyingStore",
    "getexp2": "GetExperience", "input": "InputNumber", "strcharinfo": "StrCharInfo", "getcharid": "GetCharacterId", "warpparty": "PartyWarp",
    "unitskilluseid": "UnitSkillToId", "unitskillusepos": "UnitSkillToPosition",
    "getfame": "GetFame", "getfamerank": "GetFameRank", "addfame": "AddFame",
    "openstorage": "OpenStorage", "guildopenstorage": "GuildOpenStorage",
    "getpetinfo": "GetPetInfo", "catchpet": "Pet", "petskillbonus": "PetSkillBonus", "petrecovery": "PetRecovery",
    "petskillattack": "PetSkillAttack", "petskillattack2": "PetSkillAttack2", "petskillsupport": "PetSkillSupport", "petloot": "PetLoot",
    "petautobonus": "PetAutoBonus", "petautobonus2": "PetAutoBonus2", "petautobonus3": "PetAutoBonus3",
    "mes": "Mes", "close": "Close", "next": "Next", "select": "Select",
    "message": "Message", "dispbottom": "DispBottom", "cutin": "Cutin",
    "addtimer": "AddTimer", "deltimer": "DeleteTimer", "addtimercount": "AddTimerCount",
    "initnpctimer": "InitNpcTimer", "startnpctimer": "StartNpcTimer", "stopnpctimer": "StopNpcTimer",
    "setnpctimer": "SetNpcTimer", "getnpctimer": "GetNpcTimer", "attachnpctimer": "AttachNpcTimer", "detachnpctimer": "DetachNpcTimer",
}
TOKEN = re.compile(r'\s+|/\*.*?\*/|//[^\n]*|"(?:\\.|[^"\\])*"|0[xX][0-9a-fA-F]+|\d+|(?:\.@|[.@#$\x27]+)?[A-Za-z_][\w$]*|==|!=|<=|>=|&&|\|\||<<|>>|\+=|-=|\+\+|--|[{}();,?:=+*/%<>!~&|^\-]', re.S)
PRECEDENCE = {"||": 1, "&&": 2, "|": 3, "^": 4, "&": 5, "==": 6, "!=": 6, "<": 7, ">": 7, "<=": 7, ">=": 7, "<<": 8, ">>": 8, "+": 9, "-": 9, "*": 10, "/": 10, "%": 10}
SPECIAL = {"Class", "BaseClass", "BaseJob", "BaseLevel", "JobLevel", "Zeny", "SkillPoint", "Sex", "MaxHp", "MaxSP", "Hp", "Sp"}


def literal(text):
    return json.dumps(text, ensure_ascii=False).replace("\\/", "/")


class Parser:
    def __init__(self, source, programs=None, item_id=0):
        self.tokens = []
        offset = 0
        for match in TOKEN.finditer(source):
            if source[offset:match.start()].strip():
                raise ValueError(f"Unrecognized text {source[offset:match.start()]!r}")
            token = match.group()
            offset = match.end()
            if not token.isspace() and not token.startswith(("/*", "//")):
                self.tokens.append(token)
        if source[offset:].strip():
            raise ValueError(f"Unrecognized trailing text {source[offset:]!r}")
        self.tokens.append("EOF")
        self.position = 0
        self.locals = set()
        self.dynamic = False
        self.calls = set()
        self.reads = set()
        self.writes = set()
        self.programs = programs if programs is not None else []
        self.item_id = item_id

    def peek(self):
        return self.tokens[self.position]

    def pop(self, expected=None):
        token = self.peek()
        if expected and token != expected:
            raise ValueError(f"Expected {expected}, found {token} near {self.tokens[max(0,self.position-5):self.position+5]}")
        self.position += 1
        return token

    def arguments(self, terminator):
        values = []
        while self.peek() != terminator:
            values.append(self.expression())
            if self.peek() != ",":
                break
            self.pop(",")
        return values

    def call(self, name, values):
        self.calls.add(name)
        if name not in FUNCTIONS:
            raise ValueError(f"Unknown command {name}")
        if name not in {"bonus", "bonus2", "bonus3", "bonus4", "bonus5", "skill"}:
            self.dynamic = True
        if name in {"autobonus", "autobonus2", "autobonus3", "petautobonus", "petautobonus2", "petautobonus3"}:
            for index in (0, 4):
                if index >= len(values):
                    continue
                match = re.fullmatch(r'Value::String\(("(?:\\.|[^"\\])*")\.into\(\)\)', values[index])
                if not match:
                    raise ValueError("Automatic bonus programs must be literal compiled script bodies")
                source = json.loads(match.group(1)).replace('\\"', '"')
                if not source.strip():
                    values[index] = "Value::Number(0)"
                    continue
                source_hash = hashlib.md5(source.encode()).hexdigest()
                kind = "bonus" if index == 0 else "visual"
                if name.startswith("pet"):
                    existing = next((program for program in self.programs if program.get("source_hash") == source_hash
                                     and program["item_id"] == self.item_id and program["kind"] == kind), None)
                    if existing is not None:
                        values[index] = f"Value::Number({existing['id']})"
                        self.calls.update(existing.get("calls", []))
                        continue
                program_id = len(self.programs) + 1
                program = {"id": program_id, "item_id": self.item_id, "kind": kind}
                if name.startswith("pet"):
                    program["source_hash"] = source_hash
                self.programs.append(program)
                converter = Parser(source, self.programs, self.item_id)
                program["body"] = converter.compile()
                program["calls"] = sorted(converter.calls)
                self.calls.update(converter.calls)
                values[index] = f"Value::Number({program_id})"
        if name == "callfunc":
            return f"crate::functions::call(ctx, vec![{', '.join(values)}])?"
        return f"ctx.call(Function::{FUNCTIONS[name]}, vec![{', '.join(values)}])?"

    def variable(self, name):
        if name.startswith(".@"):
            self.locals.add(name)
            return "local_" + name[2:].replace("$", "_string") + ".clone()"
        if name in SPECIAL or name in {"MDiceCone"} or name.startswith(("@", "#", "$")):
            self.dynamic = True
            self.reads.add(name)
            return f"ctx.read({literal(name)})?"
        return f"ctx.constant({literal(name)})?"

    def assign(self, name, value):
        self.dynamic = True
        if name.startswith(".@"):
            self.locals.add(name)
            return "local_" + name[2:].replace("$", "_string") + " = " + value + ";"
        self.writes.add(name)
        return f"ctx.write({literal(name)}, {value})?;"

    def expression(self, minimum=0):
        token = self.pop()
        if token == "(":
            value = self.expression()
            self.pop(")")
        elif token in {"-", "!", "~", "+"}:
            operand = self.expression(11)
            if token == "!":
                value = f"Value::Number(i32::from(!({operand}).truthy()))"
            elif token == "~":
                value = f"Value::Number(!({operand}).number_value()?)"
            elif token == "-":
                value = f"Value::Number(({operand}).number_value()?.wrapping_neg())"
            else:
                value = operand
        elif token.startswith('"'):
            value = f"Value::String({token}.into())"
        elif token[0].isdigit():
            value = f"Value::Number({int(token, 0) if token.startswith(('0x','0X')) else int(token)})"
        elif token in {"true", "false"}:
            value = f"Value::Number({int(token == 'true')})"
        elif self.peek() == "(":
            self.pop("(")
            values = self.arguments(")")
            self.pop(")")
            value = self.call(token, values)
        else:
            value = self.variable(token)
        while self.peek() in PRECEDENCE and PRECEDENCE[self.peek()] >= minimum:
            operation = self.pop()
            right = self.expression(PRECEDENCE[operation] + 1)
            if operation in {"&&", "||"}:
                value = f"Value::Number(i32::from(({value}).truthy() {operation} ({right}).truthy()))"
            else:
                value = f"({value}).binary({literal(operation)}, {right})?"
        if minimum == 0 and self.peek() == "?":
            self.pop("?")
            yes = self.expression()
            self.pop(":")
            no = self.expression()
            value = f"if ({value}).truthy() {{ {yes} }} else {{ {no} }}"
        return value

    def statement(self):
        token = self.pop()
        if token == ";":
            return ""
        if token == "{":
            body = []
            while self.peek() != "}":
                body.append(self.statement())
            self.pop("}")
            return "{\n" + "\n".join(body) + "\n}"
        if token == "if":
            self.pop("(")
            condition = self.expression()
            self.pop(")")
            body = self.statement()
            value = f"if ({condition}).truthy() {{ {body} }}"
            if self.peek() == "else":
                self.pop("else")
                value += " else { " + self.statement() + " }"
            return value
        if token == "end":
            self.pop(";")
            return "return Ok(());"
        if token == "input":
            name = self.pop()
            values = []
            if self.peek() == ",":
                self.pop(",")
                values = self.arguments(";")
            self.pop(";")
            self.calls.add("input")
            self.dynamic = True
            function = "InputString" if name.endswith("$") else "InputNumber"
            return self.assign(name, f"ctx.call(Function::{function}, vec![{', '.join(values)}])?")
        if token == "set":
            name = self.pop()
            self.pop(",")
            value = self.expression()
            self.pop(";")
            return self.assign(name, value)
        if self.peek() in {"=", "+=", "-="}:
            operation = self.pop()
            value = self.expression()
            if operation != "=":
                value = f"({self.variable(token)}).binary({literal(operation[0])}, {value})?"
            self.pop(";")
            return self.assign(token, value)
        if self.peek() == "(":
            self.pop("(")
            values = self.arguments(")")
            self.pop(")")
        else:
            values = self.arguments(";")
        self.pop(";")
        return "let _ = " + self.call(token, values) + ";"

    def compile(self):
        statements = []
        while self.peek() != "EOF":
            statements.append(self.statement())
        locals = [f"let mut local_{name[2:].replace('$', '_string')} = Value::default();" for name in sorted(self.locals)]
        return "\n".join(locals + statements)


def main():
    parser = argparse.ArgumentParser(description="Convert repository item sources to compiled Rust guest functions.")
    parser.add_argument("--catalog", type=pathlib.Path, default=ROOT / "config/items.json")
    arguments = parser.parse_args()
    items = json.loads(arguments.catalog.read_text(encoding="utf-8"))["items"]
    functions = ["use script_sdk::{Context, Function, Value};", "pub fn run(ctx: &Context, id: u32) -> Result<(), String> { match id {"]
    bodies = []
    metadata = []
    programs = []
    errors = []
    for item in items:
        source = item.get("script")
        if not source:
            continue
        converter = Parser(source, programs, item["id"])
        try:
            body = converter.compile()
        except ValueError as error:
            errors.append((item["id"], str(error)))
            continue
        functions.append(f"{item['id']} => item_{item['id']}(ctx),")
        bodies.append(f"#[inline(never)]\n#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]\nfn item_{item['id']}(ctx: &Context) -> Result<(),String> {{ {body}\nOk(()) }}")
        metadata.append({"id": item["id"], "dynamic": converter.dynamic, "source_hash": hashlib.md5(source.encode()).hexdigest(), "calls": sorted(converter.calls),
                         "reads": sorted(converter.reads), "writes": sorted(converter.writes),
                         "interactive": bool(converter.calls & {"callfunc", "input", "mes", "close", "next", "select", "cutin"})})
    if errors:
        for item_id, error in errors:
            print(f"Item {item_id}: {error}")
        raise SystemExit(f"Failed to convert {len(errors)} items; no output written")
    functions.extend(['_ => Err(format!("Unknown item script {id}")),', "} }"])
    functions.extend(bodies)
    functions.append("pub fn run_bonus(ctx: &Context, id: u32) -> Result<(), String> { match id {")
    for program in programs:
        functions.append(f"{program['id']} => bonus_{program['id']}(ctx),")
    functions.extend(['_ => Err(format!("Unknown compiled bonus program {id}")),', "} }"])
    for program in programs:
        functions.append(f"#[inline(never)]\n#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]\nfn bonus_{program['id']}(ctx: &Context) -> Result<(), String> {{ {program['body']}\nOk(()) }}")
    destination = ROOT / "config/wasm"
    destination.mkdir(exist_ok=True)
    metadata_bytes = (json.dumps(metadata, indent=2) + "\n").encode()
    fingerprint = int.from_bytes(hashlib.md5(metadata_bytes).digest()[:8], "little")
    functions.append(f"pub const CATALOG_HASH: u64 = {fingerprint};")
    (ROOT / "scripts/src/items.rs").write_text("\n".join(functions) + "\n", encoding="utf-8")
    (destination / "items.json").write_bytes(metadata_bytes)
    (destination / "bonus_programs.json").write_text(json.dumps([{k: v for k, v in program.items() if k != "body"} for program in programs], indent=2) + "\n", encoding="utf-8")
    print(f"Converted {len(metadata)} item scripts; {sum(m['dynamic'] for m in metadata)} dynamic")


if __name__ == "__main__":
    main()
