"""The command tables and statement rewrites the script-sdk-2 lowering in `sdk2.py` builds on."""

import dataclasses
import pathlib
import re

from .parser import (BackLoop, Block, Break, Call, Case, Command, DoWhile, ExprStatement, For, FunctionDefinition, Goto, If, Index, Label, Menu, Name,
                     Return, Str, Switch, While)

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
WRITABLE_PARAMETERS = {"zeny", "hp"}

# Dragons, Wargs and Mado Gear belong to renewal jobs: the commands do nothing and the queries answer "not mounted" on a pre-renewal server.
# set by the host before it runs `OnPCKillEvent` and `OnPCDieEvent`, readable as character temporary variables
EVENT_VARIABLES = {"killedrid", "killerrid"}
# Wug and Mado Gear options belong to renewal jobs
RENEWAL_OPTIONS = {"option_wug", "option_wugrider", "option_madogear"}
STORED_SCOPES = ("character", "character_temp", "account", "server", "server_temp", "npc", "instance")

RENEWAL_MOUNT_COMMANDS = {"setdragon", "setmadogear", "setwug"}
RENEWAL_MOUNT_QUERIES = {"checkdragon", "checkwug"}

STATUS_CALLS = {"sc_start": "StartStatus", "sc_start2": "StartStatus2", "sc_start4": "StartStatus4"}

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
    "areamonster": "AreaMonster", "areamobuseskill": "AreaMobUseSkill", "getpartyname": "GetPartyName", "instance_create": "InstanceCreate", "instance_destroy": "InstanceDestroy",
    "instance_enter": "InstanceEnter", "instance_npcname": "InstanceNpcName", "instance_mapname": "InstanceMapName", "instance_id": "InstanceId",
    "instance_warpall": "InstanceWarpAll", "instance_announce": "InstanceAnnounce", "instance_check_party": "InstanceCheckParty",
    "instance_check_guild": "InstanceCheckGuild", "instance_info": "InstanceInfo", "instance_live_info": "InstanceLiveInfo", "instance_list": "InstanceList",
    "getequiparmorlv": "GetEquipArmorLevel", "getequiprefinecost": "GetEquipRefineCost", "downrefitem": "DownRefineItem", "repair": "Repair",
    "getbattleflag": "GetBattleFlag", "ismounting": "IsMounting", "checkmadogear": "CheckMadogear", "guildopenstorage": "GuildOpenStorage",
    "warpparty": "PartyWarp", "itemskill": "ItemSkill", "unitskilluseid": "UnitSkillToId", "removemapflag": "RemoveMapFlag",
    "setmapflag": "SetMapFlag", "getsavepoint": "GetSavePoint", "resetlvl": "ResetLevel", "pushpc": "PushPc", "areaannounce": "AreaAnnounce",
    "mercenary_get_faith": "MercenaryGetFaith", "mercenary_set_faith": "MercenarySetFaith", "unloadnpc": "UnloadNpc", "movenpc": "MoveNpc",
    "getmonsterinfo": "GetMonsterInfo", "npctalk": "NpcTalk", "cloakonnpc": "DisableNpc", "cloakoffnpc": "EnableNpc",
    "gvgon": "GvgOn", "gvgoff": "GvgOff", "divorce": "Divorce",
    "getguildmaster": "GetGuildMaster", "getcastlename": "GetCastleName", "getgdskilllv": "GetGuildSkillLevel", "guardian": "Guardian",
    "flagemblem": "FlagEmblem",
    "maprespawnguildid": "MapRespawnGuildId", "attachrid": "AttachRid", "detachrid": "DetachRid", "playbgm": "PlayBgm",
    "misceffect": "NpcSpecialEffect", "delequip": "DelEquip", "equip": "Equip", "setitemscript": "SetItemScript",
    "requestguildinfo": "RequestGuildInfo", "wedding": "Wedding", "warpchar": "Warp", "isloggedin": "IsLoggedIn", "marriage": "Marriage",
}
for _name in ("getcastledata", "setcastledata"):
    SDK_CALLS.pop(_name, None)

def fold_is_function(node, world):
    """`if (is_function("X"))` is decided at conversion time: the converted sources either define X or they do not."""
    if isinstance(node, list):
        return [fold_is_function(item, world) for item in node]
    if isinstance(node, If) and isinstance(node.condition, Call) and node.condition.name.lower() == "is_function" and node.condition.args and isinstance(node.condition.args[0], Str):
        defined = node.condition.args[0].value.lower() in world.functions
        branch = node.then if defined else node.otherwise
        return fold_is_function(branch, world) if branch is not None else Block([])
    if dataclasses.is_dataclass(node) and not isinstance(node, type):
        for field in dataclasses.fields(node):
            setattr(node, field.name, fold_is_function(getattr(node, field.name), world))
    return node


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


def wrap_backward_labels(statements, root=None, top=True):
    """A label nested in a block that only `goto`s from after it, inside the same block, jump to is a loop: the rest of the block
    (up to the next `case`) becomes a `BackLoop` restarted by those jumps. Labels reachable from anywhere else stay as they are."""
    root = statements if root is None else root
    for node in list(statements):
        children = []
        if isinstance(node, (Block, Switch, BackLoop)):
            children.append(node.statements if isinstance(node, Block) else node.body)
        elif isinstance(node, If):
            children += [branch.statements for branch in (node.then, node.otherwise) if isinstance(branch, Block)]
        elif isinstance(node, (While, DoWhile, For)) and isinstance(node.body, Block):
            children.append(node.body.statements)
        for child in children:
            wrap_backward_labels(child, root, top=False)
    if top:
        return statements
    index = 0
    while index < len(statements):
        item = statements[index]
        if isinstance(item, Label):
            end = next((position for position in range(index + 1, len(statements)) if isinstance(statements[position], Case)), len(statements))
            region = statements[index + 1:end]
            if label_only_jumped_to_within(item.name, root, region):
                statements[index:end] = [BackLoop(item.name, region)]
        index += 1
    return statements


def label_only_jumped_to_within(name, root, region):
    def references(nodes):
        found = []

        def visit(node):
            if isinstance(node, (list, tuple)):
                for part in node:
                    visit(part)
            elif isinstance(node, Goto) and node.label == name:
                found.append("goto")
            elif isinstance(node, Menu) and any(label == name for _, label in node.options):
                found.append("menu")
            elif isinstance(node, Name) and node.name == name:
                found.append("name")
            elif hasattr(node, "__dataclass_fields__"):
                for field in node.__dataclass_fields__:
                    visit(getattr(node, field))

        visit(nodes)
        return found

    inside = references(region)
    return bool(inside) and set(inside) == {"goto"} and references(root) == inside


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
        elif isinstance(node, (Switch, BackLoop)):
            stack.extend(node.body)
