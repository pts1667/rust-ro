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

# Dragons, Wargs and Mado Gear belong to renewal jobs: the commands do nothing and the queries answer "not mounted" on a pre-renewal server.
# set by the host before it runs `OnPCKillEvent` and `OnPCDieEvent`, readable as character temporary variables
EVENT_VARIABLES = {"killedrid", "killerrid"}
# Wug and Mado Gear options belong to renewal jobs
RENEWAL_OPTIONS = {"option_wug", "option_wugrider", "option_madogear"}
STORED_SCOPES = ("character", "character_temp", "account", "server", "server_temp", "npc", "instance")
LOCALS_REF = "@@LOCALS_REF@@"
LOCALS_MUT = "@@LOCALS_MUT@@"

RENEWAL_MOUNT_COMMANDS = {"setdragon", "setmadogear", "setwug"}
RENEWAL_MOUNT_QUERIES = {"checkdragon", "checkwug"}

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
        self.local_names = {}  # rust identifier -> variable name as `getd` and `setd` spell it
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

    def known_label(self, label):
        """rathena label names are case-insensitive, and so are the generated `LABEL_*` constants."""
        return any(known.lower() == label.lower() for known in self.labels)

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
        if lower in EVENT_VARIABLES:
            return "character_temp", name
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
        self.local_names[identifier] = name.lower()
        return identifier

    def persistent_name(self, name):
        """Name as the host expects it: prefix kept, case folded."""
        return rust_string("@" + name.lower() if name.lower() in EVENT_VARIABLES else name.lower())

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
        if isinstance(node, Ternary) and self.static_truth(node.condition) is not None:
            return self.value(node.yes if self.static_truth(node.condition) else node.no)
        if isinstance(node, Ternary):
            return f"(if {self.boolean(node.condition)} {{ {self.value(node.yes)} }} else {{ {self.value(node.no)} }})"
        if isinstance(node, Call):
            return self.function_call(node)
        if isinstance(node, (Assign, IncDec)):
            return self.assignment_expression(node)
        self.block(f"construct:expression {type(node).__name__}")
        return "n(0)"

    def statements_as_expression(self, produce, result):
        """Rust block expression running the statements `produce` emits, then evaluating `result`."""
        saved_lines, saved_depth = self.lines, self.depth
        self.lines, self.depth = [], 0
        produce()
        emitted = " ".join(line.strip() for line in self.lines)
        self.lines, self.depth = saved_lines, saved_depth
        return f"{{ {emitted} {result} }}"

    def assignment_expression(self, node):
        """`a = b` or `x++` used as a value: the new value, or the old one for a postfix `x++`."""
        if isinstance(node, IncDec):
            op, target = ("+=" if node.op == "++" else "-="), node.target
            if node.prefix:
                return self.statements_as_expression(lambda: self.assign(target, op, Num(1)), self.value(target))
            return self.statements_as_expression(lambda: (self.emit(f"let previous = {self.value(target)};"), self.assign(target, op, Num(1))), "previous")
        return self.statements_as_expression(lambda: self.assign(node.target, node.op, node.value), self.value(node.target))

    def static_truth(self, node):
        """True or False when a condition is fixed on a pre-renewal server (`checkre(...)`), None otherwise."""
        if isinstance(node, Call) and node.name.lower() in ("checkre",):
            return False
        if isinstance(node, Unary) and node.op == "!":
            inner = self.static_truth(node.operand)
            return None if inner is None else not inner
        if isinstance(node, Binary) and node.op in ("&&", "||"):
            left, right = self.static_truth(node.left), self.static_truth(node.right)
            if node.op == "&&":
                return False if left is False or right is False else (True if left and right else None)
            return True if left or right else (False if left is False and right is False else None)
        return None

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
        if name == "input" and args and isinstance(args[0], (Name, Index)):
            return self.input_expression(args)
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
        if name == "checkre" or name in RENEWAL_MOUNT_QUERIES:
            return "n(0)"
        if name == "checkoption" and len(args) == 1 and isinstance(args[0], Name) and args[0].name.lower() in RENEWAL_OPTIONS:
            return "n(0)"
        if name == "set" and len(args) == 2 and isinstance(args[0], (Name, Index)):
            return self.assignment_expression(Assign("=", args[0], args[1]))
        if name == "rid2name" and len(args) == 1:
            return f"ctx.call(Function::ConvertPcInfo, vec![{self.value(args[0])}, n(0)])?"
        if name == "strpos" and len(args) in (2, 3):
            return f"strpos({self.value(args[0])}, {self.value(args[1])}, &[{self.arguments(args[2:])}])?"
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
        if name == "implode" and len(args) in (1, 2) and isinstance(args[0], (Name, Index)):
            delimiter = self.value(args[1]) if len(args) > 1 else 's("")'
            if args[0].name.startswith(".@"):
                return f"implode(&{self.declare_local(args[0].name, True)}, {delimiter})?"
            if self.variable_kind(args[0].name)[0] in STORED_SCOPES:
                return f"array_implode(ctx, {self.persistent_name(args[0].name)}, {delimiter})?"
        if name == "getarraysize":
            if len(args) == 1 and isinstance(args[0], Call) and args[0].name.lower() == "getd" and len(args[0].args) == 1:
                return f"getd_size(ctx, &{self.value(args[0].args[0])}, &{LOCALS_REF})?"
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
        if name == "getd" and len(args) == 1:
            return f"getd(ctx, &{self.value(args[0])}, &{LOCALS_REF})?"
        if name == "setd":
            return self.unsupported_function(name)
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
        if not self.known_label(label):
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
        elif isinstance(node, If) and self.static_truth(node.condition) is not None and not contains_label(node):
            branch = node.then if self.static_truth(node.condition) else node.otherwise
            if branch is not None:
                self.statement(branch)
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
        if not self.machine or not self.known_label(label):
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
            elif self.known_label(label):
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
        if isinstance(value, Assign):
            self.assign(value.target, value.op, value.value)
            value = value.target
        rendered = self.value(value) if op == "=" else None
        if isinstance(target, Call) and target.name.lower() == "getd" and len(target.args) == 1:
            if op != "=":
                value = Binary(op[:-1], target, value)
            self.setd(target.args[0], value)
            return
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
                self.emit(f"{{ let assigned = {rendered}; let position = {self.value(target.index)}; local_set(&mut {identifier}, &position, assigned, {str(target.name.endswith('$')).lower()});}}")
            else:
                identifier = self.declare_local(target.name, False)
                if identifier in self.array_locals:
                    self.emit(f"{{ let assigned = {rendered}; local_set(&mut {identifier}, &n(0), assigned, {str(target.name.endswith('$')).lower()});}}")
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
        if name in ("enable_items", "disable_items", "logmes", "refineui", "freeloop", "debugmes"):
            return
        if name in RENEWAL_MOUNT_COMMANDS:
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
        elif name == "copyarray":
            self.copyarray(args)
        elif name == "setoption" and args and isinstance(args[0], Name) and args[0].name.lower() in RENEWAL_OPTIONS:
            pass
        elif name == "sscanf":
            self.sscanf(args)
        elif name == "getinventorylist":
            self.emit("inventory_list(ctx)?;")
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
        elif name == "setd":
            if len(args) == 2:
                self.setd(args[0], args[1])
            else:
                self.block("command:setd")
        elif name in NEW_CALLS:
            self.emit(f"{self.sdk_call(NEW_CALLS[name], self.arguments(args))};")
        elif name in SDK_CALLS or name in STATUS_CALLS:
            self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
        else:
            self.block(f"command:{name}")

    def sscanf(self, args):
        if len(args) < 3 or not all(isinstance(target, (Name, Index)) for target in args[2:]):
            self.block("command:sscanf")
            return
        self.emit(f"let scanned = sscanf({self.value(args[0])}, {self.value(args[1])});")
        for position, target in enumerate(args[2:]):
            self.emit(f"if let Some(value) = scanned.get({position}).cloned() {{")
            self.depth += 1
            self.assign_rendered(target, "value")
            self.depth -= 1
            self.emit("}")

    def setd(self, name, value):
        self.emit(f"setd(ctx, &{self.value(name)}, {self.value(value)}, &mut {LOCALS_MUT})?;")

    def getmapxy(self, args):
        if len(args) < 3 or not all(isinstance(argument, (Name, Index)) for argument in args[:3]):
            self.block("command:getmapxy")
            return
        unit = self.arguments(args[3:]) or "n(0)"
        self.emit(f"let Value::Array(position) = ctx.call(Function::GetMapXy, vec![{unit}])? else {{ return Err(\"Invalid position\".into()); }};")
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
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:deletearray")
            return
        target = args[0]
        start = self.value(target.index) if isinstance(target, Index) else "n(0)"
        count = f"&[{self.value(args[1])}]" if len(args) > 1 else "&[]"
        scope = self.variable_kind(target.name)[0]
        if scope == "local":
            self.emit(f"local_delete(&mut {self.declare_local(target.name, True)}, &{start}, {count})?;")
        elif scope in STORED_SCOPES:
            self.emit(f"array_delete(ctx, {self.persistent_name(target.name)}, &{start}, {count})?;")
        else:
            self.block("command:deletearray")

    def array_values(self, target):
        """Rust expression for every entry of the array `target` names."""
        scope = self.variable_kind(target.name)[0]
        if scope == "local":
            return f"{self.declare_local(target.name, True)}.clone()"
        if scope in STORED_SCOPES:
            return f"array_values(ctx, {self.persistent_name(target.name)})?"
        self.block("command:copyarray")
        return "Vec::new()"

    def copyarray(self, args):
        if len(args) != 3 or not all(isinstance(argument, (Name, Index)) for argument in args[:2]):
            self.block("command:copyarray")
            return
        destination, source = args[0], args[1]
        string = str(destination.name.endswith("$")).lower()
        first = lambda target: self.value(target.index) if isinstance(target, Index) else "n(0)"
        values = f"slice_of(&{self.array_values(source)}, &{first(source)}, &{self.value(args[2])}, {string})?"
        scope = self.variable_kind(destination.name)[0]
        if scope == "local":
            self.emit(f"local_splice(&mut {self.declare_local(destination.name, True)}, &{first(destination)}, {values}, {string})?;")
        elif scope in STORED_SCOPES:
            self.emit(f"array_splice(ctx, {self.persistent_name(destination.name)}, &{first(destination)}, {values})?;")
        else:
            self.block("command:copyarray")

    def input(self, args):
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:input")
            return
        self.input_statements(args)

    def input_statements(self, args):
        """Reads the input box into the variable and leaves the status (-1 below the minimum, 1 above the maximum) in `status`."""
        target = args[0]
        function = "input_text" if target.name.rstrip().endswith("$") else "input_number"
        bounds = ", ".join(self.number(argument) for argument in args[1:3])
        self.emit(f"let (input, status) = {function}(ctx, &[{bounds}])?;")
        self.assign_rendered(target, "input")

    def input_expression(self, args):
        return self.statements_as_expression(lambda: self.input_statements(args), "n(status)")

    def assign_rendered(self, target, rendered):
        scope, _ = self.variable_kind(target.name)
        if scope == "local" and isinstance(target, Name):
            identifier = self.declare_local(target.name, False)
            if identifier in self.array_locals:
                self.emit(f"{{ let assigned = {rendered}; local_set(&mut {identifier}, &n(0), assigned, {str(target.name.endswith('$')).lower()});}}")
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
    "waitingroom": "WaitingRoom", "delwaitingroom": "DelWaitingRoom", "waitingroomkick": "WaitingRoomKick", "kickwaitingroomall": "WaitingRoomKickAll",
    "getwaitingroomstate": "GetWaitingRoomState", "getgmlevel": "GetGmLevel", "openmail": "OpenMail", "openauction": "OpenAuction",
    "nude": "Nude", "disable_items_noop": "Nope", "checkweight": "CheckWeight", "strnpcinfo": "StrNpcInfo", "getmapusers": "GetMapUsers",
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
    "getbattleflag": "GetBattleFlag", "ismounting": "IsMounting", "checkmadogear": "CheckMadogear", "guildopenstorage": "GuildOpenStorage",
    "warpparty": "PartyWarp", "itemskill": "ItemSkill", "unitskilluseid": "UnitSkillToId", "removemapflag": "RemoveMapFlag",
    "setmapflag": "SetMapFlag", "getsavepoint": "GetSavePoint", "resetlvl": "ResetLevel", "pushpc": "PushPc", "areaannounce": "AreaAnnounce",
    "mercenary_get_faith": "MercenaryGetFaith", "mercenary_set_faith": "MercenarySetFaith", "unloadnpc": "UnloadNpc", "movenpc": "MoveNpc",
    "getmonsterinfo": "GetMonsterInfo", "npctalk": "NpcTalk", "cloakonnpc": "DisableNpc", "cloakoffnpc": "EnableNpc",
}
for _name in ("getcastledata", "setcastledata"):
    SDK_CALLS.pop(_name, None)

STRING_FUNCTIONS = {"insertchar": "insertchar", "charisalpha": "charisalpha", "getstrlen": "strlen", "substr": "substr", "charat": "charat", "atoi": "atoi", "compare": "compare",
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


def contains_label(node):
    if isinstance(node, (list, tuple)):
        return any(contains_label(item) for item in node)
    if isinstance(node, Label):
        return True
    if not hasattr(node, "__dataclass_fields__"):
        return False
    return any(contains_label(getattr(node, field)) for field in node.__dataclass_fields__)


def has_label(statements):
    return any(isinstance(statement, Label) for statement in statements)


def hoist_nested_labels(statements, counter):
    """rathena labels are positions: one inside an `if` or a block can be entered from outside and runs on to the end of the enclosing block.
    Moves such labels to the top level, with jumps that keep every other path as it was; labels in loops and switches stay where they are."""
    out = []
    for statement in statements:
        if isinstance(statement, Block):
            inner = hoist_nested_labels(statement.statements, counter)
            out += inner if has_label(inner) else [Block(inner)]
        elif isinstance(statement, If):
            branches = [hoist_nested_labels([statement.then], counter), hoist_nested_labels([statement.otherwise], counter) if statement.otherwise else None]
            if not any(branch and has_label(branch) for branch in branches):
                out.append(statement)
                continue
            counter[0] += 1
            end = f"__hoist_end_{counter[0]}"
            heads, tails = [], []
            for branch in branches:
                if branch is None or not has_label(branch):
                    heads.append(Block(branch) if branch is not None else None)
                    continue
                split = next(index for index, item in enumerate(branch) if isinstance(item, Label))
                heads.append(Block(branch[:split] + [Goto(branch[split].name)]))
                tails.append(branch[split:])
            out.append(If(statement.condition, heads[0], heads[1]))
            out.append(Goto(end))
            for tail in tails:
                out += tail + [Goto(end)]
            out.append(Label(end))
        elif isinstance(statement, Switch) and (split := switch_label_split(statement)):
            counter[0] += 1
            end = f"__hoist_end_{counter[0]}"
            head, label, tail = split
            out += [Switch(statement.value, head), Goto(end), label, *tail, Label(end)]
        else:
            out.append(statement)
    return out


def switch_label_split(switch):
    """(switch body with the labelled tail cut out, the label, the tail) for a label inside a `case` whose tail never falls out of the switch."""
    body = switch.body
    for position, item in enumerate(body):
        if not isinstance(item, Label):
            continue
        end = next((index for index in range(position + 1, len(body)) if isinstance(body[index], Case)), len(body))
        tail = body[position + 1:end]
        last = tail[-1] if tail else None
        terminal = isinstance(last, (Goto, Return)) or (isinstance(last, ExprStatement) and isinstance(last.expression, Command) and last.expression.name.lower() in ("close", "end")) or (isinstance(last, Command) and last.name.lower() in ("close", "end"))
        if not terminal or any(isinstance(node, Break) for node in walk(tail)):
            return None
        return body[:position] + [Goto(item.name)] + body[end:], item, tail
    return None


def generate_function(world, name, body, entry_labels=()):
    """Returns (rust source, blockers, labels). `body` is a statement list."""
    generator = BodyGenerator(world, name)
    statements, generator.local_functions = expand_local_functions(list(body))
    statements = hoist_nested_labels(statements, [0])
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
    def table(wrapper, borrow):
        return ", ".join(
            f'({rust_string(generator.local_names[identifier])}, {wrapper}::{"Array" if kind == "array" else "Scalar"}({borrow}{identifier}))'
            for identifier, (kind, _) in sorted(generator.locals.items())
        )

    body_lines = [line.replace(LOCALS_REF, f"[{table('LocalRef', '&')}]").replace(LOCALS_MUT, f"[{table('LocalMut', '&mut ')}]") for line in generator.lines]
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
