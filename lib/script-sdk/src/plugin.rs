//! What a plugin declares about itself, the hooks it observes and the decisions it can intercept (`docs/adr/7-plugins.md`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Function, Request, Variable, VariableScope};

/// An NPC a plugin places, from its `npcs.json` or its `generated_npcs`: the shape of `config/wasm/npcs.json` without
/// `module`. With `replaces`, it takes the place of the NPC of the same name, and the fields it leaves out are that NPC's.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PluginNpc {
    pub name: String,
    /// The plugin's NPC entry; the NPC's name when left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_name: Option<String>,
    /// A sprite number or constant name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_size: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_size: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constructor_args: Option<Vec<crate::Value>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub test: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replaces: bool,
    /// What a player-look NPC (a job sprite) wears.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appearance: Option<NpcAppearance>,
    /// Walking speed in ms per cell, 20 to 1000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<u16>,
}

/// The looks of a player-look NPC, as `setunitdata` sets them. Headgears, weapon and shield are view ids.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct NpcAppearance {
    /// 0 female, 1 male.
    pub sex: u8,
    pub hair: u16,
    pub hair_color: u16,
    pub clothes_color: u16,
    pub head_top: u16,
    pub head_mid: u16,
    pub head_bottom: u16,
    pub weapon: u16,
    pub shield: u16,
}

/// One page of a plugin's generated NPCs, answered by its `plugin_npcs` export for `{ "from": index }`.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PluginNpcPage {
    pub npcs: Vec<PluginNpc>,
    /// Where the next page starts, when there is one.
    pub next: Option<usize>,
}

/// Size of the guest buffer (`plugin_buffer`) that carries `plugin_info` and interceptor messages, which run without host calls.
pub const PLUGIN_BUFFER_BYTES: usize = 16 * 1024;

/// Groups of host functions a plugin must declare before calling them. Reads, variables, NPC timers and events of the plugin's
/// own NPCs need none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    /// Dialogue windows, messages and effects shown to the attached player.
    Dialogue,
    /// Warps, monsters, map flags, NPCs, instances, battlegrounds, castles.
    World,
    /// Items, zeny, shops, storage, refining.
    Economy,
    /// Job, levels, skills, statuses, healing, quests of a character.
    Character,
    /// Experience and drop rates.
    Rates,
    /// Permission groups and account data.
    Accounts,
    /// Announcements to a map or the server.
    Broadcast,
    /// Plugin windows and overhead notifications (`plugin_ui`).
    Interface,
}

impl Capability {
    pub const ALL: [Capability; 8] = [
        Capability::Dialogue,
        Capability::World,
        Capability::Economy,
        Capability::Character,
        Capability::Rates,
        Capability::Accounts,
        Capability::Broadcast,
        Capability::Interface,
    ];

    const fn bit(self) -> u16 {
        1 << self as u16
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Capabilities(u16);

impl Capabilities {
    pub fn all() -> Self {
        Self::from_iter(Capability::ALL)
    }

    pub fn contains(self, capability: Capability) -> bool {
        self.0 & capability.bit() != 0
    }

    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn iter(self) -> impl Iterator<Item = Capability> {
        Capability::ALL.into_iter().filter(move |capability| self.contains(*capability))
    }
}

impl FromIterator<Capability> for Capabilities {
    fn from_iter<I: IntoIterator<Item = Capability>>(capabilities: I) -> Self {
        Self(capabilities.into_iter().fold(0, |bits, capability| bits | capability.bit()))
    }
}

/// What `plugin_info` reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Must match the plugin's folder name.
    pub name: String,
    pub version: String,
    /// The [`ABI_VERSION`](crate::ABI_VERSION) the plugin was built against.
    pub sdk_abi: u32,
    /// Plugins that must be loaded before this one.
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
    #[serde(default)]
    pub hooks: Vec<HookKind>,
    #[serde(default)]
    pub interceptors: Vec<InterceptorKind>,
    /// Ids of the windows whose events the plugin handles.
    #[serde(default)]
    pub windows: Vec<String>,
    #[serde(default)]
    pub bot_actions: Vec<BotActionInfo>,
    /// The plugin's `info.json`, compiled into the module.
    pub manifest: PluginManifest,
}

/// A plugin's `info.json`: what it is and the settings it reads, with their defaults. The operator's
/// `config/plugins/<name>.json` overrides the defaults ([`PluginManifest::configure`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginManifest {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub config: BTreeMap<String, ConfigEntry>,
}

/// A setting of [`PluginManifest::config`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigEntry {
    /// Its value when the operator does not set it. An override must have the same JSON type, unless the default is `null`.
    pub default: serde_json::Value,
    #[serde(default)]
    pub description: String,
}

impl PluginManifest {
    pub fn parse(text: &str) -> Result<Self, String> {
        let manifest: Self = serde_json::from_str(text).map_err(|error| format!("info.json is invalid: {error}"))?;
        if manifest.title.trim().is_empty() {
            return Err("info.json has no title".into());
        }
        Ok(manifest)
    }

    pub fn defaults(&self) -> serde_json::Map<String, serde_json::Value> {
        self.config.iter().map(|(key, entry)| (key.clone(), entry.default.clone())).collect()
    }

    /// The settings the plugin runs with: its defaults, replaced key by key by `overrides`. A key the manifest does not
    /// declare, or a value of another type than its default, is refused. Arrays and objects are replaced whole.
    pub fn configure(&self, overrides: &serde_json::Value) -> Result<serde_json::Map<String, serde_json::Value>, String> {
        let serde_json::Value::Object(overrides) = overrides else { return Err("it must be a JSON object".into()) };
        let mut settings = self.defaults();
        for (key, value) in overrides {
            let Some(entry) = self.config.get(key) else {
                return Err(format!("{key} is not a setting of this plugin (known: {})", self.config.keys().cloned().collect::<Vec<_>>().join(", ")));
            };
            if !entry.default.is_null() && json_type(value) != json_type(&entry.default) {
                return Err(format!("{key} must be {}, not {}", json_type(&entry.default), json_type(value)));
            }
            settings.insert(key.clone(), value.clone());
        }
        Ok(settings)
    }
}

fn json_type(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "a boolean",
        serde_json::Value::Number(_) => "a number",
        serde_json::Value::String(_) => "a string",
        serde_json::Value::Array(_) => "an array",
        serde_json::Value::Object(_) => "an object",
    }
}

/// An action a plugin offers to bots, listed in their observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotActionInfo {
    pub name: String,
    /// What it does and the `args` it takes, for whoever writes the bot.
    pub description: String,
}

/// A bot's request for one of the plugin's actions; `args` is JSON.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BotAction {
    pub name: String,
    pub args: String,
}

/// Server rates a plugin can scale, in percent of the configured rate (100 leaves it unchanged).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rate {
    BaseExp,
    JobExp,
    Drop,
    CardDrop,
}

impl Rate {
    pub const ALL: [Rate; 4] = [Rate::BaseExp, Rate::JobExp, Rate::Drop, Rate::CardDrop];

    pub fn name(self) -> &'static str {
        match self {
            Rate::BaseExp => "BaseExp",
            Rate::JobExp => "JobExp",
            Rate::Drop => "Drop",
            Rate::CardDrop => "CardDrop",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|rate| rate.name() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LevelKind {
    Base,
    Job,
}

/// Something that already happened to a character. A plugin that declared its kind runs with the character attached.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hook {
    Login { char_id: u32, account_id: u32, map: String },
    Logout { char_id: u32, account_id: u32 },
    MapEnter { char_id: u32, map: String, x: u16, y: u16 },
    LevelUp { char_id: u32, kind: LevelKind, level: u32 },
    JobChange { char_id: u32, job: u32 },
    /// The character killed another player.
    Kill { char_id: u32, victim_char_id: u32, map: String },
    Die { char_id: u32, killer_id: Option<u32>, map: String },
    MobKilled { char_id: u32, mob_id: u32, map: String, x: u16, y: u16 },
    ItemObtained { char_id: u32, item_id: u32, amount: u32 },
    ItemUsed { char_id: u32, item_id: u32 },
    TradeCompleted { char_id: u32, partner_char_id: u32 },
    SkillUsed { char_id: u32, skill_id: u32, level: u32, target_id: Option<u32> },
    /// `party_id` 0: the character left their party.
    PartyChanged { char_id: u32, party_id: u32 },
    /// `guild_id` 0: the character left their guild.
    GuildChanged { char_id: u32, guild_id: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HookKind {
    Login,
    Logout,
    MapEnter,
    LevelUp,
    JobChange,
    Kill,
    Die,
    MobKilled,
    ItemObtained,
    ItemUsed,
    TradeCompleted,
    SkillUsed,
    PartyChanged,
    GuildChanged,
}

impl HookKind {
    pub const fn bit(self) -> u32 {
        1 << self as u32
    }
}

impl Hook {
    pub fn kind(&self) -> HookKind {
        match self {
            Hook::Login { .. } => HookKind::Login,
            Hook::Logout { .. } => HookKind::Logout,
            Hook::MapEnter { .. } => HookKind::MapEnter,
            Hook::LevelUp { .. } => HookKind::LevelUp,
            Hook::JobChange { .. } => HookKind::JobChange,
            Hook::Kill { .. } => HookKind::Kill,
            Hook::Die { .. } => HookKind::Die,
            Hook::MobKilled { .. } => HookKind::MobKilled,
            Hook::ItemObtained { .. } => HookKind::ItemObtained,
            Hook::ItemUsed { .. } => HookKind::ItemUsed,
            Hook::TradeCompleted { .. } => HookKind::TradeCompleted,
            Hook::SkillUsed { .. } => HookKind::SkillUsed,
            Hook::PartyChanged { .. } => HookKind::PartyChanged,
            Hook::GuildChanged { .. } => HookKind::GuildChanged,
        }
    }

    /// The character the hook is about.
    pub fn char_id(&self) -> u32 {
        match self {
            Hook::Login { char_id, .. }
            | Hook::Logout { char_id, .. }
            | Hook::MapEnter { char_id, .. }
            | Hook::LevelUp { char_id, .. }
            | Hook::JobChange { char_id, .. }
            | Hook::Kill { char_id, .. }
            | Hook::Die { char_id, .. }
            | Hook::MobKilled { char_id, .. }
            | Hook::ItemObtained { char_id, .. }
            | Hook::ItemUsed { char_id, .. }
            | Hook::TradeCompleted { char_id, .. }
            | Hook::SkillUsed { char_id, .. }
            | Hook::PartyChanged { char_id, .. }
            | Hook::GuildChanged { char_id, .. } => *char_id,
        }
    }
}

/// A decision the game is about to make, handed to the plugins that intercept its kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interception {
    TradeRequest { char_id: u32, target_char_id: u32, map: String },
    /// A warp the server is about to make: portals, scripts, Teleport, Butterfly Wing. The placement at login and the respawn
    /// of a dead character are not intercepted.
    Warp { char_id: u32, from_map: String, to_map: String, x: u16, y: u16 },
    /// The player drops an item on the floor.
    ItemDrop { char_id: u32, item_id: u32, amount: u32, map: String },
    /// The experience a character receives for a monster, after the server rates.
    ExpGain { char_id: u32, mob_id: u32, base: u32, job: u32 },
    ItemUse { char_id: u32, item_id: u32, map: String },
    /// Public chat, without the `Name : ` prefix.
    Chat { char_id: u32, map: String, message: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterceptorKind {
    TradeRequest,
    Warp,
    ItemDrop,
    ExpGain,
    ItemUse,
    Chat,
}

impl InterceptorKind {
    pub const fn bit(self) -> u32 {
        1 << self as u32
    }
}

impl Interception {
    pub fn kind(&self) -> InterceptorKind {
        match self {
            Interception::TradeRequest { .. } => InterceptorKind::TradeRequest,
            Interception::Warp { .. } => InterceptorKind::Warp,
            Interception::ItemDrop { .. } => InterceptorKind::ItemDrop,
            Interception::ExpGain { .. } => InterceptorKind::ExpGain,
            Interception::ItemUse { .. } => InterceptorKind::ItemUse,
            Interception::Chat { .. } => InterceptorKind::Chat,
        }
    }

    /// The decision after `verdict`, as the next plugin sees it; `None` once refused.
    pub fn apply(mut self, verdict: &Verdict) -> Result<Option<Self>, String> {
        match (verdict, &mut self) {
            (Verdict::Allow, _) => Ok(Some(self)),
            (Verdict::Refuse(_), _) => Ok(None),
            (Verdict::Exp { base, job }, Interception::ExpGain { base: current_base, job: current_job, .. }) => {
                (*current_base, *current_job) = (*base, *job);
                Ok(Some(self))
            }
            (Verdict::Chat(text), Interception::Chat { message, .. }) => {
                *message = text.clone();
                Ok(Some(self))
            }
            (verdict, interception) => Err(format!("{verdict:?} does not answer {:?}", interception.kind())),
        }
    }
}

/// An interceptor's answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// Let the game go on with the decision as it is.
    Allow,
    /// Cancel it; a non-empty text is shown to the character.
    Refuse(String),
    /// [`Interception::ExpGain`] only: the experience to grant instead.
    Exp { base: u32, job: u32 },
    /// [`Interception::Chat`] only: the text to say instead.
    Chat(String),
}

impl Function {
    /// The capability a plugin needs to call this function; `None` for reads and the plugin's own state.
    pub fn capability(self) -> Option<Capability> {
        use Capability::*;
        use Function::*;
        match self {
            Print | GetLook | GetBattleFlag | StrCharInfo | JobName | EaClass | RoClass | CheckFalcon | CheckCart | CheckRiding
            | IsMounting | CheckMadogear | ReadParam | GetRefine | GetSkillLv | IsEquipped | GetEquipId | GetEquipRefineryCnt
            | GetItemInfo | GetItemName | Rand | Max | Min | Pow | GetTime | VipStatus | GetPartnerId | CountItem | CountItem2
            | CallFunction | GetFame | GetFameRank | GetPetInfo | GetMapFlag | GetCharacterId | GetUnitData | GetNpcId | DoEvent
            | DoNpcEvent | AddTimer | DeleteTimer | AddTimerCount | InitNpcTimer | StartNpcTimer | StopNpcTimer | SetNpcTimer
            | GetNpcTimer | AttachNpcTimer | DetachNpcTimer | GetSavePoint | AgitCheck | GetCastleData | GetGuildInfo
            | GetGuildSkillLevel | BgGetData | BgGetAreaUsers | MobCount | CheckQuest | IsBeginQuest | CheckWeight | StrNpcInfo
            | GetMapUsers | GetAreaUsers | Sleep | GetVariableOfNpc | GetTimeTick | GetTimeStr | GetMapXy | GetPartyMember
            | IsPartyLeader | ConvertPcInfo | InstanceNpcName | InstanceMapName | InstanceId | InstanceCheckParty
            | InstanceCheckGuild | InstanceInfo | InstanceLiveInfo | InstanceList | GetInstanceVar | GetPartyName | GetEquipName
            | GetEquipWeaponLevel | GetEquipArmorLevel | GetEquipCardId | GetEquipIsEquipped | GetEquipIsEnableRefine
            | GetEquipPercentRefinery | GetEquipRefineCost | GetBrokenId | GetGmLevel | ArrayGet | ArraySet | GetWaitingRoomState
            | GetInventoryList | MercenaryGetFaith | GetMonsterInfo | GetGuildMaster | GetCastleName | AttachRid | DetachRid
            | IsLoggedIn | RequestGuildInfo | PluginConfig | GetRate | BotObserve | BotReply => None,
            Mes | Close | Next | Select | InputNumber | InputString | Message | DispBottom | Cutin | Emotion | ShowEvent | QuestInfo
            | QuestInfoRefresh | ViewPoint | NpcTalk | SoundEffect | ProgressBar | SpecialEffect | SkillEffect | NpcSpecialEffect
            | PlayBgm | ReadBook => Some(Dialogue),
            Announce | MapAnnounce | AreaAnnounce | InstanceAnnounce | SoundEffectAll | BgInfo => Some(Broadcast),
            GetItem | DelItem | RandomGroupItem | GetGroupItem | Produce | Cooking | Shop | BuyingStore | SearchStores | OpenStorage
            | GuildOpenStorage | ConsumeItem | GetNamedItem | GetItem2 | MakeItem | SuccessRefineItem | FailedRefineItem
            | DownRefineItem | Repair | RepairAll | DelItem2 | DelEquip | OpenMail | OpenAuction | SetItemScript => Some(Economy),
            JobChange | SetCart | SetFalcon | SetRiding | SetLook | Bonus | Bonus2 | Bonus3 | Bonus4 | Bonus5 | Skill | ItemSkill
            | ItemHeal | PercentHeal | Heal | StartStatus | StartStatus2 | StartStatus4 | EndStatus | AutoBonus | AutoBonus2
            | AutoBonus3 | Pet | BirthPet | MercenaryCreate | MercenaryStartStatus | MercenaryHeal | MercenarySetFaith | SetFont
            | Homevolution | GetExperience | ResetLevel | ResetSkills | AddFame | PetSkillBonus | PetRecovery | PetSkillAttack
            | PetSkillAttack2 | PetSkillSupport | PetLoot | PetAutoBonus | PetAutoBonus2 | PetAutoBonus3 | SavePoint | Marriage
            | Divorce | Wedding | Nude | Equip | SetQuest | CompleteQuest | EraseQuest | ChangeQuest => Some(Character),
            Warp | PartyWarp | Monster | AreaMonster | KillMonster | SetMobImmunity | MapWarp | AreaWarp | AreaPercentHeal
            | EnableNpc | DisableNpc | SetCell | SetMapFlag | RemoveMapFlag | SetMapFlagNoSave | PvpOn | PvpOff | GvgOn | GvgOff
            | WarpPortal | AgitStart | AgitEnd | SetCastleData | GuardianSummon | Guardian | FlagEmblem | MapRespawnGuildId
            | BgCreate | BgJoin | BgLeave | BgDesert | BgDestroy | BgWarp | BgTeamSetXy | BgUpdateScore | BgReserve | BgUnbook
            | BgMonster | BgMonsterSetTeam | InstanceCreate | InstanceDestroy | InstanceEnter | InstanceWarpAll | SetInstanceVar
            | AreaMobUseSkill | WaitingRoom | DelWaitingRoom | WaitingRoomKick | WaitingRoomKickAll | EnableWaitingRoomEvent
            | DisableWaitingRoomEvent | WarpWaitingPc | PushPc | UnloadNpc | MoveNpc | NpcSpeed | NpcWalkTo | NpcStop | UnitWalk | UnitWalkTo | UnitStopWalk | SetNpcDisplay | SetUnitData | UnitSkill
            | UnitSkillToId | UnitSkillToPosition | SetVariableOfNpc | GuildExperience => Some(World),
            SetRate => Some(Rates),
            UiOpen | UiClose | UiOverhead => Some(Interface),
        }
    }
}

impl Request {
    /// The capability a plugin needs to send this request.
    pub fn capability(&self) -> Option<Capability> {
        match self {
            Request::Call { function, .. } => function.capability(),
            Request::Write { name, .. } if name == "Zeny" => Some(Capability::Economy),
            Request::Write { name, .. } if name == "Hp" => Some(Capability::Character),
            Request::Purchase(_) | Request::Sale(_) => Some(Capability::Economy),
            _ => None,
        }
    }

    /// The same request with the plugin's server and account variables (`$`, `$@`, `#`, `##`) renamed under `plugin`, so they
    /// cannot meet the variables of the base scripts or of other plugins.
    pub fn for_plugin(self, plugin: &str) -> Request {
        let variable = |mut variable: Variable| {
            if shared_scope(variable.scope) {
                variable.name = format!("{plugin}::{}", variable.name);
            }
            variable
        };
        match self {
            Request::Read(name) => Request::Read(namespaced(plugin, name)),
            Request::Write { name, value } => Request::Write { name: namespaced(plugin, name), value },
            Request::VariableRead { scope, name, index } if shared_scope(scope) => {
                Request::VariableRead { scope, name: format!("{plugin}::{name}"), index }
            }
            Request::VariablesWrite(variables) => Request::VariablesWrite(variables.into_iter().map(variable).collect()),
            Request::VariablesIncrement(variables) => Request::VariablesIncrement(variables.into_iter().map(variable).collect()),
            Request::Batch(requests) => Request::Batch(requests.into_iter().map(|request| request.for_plugin(plugin)).collect()),
            Request::Call { function: function @ (Function::ArrayGet | Function::ArraySet), mut arguments } => {
                if let Some(crate::Value::String(name)) = arguments.first_mut() {
                    *name = namespaced(plugin, std::mem::take(name));
                }
                Request::Call { function, arguments }
            }
            request => request,
        }
    }
}

fn shared_scope(scope: VariableScope) -> bool {
    matches!(scope, VariableScope::Server | VariableScope::ServerTemporary | VariableScope::Account)
}

/// `$name` becomes `$plugin::name`; names of other scopes are kept.
fn namespaced(plugin: &str, name: String) -> String {
    let sigil = ["##", "$@", "#", "$"].into_iter().find(|sigil| name.starts_with(sigil));
    match sigil {
        Some(sigil) => format!("{sigil}{plugin}::{}", &name[sigil.len()..]),
        None => name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn shared_variables_are_renamed_under_the_plugin() {
        let read = |name: &str| match Request::Read(name.into()).for_plugin("p") {
            Request::Read(name) => name,
            _ => unreachable!(),
        };
        assert_eq!(read("$count"), "$p::count");
        assert_eq!(read("$@count"), "$@p::count");
        assert_eq!(read("#points"), "#p::points");
        assert_eq!(read("##points"), "##p::points");
        assert_eq!(read("kills"), "kills", "character variables are shared");
        assert_eq!(read("@temp"), "@temp");
        assert_eq!(read(".npc"), ".npc");
        assert_eq!(read("Zeny"), "Zeny");

        let written = Request::VariablesWrite(vec![
            Variable { scope: VariableScope::Server, name: "x".into(), index: 0, value: Value::Number(1) },
            Variable { scope: VariableScope::Character, name: "y".into(), index: 0, value: Value::Number(1) },
        ])
        .for_plugin("p");
        let Request::VariablesWrite(variables) = written else { unreachable!() };
        assert_eq!(variables[0].name, "p::x");
        assert_eq!(variables[1].name, "y");

        let array = Request::Call { function: Function::ArraySet, arguments: vec!["$list".into(), Value::Array(vec![])] }.for_plugin("p");
        assert!(matches!(array, Request::Call { arguments, .. } if arguments[0] == Value::from("$p::list")));
    }

    #[test]
    fn writes_with_side_effects_need_their_capability() {
        assert_eq!(Request::Write { name: "Zeny".into(), value: Value::Number(1) }.capability(), Some(Capability::Economy));
        assert_eq!(Request::Write { name: "$x".into(), value: Value::Number(1) }.capability(), None);
        assert_eq!(Request::Call { function: Function::Warp, arguments: vec![] }.capability(), Some(Capability::World));
        assert_eq!(Request::Call { function: Function::Rand, arguments: vec![] }.capability(), None);
        assert_eq!(Request::Call { function: Function::SetRate, arguments: vec![] }.capability(), Some(Capability::Rates));
    }

    #[test]
    fn capability_sets_narrow_by_intersection() {
        let declared: Capabilities = [Capability::World, Capability::Rates].into_iter().collect();
        let allowed: Capabilities = [Capability::Rates, Capability::Economy].into_iter().collect();
        let effective = declared.intersection(allowed);
        assert!(effective.contains(Capability::Rates));
        assert!(!effective.contains(Capability::World));
        assert!(!effective.contains(Capability::Economy));
        assert_eq!(Capabilities::all().iter().count(), Capability::ALL.len());
    }

    #[test]
    fn interceptors_see_the_previous_answer() {
        let gain = Interception::ExpGain { char_id: 1, mob_id: 1002, base: 10, job: 5 };
        let doubled = gain.apply(&Verdict::Exp { base: 20, job: 10 }).unwrap().unwrap();
        assert_eq!(doubled, Interception::ExpGain { char_id: 1, mob_id: 1002, base: 20, job: 10 });
        assert_eq!(doubled.clone().apply(&Verdict::Refuse(String::new())).unwrap(), None);
        assert!(doubled.apply(&Verdict::Chat("hi".into())).is_err());
    }

    fn manifest() -> PluginManifest {
        PluginManifest::parse(
            r#"{
                "title": "Event", "description": "A rate event.",
                "config": {
                    "percent": { "default": 200, "description": "Experience in percent." },
                    "stock": { "default": [1, 2] },
                    "anything": { "default": null }
                }
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn overrides_replace_defaults_key_by_key() {
        let settings = manifest().configure(&serde_json::json!({ "percent": 300, "stock": [3] })).unwrap();
        assert_eq!(serde_json::Value::Object(settings), serde_json::json!({ "percent": 300, "stock": [3], "anything": null }));
        assert_eq!(manifest().configure(&serde_json::json!({})).unwrap(), manifest().defaults());
        assert!(manifest().configure(&serde_json::json!({ "anything": { "x": 1 } })).is_ok(), "a null default takes any type");
    }

    #[test]
    fn overrides_of_unknown_keys_or_other_types_are_refused() {
        let unknown = manifest().configure(&serde_json::json!({ "percnt": 300 })).unwrap_err();
        assert!(unknown.contains("percnt is not a setting") && unknown.contains("percent"), "{unknown}");
        assert_eq!(manifest().configure(&serde_json::json!({ "percent": "300" })).unwrap_err(), "percent must be a number, not a string");
        assert!(manifest().configure(&serde_json::json!([1])).is_err());
    }

    #[test]
    fn a_manifest_needs_a_title_and_known_fields() {
        assert!(PluginManifest::parse(r#"{ "title": "", "description": "" }"#).unwrap_err().contains("no title"));
        assert!(PluginManifest::parse(r#"{ "title": "t", "description": "", "config": { "k": { "defualt": 1 } } }"#).is_err());
        assert!(PluginManifest::parse(r#"{ "title": "t" }"#).unwrap_err().contains("description"));
    }
}
