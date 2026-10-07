use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Trade,
    Party,
    AllSkill,
    AllEquipment,
    SkillUnconditional,
    JoinChat,
    KickChat,
    HideSession,
    WhoDisplayAid,
    HackInfo,
    AnyWarp,
    ViewHpMeter,
    ViewEquipment,
    UseCheck,
    UseChangeMapType,
    AllCommands,
    ReceiveRequests,
    ShowBossMobs,
    DisablePvm,
    DisablePvp,
    DisableCommandsWhenDead,
    ChannelAdmin,
    TradeBounded,
    ItemUnconditional,
    CommandEnable,
    BypassStatOnClone,
    BypassMaxStat,
    Attendance,
    MacroDetect,
    MacroRegister,
    TradeUnconditional,
}

impl Permission {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "can_trade" => Self::Trade,
            "can_party" => Self::Party,
            "all_skill" => Self::AllSkill,
            "all_equipment" => Self::AllEquipment,
            "skill_unconditional" => Self::SkillUnconditional,
            "join_chat" => Self::JoinChat,
            "kick_chat" => Self::KickChat,
            "hide_session" => Self::HideSession,
            "who_display_aid" => Self::WhoDisplayAid,
            "hack_info" => Self::HackInfo,
            "any_warp" => Self::AnyWarp,
            "view_hpmeter" => Self::ViewHpMeter,
            "view_equipment" => Self::ViewEquipment,
            "use_check" => Self::UseCheck,
            "use_changemaptype" => Self::UseChangeMapType,
            "all_commands" => Self::AllCommands,
            "receive_requests" => Self::ReceiveRequests,
            "show_bossmobs" => Self::ShowBossMobs,
            "disable_pvm" => Self::DisablePvm,
            "disable_pvp" => Self::DisablePvp,
            "disable_commands_when_dead" => Self::DisableCommandsWhenDead,
            "channel_admin" => Self::ChannelAdmin,
            "can_trade_bounded" => Self::TradeBounded,
            "item_unconditional" => Self::ItemUnconditional,
            "command_enable" => Self::CommandEnable,
            "bypass_stat_onclone" => Self::BypassStatOnClone,
            "bypass_max_stat" => Self::BypassMaxStat,
            "attendance" => Self::Attendance,
            "macro_detect" => Self::MacroDetect,
            "macro_register" => Self::MacroRegister,
            "trade_unconditional" => Self::TradeUnconditional,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    At,
    Char,
}

#[derive(Debug, Clone, Deserialize)]
struct GroupEntry {
    id: u32,
    name: String,
    #[serde(default)]
    level: u8,
    #[serde(default)]
    log_commands: bool,
    #[serde(default)]
    commands: Vec<String>,
    #[serde(default)]
    char_commands: Vec<String>,
    #[serde(default)]
    permissions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GroupFile {
    groups: Vec<GroupEntry>,
    #[serde(default)]
    atcommands: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct PlayerGroup {
    pub id: u32,
    pub name: String,
    pub level: u8,
    pub log_commands: bool,
    commands: HashSet<String>,
    char_commands: HashSet<String>,
    permissions: HashSet<Permission>,
}

impl PlayerGroup {
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn can_use_command(&self, command: &str, kind: CommandKind) -> bool {
        if self.has_permission(Permission::AllCommands) {
            return true;
        }
        match kind {
            CommandKind::At => self.commands.contains(command),
            CommandKind::Char => self.char_commands.contains(command),
        }
    }
}

/// Group `0` is the default group of every account and is always defined.
pub const DEFAULT_GROUP_ID: u32 = 0;

#[derive(Debug, Clone)]
pub struct PermissionGroups {
    groups: HashMap<u32, PlayerGroup>,
    canonical_commands: HashMap<String, String>,
}

impl Default for PermissionGroups {
    fn default() -> Self {
        Self::builtin()
    }
}

impl PermissionGroups {
    /// The stock `Player` group, used when no group file is configured.
    pub fn builtin() -> Self {
        let player = PlayerGroup {
            id: DEFAULT_GROUP_ID,
            name: "Player".into(),
            level: 0,
            log_commands: false,
            commands: ["changedress", "resurrect"].map(String::from).into(),
            char_commands: HashSet::new(),
            permissions: [Permission::Trade, Permission::Party, Permission::Attendance].into(),
        };
        Self { groups: HashMap::from([(DEFAULT_GROUP_ID, player)]), canonical_commands: HashMap::new() }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let json = std::fs::read_to_string(path).map_err(|error| format!("Cannot read {}: {error}", path.display()))?;
        Self::from_json(&json)
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let file: GroupFile = serde_json::from_str(json).map_err(|error| format!("Invalid group file: {error}"))?;
        let mut groups = HashMap::new();
        for entry in file.groups {
            let permissions = entry
                .permissions
                .iter()
                .map(|name| Permission::from_name(name).ok_or_else(|| format!("Group {} has unknown permission {name}", entry.name)))
                .collect::<Result<HashSet<_>, _>>()?;
            let group = PlayerGroup {
                id: entry.id,
                name: entry.name,
                level: entry.level.min(99),
                log_commands: entry.log_commands,
                commands: entry.commands.into_iter().collect(),
                char_commands: entry.char_commands.into_iter().collect(),
                permissions,
            };
            if groups.insert(group.id, group).is_some() {
                return Err(format!("Group id {} is defined twice", entry.id));
            }
        }
        if !groups.contains_key(&DEFAULT_GROUP_ID) {
            return Err("Group 0, the default group of every account, is missing".into());
        }
        let mut canonical_commands = HashMap::new();
        for (command, aliases) in file.atcommands {
            for alias in aliases {
                canonical_commands.insert(alias, command.clone());
            }
            canonical_commands.insert(command.clone(), command);
        }
        Ok(Self { groups, canonical_commands })
    }

    pub fn group(&self, group_id: u32) -> &PlayerGroup {
        self.groups.get(&group_id).unwrap_or_else(|| &self.groups[&DEFAULT_GROUP_ID])
    }

    pub fn exists(&self, group_id: u32) -> bool {
        self.groups.contains_key(&group_id)
    }

    pub fn has_permission(&self, group_id: u32, permission: Permission) -> bool {
        self.group(group_id).has_permission(permission)
    }

    pub fn level(&self, group_id: u32) -> u8 {
        self.group(group_id).level
    }

    /// The command a typed word stands for; words the group file does not know keep their own name.
    pub fn canonical_command(&self, word: &str) -> String {
        let word = word.to_ascii_lowercase();
        self.canonical_commands.get(&word).cloned().unwrap_or(word)
    }

    /// Canonical names of the commands a group may use, sorted.
    pub fn usable_commands(&self, group_id: u32, kind: CommandKind) -> Vec<String> {
        let group = self.group(group_id);
        let mut names: Vec<String> = self
            .canonical_commands
            .values()
            .filter(|name| group.can_use_command(name, kind))
            .cloned()
            .collect();
        names.sort();
        names.dedup();
        names
    }

    pub fn can_use_command(&self, group_id: u32, word: &str, kind: CommandKind) -> bool {
        self.group(group_id).can_use_command(&self.canonical_command(word), kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stock() -> PermissionGroups {
        PermissionGroups::load(concat!(env!("CARGO_MANIFEST_DIR"), "/../config/groups.json")).unwrap()
    }

    #[test]
    fn stock_file_defines_the_rathena_groups_with_resolved_inheritance() {
        let groups = stock();
        let names: Vec<&str> = [0, 1, 2, 3, 4, 5, 10, 99].iter().map(|id| groups.group(*id).name.as_str()).collect();
        assert_eq!(names, ["Player", "Super Player", "Support", "Script Manager", "Event Manager", "VIP", "Law Enforcement", "Admin"]);
        assert_eq!(groups.level(2), 1);
        assert_eq!(groups.level(10), 2);
        assert_eq!(groups.level(99), 99);
        assert!(groups.group(10).log_commands && !groups.group(0).log_commands);
    }

    #[test]
    fn permissions_are_inherited_even_when_a_child_disables_them() {
        let groups = stock();
        assert!(groups.has_permission(0, Permission::Trade) && groups.has_permission(0, Permission::Party));
        assert!(groups.has_permission(4, Permission::Trade), "Event Manager's can_trade: false is overridden by its parents, as in rathena");
        assert!(groups.has_permission(1, Permission::Attendance));
        assert!(!groups.has_permission(0, Permission::AnyWarp));
        assert!(groups.has_permission(3, Permission::AnyWarp));
        assert!(groups.has_permission(10, Permission::HideSession) && !groups.has_permission(2, Permission::HideSession));
        assert!(groups.has_permission(99, Permission::AllCommands) && !groups.has_permission(99, Permission::AllSkill));
    }

    #[test]
    fn commands_come_from_the_group_and_its_parents() {
        let groups = stock();
        assert!(groups.can_use_command(0, "changedress", CommandKind::At));
        assert!(!groups.can_use_command(0, "go", CommandKind::At));
        assert!(groups.can_use_command(1, "go", CommandKind::At), "Super Player");
        assert!(groups.can_use_command(1, "changedress", CommandKind::At), "inherited from Player");
        assert!(groups.can_use_command(2, "jumpto", CommandKind::At));
        assert!(!groups.can_use_command(2, "item", CommandKind::At));
        assert!(groups.can_use_command(4, "item", CommandKind::At));
        assert!(groups.can_use_command(4, "item", CommandKind::Char) && !groups.can_use_command(2, "item", CommandKind::Char));
        assert!(groups.can_use_command(99, "anythingatall", CommandKind::At), "all_commands");
    }

    #[test]
    fn aliases_resolve_to_the_canonical_command() {
        let groups = stock();
        assert_eq!(groups.canonical_command("blvl"), "baselevelup");
        assert_eq!(groups.canonical_command("WARP"), "mapmove");
        assert_eq!(groups.canonical_command("rura"), "mapmove");
        assert_eq!(groups.canonical_command("inspect"), "inspect", "unknown words keep their own name");
        assert!(groups.can_use_command(10, "warp", CommandKind::At), "Law Enforcement may use mapmove, and so its alias");
        assert!(!groups.can_use_command(2, "warp", CommandKind::At));
    }

    #[test]
    fn unknown_group_ids_fall_back_to_the_default_group() {
        let groups = stock();
        assert!(!groups.exists(42));
        assert_eq!(groups.group(42).id, 0);
        assert!(groups.has_permission(42, Permission::Trade));
    }

    #[test]
    fn builtin_groups_match_the_stock_player_group() {
        let builtin = PermissionGroups::builtin();
        assert!(builtin.has_permission(0, Permission::Trade) && builtin.has_permission(0, Permission::Party));
        assert!(builtin.can_use_command(0, "resurrect", CommandKind::At) && !builtin.can_use_command(0, "item", CommandKind::At));
    }

    #[test]
    fn invalid_files_are_rejected() {
        assert!(PermissionGroups::from_json("{}").is_err());
        assert!(PermissionGroups::from_json(r#"{"groups": [{"id": 1, "name": "x"}]}"#).is_err(), "group 0 is mandatory");
        assert!(PermissionGroups::from_json(r#"{"groups": [{"id": 0, "name": "x", "permissions": ["fly"]}]}"#).is_err());
        assert!(PermissionGroups::from_json(r#"{"groups": [{"id": 0, "name": "a"}, {"id": 0, "name": "b"}]}"#).is_err());
        assert!(PermissionGroups::from_json(r#"{"groups": [{"id": 0, "name": "a", "level": 200}]}"#).is_ok());
    }
}
