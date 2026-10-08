import re
from dataclasses import dataclass, field
from typing import Optional

from .lexer import ScriptError, strip_comments, tokenize


# ---- expressions
@dataclass
class Num:
    value: int


@dataclass
class Str:
    value: str


@dataclass
class Name:
    name: str
    line: int = 0


@dataclass
class Index:
    name: str
    index: object
    line: int = 0


@dataclass
class Call:
    name: str
    args: list
    line: int = 0


@dataclass
class Unary:
    op: str
    operand: object


@dataclass
class Binary:
    op: str
    left: object
    right: object


@dataclass
class Ternary:
    condition: object
    yes: object
    no: object


@dataclass
class Assign:
    op: str
    target: object
    value: object


@dataclass
class IncDec:
    op: str
    target: object
    prefix: bool


# ---- statements
@dataclass
class Block:
    statements: list


@dataclass
class If:
    condition: object
    then: object
    otherwise: Optional[object]


@dataclass
class While:
    condition: object
    body: object


@dataclass
class DoWhile:
    body: object
    condition: object


@dataclass
class For:
    init: Optional[object]
    condition: Optional[object]
    step: Optional[object]
    body: object


@dataclass
class Case:
    value: Optional[object]  # None is `default`


@dataclass
class Switch:
    value: object
    body: list


@dataclass
class Label:
    name: str


@dataclass
class Goto:
    label: str


@dataclass
class BackLoop:
    """Statements that follow a label up to the end of their block, restarted by a `goto` to it from inside."""
    name: str
    body: list


@dataclass
class Break:
    pass


@dataclass
class Continue:
    pass


@dataclass
class Return:
    value: Optional[object]


@dataclass
class Command:
    name: str
    args: list
    line: int = 0


@dataclass
class Menu:
    options: list  # (text, label) where label None means "continue after the menu"
    line: int = 0


@dataclass
class FunctionDefinition:
    name: str
    body: object


@dataclass
class ExprStatement:
    expression: object


KEYWORDS = {"if", "while", "do", "for", "switch", "case", "default", "break", "continue", "return", "goto", "function"}
ASSIGN_OPERATORS = {"=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="}
BINARY_LEVELS = [["||"], ["&&"], ["|"], ["^"], ["&"], ["==", "!="], ["<", "<=", ">", ">="], ["<<", ">>"], ["+", "-"], ["*", "/", "%"]]


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    @property
    def token(self):
        return self.tokens[self.pos]

    def peek(self, offset=1):
        return self.tokens[min(self.pos + offset, len(self.tokens) - 1)]

    def advance(self):
        token = self.tokens[self.pos]
        self.pos += 1
        return token

    def accept(self, *ops):
        if self.token.is_op(*ops):
            return self.advance()
        return None

    def expect(self, op):
        if not self.token.is_op(op):
            raise ScriptError(f"expected '{op}' but found {self.token.value!r}", self.token.line)
        return self.advance()

    def at_keyword(self, word):
        return self.token.kind == "id" and self.token.value.lower() == word

    def parse_all(self):
        statements = []
        while self.token.kind != "eof":
            statements.append(self.statement())
        return statements

    def block(self):
        self.expect("{")
        statements = []
        while not self.token.is_op("}"):
            if self.token.kind == "eof":
                raise ScriptError("missing '}'", self.token.line)
            statements.append(self.statement())
        self.advance()
        return Block(statements)

    def statement(self):
        token = self.token
        if token.is_op("{"):
            return self.block()
        if token.is_op(";"):
            self.advance()
            return Block([])
        if token.kind != "id":
            raise ScriptError(f"unexpected {token.value!r} at the start of a statement", token.line)
        word = token.value
        if word.lower() in KEYWORDS:
            word = word.lower()
        if word == "if":
            return self.if_statement()
        if word == "while":
            self.advance()
            condition = self.parenthesized()
            return While(condition, self.statement())
        if word == "do":
            self.advance()
            body = self.statement()
            if not self.at_keyword("while"):
                raise ScriptError("do without while", token.line)
            self.advance()
            condition = self.parenthesized()
            self.accept(";")
            return DoWhile(body, condition)
        if word == "for":
            return self.for_statement()
        if word == "switch":
            return self.switch_statement()
        if word in ("case", "default"):
            self.advance()
            value = None if word == "default" else self.expression()
            self.expect(":")
            return Case(value)
        if word in ("break", "continue"):
            self.advance()
            self.expect(";")
            return Break() if word == "break" else Continue()
        if word == "return":
            self.advance()
            value = None if self.token.is_op(";") else self.expression()
            self.expect(";")
            return Return(value)
        if word == "goto":
            self.advance()
            label = self.advance()
            self.expect(";")
            return Goto(label.value)
        if word == "function" and self.peek().kind == "id" and self.peek(2).is_op("{"):
            self.advance()
            name = self.advance().value
            return FunctionDefinition(name, self.block())
        if self.peek().is_op(":") and not self.peek(2).is_op(":"):
            self.advance()
            self.advance()
            return Label(word)
        return self.simple_statement()

    def parenthesized(self):
        saved = self.pos
        self.expect("(")
        value = self.expression()
        self.expect(")")
        if self.token.kind == "op" and self.token.value in ("&&", "||", "==", "!=", "<=", ">=", "<", ">", "*", "/", "%", "|", "^", "?"):
            self.pos = saved
            return self.expression()
        return value

    def if_statement(self):
        self.advance()
        condition = self.parenthesized()
        then = self.statement()
        otherwise = None
        if self.at_keyword("else"):
            self.advance()
            otherwise = self.statement()
        return If(condition, then, otherwise)

    def for_statement(self):
        self.advance()
        self.expect("(")
        init = None if self.token.is_op(";") else self.simple_expression_statement()
        self.expect(";")
        condition = None if self.token.is_op(";") else self.expression()
        self.expect(";")
        step = None if self.token.is_op(")") else self.simple_expression_statement()
        self.expect(")")
        return For(init, condition, step, self.statement())

    def simple_expression_statement(self):
        if self.at_keyword("set"):
            self.advance()
            target = self.unary()
            self.expect(",")
            return Assign("=", target, self.expression())
        return self.expression()

    def switch_statement(self):
        self.advance()
        value = self.parenthesized()
        self.expect("{")
        body = []
        while not self.token.is_op("}"):
            if self.token.kind == "eof":
                raise ScriptError("missing '}' of switch", self.token.line)
            body.append(self.statement())
        self.advance()
        return Switch(value, body)

    def argument_list(self):
        args = []
        if not self.token.is_op(";"):
            args.append(self.expression())
            while self.accept(","):
                args.append(self.expression())
        return args

    def simple_statement(self):
        token = self.token
        name = token.value
        following = self.peek()
        if following.kind == "op" and following.value in ASSIGN_OPERATORS | {"++", "--", "["}:
            expression = self.expression()
            # rathena skips whatever ends an assignment, so one stray `)` before the `;` loads (Baba Yaga#rus32)
            self.accept(")")
            self.expect(";")
            return ExprStatement(expression)
        self.advance()
        if name == "menu":
            return self.menu(token.line)
        if self.token.is_op("("):
            saved = self.pos
            try:
                self.advance()
                args = []
                if not self.token.is_op(")"):
                    args.append(self.expression())
                    while self.accept(","):
                        args.append(self.expression())
                self.expect(")")
                if self.token.is_op(";"):
                    self.advance()
                    return self.command(name, args, token.line)
            except ScriptError:
                pass
            self.pos = saved
        args = self.argument_list()
        if not self.token.is_op(";"):
            raise ScriptError(f"unexpected {self.token.value!r} in the arguments of {name}", self.token.line)
        self.advance()
        return self.command(name, args, token.line)

    def command(self, name, args, line):
        if name == "set":
            if len(args) != 2 or not isinstance(args[0], (Name, Index, Call)):
                raise ScriptError("malformed set", line)
            return ExprStatement(Assign("=", args[0], args[1]))
        return Command(name, args, line)

    def menu(self, line):
        options = []
        while True:
            text = self.advance()
            if text.kind != "str":
                raise ScriptError("menu option text must be a string literal", text.line)
            self.expect(",")
            if self.accept("-"):
                label = None
            else:
                token = self.advance()
                if token.kind != "id":
                    raise ScriptError("menu option needs a label", token.line)
                label = token.value
            options.append((text.value, label))
            if not self.accept(","):
                break
        self.expect(";")
        return Menu(options, line)

    def expression(self):
        left = self.ternary()
        if self.token.kind == "op" and self.token.value in ASSIGN_OPERATORS:
            if not isinstance(left, (Name, Index, Call)):
                raise ScriptError("assignment to something that is not a variable", self.token.line)
            op = self.advance().value
            return Assign(op, left, self.expression())
        return left

    def ternary(self):
        condition = self.binary(0)
        if self.accept("?"):
            yes = self.expression()
            self.expect(":")
            return Ternary(condition, yes, self.expression())
        return condition

    def binary(self, level):
        if level == len(BINARY_LEVELS):
            return self.unary()
        left = self.binary(level + 1)
        while self.token.kind == "op" and self.token.value in BINARY_LEVELS[level]:
            op = self.advance().value
            left = Binary(op, left, self.binary(level + 1))
        return left

    def unary(self):
        token = self.token
        if token.is_op("!", "~", "-", "+"):
            self.advance()
            return Unary(token.value, self.unary())
        if token.is_op("++", "--"):
            self.advance()
            return IncDec(token.value, self.unary(), True)
        node = self.primary()
        if self.token.is_op("++", "--") and isinstance(node, (Name, Index)):
            return IncDec(self.advance().value, node, False)
        return node

    def primary(self):
        token = self.advance()
        if token.kind == "num":
            return Num(token.value)
        if token.kind == "str":
            return Str(token.value)
        if token.is_op("("):
            value = self.expression()
            self.expect(")")
            return value
        if token.kind != "id":
            raise ScriptError(f"unexpected {token.value!r} in an expression", token.line)
        if self.token.is_op("("):
            self.advance()
            args = []
            if not self.token.is_op(")"):
                args.append(self.expression())
                while self.accept(","):
                    args.append(self.expression())
            self.expect(")")
            return Call(token.value, args, token.line)
        if self.token.is_op("["):
            self.advance()
            index = self.expression()
            self.expect("]")
            return Index(token.value, index, token.line)
        return Name(token.value, token.line)


def parse_body(text, first_line=1):
    return Parser(tokenize(text, first_line)).parse_all()


@dataclass
class NpcDefinition:
    kind: str  # script, function, duplicate, shop, cashshop, warp, monster, mapflag
    file: str
    line: int
    place: Optional[tuple] = None  # map, x, y, dir
    name: str = ""
    sprite: str = ""
    trigger: Optional[tuple] = None
    source: Optional[str] = None
    exname: Optional[str] = None
    details: str = ""
    body: list = field(default_factory=list)
    error: Optional[str] = None


HEADER = re.compile(
    r"^(?P<head>[^\t\n]+)\t+(?P<kind>script|shop|cashshop|warp2?|monster|mapflag|duplicate\([^)\n]*\))\t+(?P<name>[^\t\n]*)(?:\t+(?P<rest>[^\n]*))?$",
    re.M,
)


def matching_brace(text, start):
    depth = 0
    i = start
    n = len(text)
    while i < n:
        c = text[i]
        if c == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ScriptError("unbalanced braces")


def parse_place(head):
    parts = head.split(",")
    if head == "-":
        return None
    if len(parts) == 4:
        return (parts[0], int(parts[1]), int(parts[2]), int(parts[3]))
    if len(parts) == 3:
        return (parts[0], int(parts[1]), int(parts[2]), 0)
    if len(parts) == 1:
        return (parts[0], 0, 0, 0)
    raise ScriptError(f"unknown NPC placement {head!r}")


def parse_file(path, display_name=None):
    text = strip_comments(path.read_text(encoding="utf-8", errors="replace"))
    definitions = []
    for match in HEADER.finditer(text):
        head, kind = match["head"].strip(), match["kind"]
        line = text.count("\n", 0, match.start()) + 1
        definition = NpcDefinition(kind="", file=display_name or str(path), line=line, name=match["name"].strip())
        rest = (match["rest"] or "").strip()
        if "::" in definition.name:
            definition.name, definition.exname = [part.strip() for part in definition.name.split("::", 1)]
        try:
            if head == "function" and kind == "script":
                definition.kind = "function"
            else:
                definition.kind = "duplicate" if kind.startswith("duplicate(") else kind.rstrip("2")
                definition.place = parse_place(head)
            if definition.kind == "duplicate":
                definition.source = kind[len("duplicate("):-1]
                fields = rest.split(",")
                definition.sprite = fields[0].strip()
                if len(fields) == 3:
                    definition.trigger = (int(fields[1]), int(fields[2]))
            elif definition.kind in ("script", "function"):
                brace = text.index("{", match.end("name"))
                end = matching_brace(text, brace)
                header_fields = text[match.end("name"):brace].strip().rstrip(",").split(",")
                if definition.kind == "script":
                    definition.sprite = header_fields[0].strip()
                    if len(header_fields) == 3:
                        definition.trigger = (int(header_fields[1]), int(header_fields[2]))
                definition.body = parse_body(text[brace + 1:end], text.count("\n", 0, brace + 1) + 1)
            else:
                sprite, _, details = rest.partition(",")
                definition.sprite = sprite.strip()
                definition.details = details.strip()
        except (ScriptError, ValueError) as error:
            definition.error = str(error)
        definitions.append(definition)
    return definitions
