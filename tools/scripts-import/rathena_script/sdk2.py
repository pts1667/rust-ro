"""Lowers parsed rathena scripts to readable script-sdk-2 Rust.

A script with no labels, `goto`, `callsub` or local functions becomes plain functions: one for the main dialogue and one per
`On...` event label. Anything else becomes a state machine: one `Step` enum with a variant per label, and one `run` function
that dispatches on it. Locals are declared where they are used, and every call runs with its own locals.

Each rathena command issues the same host requests in the same order as the retired numeric generator did; the trace snapshot
of `server/src/server/script/npc_trace.rs` holds the scripts to that.
"""

import re

from .codegen import (EVENT_VARIABLES, NEW_CALLS, fold_is_function, READABLE_PARAMETERS, RENEWAL_MOUNT_COMMANDS, RENEWAL_MOUNT_QUERIES, RENEWAL_OPTIONS, SDK_CALLS,
                      SDK_FUNCTIONS, STATUS_CALLS, STORED_SCOPES, WRITABLE_PARAMETERS, expand_local_functions, find_array_locals,
                      hoist_nested_labels, local_identifier, rust_string, walk, wrap32, wrap_backward_labels)
from .parser import (Assign, BackLoop, Binary, Block, Break, Call, Case, Command, Continue, DoWhile, ExprStatement, For, FunctionDefinition, Goto,
                     If, IncDec, Index, Label, Menu, Name, Num, Return, Str, Switch, Ternary, Unary, While)

COMPARISONS = ("==", "!=", "<", "<=", ">", ">=")
ARITHMETIC_METHODS = {"-": "try_sub", "*": "try_mul", "/": "try_div", "%": "try_rem"}
LOCALS_REF = "@@LOCALS_REF@@"
LOCALS_MUT = "@@LOCALS_MUT@@"
# Commands and functions that take plain values and return one, lowered to the `runtime` helper of the same name
STRING_CALLS = {
    "insertchar": ("insertchar", True), "charisalpha": ("charisalpha", True), "getstrlen": ("strlen", False), "substr": ("substr", True),
    "charat": ("charat", True), "atoi": ("atoi", False), "compare": ("compare", False), "strtolower": ("strtolower", False),
    "strtoupper": ("strtoupper", False), "countstr": ("countstr", False), "delchar": ("delchar", False),
}
FALLIBLE_RUNTIME = {"insertchar", "charisalpha", "substr", "charat"}
ON_LABEL = re.compile(r"^On", re.IGNORECASE)


class Unsupported(Exception):
    """The script uses a construct this lowering does not translate."""


RUST_KEYWORDS = {"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
                 "match", "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
                 "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
                 "typeof", "unsized", "virtual", "yield", "try"}


def integer_literal(node):
    """The i32 a number literal or a negated one stands for, as Rust source; None for anything else."""
    if isinstance(node, Num):
        return str(wrap32(node.value))
    if isinstance(node, Unary) and node.op == "-" and isinstance(node.operand, Num):
        return str(wrap32(-node.operand.value))
    return None


def fn_name(text):
    """A Rust identifier for a rathena name: lower case, `_` for every other character, never a keyword or a leading digit."""
    name = re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_") or "script"
    if name[0].isdigit():
        name = f"s_{name}"
    return f"{name}_" if name in RUST_KEYWORDS else name


def step_name(label):
    """`Step` variant of a rathena label: `L_Potions` becomes `LPotions`, `1a` becomes `S1a`."""
    name = "".join(part[:1].upper() + part[1:] for part in re.split(r"[^A-Za-z0-9]+", label) if part) or "Step"
    return f"S{name}" if name[0].isdigit() else name


def terminal(node):
    """Whether control never continues past `node`: an `end`, `close`, `return` or `goto` on every path."""
    if isinstance(node, Command):
        return node.name.lower() in ("end", "close", "close3")
    if isinstance(node, (Return, Goto)):
        return True
    if isinstance(node, Block):
        return bool(node.statements) and terminal(node.statements[-1])
    if isinstance(node, If):
        return node.otherwise is not None and terminal(node.then) and terminal(node.otherwise)
    return False


def is_mes(node):
    """A `mes` statement with one argument: one line of dialogue."""
    return isinstance(node, Command) and node.name.lower() == "mes" and len(node.args) == 1


def speaker_title(node):
    """The name of a `[Name]` title line: a text literal, or `"[" + name + "]"`. Returns the name node or text, else None."""
    if isinstance(node, Str):
        match = re.fullmatch(r"\[([^\[\]]+)\]", node.value)
        return match.group(1) if match else None
    if isinstance(node, Binary) and node.op == "+" and isinstance(node.right, Str) and node.right.value == "]":
        inner = node.left
        if isinstance(inner, Binary) and inner.op == "+" and isinstance(inner.left, Str) and inner.left.value == "[":
            return inner.right
    return None


class Emitter:
    """Writes the Rust of one body: a plain function or a state machine."""

    def __init__(self, world, base, step_type, machine):
        self.world = world
        self.base = base
        self.step_type = step_type
        self.machine = machine
        self.blockers = set()
        self.lines = []
        self.depth = 1
        self.locals = {}  # rust identifier -> (kind, string)
        self.local_names = {}  # rust identifier -> rathena name, lower case, as getd and setd spell it
        self.labels = {}  # lower-case label -> spelling
        self.variants = {}  # lower-case label -> its `Step` variant, unique within the body
        self.array_locals = set()
        self.local_functions = {}  # lower-case name -> label spelling
        self.targets = []  # (kind, id) for break and continue
        self.back_targets = {}  # label -> loop id of the `BackLoop` being emitted
        self.counter = 0

    # ---- output

    def emit(self, text):
        self.lines.append("    " * self.depth + text)

    def block(self, reason):
        self.blockers.add(reason)

    def fresh(self):
        self.counter += 1
        return self.counter

    def known_label(self, label):
        return label.lower() in self.labels

    def variant(self, label):
        return self.variants[label.lower()]

    def step(self, label):
        return f"{self.step_type}::{self.variant(label)}"

    # ---- variables

    def variable_kind(self, name):
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

    def persistent(self, name):
        """Name as the host expects it: prefix kept, case folded."""
        return rust_string("@" + name.lower() if name.lower() in EVENT_VARIABLES else name.lower())

    def read_name(self, node):
        scope, canonical = self.variable_kind(node.name)
        if scope == "local":
            if node.name == ".@":
                self.block("construct:empty local name")
            identifier = self.declare_local(node.name, False)
            if identifier in self.array_locals:
                return f"runtime::local_get(&{identifier}, &Val::from(0), {str(node.name.endswith('$')).lower()})"
            return f"{identifier}.clone()"
        if scope == "constant":
            return f"ctx.constant({rust_string(canonical)})?"
        if scope == "param":
            if canonical.lower() in READABLE_PARAMETERS:
                return f"ctx.var({rust_string(canonical)}).get()?"
            self.block(f"param:{canonical}")
            return "Val::from(0)"
        return f"ctx.var({self.persistent(node.name)}).get()?"

    def read_index(self, node):
        scope, canonical = self.variable_kind(node.name)
        index = self.value(node.index)
        if scope == "local":
            identifier = self.declare_local(node.name, True)
            return f"runtime::local_get(&{identifier}, &{index}, {str(node.name.endswith('$')).lower()})"
        if scope in ("constant", "param"):
            self.block(f"construct:indexed {scope}")
            return "Val::from(0)"
        return f"ctx.var({self.persistent(node.name)}).get_at(runtime::index(&{index})?)?"

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
        return f"&{self.value(node)}.text()"

    def number(self, node):
        if isinstance(node, Num):
            return str(wrap32(node.value))
        return f"{self.value(node)}.number()?"

    def value(self, node):
        """Rust expression of type `Val`."""
        if isinstance(node, Num):
            return f"Val::from({wrap32(node.value)})"
        if isinstance(node, Str):
            return f"Val::from({rust_string(self.literal(node.value))})"
        if isinstance(node, Name):
            lower = node.name.lower()
            if lower in ("true", "false", "null"):
                return f"Val::from({1 if lower == 'true' else 0})"
            return self.read_name(node)
        if isinstance(node, Index):
            return self.read_index(node)
        if isinstance(node, Unary):
            if node.op == "!":
                return f"Val::from(!({self.boolean(node.operand)}))"
            if node.op == "-":
                if isinstance(node.operand, Num):
                    return f"Val::from({wrap32(-node.operand.value)})"
                return self.arithmetic("-", "Val::from(0)", self.value(node.operand))
            if node.op == "+":
                return self.value(node.operand)
            return f"runtime::op(&{self.value(node.operand)}, \"^\", &Val::from(-1))?"
        if isinstance(node, Binary):
            if node.op in ("&&", "||") or self.native_comparison(node) is not None:
                return f"Val::from({self.boolean(node)})"
            return self.arithmetic(node.op, self.value(node.left), self.value(node.right))
        if isinstance(node, Ternary):
            static = self.static_truth(node.condition)
            if static is not None:
                return self.value(node.yes if static else node.no)
            return f"(if {self.boolean(node.condition)} {{ {self.value(node.yes)} }} else {{ {self.value(node.no)} }})"
        if isinstance(node, Call):
            return self.function_call(node)
        if isinstance(node, (Assign, IncDec)):
            return self.assignment_expression(node)
        self.block(f"construct:expression {type(node).__name__}")
        return "Val::from(0)"

    def statements_as_expression(self, produce, result):
        """Rust block expression running the statements `produce` emits, then yielding `result`."""
        saved_lines, saved_depth = self.lines, self.depth
        self.lines, self.depth = [], 0
        produce()
        emitted = " ".join(line.strip() for line in self.lines)
        self.lines, self.depth = saved_lines, saved_depth
        return f"{{ {emitted} {result} }}"

    def assignment_expression(self, node):
        if isinstance(node, IncDec):
            op, target = ("+=" if node.op == "++" else "-="), node.target
            if node.prefix:
                return self.statements_as_expression(lambda: self.assign(target, op, Num(1)), self.value(target))
            return self.statements_as_expression(lambda: (self.emit(f"let previous = {self.value(target)};"), self.assign(target, op, Num(1))), "previous")
        return self.statements_as_expression(lambda: self.assign(node.target, node.op, node.value), self.value(node.target))

    def static_truth(self, node):
        """True or False when a condition is fixed on a pre-renewal server, None otherwise."""
        if isinstance(node, Call) and node.name.lower() == "checkre":
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
            return f"!({self.boolean(node.operand)})"
        if isinstance(node, Num):
            return "true" if node.value else "false"
        native = self.native_comparison(node)
        if native is not None:
            return native
        return f"{self.value(node)}.is_true()"

    def native_comparison(self, node):
        """A comparison as Rust: `==` and `!=` through `Val` (`loosely_equals` for two values, as `runtime::op` does), and an
        ordering against a literal takes the number first, failing on a text just as `op` does. None for any other comparison."""
        if not isinstance(node, Binary) or node.op not in COMPARISONS:
            return None
        if node.op in ("==", "!="):
            if isinstance(node.right, Str):
                right = rust_string(self.literal(node.right.value))
            else:
                right = integer_literal(node.right)
            if right is None:
                same = f"{self.operand(node.left)}.loosely_equals(&{self.value(node.right)})"
                return same if node.op == "==" else f"!{same}"
            return f"{self.operand(node.left)} {node.op} {right}"
        right = integer_literal(node.right)
        if right is None:
            return None
        return f"{self.operand(node.left)}.number()? {node.op} {right}"

    def operand(self, node):
        code = self.value(node)
        return f"({code})" if code.startswith("{") else code

    def arithmetic(self, operator, left, right):
        """Rust for a binary operator on two `Val` expressions: `+` as Rust's, the checked `try_*` methods for the rest,
        and `runtime::op` for the operators `Val` does not implement."""
        if operator == "+":
            return f"({self.operand_code(left)} + {self.operand_code(right)})"
        if operator in ARITHMETIC_METHODS:
            return f"({self.operand_code(left)}.{ARITHMETIC_METHODS[operator]}({right})?)"
        return f"runtime::op(&{left}, {rust_string(operator)}, &{right})?"

    @staticmethod
    def operand_code(code):
        return f"({code})" if code.startswith("{") else code

    def arguments(self, args):
        return ", ".join(self.value(argument) for argument in args)

    def call_host(self, variant, arguments):
        if variant not in SDK_FUNCTIONS:
            self.block(f"sdk:{variant}")
        return f"ctx.call(Function::{variant}, vec![{arguments}])?"

    def function_call(self, node):
        name = node.name.lower()
        args = node.args
        if name in self.local_functions and name not in ("callfunc", "callsub", "getarg", "select"):
            return f"{self.base}_run(ctx, {self.step(self.local_functions[name])}, vec![{self.arguments(args)}])?"
        if name in self.world.functions and name not in ("callfunc", "callsub", "getarg", "select"):
            return f"{self.world.functions[name]}(ctx, vec![{self.arguments(args)}])?"
        if name == "input" and args and isinstance(args[0], (Name, Index)):
            return self.input_expression(args)
        if name == "getarg":
            default = self.value(args[1]) if len(args) > 1 else "Val::from(0)"
            return f"runtime::arg(&args, {self.number(args[0])}, {default})"
        if name == "select":
            return f"Val::from(runtime::select_values(ctx, &[{self.arguments(args)}])?)"
        if name == "callfunc":
            return self.callfunc(args)
        if name == "callsub":
            return self.callsub(args)
        if name == "getguildname":
            return self.call_host("GetGuildInfo", f"{self.value(args[0])}, Val::from(0)")
        if name == "is_guild_leader":
            guild = self.value(args[0]) if args else "ctx.call(Function::GetCharacterId, vec![Val::from(2)])?"
            return self.call_host("GetGuildInfo", f"{guild}, Val::from(2)")
        if name == "is_party_leader":
            party = self.value(args[0]) if args else "ctx.call(Function::GetCharacterId, vec![Val::from(1)])?"
            return self.call_host("IsPartyLeader", party)
        if name == "killmonsterall" and len(args) == 1:
            return self.call_host("KillMonster", f"{self.value(args[0])}, Val::from(\"All\")")
        if name == "playerattached":
            return "Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true())"
        if name == "checkre" or name in RENEWAL_MOUNT_QUERIES:
            return "Val::from(0)"
        if name == "checkoption" and len(args) == 1 and isinstance(args[0], Name) and args[0].name.lower() in RENEWAL_OPTIONS:
            return "Val::from(0)"
        if name == "set" and len(args) == 2 and isinstance(args[0], (Name, Index)):
            return self.assignment_expression(Assign("=", args[0], args[1]))
        if name == "rid2name" and len(args) == 1:
            return self.call_host("ConvertPcInfo", f"{self.value(args[0])}, Val::from(0)")
        if name == "strpos" and len(args) in (2, 3):
            start = f"Some(&{self.value(args[2])})" if len(args) == 3 else "None"
            return f"runtime::strpos(&{self.value(args[0])}, &{self.value(args[1])}, {start})?"
        if name == "getattachedrid":
            return self.call_host("GetCharacterId", "Val::from(3)")
        if name == "basicskillcheck":
            return "Val::from(1)"
        if name == "getargcount":
            return "Val::from(args.len() as i32)"
        if name == "sprintf" and args:
            return f"runtime::sprintf(&{self.value(args[0])}, &[{self.arguments(args[1:])}])?"
        if name == "replacestr" and 3 <= len(args) <= 5:
            return f"runtime::replacestr(&{self.value(args[0])}, &{self.value(args[1])}, &{self.value(args[2])}, &[{self.arguments(args[3:])}])?"
        if name == "implode" and len(args) in (1, 2) and isinstance(args[0], (Name, Index)):
            delimiter = self.value(args[1]) if len(args) > 1 else 'Val::from("")'
            if args[0].name.startswith(".@"):
                return f"runtime::implode(&{self.declare_local(args[0].name, True)}, &{delimiter})?"
            if self.variable_kind(args[0].name)[0] in STORED_SCOPES:
                return f"runtime::array_implode(ctx, {self.persistent(args[0].name)}, &{delimiter})?"
        if name == "getarraysize":
            if len(args) == 1 and isinstance(args[0], Call) and args[0].name.lower() == "getd" and len(args[0].args) == 1:
                return f"runtime::getd_size(ctx, &{self.value(args[0].args[0])}, &{LOCALS_REF})?"
            if len(args) == 1 and isinstance(args[0], (Name, Index)) and args[0].name.startswith(".@"):
                identifier = self.declare_local(args[0].name, True)
                return f"Val::from({identifier}.len() as i32)"
            if len(args) == 1 and isinstance(args[0], Name) and self.variable_kind(args[0].name)[0] in STORED_SCOPES:
                return f"runtime::array_size(ctx, {self.persistent(args[0].name)})?"
            self.block("function:getarraysize on this kind of variable")
            return "Val::from(0)"
        if name == "getvariableofnpc":
            parts = self.npc_variable_parts(args)
            return f"ctx.call(Function::GetVariableOfNpc, vec![{parts}])?" if parts else "Val::from(0)"
        if name in STRING_CALLS:
            helper, fallible = STRING_CALLS[name]
            code = f"runtime::{helper}({', '.join('&' + self.value(arg) for arg in args)})"
            return f"{code}?" if fallible else code
        if name == "getd" and len(args) == 1:
            return f"runtime::getd(ctx, &{self.value(args[0])}, &{LOCALS_REF})?"
        if name in SDK_CALLS:
            return self.call_host(SDK_CALLS[name], self.arguments(args))
        if name in STATUS_CALLS:
            return self.call_host(STATUS_CALLS[name], self.arguments(args))
        if name in NEW_CALLS:
            return self.call_host(NEW_CALLS[name], self.arguments(args))
        self.block(f"function:{name}")
        return "Val::from(0)"

    def npc_variable_parts(self, args):
        """`name, npc, index` for the host, or None when the reference is not a `.variable` of another NPC."""
        target = args[0] if len(args) == 2 else None
        if not isinstance(target, (Name, Index)) or not target.name.startswith(".") or target.name.startswith(".@"):
            self.block("construct:getvariableofnpc target")
            return None
        index = self.value(target.index) if isinstance(target, Index) else "Val::from(0)"
        return f"Val::from({self.persistent(target.name)}), {self.value(args[1])}, {index}"

    def callfunc(self, args):
        if not args or not isinstance(args[0], Str):
            self.block("construct:callfunc with a computed name")
            return "Val::from(0)"
        target = args[0].value.lower()
        if target in self.world.functions:
            return f"{self.world.functions[target]}(ctx, vec![{self.arguments(args[1:])}])?"
        self.block(f"callfunc:{args[0].value}")
        return "Val::from(0)"

    def callsub(self, args):
        if not args or not isinstance(args[0], Name):
            self.block("construct:callsub with a computed label")
            return "Val::from(0)"
        label = args[0].name
        if not self.known_label(label):
            self.block(f"construct:callsub to unknown label {label}")
            return "Val::from(0)"
        return f"{self.base}_run(ctx, {self.step(label)}, vec![{self.arguments(args[1:])}])?"

    # ---- statements

    def statements(self, statements):
        index = 0
        while index < len(statements):
            if is_mes(statements[index]):
                run = []
                while index < len(statements) and is_mes(statements[index]):
                    run.append(statements[index])
                    index += 1
                self.dialogue(run)
                continue
            self.statement(statements[index])
            index += 1

    def dialogue(self, mes_statements):
        """One dialogue box for consecutive `mes` statements: a line per statement, and a leading `[Name]` line as the speaker."""
        lines = [node.args[0] for node in mes_statements]
        speaker = None
        if len(lines) > 1:
            title = speaker_title(lines[0])
            if title is not None:
                speaker = rust_string(title) if isinstance(title, str) else self.value(title)
                lines = lines[1:]
        if speaker is None and len(lines) == 1 and isinstance(lines[0], Str):
            self.emit(f"ctx.mes({rust_string(self.literal(lines[0].value))})?;")
            return
        values = ", ".join(rust_string(self.literal(line.value)) if isinstance(line, Str) else self.value(line) for line in lines)
        if speaker is None:
            self.emit(f"ctx.lines(args![{values}])?;")
        else:
            self.emit(f"ctx.lines_as({speaker}, args![{values}])?;")

    def body(self, statement):
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
        elif isinstance(node, If) and self.static_truth(node.condition) is not None:
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
            self.emit(f"return Ok({self.value(node.value) if node.value else 'Val::from(0)'});")
        elif isinstance(node, Goto):
            self.goto(node.label)
        elif isinstance(node, BackLoop):
            self.back_loop(node)
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

    def back_loop(self, node):
        identifier = self.fresh()
        self.emit(f"'j{identifier}: loop {{")
        self.back_targets[node.name] = identifier
        self.depth += 1
        self.statements(node.body)
        self.emit(f"break 'j{identifier};")
        self.depth -= 1
        self.emit("}")
        del self.back_targets[node.name]

    def goto(self, label):
        if label in self.back_targets:
            self.emit(f"continue 'j{self.back_targets[label]};")
            return
        if not self.known_label(label):
            self.block(f"construct:goto {label}")
            return
        self.emit(f"step = {self.step(label)};")
        self.emit("continue 'machine;")

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
        if isinstance(node, (While, For)) and node.condition is not None:
            self.emit(f"if !({self.boolean(node.condition)}) {{ break 'l{identifier}; }}")
        self.emit(f"'b{identifier}: {{")
        self.targets.append(("loop", identifier))
        self.body(node.body)
        self.targets.pop()
        self.emit("}")
        if isinstance(node, For) and node.step is not None:
            self.expression_statement(node.step)
        if isinstance(node, DoWhile):
            self.emit(f"if !({self.boolean(node.condition)}) {{ break 'l{identifier}; }}")
        self.depth -= 1
        self.emit("}")

    def switch(self, node):
        identifier = self.fresh()
        cases = [item.value for item in node.body if isinstance(item, Case) and item.value is not None]
        self.emit(f"'b{identifier}: {{")
        self.depth += 1
        self.emit(f"let subject{identifier} = {self.value(node.value)};")
        self.emit(f"let mut matched{identifier} = false;")
        none_match = " && ".join(f"!subject{identifier}.loosely_equals(&{self.value(case)})" for case in cases) or "true"
        self.emit(f"let no_case{identifier} = {none_match};")
        self.targets.append(("switch", identifier))
        open_group, group = False, []
        for item in node.body:
            if isinstance(item, Case):
                if open_group:
                    self.statements(group)
                    group = []
                    self.depth -= 1
                    self.emit("}")
                    open_group = False
                if item.value is None:
                    self.emit(f"if !matched{identifier} && no_case{identifier} {{ matched{identifier} = true; }}")
                else:
                    self.emit(f"if !matched{identifier} && subject{identifier}.loosely_equals(&{self.value(item.value)}) {{ matched{identifier} = true; }}")
                continue
            if not open_group:
                self.emit(f"if matched{identifier} {{")
                self.depth += 1
                open_group = True
            group.append(item)
        if open_group:
            self.statements(group)
            self.depth -= 1
            self.emit("}")
        self.targets.pop()
        self.depth -= 1
        self.emit("}")

    def menu(self, node):
        options = []
        for text, label in node.options:
            for part in text.split(":"):
                if part:
                    options.append((part, label))
        self.emit(f"let choice = runtime::select(ctx, &[{', '.join(rust_string(text) for text, _ in options)}])?;")
        self.emit("ctx.var(\"@menu\").set(choice)?;")
        self.emit("match choice {")
        self.depth += 1
        for number, (_, label) in enumerate(options, 1):
            if label is None:
                self.emit(f"{number} => {{}}")
            elif self.known_label(label):
                self.emit(f"{number} => {{ step = {self.step(label)}; continue 'machine; }}")
            else:
                self.block(f"construct:menu label {label}")
        self.emit("_ => return Err(Stop::Error(\"Invalid menu selection\".into())),")
        self.depth -= 1
        self.emit("}")

    def expression_statement(self, node):
        if isinstance(node, Assign):
            self.assign(node.target, node.op, node.value)
        elif isinstance(node, IncDec):
            self.assign(node.target, "+=" if node.op == "++" else "-=", Num(1))
        elif isinstance(node, Call):
            self.emit(f"{self.function_call(node)};")
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
        if isinstance(target, Call) and target.name.lower() == "getarg" and len(target.args) == 1:
            if op != "=":
                value = Binary(op[:-1], target, value)
            index, rendered = self.number(target.args[0]), self.value(value)
            # Read the index and value before `args` is borrowed mutably
            self.emit(f"let index = {index};")
            self.emit(f"let value = {rendered};")
            self.emit("runtime::set_arg(&mut args, index, value)?;")
            return
        if isinstance(target, Call):
            parts = self.npc_variable_parts(target.args) if target.name.lower() == "getvariableofnpc" else None
            if parts is None:
                self.block(f"function:{target.name} as an assignment target")
                return
            if op != "=":
                rendered = self.arithmetic(op[:-1], self.value(target), self.value(value))
            self.emit(f"ctx.call(Function::SetVariableOfNpc, vec![{parts}, {rendered}])?;")
            return
        scope, canonical = self.variable_kind(target.name)
        if op != "=":
            rendered = self.arithmetic(op[:-1], self.value(target), self.value(value))
        if scope == "local":
            if isinstance(target, Index):
                identifier = self.declare_local(target.name, True)
                self.emit(f"{{ let assigned = {rendered}; let position = {self.value(target.index)}; runtime::local_set(&mut {identifier}, &position, assigned, {str(target.name.endswith('$')).lower()}); }}")
            else:
                identifier = self.declare_local(target.name, False)
                if identifier in self.array_locals:
                    self.emit(f"{{ let assigned = {rendered}; runtime::local_set(&mut {identifier}, &Val::from(0), assigned, {str(target.name.endswith('$')).lower()}); }}")
                else:
                    self.emit(f"{identifier} = {rendered};")
        elif scope == "param":
            if canonical.lower() in WRITABLE_PARAMETERS and isinstance(target, Name):
                self.emit(f"ctx.var({rust_string(canonical)}).set({rendered})?;")
            else:
                self.block(f"param write:{canonical}")
        elif scope == "constant":
            self.block("construct:assignment to constant")
        elif isinstance(target, Index):
            self.emit(f"ctx.var({self.persistent(target.name)}).set_at(runtime::index(&{self.value(target.index)})?, {rendered})?;")
        else:
            self.emit(f"ctx.var({self.persistent(target.name)}).set({rendered})?;")

    def setd(self, name, value):
        self.emit(f"runtime::setd(ctx, &{self.value(name)}, {self.value(value)}, &mut {LOCALS_MUT})?;")

    def command(self, node):
        name = node.name.lower()
        args = node.args
        if name in ("enable_items", "disable_items", "logmes", "refineui", "freeloop", "debugmes", "function", "setarray_noop"):
            return
        if name in RENEWAL_MOUNT_COMMANDS:
            return
        if name == "npcskill" and len(args) == 4:
            self.emit(f"runtime::npc_skill(ctx, &{self.value(args[0])}, &{self.value(args[1])}, &{self.value(args[2])}, &{self.value(args[3])})?;")
            return
        if name == "unitwarp" and len(args) == 4 and isinstance(args[0], Num) and args[0].value == 0:
            self.emit(f"ctx.call(Function::Warp, vec![{self.arguments(args[1:])}])?;")
            return
        if name == "explode" and len(args) == 3 and isinstance(args[0], (Name, Index)) and args[0].name.startswith(".@"):
            identifier = self.declare_local(args[0].name, True)
            self.emit(f"{identifier} = runtime::explode(&{self.value(args[1])}, &{self.value(args[2])});")
            return
        if name in self.local_functions or name in self.world.functions:
            if name not in ("callfunc", "callsub", "select"):
                self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
                return
        if name in ("getguildname", "is_guild_leader", "killmonsterall", "is_party_leader"):
            self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
            return
        if name == "mes":
            if len(args) == 1:
                self.dialogue([node])
            else:
                self.block("command:mes with several arguments")
        elif name == "next":
            self.emit("ctx.next()?;")
        elif name in ("close", "close2", "close3"):
            self.emit("ctx.close_window()?;")
            if name == "close3":
                self.emit('ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;')
            if name != "close2":
                self.emit("return Err(Stop::End);")
        elif name == "end":
            self.emit("return Err(Stop::End);")
        elif name == "set":
            self.block("construct:set")
        elif name == "getpartymember":
            self.emit(f"runtime::party_members(ctx, {self.value(args[0])}, {self.value(args[1]) if len(args) > 1 else 'Val::from(0)'})?;")
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
            self.emit("runtime::inventory_list(ctx)?;")
        elif name == "input":
            self.input(args)
        elif name == "callsub":
            self.emit(f"{self.callsub(args)};")
        elif name == "callfunc":
            self.emit(f"{self.callfunc(args)};")
        elif name == "select":
            self.emit(f"let choice = runtime::select_values(ctx, &[{self.arguments(args)}])?;")
            self.emit("ctx.var(\"@menu\").set(choice)?;")
        elif name == "setd":
            if len(args) == 2:
                self.setd(args[0], args[1])
            else:
                self.block("command:setd")
        elif name in NEW_CALLS:
            self.emit(f"{self.call_host(NEW_CALLS[name], self.arguments(args))};")
        elif name in SDK_CALLS or name in STATUS_CALLS:
            self.emit(f"{self.function_call(Call(node.name, args, node.line))};")
        else:
            self.block(f"command:{name}")

    def sscanf(self, args):
        if len(args) < 3 or not all(isinstance(target, (Name, Index)) for target in args[2:]):
            self.block("command:sscanf")
            return
        self.emit(f"let scanned = runtime::sscanf(&{self.value(args[0])}, &{self.value(args[1])});")
        for position, target in enumerate(args[2:]):
            self.emit(f"if let Some(value) = scanned.get({position}).cloned() {{")
            self.depth += 1
            self.assign_rendered(target, "value")
            self.depth -= 1
            self.emit("}")

    def getmapxy(self, args):
        if len(args) < 3 or not all(isinstance(argument, (Name, Index)) for argument in args[:3]):
            self.block("command:getmapxy")
            return
        unit = self.arguments(args[3:]) or "Val::from(0)"
        self.emit(f"let position = ctx.call(Function::GetMapXy, vec![{unit}])?.into_array().ok_or_else(|| Stop::Error(\"Invalid position\".into()))?;")
        for number, target in enumerate(args[:3]):
            self.assign_rendered(target, f"position[{number}].clone()")

    def setarray(self, args):
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:setarray")
            return
        target = args[0]
        start = self.value(target.index) if isinstance(target, Index) else "Val::from(0)"
        scope, _ = self.variable_kind(target.name)
        if scope == "local":
            identifier = self.declare_local(target.name, True)
            self.emit(f"let base = {start}.number()?;")
            for offset, argument in enumerate(args[1:]):
                self.emit(f"runtime::local_set(&mut {identifier}, &Val::from(base + {offset}), {self.value(argument)}, {str(target.name.endswith('$')).lower()});")
        elif scope in STORED_SCOPES:
            self.emit(f"let base = {start}.number()?;")
            for offset, argument in enumerate(args[1:]):
                self.emit(f"ctx.var({self.persistent(target.name)}).set_at(runtime::index(&Val::from(base + {offset}))?, {self.value(argument)})?;")
        else:
            self.block("command:setarray target")

    def deletearray(self, args):
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:deletearray")
            return
        target = args[0]
        start = self.value(target.index) if isinstance(target, Index) else "Val::from(0)"
        count = f"Some(&{self.value(args[1])})" if len(args) > 1 else "None"
        scope = self.variable_kind(target.name)[0]
        if scope == "local":
            self.emit(f"runtime::local_delete(&mut {self.declare_local(target.name, True)}, &{start}, {count})?;")
        elif scope in STORED_SCOPES:
            self.emit(f"runtime::array_delete(ctx, {self.persistent(target.name)}, &{start}, {count})?;")
        else:
            self.block("command:deletearray")

    def array_values(self, target):
        scope = self.variable_kind(target.name)[0]
        if scope == "local":
            return f"{self.declare_local(target.name, True)}.clone()"
        if scope in STORED_SCOPES:
            return f"runtime::array_values(ctx, {self.persistent(target.name)})?"
        self.block("command:copyarray")
        return "Vec::new()"

    def copyarray(self, args):
        if len(args) != 3 or not all(isinstance(argument, (Name, Index)) for argument in args[:2]):
            self.block("command:copyarray")
            return
        destination, source = args[0], args[1]
        text = str(destination.name.endswith("$")).lower()
        first = lambda target: self.value(target.index) if isinstance(target, Index) else "Val::from(0)"
        values = f"runtime::slice_of(&{self.array_values(source)}, &{first(source)}, &{self.value(args[2])}, {text})?"
        scope = self.variable_kind(destination.name)[0]
        if scope == "local":
            self.emit(f"runtime::local_splice(&mut {self.declare_local(destination.name, True)}, &{first(destination)}, {values}, {text})?;")
        elif scope in STORED_SCOPES:
            self.emit(f"runtime::array_splice(ctx, {self.persistent(destination.name)}, &{first(destination)}, {values})?;")
        else:
            self.block("command:copyarray")

    def input(self, args):
        if not args or not isinstance(args[0], (Name, Index)):
            self.block("command:input")
            return
        self.input_statements(args)

    def input_statements(self, args):
        """Reads the input box into the target and leaves the status in `status`: -1 below the minimum, 1 above the maximum."""
        target = args[0]
        function = "input_text" if target.name.rstrip().endswith("$") else "input_number"
        bounds = [self.number(argument) for argument in args[1:3]]
        bounds += ["None"] * (2 - len(bounds))
        self.emit(f"let (input, status) = runtime::{function}(ctx, {', '.join(f'Some({b})' if b != 'None' else 'None' for b in bounds)})?;")
        self.assign_rendered(target, "input")

    def input_expression(self, args):
        return self.statements_as_expression(lambda: self.input_statements(args), "Val::from(status)")

    def assign_rendered(self, target, rendered):
        scope, _ = self.variable_kind(target.name)
        if scope == "local" and isinstance(target, Name):
            identifier = self.declare_local(target.name, False)
            if identifier in self.array_locals:
                self.emit(f"{{ let assigned = {rendered}; runtime::local_set(&mut {identifier}, &Val::from(0), assigned, {str(target.name.endswith('$')).lower()}); }}")
            else:
                self.emit(f"{identifier} = {rendered};")
        elif scope in STORED_SCOPES and isinstance(target, Name):
            self.emit(f"ctx.var({self.persistent(target.name)}).set({rendered})?;")
        else:
            self.block("command:input target")

    def declarations(self):
        """`let` lines for the locals the body uses, all starting at zero or empty."""
        declarations = []
        for identifier, (kind, string) in sorted(self.locals.items()):
            if kind == "array":
                declarations.append(f"    let mut {identifier}: Vec<Val> = Vec::new();")
            else:
                declarations.append(f"    let mut {identifier} = Val::from({'\"\"' if string else '0'});")
        return declarations

    def substitute(self, lines):
        """Fills the `getd`/`setd` tables, which need every local of the body, into lines emitted before all were seen."""
        names = sorted(self.locals.items())
        refs = ", ".join(f"({rust_string(self.local_names[identifier])}, runtime::Local::{'Array' if kind == 'array' else 'Scalar'}(&{identifier}))" for identifier, (kind, _) in names)
        muts = ", ".join(f"({rust_string(self.local_names[identifier])}, runtime::LocalMut::{'Array' if kind == 'array' else 'Scalar'}(&mut {identifier}))" for identifier, (kind, _) in names)
        return [line.replace(LOCALS_REF, f"[{refs}]").replace(LOCALS_MUT, f"[{muts}]") for line in lines]


def writes_arguments(statements):
    """Whether a body sets an argument with `set getarg(n), ..` or `getarg(n)++`, which needs `args` to be mutable."""
    for node in walk(statements):
        expression = node.expression if isinstance(node, ExprStatement) else node
        if isinstance(expression, (Assign, IncDec)) and isinstance(expression.target, Call) and expression.target.name.lower() == "getarg":
            return True
    return False


def args_parameter(statements):
    return ("mut " if writes_arguments(statements) else "") + "args: Vec<Val>"


def needs_machine(statements, labels):
    """Whether the body is a state machine: a label other than an event, a jump, a callsub or a local function, or a segment
    that falls into the next label. Such a body cannot be split into one function per event."""
    if any(isinstance(node, FunctionDefinition) for node in statements):
        return True
    for label in labels:
        if not ON_LABEL.match(label.name):
            return True
    for node in walk(statements):
        if isinstance(node, Goto):
            return True
        if isinstance(node, Command) and node.name.lower() == "callsub":
            return True
        if isinstance(node, Menu) and any(label is not None for _, label in node.options):
            return True
    return False


def falls_through(statements):
    """Whether a segment ends without `end` and runs into the next label, which only a state machine can express."""
    parts = segments(statements)
    return any(not code or not terminal(code[-1]) for _, code in parts[:-1])


def segments(statements):
    """(label or None, statements) for the main dialogue and each label's code, in source order."""
    result = [(None, [])]
    for node in statements:
        if isinstance(node, Label):
            result.append((node.name, []))
        else:
            result[-1][1].append(node)
    return result


class Lowered:
    """The Rust items of one script or function. `npc` is the NPC's entry function, `events` the `(label, function)` pairs,
    and `items` the source texts in the order they are written."""

    def __init__(self, name, items, npc=None, events=(), step_type=None):
        self.name = name
        self.items = items
        self.npc = npc
        self.events = list(events)
        self.step_type = step_type


def prepare(body, world):
    statements, local_functions = expand_local_functions(fold_is_function(list(body), world))
    return wrap_backward_labels(hoist_nested_labels(statements, [0])), local_functions


def lower_script(script, world, base):
    """Lowers an NPC script whose functions are named from `base`. Raises `Unsupported` with the first construct it cannot translate."""
    source = list(script.body)
    statements, local_functions = prepare(source, world)
    labels = [node for node in statements if isinstance(node, Label)]
    machine = needs_machine(statements, labels) or bool(local_functions) or falls_through(statements)
    if not machine:
        return lower_segments(statements, world, base)
    return lower_machine(statements, local_functions, world, base, npc=True)


def lower_function(definition, world, base):
    """Lowers a `function script` whose Rust function is named `base`. It takes its arguments as `args` and returns a value."""
    statements, local_functions = prepare(list(definition.body), world)
    labels = [node for node in statements if isinstance(node, Label)]
    if not needs_machine(statements, labels) and not local_functions and not labels:
        emitter = Emitter(world, base, None, machine=False)
        emitter.depth = 1
        emitter.array_locals = find_array_locals(statements)
        emitter.statements(statements)
        if statements and not terminal(statements[-1]):
            emitter.emit("Ok(Val::from(0))")
        check(emitter)
        text = f"pub fn {base}(ctx: &Ctx, {args_parameter(statements)}) -> Result<Val, Stop> {{\n" + "\n".join(emitter.declarations() + emitter.substitute(emitter.lines)) + "\n}"
        return Lowered(base, [text])
    return lower_machine(statements, local_functions, world, base, npc=False)


def check(emitter):
    if emitter.blockers:
        raise Unsupported(sorted(emitter.blockers)[0])


def lower_segments(statements, world, base):
    """Plain functions: the main dialogue and one per event label, each with its own locals."""
    parts = segments(statements)
    items, events = [], []
    for label, code in parts:
        name = base if label is None else f"{base}_{fn_name(label)}"
        emitter = Emitter(world, name, None, machine=False)
        emitter.array_locals = find_array_locals(code)
        emitter.statements(code)
        if not code or not terminal(code[-1]):
            emitter.emit("Ok(Val::from(0))")
        check(emitter)
        items.append(f"fn {name}_body(ctx: &Ctx, {args_parameter(code)}) -> Result<Val, Stop> {{\n" + "\n".join(emitter.declarations() + emitter.substitute(emitter.lines)) + "\n}")
        items.append(f"pub fn {name}(ctx: &Ctx) -> Script {{\n    {name}_body(ctx, Vec::new()).map(|_| ())\n}}")
        if label is not None:
            events.append((label, name))
    return Lowered(base, items, npc=base, events=events)



def unique_variants(labels):
    """`Step` variant of each lower-case label: its name, numbered where two labels would share it (`SubGarrison` and `sub_garrison`)."""
    used, variants = {"Start"}, {}
    for key, spelling in labels.items():
        name = step_name(spelling)
        unique, number = name, 2
        while unique in used:
            unique = f"{name}{number}"
            number += 1
        used.add(unique)
        variants[key] = unique
    return variants


def lower_machine(statements, local_functions, world, base, npc):
    """A state machine: one `Step` variant per label, dispatched by one `run` function that keeps its locals between steps.

    Each `run` call starts with fresh locals, so a `callsub` or local function gets its own."""
    labels = {}
    for node in statements:
        if isinstance(node, Label):
            labels.setdefault(node.name.lower(), node.name)
    step_type = f"{step_name(base)}Step"
    emitter = Emitter(world, base, step_type, machine=True)
    emitter.labels = dict(labels)
    emitter.variants = unique_variants(labels)
    emitter.local_functions = {name.lower(): label for name, label in local_functions.items()}
    emitter.array_locals = find_array_locals(statements)
    segs = segments(statements)
    variants = ["Start"] + list(emitter.variants.values())
    arms = []
    for index, (label, code) in enumerate(segs):
        emitter.lines = []
        emitter.depth = 3
        emitter.statements(code)
        if not code or not terminal(code[-1]):
            if index + 1 < len(segs):
                emitter.emit(f"step = {step_type}::{emitter.variant(segs[index + 1][0])};")
                emitter.emit("continue 'machine;")
            else:
                emitter.emit("return Ok(Val::from(0));")
        variant = "Start" if label is None else emitter.variant(label)
        arms.append((variant, emitter.lines))
    check(emitter)
    arm_texts = []
    for variant, lines in arms:
        body = "\n".join(emitter.substitute(lines))
        arm_texts.append("            " + step_type + "::" + variant + " => {\n" + body + "\n            }")
    enum_lines = "".join("    " + variant + ",\n" for variant in variants)
    enum_text = "#[derive(Clone, Copy, Debug)]\nenum " + step_type + " {\n" + enum_lines + "}"
    run_text = (
        "fn " + base + "_run(ctx: &Ctx, mut step: " + step_type + ", " + args_parameter(statements) + ") -> Result<Val, Stop> {\n"
        + "\n".join(emitter.declarations())
        + "\n    'machine: loop {\n        match step {\n"
        + "\n".join(arm_texts)
        + "\n        }\n    }\n}"
    )
    items = [enum_text, run_text]
    events = []
    if npc:
        items.append("pub fn " + base + "(ctx: &Ctx) -> Script {\n    " + base + "_run(ctx, " + step_type + "::Start, Vec::new()).map(|_| ())\n}")
        for label, _ in segs[1:]:
            if ON_LABEL.match(label):
                name = base + "_" + fn_name(label)
                items.append("pub fn " + name + "(ctx: &Ctx) -> Script {\n    " + base + "_run(ctx, " + step_type + "::" + emitter.variant(label) + ", Vec::new()).map(|_| ())\n}")
                events.append((label, name))
    else:
        items.append("pub fn " + base + "(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {\n    " + base + "_run(ctx, " + step_type + "::Start, args)\n}")
    return Lowered(base, items, npc=base if npc else None, events=events, step_type=step_type)
