"""Lowers parsed rathena scripts to Rust that runs on the script-sdk `Context`.

Everything the converter cannot lower faithfully is recorded as a blocker instead of being guessed; an NPC with
blockers is not emitted.
"""

import pathlib
import re

from .parser import (Assign, Binary, Block, Break, Call, Case, Command, Continue, DoWhile, ExprStatement, For, FunctionDefinition, Goto,
                     If, IncDec, Index, Label, Menu, Name, Num, Return, Str, Switch, Ternary, Unary, While)

# rathena command or function -> script-sdk `Function` variant, called with the arguments as written
SDK_CALLS = {
    "getitem": "GetItem", "delitem": "DelItem", "countitem": "CountItem", "warp": "Warp", "getexp": "GetExperience",
    "donpcevent": "DoNpcEvent", "doevent": "DoEvent", "enablenpc": "EnableNpc", "disablenpc": "DisableNpc",
    "hideonnpc": "DisableNpc", "hideoffnpc": "EnableNpc", "announce": "Announce", "monster": "Monster", "killmonster": "KillMonster",
    "mobcount": "MobCount", "specialeffect": "NpcSpecialEffect", "specialeffect2": "SpecialEffect", "percentheal": "PercentHeal", "skill": "Skill",
    "getskilllv": "GetSkillLv", "strcharinfo": "StrCharInfo", "rand": "Rand", "gettime": "GetTime", "isequipped": "IsEquipped",
    "getitemname": "GetItemName", "getequipid": "GetEquipId", "eaclass": "EaClass", "getcharid": "GetCharacterId",
    "getnpcid": "GetNpcId", "initnpctimer": "InitNpcTimer", "stopnpctimer": "StopNpcTimer", "startnpctimer": "StartNpcTimer",
    "setnpctimer": "SetNpcTimer", "getnpctimer": "GetNpcTimer", "mapwarp": "MapWarp", "areawarp": "AreaWarp", "savepoint": "SavePoint",
    "getiteminfo": "GetItemInfo", "vip_status": "VipStatus", "getpartnerid": "GetPartnerId", "setquest": "SetQuest",
    "completequest": "CompleteQuest", "erasequest": "EraseQuest", "changequest": "ChangeQuest", "checkquest": "CheckQuest",
    "isbegin_quest": "IsBeginQuest", "questinfo": "QuestInfo", "showevent": "ShowEvent", "cutin": "Cutin", "sc_end": "EndStatus",
    "checkriding": "CheckRiding", "checkfalcon": "CheckFalcon", "checkcart": "CheckCart", "getequiprefinerycnt": "GetEquipRefineryCnt",
    "getequipweaponlv": None, "setcell": "SetCell", "pow": "Pow", "min": "Min", "max": "Max", "jobchange": "JobChange",
    "jobname": "JobName", "readparam": "ReadParam", "getrefine": "GetRefine", "heal": "Heal", "itemheal": "ItemHeal",
    "message": "Message", "dispbottom": "DispBottom", "openstorage": "OpenStorage", "getlook": "GetLook", "setlook": "SetLook",
    "resetstatus": None, "resetskill": "ResetSkills", "getfame": None, "setcart": "SetCart", "setfalcon": "SetFalcon",
    "setriding": "SetRiding", "unitwarp": None,
}
SDK_CALLS = {name: variant for name, variant in SDK_CALLS.items() if variant}

# rathena parameters the host answers through `Request::Read`
READABLE_PARAMETERS = {"zeny", "class", "upper", "baselevel", "joblevel", "skillpoint", "hp", "sp", "maxhp", "maxsp", "sex", "baseclass", "basejob", "weight", "maxweight"}
WRITABLE_PARAMETERS = {"zeny"}

STATUS_CALLS = {"sc_start": "StartStatus", "sc_start2": "StartStatus2", "sc_start4": "StartStatus4"}

ENDS_SCRIPT = {"end", "close", "close3"}
IGNORED_COMMANDS = {"function", "setarray_noop"}


def sdk_functions():
    """Variants of `script_sdk::Function`, the only calls the host can answer."""
    source = (pathlib.Path(__file__).resolve().parents[3] / "lib/script-sdk/src/lib.rs").read_text(encoding="utf-8")
    body = re.search(r"pub enum Function \{(.*?)^\}", source, re.S | re.M).group(1)
    return set(re.findall(r"^\s*(\w+),", body, re.M))


SDK_FUNCTIONS = sdk_functions()


def rust_string(value):
    out = ['"']
    for char in value:
        if char == "\\":
            out.append("\\\\")
        elif char == '"':
            out.append('\\"')
        elif char == "\n":
            out.append("\\n")
        elif char == "\r":
            out.append("\\r")
        elif char == "\t":
            out.append("\\t")
        elif char == "\0":
            out.append("\\0")
        else:
            out.append(char)
    out.append('"')
    return "".join(out)


def local_identifier(name):
    base = name[2:]
    string = base.endswith("$")
    return "l_" + base.rstrip("$").lower() + ("_s" if string else "")


class Context:
    """What the generator knows about the whole corpus."""

    def __init__(self, constants, parameters, functions, exnames=None):
        self.constants = constants
        self.parameters = parameters
        self.functions = functions  # lower-cased name -> rust function name
        self.exnames = exnames or {}  # rathena unique name (`Name::Unique`) -> name the engine knows the NPC by


class BodyGenerator:
    """Generates one Rust function from a script body (an NPC, an event label set, or a `function script`)."""

    def __init__(self, world, name):
        self.world = world
        self.name = name
        self.blockers = set()
        self.lines = []
        self.depth = 0
        self.locals = {}  # rust identifier -> ("scalar" | "array", is_string)
        self.labels = {}
        self.machine = False
        self.targets = []  # stack of (kind, label id) for break and continue
        self.counter = 0
        self.callsub_labels = set()
        self.helper_functions = []
        self.array_locals = set()
        self.local_functions = {}

    # ---- output
    def sdk_call(self, variant, arguments):
        if variant not in SDK_FUNCTIONS:
            self.block(f"sdk:{variant}")
        return f"ctx.call(Function::{variant}, vec![{arguments}])?"

    def emit(self, text):
        self.lines.append("    " * self.depth + text)

    def block(self, reason):
        self.blockers.add(reason)

    def fresh(self):
        self.counter += 1
        return self.counter

    # ---- variables
    def variable_kind(self, name):
        """(scope, canonical) with scope in local, character_temp, server_temp, server, account, npc, instance, character, param, constant."""
        for prefix, scope in ((".@", "local"), ("$@", "server_temp"), ("##", "account"), ("''", "instance"), ("@", "character_temp"), ("$", "server"),
                              ("#", "account"), ("'", "instance"), (".", "npc")):
            if name.startswith(prefix):
                return scope, name
        lower = name.lower()
        if lower in self.world.parameters:
            return "param", self.world.parameters[lower]
        if lower in self.world.constants:
            return "constant", self.world.constants[lower]
        return "character", name.lower()

    def declare_local(self, name, array):
        identifier = local_identifier(name)
        kind = "array" if array or identifier in self.array_locals else "scalar"
        existing = self.locals.get(identifier)
        if existing and existing[0] != kind:
            self.block(f"construct:local {name} used as scalar and array")
        self.locals[identifier] = (kind, name.endswith("$"))
        return identifier

    def persistent_name(self, name):
        """Name as the host expects it: prefix kept, case folded."""
        return rust_string(name.lower())

    def read_name(self, node):
        scope, canonical = self.variable_kind(node.name)
        if scope == "local":
            if node.name == ".@":
                self.block("construct:empty local name")
            identifier = self.declare_local(node.name, False)
            if identifier in self.array_locals:
                return f"local_get(&{identifier}, &n(0), {str(node.name.endswith('$')).lower()})"
            return f"{identifier}.clone()"
        if scope == "constant":
            return f"constant(ctx, {rust_string(canonical)})?"
        if scope == "param":
            lower = canonical.lower()
            if lower in READABLE_PARAMETERS:
                return f"get(ctx, {rust_string(canonical)})?"
            self.block(f"param:{canonical}")
            return "n(0)"
        return f"get(ctx, {self.persistent_name(node.name)})?"

    def read_index(self, node):
        scope, canonical = self.variable_kind(node.name)
        index = self.bool_free_value(node.index)
        if scope == "local":
            identifier = self.declare_local(node.name, True)
            return f"local_get(&{identifier}, &{index}, {str(node.name.endswith('$')).lower()})"
        if scope in ("constant", "param"):
            self.block(f"construct:indexed {scope}")
            return "n(0)"
        return f"get_at(ctx, {self.persistent_name(node.name)}, &{index})?"

    def bool_free_value(self, node):
        return self.value(node)

    # ---- expressions
    def literal(self, value):
        """Event targets name an NPC by its unique name; the engine finds it by its own name."""
        target, separator, label = value.partition("::")
        if separator and target in self.world.exnames:
            return f"{self.world.exnames[target]}::{label}"
        return value

    def text(self, node):
        if isinstance(node, Str):
            return rust_string(self.literal(node.value))
        return f"{self.value(node)}.text()"

    def number(self, node):
        """Rust i32 expression."""
        if isinstance(node, Num):
            return str(wrap32(node.value))
        return f"{self.value(node)}.number_value()?"

    def value(self, node):
        if isinstance(node, Num):
            return f"n({wrap32(node.value)})"
        if isinstance(node, Str):
            return f"s({rust_string(self.literal(node.value))})"
        if isinstance(node, Name):
            lower = node.name.lower()
            if lower == "true":
                return "n(1)"
            if lower == "false":
                return "n(0)"
            if lower == "null":
                return "n(0)"
            return self.read_name(node)
        if isinstance(node, Index):
            return self.read_index(node)
        if isinstance(node, Unary):
            if node.op == "!":
                return f"n(i32::from(!{self.boolean(node.operand)}))"
            if node.op == "-":
                if isinstance(node.operand, Num):
                    return f"n({wrap32(-node.operand.value)})"
                return f"op(n(0), \"-\", {self.value(node.operand)})?"
            if node.op == "+":
                return self.value(node.operand)
            return f"op({self.value(node.operand)}, \"^\", n(-1))?"
        if isinstance(node, Binary):
            if node.op in ("&&", "||"):
                return f"n(i32::from({self.boolean(node)}))"
            return f"op({self.value(node.left)}, {rust_string(node.op)}, {self.value(node.right)})?"
        if isinstance(node, Ternary):
            return f"(if {self.boolean(node.condition)} {{ {self.value(node.yes)} }} else {{ {self.value(node.no)} }})"
        if isinstance(node, Call):
            return self.function_call(node)
        if isinstance(node, (Assign, IncDec)):
            self.block("construct:assignment inside an expression")
            return "n(0)"
        self.block(f"construct:expression {type(node).__name__}")
        return "n(0)"

    def boolean(self, node):
        if isinstance(node, Binary) and node.op == "&&":
            return f"({self.boolean(node.left)} && {self.boolean(node.right)})"
        if isinstance(node, Binary) and node.op == "||":
            return f"({self.boolean(node.left)} || {self.boolean(node.right)})"
        if isinstance(node, Unary) and node.op == "!":
            return f"!{self.boolean(node.operand)}"
        if isinstance(node, Num):
            return "true" if node.value else "false"
        return f"{self.value(node)}.truthy()"

    def arguments(self, args):
        return ", ".join(self.value(argument) for argument in args)

    def function_call(self, node):
        name = node.name.lower()
        args = node.args
        if name in self.local_functions and name not in ("callfunc", "callsub", "getarg", "select"):
            return f"{self.name}(ctx, LABEL_{label_constant(self.local_functions[name])}, vec![{self.arguments(args)}])?"
        if name in self.world.functions and name not in ("callfunc", "callsub", "getarg", "select"):
            return f"{self.world.functions[name]}(ctx, 0, vec![{self.arguments(args)}])?"
        if name == "getarg":
            default = self.value(args[1]) if len(args) > 1 else "n(0)"
            return f"arg(&args, {self.number(args[0])}, {default})"
        if name == "select":
            return self.select(args)
        if name == "callfunc":
            return self.callfunc(args)
        if name == "callsub":
            return self.callsub(args)
        if name == "getguildname":
            return f"ctx.call(Function::GetGuildInfo, vec![{self.value(args[0])}, n(0)])?"
        if name == "is_guild_leader":
            guild = self.value(args[0]) if args else "ctx.call(Function::GetCharacterId, vec![n(2)])?"
            return f"ctx.call(Function::GetGuildInfo, vec![{guild}, n(2)])?"
        if name == "is_party_leader":
            party = self.value(args[0]) if args else "ctx.call(Function::GetCharacterId, vec![n(1)])?"
            return f"ctx.call(Function::IsPartyLeader, vec![{party}])?"
        if name == "killmonsterall" and len(args) == 1:
            return f"ctx.call(Function::KillMonster, vec![{self.value(args[0])}, s(\"All\")])?"
        if name == "playerattached":
            return "n(i32::from(ctx.call(Function::GetCharacterId, vec![n(0)])?.truthy()))"
        if name == "checkre":
            return "n(0)"
        if name == "getattachedrid":
            return "ctx.call(Function::GetCharacterId, vec![n(3)])?"
        if name == "basicskillcheck":
            return "n(1)"
        if name == "getargcount":
            return "n(args.len() as i32)"
        if name == "sprintf" and args:
            return f"sprintf({self.value(args[0])}, vec![{self.arguments(args[1:])}])?"
        if name == "replacestr" and 3 <= len(args) <= 5:
            return f"replacestr({self.value(args[0])}, {self.value(args[1])}, {self.value(args[2])}, &[{self.arguments(args[3:])}])?"
        if name == "implode" and len(args) in (1, 2) and isinstance(args[0], (Name, Index)) and args[0].name.startswith(".@"):
            delimiter = self.value(args[1]) if len(args) > 1 else 's("")'
            return f"implode(&{self.declare_local(args[0].name, True)}, {delimiter})?"
        if name == "getarraysize":
            if len(args) == 1 and isinstance(args[0], (Name, Index)) and args[0].name.startswith(".@"):
                identifier = self.declare_local(args[0].name, True)
                return f"n({identifier}.len() as i32)"
            if len(args) == 1 and isinstance(args[0], Name) and self.variable_kind(args[0].name)[0] in ("character", "character_temp", "account", "server", "server_temp", "npc", "instance"):
                return f"array_size(ctx, {self.persistent_name(args[0].name)})?"
            self.block("function:getarraysize on this kind of variable")
            return "n(0)"
        if name == "getvariableofnpc":
            parts = self.npc_variable_parts(args)
            return f"ctx.call(Function::GetVariableOfNpc, vec![{parts}])?" if parts else "n(0)"
        if name in STRING_FUNCTIONS:
            return f"{STRING_FUNCTIONS[name]}({self.arguments(args)})?"
        if name in ("getd", "setd"):
            return f"get(ctx, &{self.text(args[0])})?" if name == "getd" and len(args) == 1 else self.unsupported_function(name)
        if name in SDK_CALLS:
            return self.sdk_call(SDK_CALLS[name], self.arguments(args))
        if name in STATUS_CALLS:
            return self.sdk_call(STATUS_CALLS[name], self.arguments(args))
        if name in NEW_CALLS:
            return self.sdk_call(NEW_CALLS[name], self.arguments(args))
        return self.unsupported_function(name)

    def npc_variable_parts(self, args):
        """`name, npc, index` for the host, or None when the reference is not a `.variable` of another NPC."""
        target = args[0] if len(args) == 2 else None
        if not isinstance(target, (Name, Index)) or not target.name.startswith(".") or target.name.startswith(".@"):
            self.block("construct:getvariableofnpc target")
            return None
        index = self.value(target.index) if isinstance(target, Index) else "n(0)"
        return f"s({self.persistent_name(target.name)}), {self.value(args[1])}, {index}"

    def unsupported_function(self, name):
        self.block(f"function:{name}")
        return "n(0)"

    def select(self, args):
        return f"n({self.select_number(args)})"

    def select_number(self, args):
        if all(isinstance(argument, Str) for argument in args):
            options = [part for argument in args for part in argument.value.split(":") if part]
            return f"select(ctx, &[{', '.join(rust_string(option) for option in options)}])?"
        return f"select_text(ctx, &[{self.arguments(args)}])?"

    def callfunc(self, args):
        if not args or not isinstance(args[0], Str):
            self.block("construct:callfunc with a computed name")
            return "n(0)"
        target = args[0].value.lower()
        rest = self.arguments(args[1:])
        if target in self.world.functions:
            return f"{self.world.functions[target]}(ctx, 0, vec![{rest}])?"
        self.block(f"callfunc:{args[0].value}")
        return "n(0)"

    def callsub(self, args):
        if not args or not isinstance(args[0], Name):
            self.block("construct:callsub with a computed label")
            return "n(0)"
        label = args[0].name
        self.callsub_labels.add(label)
        if label not in self.labels:
            self.block(f"construct:callsub to unknown label {label}")
        return f"{self.name}(ctx, LABEL_{label_constant(label)}, vec![{self.arguments(args[1:])}])?"

    # ---- statements
    def statements(self, statements):
        for statement in statements:
            self.statement(statement)

    def body(self, statement):
        """Emit a nested statement as a braced block's contents."""
        self.depth += 1
        if isinstance(statement, Block):
            self.statements(statement.statements)
        else:
            self.statement(statement)
        self.depth -= 1

    def statement(self, node):
        if isinstance(node, Block):
            self.emit("{")
            self.body(node)
            self.emit("}")
        elif isinstance(node, ExprStatement):
            self.expression_statement(node.expression)
        elif isinstance(node, If):
            self.emit(f"if {self.boolean(node.condition)} {{")
            self.body(node.then)
            if node.otherwise is None:
                self.emit("}")
            else:
                self.emit("} else {")
                self.body(node.otherwise)
                self.emit("}")
        elif isinstance(node, (While, DoWhile, For)):
            self.loop(node)
        elif isinstance(node, Switch):
            self.switch(node)
        elif isinstance(node, Break):
            self.break_statement(False)
        elif isinstance(node, Continue):
            self.break_statement(True)
        elif isinstance(node, Return):
            self.emit(f"return Ok({self.value(node.value) if node.value else 'n(0)'});")
        elif isinstance(node, Goto):
            self.goto(node.label)
        elif isinstance(node, Label):
            self.block(f"construct:label {node.name} inside a block")
        elif isinstance(node, Menu):
            self.menu(node)
        elif isinstance(node, Case):
            self.block("construct:case outside a switch")
        elif isinstance(node, FunctionDefinition):
            self.block("construct:function definition inside a script")
        elif isinstance(node, Command):
            self.command(node)
        else:
            self.block(f"construct:statement {type(node).__name__}")

    def goto(self, label):
        if not self.machine or label not in self.labels:
            self.block(f"construct:goto {label}")
            return
        self.emit(f"pc = LABEL_{label_constant(label)}; continue 'sm;")

    def break_statement(self, is_continue):
        for kind, identifier in reversed(self.targets):
            if is_continue and kind == "switch":
                continue
            if is_continue:
                self.emit(f"break 'b{identifier};")
            else:
                self.emit(f"break 'l{identifier};" if kind == "loop" else f"break 'b{identifier};")
            return
        self.block("construct:break or continue outside a loop")

    def loop(self, node):
        identifier = self.fresh()
        if isinstance(node, For) and node.init is not None:
            self.expression_statement(node.init)
        self.emit(f"'l{identifier}: loop {{")
        self.depth += 1
        if isinstance(node, While):
            self.emit(f"if !{self.boolean(node.condition)} {{ break; }}")
        elif isinstance(node, For) and node.condition is not None:
            self.emit(f"if !{self.boolean(node.condition)} {{ break; }}")
        self.emit(f"'b{identifier}: {{")
        self.targets.append(("loop", identifier))
        self.body(node.body)
        self.targets.pop()
        self.emit("}")
        if isinstance(node, For) and node.step is not None:
            self.expression_statement(node.step)
        if isinstance(node, DoWhile):
            self.emit(f"if !{self.boolean(node.condition)} {{ break; }}")
        self.depth -= 1
        self.emit("}")

    def switch(self, node):
        identifier = self.fresh()
        cases = [item.value for item in node.body if isinstance(item, Case) and item.value is not None]
        self.emit(f"'b{identifier}: {{")
        self.depth += 1
        self.emit(f"let sw{identifier} = {self.value(node.value)};")
        self.emit(f"let mut m{identifier} = false;")
        none_match = " && ".join(f"!eq(&sw{identifier}, &{self.value(case)})" for case in cases) or "true"
        self.emit(f"let d{identifier} = {none_match};")
        self.targets.append(("switch", identifier))
        open_group = False
        for item in node.body:
            if isinstance(item, Case):
                if open_group:
                    self.depth -= 1
                    self.emit("}")
                    open_group = False
                if item.value is None:
                    self.emit(f"if !m{identifier} && d{identifier} {{ m{identifier} = true; }}")
                else:
                    self.emit(f"if !m{identifier} && eq(&sw{identifier}, &{self.value(item.value)}) {{ m{identifier} = true; }}")
                continue
            if not open_group:
                self.emit(f"if m{identifier} {{")
                self.depth += 1
                open_group = True
            self.statement(item)
        if open_group:
            self.depth -= 1
            self.emit("}")
        self.targets.pop()
        self.depth -= 1
        self.emit("}")

    def menu(self, node):
        if not self.machine:
            self.block("construct:menu")
            return
        options = []
        for text, label in node.options:
            for part in text.split(":"):
                if part:
                    options.append((part, label))
        self.emit(f"let choice = select(ctx, &[{', '.join(rust_string(text) for text, _ in options)}])?;")
        self.emit('ctx.write("@menu", n(choice))?;')
        self.emit("match choice {")
        self.depth += 1
        for number, (_, label) in enumerate(options, 1):
            if label is None:
                self.emit(f"{number} => {{}}")
            elif label in self.labels:
                self.emit(f"{number} => {{ pc = LABEL_{label_constant(label)}; continue 'sm; }}")
            else:
                self.block(f"construct:menu label {label}")
        self.emit("_ => return Err(\"Invalid menu selection\".into()),")
        self.depth -= 1
        self.emit("}")

    def expression_statement(self, node):
        if isinstance(node, Assign):
            self.assign(node.target, node.op, node.value)
        elif isinstance(node, IncDec):
            self.assign(node.target, "+=" if node.op == "++" else "-=", Num(1))
        else:
            self.block("construct:expression statement")

    def assign(self, target, op, value):
        rendered = self.value(value) if op == "=" else None
        if isinstance(target, Call):
            parts = self.npc_variable_parts(target.args) if target.name.lower() == "getvariableofnpc" else None
            if parts is None:
                self.block(f"function:{target.name} as an assignment target")
                return
            if op != "=":
                rendered = f"op({self.value(target)}, {rust_string(op[:-1])}, {self.value(value)})?"
            self.emit(f"ctx.call(Function::SetVariableOfNpc, vec![{parts}, {rendered}])?;")
            return
        scope, canonical = self.variable_kind(target.name)
        if op != "=":
            current = self.value(target)
            rendered = f"op({current}, {rust_string(op[:-1])}, {self.value(value)})?"
        if scope == "local":
            if isinstance(target, Index):
                identifier = self.declare_local(target.name, True)
                self.emit(f"local_set(&mut {identifier}, &{self.value(target.index)}, {rendered}, {str(target.name.endswith('$')).lower()});")
            else:
                identifier = self.declare_local(target.name, False)
                if identifier in self.array_locals:
                    self.emit(f"local_set(&mut {identifier}, &n(0), {rendered}, {str(target.name.endswith('$')).lower()});")
                else:
                    self.emit(f"{identifier} = {rendered};")
        elif scope == "param":
            if canonical.lower() in WRITABLE_PARAMETERS and isinstance(target, Name):
                self.emit(f"ctx.write({rust_string(canonical)}, {rendered})?;")
            else:
                self.block(f"param write:{canonical}")
        elif scope == "constant":
            self.block(f"construct:assignment to {scope}")
        elif isinstance(target, Index):
            self.emit(f"set_at(ctx, {self.persistent_name(target.name)}, &{self.value(target.index)}, {rendered})?;")
        else:
            self.emit(f"set(ctx, {self.persistent_name(target.name)}, {rendered})?;")

    def command(self, node):
        name = node.name.lower()
        args = node.args
        if name in ("enable_items", "disable_items", "logmes", "refineui"):
            return
        if name == "npcskill" and len(args) == 4:
            self.emit(f"npc_skill(ctx, {self.value(args[0])}, {self.value(args[1])}, {self.value(args[2])}, {self.value(args[3])})?;")
            return
        if name == "unitwarp" and len(args) == 4 and isinstance(args[0], Num) and args[0].value == 0:
            self.emit(f"ctx.call(Function::Warp, vec![{self.arguments(args[1:])}])?;")
            return
        if name == "explode" and len(args) == 3 and isinstance(args[0], (Name, Index)) and args[0].name.startswith(".@"):
            identifier = self.declare_local(args[0].name, True)
            self.emit(f"{identifier} = explode({self.value(args[1])}, {self.value(args[2])})?;")
            return
        if name in self.local_functions and name not in ("callfunc", "callsub", "select"):
            self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
            return
        if name in self.world.functions and name not in ("callfunc", "callsub", "select"):
            self.emit(f"{self.world.functions[name]}(ctx, 0, vec![{self.arguments(args)}])?;")
            return
        if name in ("getguildname", "is_guild_leader", "killmonsterall", "is_party_leader"):
            self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
            return
        if name == "mes":
            if len(args) == 1:
                self.emit(f"ctx.mes({self.text(args[0])})?;")
            else:
                self.block("command:mes with several arguments")
        elif name == "next":
            self.emit("ctx.next()?;")
        elif name in ("close", "close2", "close3"):
            self.emit("ctx.close()?;")
            if name == "close3":
                self.emit('ctx.call(Function::Cutin, vec![s(""), n(255)])?;')
            if name != "close2":
                self.emit("return Err(END.into());")
        elif name == "end":
            self.emit("return Err(END.into());")
        elif name == "set":
            self.block("construct:set")
        elif name == "getpartymember":
            self.emit(f"party_members(ctx, {self.value(args[0])}, {self.value(args[1]) if len(args) > 1 else 'n(0)'})?;")
        elif name == "getmapxy":
            self.getmapxy(args)
        elif name == "setarray":
            self.setarray(args)
        elif name == "deletearray":
            self.deletearray(args)
        elif name == "input":
            self.input(args)
        elif name == "callsub":
            self.emit(f"{self.callsub(args)};")
        elif name == "callfunc":
            self.emit(f"{self.callfunc(args)};")
        elif name == "select":
            self.emit(f"let choice = {self.select_number(args)};")
            self.emit('ctx.write("@menu", n(choice))?;')
        elif name == "function":
            pass
        elif name in ("setd",):
            if len(args) == 2:
                self.emit(f"set(ctx, &{self.text(args[0])}.to_lowercase(), {self.value(args[1])})?;")
            else:
                self.block("command:setd")
        elif name in NEW_CALLS:
            self.emit(f"{self.sdk_call(NEW_CALLS[name], self.arguments(args))};")
        elif name in SDK_CALLS or name in STATUS_CALLS:
            self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
        else:
            self.block(f"command:{name}")

    def getmapxy(self, args):
        if len(args) < 4 or not all(isinstance(argument, (Name, Index)) for argument in args[:3]):
            self.block("command:getmapxy")
            return
        self.emit(f"let Value::Array(position) = ctx.call(Function::GetMapXy, vec![{self.arguments(args[3:])}])? else {{ return Err(\"Invalid position\".into()); }};")
        for number, target in enumerate(args[:3]):
            self.assign_rendered(target, f"position[{number}].clone()")

    def setarray(self, args):
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:setarray")
            return
        target = args[0]
        start = self.value(target.index) if isinstance(target, Index) else "n(0)"
        scope, _ = self.variable_kind(target.name)
        if scope == "local":
            identifier = self.declare_local(target.name, True)
            self.emit(f"let base = {start}.number_value()?;")
            for offset, argument in enumerate(args[1:]):
                self.emit(f"local_set(&mut {identifier}, &n(base + {offset}), {self.value(argument)}, {str(target.name.endswith('$')).lower()});")
        elif scope in ("character", "character_temp", "account", "server", "server_temp", "npc", "instance"):
            self.emit(f"let base = {start}.number_value()?;")
            for offset, argument in enumerate(args[1:]):
                self.emit(f"set_at(ctx, {self.persistent_name(target.name)}, &n(base + {offset}), {self.value(argument)})?;")
        else:
            self.block("command:setarray target")

    def deletearray(self, args):
        if len(args) >= 1 and isinstance(args[0], (Name, Index)) and args[0].name.startswith(".@") and len(args) == 1:
            identifier = self.declare_local(args[0].name, True)
            self.emit(f"{identifier}.clear();")
        else:
            self.block("command:deletearray")

    def input(self, args):
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:input")
            return
        target = args[0]
        string = target.name.rstrip().endswith("$")
        function = "InputString" if string else "InputNumber"
        extra = ", ".join(self.value(argument) for argument in args[1:])
        self.emit(f"let input = ctx.call(Function::{function}, vec![{extra}])?;")
        self.assign_rendered(target, "input")

    def assign_rendered(self, target, rendered):
        scope, _ = self.variable_kind(target.name)
        if scope == "local" and isinstance(target, Name):
            identifier = self.declare_local(target.name, False)
            if identifier in self.array_locals:
                self.emit(f"local_set(&mut {identifier}, &n(0), {rendered}, {str(target.name.endswith('$')).lower()});")
            else:
                self.emit(f"{identifier} = {rendered};")
        elif scope in ("character", "character_temp", "account", "server", "server_temp", "npc", "instance") and isinstance(target, Name):
            self.emit(f"set(ctx, {self.persistent_name(target.name)}, {rendered})?;")
        else:
            self.block("command:input target")


def label_constant(label):
    return re.sub(r"[^A-Za-z0-9]", "_", label).upper()


def wrap32(value):
    value &= 0xFFFFFFFF
    return value - (1 << 32) if value & 0x80000000 else value


# rathena commands and functions without a script-sdk variant yet; sdk_call reports the missing ones
NEW_CALLS = {
    "emotion": "Emotion", "mapannounce": "MapAnnounce", "soundeffect": "SoundEffect", "soundeffectall": "SoundEffectAll",
    "viewpoint": "ViewPoint", "disable_items": "DisableItems",
    "npcskill": "NpcSkill", "consumeitem": "ConsumeItem", "setnpcdisplay": "SetNpcDisplay",
    "waitingroom": "WaitingRoom", "nude": "Nude", "disable_items_noop": "Nope", "checkweight": "CheckWeight", "strnpcinfo": "StrNpcInfo", "getmapusers": "GetMapUsers",
    "getareausers": "GetAreaUsers",
    "getnameditem": "GetNamedItem",
    "makeitem": "MakeItem", "getcastledata": "GetCastleData", "setcastledata": "SetCastleData", "gettimetick": "GetTimeTick",
    "gettimestr": "GetTimeStr", "unitwarp": "UnitWarp", "countitem2": "CountItem2", "delitem2": "DelItem2",
    "getitem2": "GetItem2", "getequipname": "GetEquipName", "getequipweaponlv": "GetEquipWeaponLevel", "getequipcardid": "GetEquipCardId",
    "getequippercentrefinery": "GetEquipPercentRefinery", "getequipisenableref": "GetEquipIsEnableRefine",
    "getequipisequiped": "GetEquipIsEquipped", "getbrokenid": "GetBrokenId", "repairall": "RepairAll", "successrefitem": "SuccessRefineItem",
    "failedrefitem": "FailedRefineItem", "getinventorylist": "GetInventoryList", "readbook": "ReadBook", "convertpcinfo": "ConvertPcInfo",
    "getattachedrid": "GetAttachedRid", "implode": "Implode", "enablewaitingroomevent": "EnableWaitingRoomEvent",
    "disablewaitingroomevent": "DisableWaitingRoomEvent", "warpwaitingpc": "WarpWaitingPc", "sleep": "Sleep", "sleep2": "Sleep", "progressbar": "ProgressBar",
    "getvariableofnpc": "GetVariableOfNpc",
    "areamonster": "AreaMonster", "getpartyname": "GetPartyName", "instance_create": "InstanceCreate", "instance_destroy": "InstanceDestroy",
    "instance_enter": "InstanceEnter", "instance_npcname": "InstanceNpcName", "instance_mapname": "InstanceMapName", "instance_id": "InstanceId",
    "instance_warpall": "InstanceWarpAll", "instance_announce": "InstanceAnnounce", "instance_check_party": "InstanceCheckParty",
    "instance_check_guild": "InstanceCheckGuild", "instance_info": "InstanceInfo", "instance_live_info": "InstanceLiveInfo", "instance_list": "InstanceList",
    "getequiparmorlv": "GetEquipArmorLevel", "getequiprefinecost": "GetEquipRefineCost", "downrefitem": "DownRefineItem", "repair": "Repair",
    "getbattleflag": "GetBattleFlag",
}
for _name in ("getcastledata", "setcastledata"):
    SDK_CALLS.pop(_name, None)

STRING_FUNCTIONS = {"getstrlen": "strlen", "substr": "substr", "charat": "charat", "atoi": "atoi", "compare": "compare",
                    "strtolower": "strtolower", "strtoupper": "strtoupper", "countstr": "countstr", "delchar": "delchar"}


def expand_local_functions(statements):
    """`function Name { }` inside a script is a label with arguments that is jumped over when execution falls into it."""
    expanded, names = [], {}
    for statement in statements:
        if isinstance(statement, FunctionDefinition):
            names[statement.name.lower()] = statement.name
            skip = f"__after_{statement.name}"
            expanded += [Goto(skip), Label(statement.name), *statement.body.statements, Return(None), Label(skip)]
        else:
            expanded.append(statement)
    return expanded, names


def generate_function(world, name, body, entry_labels=()):
    """Returns (rust source, blockers, labels). `body` is a statement list."""
    generator = BodyGenerator(world, name)
    statements, generator.local_functions = expand_local_functions(list(body))
    generator.array_locals = find_array_locals(statements)
    labels = [statement.name for statement in statements if isinstance(statement, Label)]
    generator.labels = {label: index + 1 for index, label in enumerate(labels)}
    generator.machine = bool(labels) or any(isinstance(statement, (Goto, Menu)) for statement in walk(statements))
    blocks = [[]]
    for statement in statements:
        if isinstance(statement, Label):
            blocks.append([])
        else:
            blocks[-1].append(statement)
    generator.depth = 1
    if generator.machine:
        generator.depth = 3
        for index, block in enumerate(blocks):
            generator.emit(f"{index} => {{")
            generator.depth += 1
            generator.statements(block)
            generator.emit(f"pc = {index + 1};")
            generator.depth -= 1
            generator.emit("}")
        generator.depth = 3
    else:
        generator.statements(blocks[0])
    body_lines = generator.lines
    declarations = []
    for identifier, (kind, string) in sorted(generator.locals.items()):
        if kind == "array":
            declarations.append(f"    let mut {identifier}: Vec<Value> = Vec::new();")
        else:
            declarations.append(f"    let mut {identifier} = {'s(\"\")' if string else 'n(0)'};")
    constants = [f"const LABEL_{label_constant(label)}: usize = {index};" for label, index in generator.labels.items()]
    out = []
    out.append(f"fn {name}(ctx: &Context, mut pc: usize, args: Vec<Value>) -> Result<Value, String> {{")
    out.extend("    " + constant for constant in constants)
    out.extend(declarations)
    if generator.machine:
        out.append("    'sm: loop {")
        out.append("        match pc {")
        out.extend(["    " + line for line in body_lines])
        out.append(f"            {len(blocks)} => return Ok(n(0)),")
        out.append("            _ => return Err(\"Invalid script position\".into()),")
        out.append("        }")
        out.append("    }")
    else:
        out.append("    let _ = pc;")
        out.extend(body_lines)
        out.append("    Ok(n(0))")
    out.append("}")
    return "\n".join(out), generator.blockers, generator.labels


def find_array_locals(statements):
    """Local variables used as arrays somewhere: rathena treats `.@x` as element 0 of `.@x[]`, so every use becomes an array access."""
    found = set()

    def visit(node):
        if isinstance(node, (list, tuple)):
            for item in node:
                visit(item)
            return
        if not hasattr(node, "__dataclass_fields__"):
            return
        if isinstance(node, Index) and node.name.startswith(".@"):
            found.add(local_identifier(node.name))
        if isinstance(node, Command) and node.name.lower() in ("setarray", "deletearray", "copyarray", "explode") and node.args:
            first = node.args[0]
            if isinstance(first, (Name, Index)) and first.name.startswith(".@"):
                found.add(local_identifier(first.name))
        if isinstance(node, Call) and node.name.lower() in ("getarraysize", "implode") and node.args and isinstance(node.args[0], (Name, Index)) and node.args[0].name.startswith(".@"):
            found.add(local_identifier(node.args[0].name))
        for field in node.__dataclass_fields__:
            visit(getattr(node, field))

    visit(statements)
    return found


def walk(statements):
    """Every statement node below `statements`, including nested bodies."""
    stack = list(statements)
    while stack:
        node = stack.pop()
        yield node
        if isinstance(node, Block):
            stack.extend(node.statements)
        elif isinstance(node, If):
            stack.append(node.then)
            if node.otherwise is not None:
                stack.append(node.otherwise)
        elif isinstance(node, (While, For, DoWhile)):
            stack.append(node.body)
        elif isinstance(node, Switch):
            stack.extend(node.body)
