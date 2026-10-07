import re
from dataclasses import dataclass


class ScriptError(Exception):
    """A construct the converter does not understand; reported, never guessed."""

    def __init__(self, message, line=None):
        super().__init__(f"line {line}: {message}" if line else message)
        self.message = message
        self.line = line


@dataclass
class Token:
    kind: str  # num, str, id, op, eof
    value: object
    line: int

    def is_op(self, *values):
        return self.kind == "op" and self.value in values

    def __repr__(self):
        return f"{self.kind}:{self.value!r}"


OPERATORS = ["<<=", ">>=", "==", "!=", "<=", ">=", "&&", "||", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "++", "--",
             "+", "-", "*", "/", "%", "=", "<", ">", "!", "~", "&", "|", "^", "?", ":", ",", ";", "(", ")", "{", "}", "[", "]"]
NUMBER = re.compile(r"0[xX][0-9a-fA-F]+|\d+")
NAME = re.compile(r"(?:(?:\$@|\$|##|#|\.@|\.|@|''|')[A-Za-z0-9_]+|[A-Za-z_][A-Za-z0-9_]*)\$?")
ESCAPES = {"n": "\n", "t": "\t", "\\": "\\", '"': '"'}


def strip_comments(text):
    """Remove // and /* */ comments while keeping line numbers and string contents."""
    out = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append(text[i:j + 1])
            i = j + 1
        elif text.startswith("//", i):
            while i < n and text[i] != "\n":
                i += 1
        elif text.startswith("/*", i):
            end = text.find("*/", i + 2)
            end = n if end < 0 else end + 2
            out.append("\n" * text.count("\n", i, end))
            i = end
        else:
            out.append(c)
            i += 1
    return "".join(out)


def tokenize(text, first_line=1):
    tokens = []
    i, n, line = 0, len(text), first_line
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1
            i += 1
        elif c.isspace():
            i += 1
        elif c == '"':
            j = i + 1
            chars = []
            while j < n and text[j] != '"':
                if text[j] == "\\" and j + 1 < n:
                    chars.append(ESCAPES.get(text[j + 1], "\\" + text[j + 1]))
                    j += 2
                    continue
                if text[j] == "\n":
                    line += 1
                chars.append(text[j])
                j += 1
            if j >= n:
                raise ScriptError("unterminated string", line)
            tokens.append(Token("str", "".join(chars), line))
            i = j + 1
        elif c.isdigit():
            m = NUMBER.match(text, i)
            literal = m.group()
            octal = len(literal) > 1 and literal[0] == "0" and literal[1] not in "xX"
            tokens.append(Token("num", int(literal, 8) if octal else int(literal, 0), line))
            i = m.end()
        elif m := NAME.match(text, i):
            tokens.append(Token("id", m.group(), line))
            i = m.end()
        else:
            for op in OPERATORS:
                if text.startswith(op, i):
                    tokens.append(Token("op", op, line))
                    i += len(op)
                    break
            else:
                raise ScriptError(f"unexpected character {c!r}", line)
    tokens.append(Token("eof", None, line))
    return tokens
