"""Names a rathena script may use as constants, so bare identifiers can be told apart from character variables.
rathena identifiers are case-insensitive: both maps are keyed by the lower-cased name and give the canonical spelling."""

import re

import yaml

try:
    Loader = yaml.CSafeLoader
except AttributeError:
    Loader = yaml.SafeLoader


def load_names(rathena):
    constants, parameters = set(), set()
    header = (rathena / "src/map/script_constants.hpp").read_text(encoding="utf-8", errors="replace")
    for name in re.findall(r"export_(?:deprecated_)?constant\((\w+)\)", header):
        constants.add(name)
    for name in re.findall(r"export_(?:deprecated_)?constant_npc\((\w+)\)", header):
        constants.add(name[3:])
    for name in re.findall(r'export_(?:deprecated_)?constant[23]?\("([^"]+)"', header):
        constants.add(name)
    for name in re.findall(r'export_parameter\("([^"]+)"', header):
        parameters.add(name)
    for database, key in (("item_db.yml", "AegisName"), ("mob_db.yml", "AegisName"), ("skill_db.yml", "Name")):
        for path in sorted((rathena / "db/pre-re").glob(database.replace(".yml", "*.yml"))):
            body = yaml.load(path.read_text(encoding="utf-8"), Loader=Loader).get("Body") or []
            constants.update(str(entry[key]) for entry in body if key in entry)
    return {name.lower(): name for name in constants}, {name.lower(): name for name in parameters}
