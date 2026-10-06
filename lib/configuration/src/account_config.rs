use std::net::Ipv4Addr;

use serde::{Deserialize, Deserializer};

/// `MAX_CHARS` of rathena for client versions 20100413 to 20180123.
pub const MAX_CHARS: u8 = 12;
pub const PINCODE_LENGTH: usize = 4;

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct LoginConfig {
    pub new_account: bool,
    pub allowed_regs: u32,
    pub time_allowed: u32,
    pub acc_name_min_length: usize,
    pub password_min_length: usize,
    pub group_id_to_connect: i32,
    pub min_group_id_to_connect: i32,
    pub start_limited_time: i64,
    pub use_md5_passwords: bool,
    pub log_login: bool,
    pub date_format: String,
    pub chars_per_account: u8,
    pub ipban_enable: bool,
    pub ipban_dynamic_pass_failure_ban: bool,
    pub ipban_dynamic_pass_failure_ban_interval: u32,
    pub ipban_dynamic_pass_failure_ban_limit: u32,
    pub ipban_dynamic_pass_failure_ban_duration: u32,
    pub ipban_cleanup_interval: u32,
    pub use_dnsbl: bool,
    pub dnsbl_servers: Vec<String>,
    pub subnets: Vec<SubnetConfig>,
}

impl Default for LoginConfig {
    fn default() -> Self {
        Self {
            new_account: false,
            allowed_regs: 1,
            time_allowed: 10,
            acc_name_min_length: 4,
            password_min_length: 4,
            group_id_to_connect: -1,
            min_group_id_to_connect: -1,
            start_limited_time: -1,
            use_md5_passwords: false,
            log_login: true,
            date_format: "%Y-%m-%d %H:%M:%S".into(),
            chars_per_account: 0,
            ipban_enable: true,
            ipban_dynamic_pass_failure_ban: true,
            ipban_dynamic_pass_failure_ban_interval: 5,
            ipban_dynamic_pass_failure_ban_limit: 7,
            ipban_dynamic_pass_failure_ban_duration: 5,
            ipban_cleanup_interval: 60,
            use_dnsbl: false,
            dnsbl_servers: vec!["bl.blocklist.de".into(), "socks.dnsbl.sorbs.net".into()],
            subnets: Vec::new(),
        }
    }
}

impl LoginConfig {
    /// Slots granted to an account whose record does not set `char_slots`.
    pub fn default_char_slots(&self) -> u8 {
        match self.chars_per_account {
            0 => MAX_CHARS,
            slots => slots.min(MAX_CHARS),
        }
    }
}

/// `conf/subnet_athena.conf`: clients inside `mask` are sent `char_ip` as the char server address and `map_ip` as the map server address.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubnetConfig {
    pub mask: Ipv4Addr,
    pub char_ip: Ipv4Addr,
    pub map_ip: Ipv4Addr,
}

impl SubnetConfig {
    pub fn matches(&self, client: Ipv4Addr) -> bool {
        let mask = u32::from(self.mask);
        u32::from(self.char_ip) & mask == u32::from(client) & mask
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartPoint {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartLocation {
    pub map: String,
    pub point: StartPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartItem {
    pub item_id: u32,
    pub amount: u16,
    /// Equipment location bit mask, `0` when the item starts unequipped.
    pub equip: u32,
}

pub fn parse_start_locations(value: &str) -> Result<Vec<StartLocation>, String> {
    value
        .split(':')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let parts: Vec<&str> = entry.split(',').map(str::trim).collect();
            let [map, x, y] = parts.as_slice() else {
                return Err(format!("start point \"{entry}\" must look like map,x,y"));
            };
            if map.is_empty() {
                return Err(format!("start point \"{entry}\" has no map"));
            }
            Ok(StartLocation {
                map: (*map).to_string(),
                point: StartPoint {
                    x: x.parse().map_err(|_| format!("start point \"{entry}\" has an invalid x"))?,
                    y: y.parse().map_err(|_| format!("start point \"{entry}\" has an invalid y"))?,
                },
            })
        })
        .collect()
}

pub fn parse_start_items(value: &str) -> Result<Vec<StartItem>, String> {
    value
        .split(':')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let parts: Vec<&str> = entry.split(',').map(str::trim).collect();
            let [item_id, amount, equip] = parts.as_slice() else {
                return Err(format!("start item \"{entry}\" must look like item_id,amount,equip"));
            };
            Ok(StartItem {
                item_id: item_id.parse().map_err(|_| format!("start item \"{entry}\" has an invalid item id"))?,
                amount: amount.parse().map_err(|_| format!("start item \"{entry}\" has an invalid amount"))?,
                equip: equip.parse().map_err(|_| format!("start item \"{entry}\" has an invalid equip position"))?,
            })
        })
        .collect()
}

fn deserialize_start_locations<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<StartLocation>, D::Error> {
    let value = String::deserialize(deserializer)?;
    let locations = parse_start_locations(&value).map_err(serde::de::Error::custom)?;
    if locations.is_empty() {
        return Err(serde::de::Error::custom("start_point needs at least one location"));
    }
    Ok(locations)
}

fn deserialize_start_items<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<StartItem>, D::Error> {
    let value = String::deserialize(deserializer)?;
    parse_start_items(&value).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct PincodeConfig {
    pub enabled: bool,
    /// Days after which a PIN has to be changed, `0` disables the expiry.
    pub changetime: u32,
    pub maxtry: u32,
    pub force: bool,
    pub allow_repeated: bool,
    pub allow_sequential: bool,
}

impl Default for PincodeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            changetime: 0,
            maxtry: 3,
            force: true,
            allow_repeated: false,
            allow_sequential: false,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct CharServerConfig {
    pub server_name: String,
    pub wisp_server_name: String,
    pub char_maintenance: u16,
    pub char_new: bool,
    pub char_new_display: u16,
    pub max_connect_user: i32,
    pub gm_allow_group: i32,
    #[serde(deserialize_with = "deserialize_start_locations")]
    pub start_point: Vec<StartLocation>,
    #[serde(deserialize_with = "deserialize_start_items")]
    pub start_items: Vec<StartItem>,
    pub start_zeny: u32,
    pub start_status_points: u16,
    pub log_char: bool,
    pub char_name_min_length: usize,
    pub name_ignoring_case: bool,
    /// `0`: any name, `1`: only `char_name_letters`, `2`: anything but `char_name_letters`.
    pub char_name_option: u8,
    pub char_name_letters: String,
    pub char_del_level: i32,
    pub char_del_delay: u32,
    /// Bit mask: `1` e-mail, `2` birthdate.
    pub char_del_option: u8,
    /// Bit mask: `1` guild, `2` party.
    pub char_del_restriction: u8,
    pub pincode: PincodeConfig,
    pub char_move_enabled: bool,
    pub char_movetoused: bool,
    pub char_moves_unlimited: bool,
    pub char_rename_party: bool,
    pub char_rename_guild: bool,
    pub default_map: String,
    pub default_map_x: u16,
    pub default_map_y: u16,
    pub fame_list_alchemist: u8,
    pub fame_list_blacksmith: u8,
    pub fame_list_taekwon: u8,
    pub guild_exp_rate: u32,
}

impl Default for CharServerConfig {
    fn default() -> Self {
        Self {
            server_name: "Rust ragnarok".into(),
            wisp_server_name: "Server".into(),
            char_maintenance: 0,
            char_new: true,
            char_new_display: 0,
            max_connect_user: -1,
            gm_allow_group: 99,
            start_point: parse_start_locations("new_1-1,53,111").expect("default start point"),
            start_items: parse_start_items("1201,1,2:2301,1,16").expect("default start items"),
            start_zeny: 0,
            start_status_points: 48,
            log_char: true,
            char_name_min_length: 4,
            name_ignoring_case: false,
            char_name_option: 1,
            char_name_letters: "abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890".into(),
            char_del_level: 0,
            char_del_delay: 86400,
            char_del_option: 2,
            char_del_restriction: 3,
            pincode: PincodeConfig::default(),
            char_move_enabled: true,
            char_movetoused: true,
            char_moves_unlimited: false,
            char_rename_party: false,
            char_rename_guild: false,
            default_map: "prontera".into(),
            default_map_x: 156,
            default_map_y: 191,
            fame_list_alchemist: 10,
            fame_list_blacksmith: 10,
            fame_list_taekwon: 10,
            guild_exp_rate: 100,
        }
    }
}

pub const CHAR_DEL_EMAIL: u8 = 1;
pub const CHAR_DEL_BIRTHDATE: u8 = 2;
pub const CHAR_DEL_RESTRICT_GUILD: u8 = 1;
pub const CHAR_DEL_RESTRICT_PARTY: u8 = 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_rathena_pre_renewal_configuration() {
        let login = LoginConfig::default();
        assert!(!login.new_account);
        assert_eq!((login.acc_name_min_length, login.password_min_length), (4, 4));
        assert_eq!(login.default_char_slots(), 12);
        let char_server = CharServerConfig::default();
        assert_eq!(char_server.start_status_points, 48);
        assert_eq!(char_server.start_items, vec![
            StartItem { item_id: 1201, amount: 1, equip: 2 },
            StartItem { item_id: 2301, amount: 1, equip: 16 },
        ]);
        assert_eq!((char_server.char_del_delay, char_server.char_del_option, char_server.char_del_restriction), (86400, 2, 3));
    }

    #[test]
    fn parses_colon_separated_start_points_and_items() {
        let points = parse_start_locations("new_1-1,53,111:iz_int,18,26").unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[1], StartLocation { map: "iz_int".into(), point: StartPoint { x: 18, y: 26 } });
        assert!(parse_start_locations("prontera,156").is_err());
        assert!(parse_start_locations("prontera,a,1").is_err());
        assert!(parse_start_items("1201,1").is_err());
        assert_eq!(parse_start_items("").unwrap(), vec![]);
    }

    #[test]
    fn partial_json_keeps_defaults_and_validates_start_values() {
        let config: CharServerConfig =
            serde_json::from_str(r#"{"start_zeny": 500, "start_point": "prontera,156,191", "pincode": {"enabled": true}}"#).unwrap();
        assert_eq!(config.start_zeny, 500);
        assert_eq!(config.start_point[0].map, "prontera");
        assert!(config.pincode.enabled);
        assert_eq!(config.pincode.maxtry, 3);
        assert!(serde_json::from_str::<CharServerConfig>(r#"{"start_point": ""}"#).is_err());
        assert!(serde_json::from_str::<CharServerConfig>(r#"{"start_items": "x,1,2"}"#).is_err());
    }

    #[test]
    fn subnet_matches_clients_inside_the_mask() {
        let subnet = SubnetConfig {
            mask: "255.255.255.0".parse().unwrap(),
            char_ip: "192.168.1.10".parse().unwrap(),
            map_ip: "192.168.1.10".parse().unwrap(),
        };
        assert!(subnet.matches("192.168.1.77".parse().unwrap()));
        assert!(!subnet.matches("192.168.2.77".parse().unwrap()));
    }
}
