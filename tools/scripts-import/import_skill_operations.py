#!/usr/bin/env python3
"""Import pre-renewal skill/status metadata used by the compiled game API."""

import argparse
import json
import pathlib
import re

import yaml


CLASSIC_SKILLS = """
AC_CONCENTRATION AC_DOUBLE AL_ANGELUS AL_BLESSING AL_CRUCIS AL_DECAGI AL_HEAL
AL_INCAGI AL_PNEUMA AL_TELEPORT AS_CLOAKING AS_SONICBLOW AS_SPLASHER
BA_FROSTJOKER BS_GREED BS_HAMMERFALL BS_WEAPONPERFECT CG_ARROWVULCAN
CG_TAROTCARD CH_PALMSTRIKE CH_SOULCOLLECT CR_AUTOGUARD CR_GRANDCROSS
DC_SCREAM DC_WINKCHARM GS_GLITTERING GS_RAPIDSHOWER GS_SPREADATTACK GS_TRACKING
HP_ASSUMPTIO KN_BOWLINGBASH KN_PIERCE LK_AURABLADE LK_CONCENTRATION LK_JOINTBEAT
MC_IDENTIFY MC_LOUD MC_MAMMONITE MG_COLDBOLT MG_FIREBALL MG_FIREBOLT
MG_FIREWALL MG_FROSTDIVER MG_LIGHTNINGBOLT MG_SIGHT MG_SOULSTRIKE MG_STONECURSE
MG_THUNDERSTORM MO_CALLSPIRITS MO_EXPLOSIONSPIRITS MO_INVESTIGATE NJ_HUUMA NJ_ISSEN
NPC_ANTIMAGIC NPC_CRITICALWOUND NPC_DARKSTRIKE NPC_DRAGONFEAR NPC_EARTHQUAKE
NPC_HELLJUDGEMENT NPC_HELLPOWER NPC_MAGICMIRROR NPC_PULSESTRIKE NPC_SLOWCAST
NPC_STONESKIN NPC_VAMPIRE_GIFT NPC_WIDEBLEEDING NPC_WIDECONFUSE NPC_WIDECURSE
NPC_WIDESILENCE NPC_WIDESOULDRAIN NV_FIRSTAID PA_PRESSURE PR_ASPERSIO PR_GLORIA
PR_IMPOSITIO PR_KYRIE PR_LEXAETERNA PR_LEXDIVINA PR_MAGNIFICAT PR_STRECOVERY
PR_TURNUNDEAD RG_BACKSTAP RG_INTIMIDATE RG_RAID RG_STRIPARMOR RG_STRIPWEAPON
SA_DELUGE SA_DISPELL SA_FLAMELAUNCHER SA_FROSTWEAPON SA_LANDPROTECTOR
SA_LIGHTNINGLOADER SA_SEISMICWEAPON SA_SPELLBREAKER SM_BASH SM_ENDURE SM_MAGNUM
SM_PROVOKE SM_SELFPROVOKE ST_FULLSTRIP TF_BACKSLIDING TF_DETOXIFY TF_PICKSTONE
TF_POISON TK_HIGHJUMP TK_RUN WS_CARTTERMINATION WZ_EARTHSPIKE WZ_ESTIMATION
WZ_FROSTNOVA WZ_HEAVENDRIVE WZ_JUPITEL WZ_METEOR WZ_QUAGMIRE WZ_STORMGUST
WZ_VERMILION WZ_WATERBALL AL_CURE ALL_RESURRECTION ALL_PARTYFLEE ALL_REVERSEORCISH
ITEM_ENCHANTARMS
MC_VENDING MC_PUSHCART AM_CALLHOMUN AM_REST AM_RESURRECTHOMUN
""".split()


def write(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def import_metadata(root, reference):
    skills = yaml.safe_load((reference / "db/pre-re/skill_db.yml").read_text(encoding="utf-8"))["Body"]
    by_name = {skill["Name"]: skill for skill in skills}
    missing = set(CLASSIC_SKILLS) - by_name.keys()
    if missing:
        raise ValueError(f"Missing pre-renewal skill definitions: {sorted(missing)}")
    fields = ("Id", "Name", "MaxLevel", "Type", "TargetType", "Range", "HitCount", "Element", "SplashArea", "CastTime", "CastCancel", "CastTimeFlags", "AfterCastActDelay", "AfterCastWalkDelay", "Duration1", "Duration2", "Cooldown", "Knockback", "Status", "Flags", "DamageFlags", "Unit", "Requires")
    selected_skills = set(CLASSIC_SKILLS) | {skill["Name"] for skill in skills if 0 < skill["Id"] < 1000 or 8001 <= skill["Id"] <= 8016 or 8201 <= skill["Id"] <= 8240}
    skill_fields = fields
    kinds = {name for name in re.findall(r'\w+\s*=\s*\d+\s*=>\s*"([A-Z0-9_]+)"', (root / "lib/models/src/status_change.rs").read_text(encoding="utf-8"))}
    statuses = yaml.safe_load((reference / "db/pre-re/status.yml").read_text(encoding="utf-8"))["Body"]
    fields = ("Icon", "DurationLookup", "States", "Opt1", "Opt2", "Options", "Flags", "Fail", "EndOnStart", "EndReturn", "EndOnEnd", "MinRate", "MinDuration")
    selected = {status["Status"].upper(): {field: status[field] for field in fields if field in status} for status in statuses if status["Status"].upper() in kinds}
    if kinds - selected.keys():
        raise ValueError(f"Missing pre-renewal status definitions: {sorted(kinds - selected.keys())}")
    selected_skills.update(status["DurationLookup"] for status in selected.values() if status.get("DurationLookup") in by_name)
    write(root / "server/src/server/script/skill_metadata.json", [{field: by_name[name][field] for field in skill_fields if field in by_name[name]} for name in sorted(selected_skills)])
    write(root / "lib/models/src/status_change_metadata.json", selected)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--rathena", type=pathlib.Path, default=pathlib.Path("../rathena"))
    args = parser.parse_args()
    import_metadata(pathlib.Path(__file__).resolve().parents[2], args.rathena.resolve())
