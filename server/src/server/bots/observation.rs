//! What a bot sees: the whole map instance it stands on, nothing is hidden by a field of view.
use models::enums::cell::CellType;
use models::enums::class::JobName;
use models::enums::{EnumWithMaskValueU16, EnumWithNumberValue};
use serde::Serialize;

use super::interaction::target_kind;
use crate::server::Server;
use crate::server::model::game_systems::PlayerTradePhase;
use crate::server::service::status_service::StatusService;
use crate::server::model::movement::Movable;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::{Character, CharacterAction};
use crate::server::state::server::ServerState;
use crate::util::coordinate;

/// Sprite of the NPCs that only exist to run a script (event timers, touch areas), nobody can click them.
const INVISIBLE_NPC_SPRITE: u16 = 111;

#[derive(Debug, Serialize)]
pub struct StatsView {
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
}

#[derive(Debug, Serialize)]
pub struct SelfView {
    pub name: String,
    pub char_id: u32,
    pub job: String,
    pub base_level: u32,
    pub job_level: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u32,
    pub max_sp: u32,
    pub zeny: u32,
    pub weight: u32,
    pub status_points: u32,
    pub skill_points: u32,
    pub stats: StatsView,
    pub x: u16,
    pub y: u16,
    pub action: &'static str,
    pub moving: bool,
    pub attack_target: Option<u32>,
    pub dead: bool,
}

#[derive(Debug, Serialize)]
pub struct MapView {
    pub name: String,
    pub instance: u8,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Serialize)]
pub struct PlayerView {
    pub id: u32,
    pub name: String,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Serialize)]
pub struct MobView {
    pub id: u32,
    pub name: String,
    pub mob_id: i16,
    pub x: u16,
    pub y: u16,
    pub hp: u32,
    pub max_hp: u32,
}

#[derive(Debug, Serialize)]
pub struct NpcView {
    pub id: u32,
    pub name: String,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Serialize)]
pub struct GroundItemView {
    pub id: u32,
    pub item_id: i32,
    pub name: String,
    pub amount: u16,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Serialize)]
pub struct WarpView {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    /// Cells the warp covers around its centre, walking onto any of them triggers it.
    pub half_width: u16,
    pub half_height: u16,
    pub to_map: String,
    pub to_x: u16,
    pub to_y: u16,
}

#[derive(Debug, Serialize)]
pub struct InventoryView {
    pub index: usize,
    pub item_id: i32,
    pub name: String,
    pub amount: i16,
    pub equipped: bool,
    /// `Healing`, `Weapon`, `Armor`, `Etc`... as the item database names it.
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct GameView {
    /// Whether the character stands on its map, the client side loading is done.
    pub ready: bool,
    #[serde(rename = "self")]
    pub me: SelfView,
    pub map: MapView,
    pub players: Vec<PlayerView>,
    pub mobs: Vec<MobView>,
    pub npcs: Vec<NpcView>,
    pub items: Vec<GroundItemView>,
    pub warps: Vec<WarpView>,
    pub inventory: Vec<InventoryView>,
    pub skills: Vec<SkillView>,
    pub party: Option<PartyView>,
    pub party_invitation: Option<PartyInvitationView>,
    pub trade: Option<TradeView>,
}

#[derive(Debug, Serialize)]
pub struct SkillView {
    pub id: u32,
    pub name: String,
    pub level: u8,
    /// `target`, `ground`, `self` or `passive`: what the skill is aimed at.
    pub aim: &'static str,
    /// Cells, absent when the skill uses the range of the weapon.
    pub range: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PartyMemberView {
    pub char_id: u32,
    /// Empty while the member is offline.
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct PartyView {
    pub id: u32,
    pub name: String,
    pub leader: u32,
    pub members: Vec<PartyMemberView>,
}

#[derive(Debug, Serialize)]
pub struct PartyInvitationView {
    pub party_id: u32,
    pub inviter: u32,
    pub inviter_name: String,
}

#[derive(Debug, Serialize)]
pub struct TradeItemView {
    pub item_id: i32,
    pub name: String,
    pub amount: i16,
}

#[derive(Debug, Serialize)]
pub struct TradeView {
    pub partner: u32,
    pub partner_name: String,
    /// `requested` (waiting for an answer), `accepted` (offers are open), `locked` or `confirmed`.
    pub phase: &'static str,
    /// The request comes from the partner: answer it with `trade_accept` or `trade_decline`.
    pub incoming_request: bool,
    pub my_items: Vec<TradeItemView>,
    pub my_zeny: u32,
    pub partner_items: Vec<TradeItemView>,
    pub partner_zeny: u32,
}

#[derive(Debug, Serialize)]
pub struct MapGrid {
    pub map: String,
    pub width: u16,
    pub height: u16,
    /// `rows[0]` is `y_max`, the last row is `y_min`; `rows[i][j]` is the cell at `x_min + j`. `.` can be walked on, `#` cannot.
    pub x_min: u16,
    pub y_min: u16,
    pub y_max: u16,
    pub rows: Vec<String>,
}

fn action_name(action: &CharacterAction) -> &'static str {
    match action {
        CharacterAction::Idle => "idle",
        CharacterAction::Moving => "moving",
        CharacterAction::Attacking { .. } => "attacking",
        CharacterAction::UsingSkill { .. } => "using_skill",
        CharacterAction::Sitting => "sitting",
        CharacterAction::Dead => "dead",
    }
}

/// Name shown above an NPC: scripts append `#unique id` to tell same-named NPCs apart.
fn display_name(name: &str) -> String {
    name.split('#').next().unwrap_or(name).to_string()
}

fn self_view(server: &Server, character: &Character, tick: u128) -> SelfView {
    let snapshot = server.server_service().get_status_snapshot(&character.status, tick);
    let job = JobName::try_from_value(character.status.job as usize)
        .map(|job| format!("{job:?}"))
        .unwrap_or_else(|_| character.status.job.to_string());
    SelfView {
        name: character.name.clone(),
        char_id: character.char_id,
        job,
        base_level: character.status.base_level,
        job_level: character.status.job_level,
        hp: character.status.hp,
        max_hp: snapshot.max_hp(),
        sp: character.status.sp,
        max_sp: snapshot.max_sp(),
        zeny: character.get_zeny(),
        weight: character.weight(),
        status_points: character.status.status_point,
        skill_points: character.status.skill_point,
        stats: StatsView {
            str: character.status.str,
            agi: character.status.agi,
            vit: character.status.vit,
            int: character.status.int,
            dex: character.status.dex,
            luk: character.status.luk,
        },
        x: character.x(),
        y: character.y(),
        action: action_name(&character.action),
        moving: character.is_moving(),
        attack_target: character.is_attacking().then(|| character.attack().target),
        dead: character.is_dead(),
    }
}

/// The character alone, cheap enough to poll while it walks.
#[derive(Debug, Serialize)]
pub struct StatusView {
    pub ready: bool,
    #[serde(rename = "self")]
    pub me: SelfView,
    pub map: MapView,
}

fn map_view(character: &Character, instance: Option<&crate::server::model::map_instance::MapInstance>) -> MapView {
    MapView {
        name: crate::server::model::map::Map::name_without_ext(character.current_map_name()),
        instance: character.current_map_instance(),
        width: instance.map_or(0, |instance| instance.x_size()),
        height: instance.map_or(0, |instance| instance.y_size()),
    }
}

fn skill_views(character: &Character) -> Vec<SkillView> {
    let config = GlobalConfigService::instance();
    StatusService::instance()
        .to_snapshot(&character.status)
        .known_skills()
        .iter()
        .filter(|skill| skill.level > 0)
        .map(|skill| {
            let id = skill.value.id();
            let skill_config = config.find_skill_config(&script_sdk::Value::Number(id as i32));
            SkillView {
                id,
                name: skill_config.map_or_else(|| format!("{:?}", skill.value), |found| found.name.clone()),
                level: skill.level,
                aim: skill_config.map_or("passive", |found| target_kind(*found.target_type())),
                range: skill_config.and_then(|found| {
                    found
                        .range_per_level()
                        .as_ref()
                        .and_then(|ranges| ranges.get(usize::from(skill.level) - 1).copied())
                        .or(*found.range())
                        .filter(|range| *range > 0)
                }),
            }
        })
        .collect()
}

fn party_view(state: &ServerState, character: &Character) -> Option<PartyView> {
    let party = character.game_systems.party.as_ref()?;
    Some(PartyView {
        id: party.id,
        name: party.name.clone(),
        leader: party.leader_char_id,
        members: party
            .members
            .iter()
            .map(|id| PartyMemberView { char_id: *id, name: state.get_character(*id).map(|member| member.name.clone()).unwrap_or_default() })
            .collect(),
    })
}

fn trade_view(state: &ServerState, character: &Character) -> Option<TradeView> {
    let trade = character.game_systems.trade.as_ref()?;
    let partner = state.get_character(trade.partner_id);
    let items = |items: &[crate::server::model::game_systems::PlayerTradeItem]| {
        items
            .iter()
            .map(|offered| TradeItemView {
                item_id: offered.item.item_id,
                name: GlobalConfigService::instance().find_item(offered.item.item_id).map(|model| model.name_english.clone()).unwrap_or_default(),
                amount: offered.amount,
            })
            .collect()
    };
    let partner_trade = partner.and_then(|partner| partner.game_systems.trade.as_ref());
    Some(TradeView {
        partner: trade.partner_id,
        partner_name: partner.map(|partner| partner.name.clone()).unwrap_or_default(),
        phase: match trade.phase {
            PlayerTradePhase::Requested => "requested",
            PlayerTradePhase::Accepted => "accepted",
            PlayerTradePhase::Locked => "locked",
            PlayerTradePhase::Confirmed => "confirmed",
        },
        incoming_request: trade.phase == PlayerTradePhase::Requested && trade.requested_by != character.char_id,
        my_items: items(&trade.items),
        my_zeny: trade.zeny,
        partner_items: partner_trade.map(|other| items(&other.items)).unwrap_or_default(),
        partner_zeny: partner_trade.map_or(0, |other| other.zeny),
    })
}

pub fn status(server: &Server, state: &ServerState, char_id: u32, tick: u128) -> Result<StatusView, String> {
    let character = state.get_character(char_id).ok_or("Character is not in game")?;
    let instance = state.get_map_instance_from_character(character);
    Ok(StatusView {
        ready: character.loaded_from_client_side && instance.is_some(),
        me: self_view(server, character, tick),
        map: map_view(character, instance.as_deref()),
    })
}

pub fn observe(server: &Server, state: &ServerState, char_id: u32, tick: u128) -> Result<GameView, String> {
    let character = state.get_character(char_id).ok_or("Character is not in game")?;
    let instance = state.get_map_instance_from_character(character);
    let config = GlobalConfigService::instance();
    let mut view = GameView {
        ready: character.loaded_from_client_side && instance.is_some(),
        me: self_view(server, character, tick),
        map: map_view(character, instance.as_deref()),
        players: Vec::new(),
        mobs: Vec::new(),
        npcs: Vec::new(),
        items: Vec::new(),
        warps: Vec::new(),
        inventory: character
            .inventory_iter()
            .map(|(index, item)| InventoryView {
                index,
                item_id: item.item_id,
                name: item.name_english.clone(),
                amount: item.amount,
                equipped: item.equip != 0,
                kind: format!("{:?}", item.item_type()),
            })
            .collect(),
        skills: Vec::new(),
        party: None,
        party_invitation: None,
        trade: None,
    };
    view.skills = skill_views(character);
    view.party = party_view(state, character);
    view.party_invitation = character.game_systems.party_invitation.as_ref().map(|invitation| PartyInvitationView {
        party_id: invitation.party_id,
        inviter: invitation.inviter_id,
        inviter_name: state.get_character(invitation.inviter_id).map(|inviter| inviter.name.clone()).unwrap_or_default(),
    });
    view.trade = trade_view(state, character);
    view.players = state
        .characters()
        .values()
        .filter(|other| other.char_id != char_id && other.map_instance_key == character.map_instance_key)
        .map(|other| PlayerView { id: other.char_id, name: other.name.clone(), x: other.x(), y: other.y() })
        .collect();
    let Some(instance) = instance else {
        return Ok(view);
    };
    let map_state = instance.state();
    view.mobs = map_state
        .mobs()
        .values()
        .filter(|mob| !mob.to_remove)
        .map(|mob| MobView {
            id: mob.id,
            name: mob.name_english.clone(),
            mob_id: mob.mob_id,
            x: mob.x(),
            y: mob.y(),
            hp: mob.status.hp(),
            max_hp: mob.status.max_hp(),
        })
        .collect();
    view.npcs = map_state
        .script_skill_state
        .npcs
        .values()
        .filter(|npc| !npc.hidden && npc.sprite != INVISIBLE_NPC_SPRITE)
        .map(|npc| NpcView { id: npc.id, name: display_name(&npc.script.name), x: npc.x, y: npc.y })
        .collect();
    view.items = map_state
        .dropped_items()
        .values()
        .map(|item| GroundItemView {
            id: item.map_item_id,
            item_id: item.item_id,
            name: config.find_item(item.item_id).map_or_else(|| item.item_id.to_string(), |model| model.name_english.clone()),
            amount: item.amount,
            x: item.x(),
            y: item.y(),
        })
        .collect();
    view.warps = instance
        .map()
        .warps()
        .iter()
        .map(|warp| WarpView {
            id: warp.id,
            x: warp.x,
            y: warp.y,
            half_width: warp.x_size,
            half_height: warp.y_size,
            to_map: warp.dest_map_name.clone(),
            to_x: warp.to_x,
            to_y: warp.to_y,
        })
        .collect();
    Ok(view)
}

pub fn is_walkable(cells: &[u16], width: u16, x: u16, y: u16) -> bool {
    cells.get(coordinate::get_cell_index_of(x, y, width)).is_some_and(|cell| cell & CellType::Walkable.as_flag() == 1)
}

/// The walkable cells of the map of a bot, the whole map or the square of `radius` cells around `center`.
pub fn map_grid(state: &ServerState, char_id: u32, center: Option<(u16, u16, u16)>) -> Result<MapGrid, String> {
    let character = state.get_character(char_id).ok_or("Character is not in game")?;
    let instance = state.get_map_instance_from_character(character).ok_or("The map of the character is not loaded yet")?;
    let (width, height) = (instance.x_size(), instance.y_size());
    let map_state = instance.state();
    let (x_min, x_max, y_min, y_max) = match center {
        Some((x, y, radius)) => (x.saturating_sub(radius), x.saturating_add(radius).min(width - 1), y.saturating_sub(radius), y.saturating_add(radius).min(height - 1)),
        None => (0, width - 1, 0, height - 1),
    };
    let rows = (y_min..=y_max)
        .rev()
        .map(|y| (x_min..=x_max).map(|x| if is_walkable(map_state.cells(), width, x, y) { '.' } else { '#' }).collect())
        .collect();
    Ok(MapGrid {
        map: crate::server::model::map::Map::name_without_ext(character.current_map_name()),
        width,
        height,
        x_min,
        y_min,
        y_max,
        rows,
    })
}
