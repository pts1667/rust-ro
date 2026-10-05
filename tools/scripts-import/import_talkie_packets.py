import argparse
import ast
import json
import pathlib
import re


def condition(expression, version):
    values = {
        "PACKETVER": version,
        "PACKETVER_MAIN_NUM": version,
        "PACKETVER_RE_NUM": 0,
        "PACKETVER_ZERO_NUM": 0,
    }
    expression = re.sub(r"/\*.*?\*/|//.*", "", expression).strip()
    expression = re.sub(r"defined\s*\(?\s*(\w+)\s*\)?", "0", expression)
    expression = expression.replace("&&", " and ").replace("||", " or ")
    expression = re.sub(r"!(?!=)", " not ", expression)
    node = ast.parse(expression.strip(), mode="eval").body

    def evaluate(node):
        if isinstance(node, ast.Constant) and isinstance(node.value, (int, bool)):
            return node.value
        if isinstance(node, ast.Name):
            return values[node.id]
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.Not):
            return not evaluate(node.operand)
        if isinstance(node, ast.BoolOp):
            parts = [bool(evaluate(part)) for part in node.values]
            return all(parts) if isinstance(node.op, ast.And) else any(parts)
        if isinstance(node, ast.Compare) and len(node.ops) == 1:
            left, right = evaluate(node.left), evaluate(node.comparators[0])
            operation = node.ops[0]
            if isinstance(operation, ast.Eq):
                return left == right
            if isinstance(operation, ast.NotEq):
                return left != right
            if isinstance(operation, ast.Lt):
                return left < right
            if isinstance(operation, ast.LtE):
                return left <= right
            if isinstance(operation, ast.Gt):
                return left > right
            if isinstance(operation, ast.GtE):
                return left >= right
        raise ValueError(f"Unsupported packet condition: {expression}")

    return bool(evaluate(node))


def apply_header(lines, version, packets):
    active = True
    branches = []
    for line in lines:
        line = line.strip()
        if line.startswith("#ifndef CLIF_"):
            branches.append([active, True])
        elif line.startswith("#if "):
            selected = condition(line[4:], version)
            branches.append([active, selected])
            active = active and selected
        elif line.startswith("#elif "):
            parent, selected = branches[-1]
            current = not selected and condition(line[6:], version)
            branches[-1][1] = selected or current
            active = parent and current
        elif line.startswith("#else"):
            parent, selected = branches[-1]
            active = parent and not selected
            branches[-1][1] = True
        elif line.startswith("#endif"):
            active = branches.pop()[0]
        elif active:
            declaration = re.match(r"(?:parseable_)?packet\(\s*(0x[\da-fA-F]+)\s*,(.*)\);", line)
            if not declaration:
                continue
            packet_id = int(declaration[1], 16)
            fields = [field.strip() for field in declaration[2].split(",")]
            if len(fields) == 7 and fields[1] == "clif_parse_UseSkillToPosMoreInfo":
                length = int(fields[0])
                offsets = tuple(int(field) for field in fields[2:])
                if any(offset < 2 or offset + 2 > length for offset in offsets[:4]) or length - offsets[4] not in (21, 80):
                    raise ValueError(f"Invalid Talkie Box layout: {line}")
                packets[packet_id] = (length, offsets)
            else:
                packets.pop(packet_id, None)
    if branches:
        raise ValueError("Unclosed packet condition")


def main():
    parser = argparse.ArgumentParser(description="Import main-client Talkie Box layouts from rAthena's shared packet headers.")
    parser.add_argument("rathena", type=pathlib.Path)
    parser.add_argument("--output", type=pathlib.Path, default=pathlib.Path("server/src/server/request_handler/talkie_box_packets.json"))
    arguments = parser.parse_args()
    headers = [(arguments.rathena / "src/map" / name).read_text().splitlines() for name in ("clif_packetdb.hpp", "clif_shuffle.hpp")]
    boundaries = {0}
    for lines in headers:
        for line in lines:
            if line.strip().startswith(("#if ", "#elif ")):
                for number in re.findall(r"\b\d{8}\b", line):
                    boundaries.update((int(number), int(number) + 1))
    open_ranges = {}
    rows = []
    for version in sorted(boundaries):
        packets = {}
        for lines in headers:
            apply_header(lines, version, packets)
        for key, start in list(open_ranges.items()):
            packet_id, length, offsets = key
            if packets.get(packet_id) != (length, offsets):
                rows.append([start, version, packet_id, length, list(offsets)])
                del open_ranges[key]
        for packet_id, (length, offsets) in packets.items():
            open_ranges.setdefault((packet_id, length, offsets), version)
    for (packet_id, length, offsets), start in open_ranges.items():
        rows.append([start, 4294967295, packet_id, length, list(offsets)])
    rows.sort()
    arguments.output.write_text("[\n" + ",\n".join("  " + json.dumps(row, separators=(",", ":")) for row in rows) + "\n]\n", encoding="utf-8")
    print(f"Imported {len(rows)} Talkie Box packet ranges into {arguments.output}")


if __name__ == "__main__":
    main()
