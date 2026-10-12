//! What a bot sees: the whole map instance it stands on, nothing is hidden by a field of view.
use models::enums::cell::CellType;
use models::enums::class::JobName;
use models::enums::item::EquipmentLocation;
use models::enums::mob::{MobClass, MobMode};
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithNumberValue};
use serde::Serialize;

use super::commerce::{CommerceView, commerce_view};
use super::companions::{CompanionsView, companions_view};
use super::interaction::target_kind;
use super::menu::{CraftingView, SkillMenuView, crafting_view, skill_menu_view};
use crate::server::Server;
use crate::server::model::game_systems::PlayerTradePhase;
use crate::server::model::map_regions::{self, MapRegions};
use crate::server::service::status_service::StatusService;
use crate::server::model::movement::Movable;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::{Character, CharacterAction};
use crate::server::state::mob::{Mob, MobAction};
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
    /// The walkable region of the map the bot stands in, see `/db/maps`: a warp of another region can not be walked to.
    pub region: Option<u32>,
    /// The most the bot can carry, in the units of `weight`. Above half of it HP and SP stop regenerating, above 90% it can
    /// not attack.
    pub max_weight: u32,
    pub base_exp: u32,
    /// Experience the next base level needs, 0 at the highest level.
    pub next_base_exp: u32,
    pub job_exp: u32,
    pub next_job_exp: u32,
    /// The figures of the status window, equipment and statuses included.
    pub combat: CombatView,
    /// Status effects on the bot (`Poison`, `Blessing`...), with the milliseconds they have left.
    pub statuses: Vec<ActiveStatusView>,
}

#[derive(Debug, Serialize)]
pub struct CombatView {
    /// The two parts of ATK the status window shows (`atk` + `atk2`): from stats, and from the weapon with its refine.
    pub atk: i32,
    pub atk2: i32,
    pub matk_min: u16,
    pub matk_max: u16,
    /// Hard defense, a percentage of the damage taken off.
    pub def: i16,
    pub mdef: i16,
    pub hit: i16,
    pub flee: i16,
    pub perfect_dodge: f64,
    pub critical: f64,
    pub aspd: f64,
    /// Milliseconds per cell walked.
    pub speed: u16,
    pub element: String,
}

#[derive(Debug, Serialize)]
pub struct ActiveStatusView {
    pub name: String,
    /// `null` for a status that lasts until something ends it.
    pub remaining_ms: Option<u64>,
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
    /// Whether `attack` can hit the player here: in a duel, or on a map where players fight.
    pub attackable: bool,
    /// What its sprite shows: the job, lying dead, and what it is hitting.
    pub job: String,
    pub dead: bool,
    pub attack_target: Option<u32>,
    /// A member of the bot's party (its name shows in the party colour).
    pub party: bool,
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
    pub level: u32,
    /// Attacks characters it sees, without being attacked first.
    pub aggressive: bool,
    /// A boss or an MVP: immune to many statuses, and usually far stronger than its level says.
    pub boss: bool,
    /// `idle`, `moving`, `chasing`, `attacking`, `flinching` or `returning`.
    pub action: &'static str,
    /// The character or monster it chases or attacks, a bot's own id when it is after the bot.
    pub target: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct NpcView {
    pub id: u32,
    pub name: String,
    pub x: u16,
    pub y: u16,
    /// The walkable region it is talked to from.
    pub region: Option<u32>,
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
    /// The walkable region the warp is used from, compare with `self.region`.
    pub region: Option<u32>,
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
    /// Weight of one, in the units of `self.weight`.
    pub weight: i32,
    /// What an NPC shop pays for one, before the Overcharge skill.
    pub sell_price: i32,
    /// For equipment: where it goes and whether the bot can wear it.
    pub equip: Option<EquipView>,
}

#[derive(Debug, Serialize)]
pub struct EquipView {
    /// `HeadTop`, `HeadMid`, `HeadLow`, `Armor`, `HandRight` (weapon), `HandLeft` (shield), `Garment`, `Shoes`,
    /// `AccessoryLeft`, `AccessoryRight`, `Ammo`: an item listing both hands takes both.
    pub slots: Vec<&'static str>,
    pub attack: Option<i16>,
    pub defense: Option<i16>,
    pub weapon_level: Option<i16>,
    pub equip_level: Option<i16>,
    pub refine: i16,
    pub identified: bool,
    /// Card item ids in its slots.
    pub cards: Vec<i16>,
    /// Whether `equip` would take it now; `why_not` says what is missing (`level`, `job` or `gender`).
    pub can_equip: bool,
    pub why_not: Option<&'static str>,
}

/// An item of the storage the bot has open, `index` is what withdrawing takes.
#[derive(Debug, Serialize)]
pub struct StorageItemView {
    pub index: usize,
    pub item_id: i32,
    pub name: String,
    pub amount: i16,
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
    /// The items of the storage while the bot has one open (a Kafra opened it), `null` otherwise.
    pub storage: Option<Vec<StorageItemView>>,
    pub skills: Vec<SkillView>,
    pub party: Option<PartyView>,
    pub party_invitation: Option<PartyInvitationView>,
    pub trade: Option<TradeView>,
    /// A skill waiting for a choice (Teleport, Warp Portal, Arrow Crafting...): answer with `skill_choose`.
    pub skill_menu: Option<SkillMenuView>,
    /// The window of Pharmacy or a forging skill: `craft` makes one of its items.
    pub crafting: Option<CraftingView>,
    /// The duel the bot is in.
    pub duel: Option<u32>,
    /// A duel invitation waits for `duel_accept` or `duel_decline`.
    pub duel_invitation: bool,
    pub companions: CompanionsView,
    #[serde(flatten)]
    pub commerce: CommerceView,
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
    pub exp_share: bool,
    pub item_pickup: bool,
    pub item_share: bool,
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
    /// The partner's phase, as the trade window shows its lock and confirm marks.
    pub partner_phase: &'static str,
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

/// Equipment slots of the item database, in the order the status window shows them.
const EQUIPMENT_SLOTS: [(EquipmentLocation, &str); 11] = [
    (EquipmentLocation::HeadTop, "HeadTop"),
    (EquipmentLocation::HeadMid, "HeadMid"),
    (EquipmentLocation::HeadLow, "HeadLow"),
    (EquipmentLocation::Armor, "Armor"),
    (EquipmentLocation::HandRight, "HandRight"),
    (EquipmentLocation::HandLeft, "HandLeft"),
    (EquipmentLocation::Garment, "Garment"),
    (EquipmentLocation::Shoes, "Shoes"),
    (EquipmentLocation::AccessoryLeft, "AccessoryLeft"),
    (EquipmentLocation::AccessoryRight, "AccessoryRight"),
    (EquipmentLocation::Ammo, "Ammo"),
];

/// Rounded in `f64`, which prints `135.2` where an `f32` prints `135.1999969482422`.
fn tenths(value: f32) -> f64 {
    (f64::from(value) * 10.0).round() / 10.0
}

fn mob_view(mob: &Mob) -> MobView {
    let (action, target) = match mob.action {
        MobAction::Idle => ("idle", None),
        MobAction::Moving => ("moving", None),
        MobAction::Chasing { target_id } => ("chasing", Some(target_id)),
        MobAction::Attacking { target_id, .. } => ("attacking", Some(target_id)),
        MobAction::Flinching { .. } => ("flinching", None),
        MobAction::Returning => ("returning", None),
    };
    MobView {
        id: mob.id,
        name: mob.name_english.clone(),
        mob_id: mob.mob_id,
        x: mob.x(),
        y: mob.y(),
        hp: mob.status.hp(),
        max_hp: mob.status.max_hp(),
        level: mob.status.base_level(),
        aggressive: mob.mode & MobMode::Aggressive.as_flag() != 0,
        boss: mob.mode & MobMode::Boss.as_flag() != 0 || *mob.status.mob_class() == MobClass::Boss,
        action,
        target: target.or(mob.target_id),
    }
}

fn inventory_view(server: &Server, character: &Character, index: usize, item: &crate::repository::model::item_model::InventoryItemModel) -> InventoryView {
    let model = GlobalConfigService::instance().find_item(item.item_id);
    let equip = model.filter(|model| model.location != 0).map(|model| {
        let inventory = server.inventory_service();
        let why_not = if !inventory.check_base_level_requirement(character, model) {
            Some("level")
        } else if !inventory.check_job_requirement(character, model) {
            Some("job")
        } else if !inventory.check_gender_requirement(character, model) {
            Some("gender")
        } else {
            None
        };
        EquipView {
            slots: EQUIPMENT_SLOTS
                .iter()
                .filter(|(location, _)| model.location & u64::from(location.as_flag()) != 0)
                .map(|(_, name)| *name)
                .collect(),
            attack: model.attack,
            defense: model.defense,
            weapon_level: model.weapon_level,
            equip_level: model.equip_level_min,
            refine: item.refine,
            identified: item.is_identified,
            cards: [item.card0, item.card1, item.card2, item.card3].into_iter().filter(|card| *card > 0).collect(),
            can_equip: why_not.is_none(),
            why_not,
        }
    });
    InventoryView {
        index,
        item_id: item.item_id,
        name: item.name_english.clone(),
        amount: item.amount,
        equipped: item.equip != 0,
        kind: format!("{:?}", item.item_type()),
        weight: item.weight,
        sell_price: model.map_or(0, |model| model.price_sell.unwrap_or(model.price_buy.unwrap_or(0) / 2)),
        equip,
    }
}

fn job_name(job: u32) -> String {
    JobName::try_from_value(job as usize).map(|job| format!("{job:?}")).unwrap_or_else(|_| job.to_string())
}

fn self_view(server: &Server, character: &Character, tick: u128) -> SelfView {
    let snapshot = server.server_service().get_status_snapshot(&character.status, tick);
    let character_service = server.character_service();
    let job = job_name(character.status.job);
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
        region: map_regions::of_map(character.current_map_name()).and_then(|regions| regions.at(character.x(), character.y())),
        max_weight: character_service.max_weight(character),
        base_exp: character.status.base_exp,
        next_base_exp: character_service.next_base_level_required_exp(&character.status),
        job_exp: character.status.job_exp,
        next_job_exp: character_service.next_job_level_required_exp(&character.status),
        combat: CombatView {
            atk: snapshot.atk_left_side(),
            atk2: snapshot.atk_right_side(),
            matk_min: snapshot.matk_min(),
            matk_max: snapshot.matk_max(),
            def: snapshot.def(),
            mdef: snapshot.mdef(),
            hit: snapshot.hit(),
            flee: snapshot.flee(),
            perfect_dodge: tenths(snapshot.perfect_dodge()),
            critical: tenths(snapshot.crit()),
            aspd: tenths(snapshot.aspd()),
            speed: snapshot.speed(),
            element: format!("{:?}", snapshot.element()),
        },
        statuses: character
            .status
            .active_statuses
            .iter()
            .filter(|change| !change.expired(tick))
            .map(|change| ActiveStatusView {
                name: format!("{:?}", change.kind),
                remaining_ms: change.expires_at.map(|end| end.saturating_sub(tick) as u64),
            })
            .collect(),
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
        exp_share: party.exp_share,
        item_pickup: party.item_pickup,
        item_share: party.item_share,
        members: party
            .members
            .iter()
            .map(|id| PartyMemberView { char_id: *id, name: state.get_character(*id).map(|member| member.name.clone()).unwrap_or_default() })
            .collect(),
    })
}

fn phase_name(phase: PlayerTradePhase) -> &'static str {
    match phase {
        PlayerTradePhase::Requested => "requested",
        PlayerTradePhase::Accepted => "accepted",
        PlayerTradePhase::Locked => "locked",
        PlayerTradePhase::Confirmed => "confirmed",
    }
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
        phase: phase_name(trade.phase),
        incoming_request: trade.phase == PlayerTradePhase::Requested && trade.requested_by != character.char_id,
        partner_phase: partner_trade.map_or("requested", |other| phase_name(other.phase)),
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

fn storage_view(character: &Character) -> Option<Vec<StorageItemView>> {
    let systems = &character.game_systems;
    let records = if systems.guild_storage_open.is_some() {
        &systems.guild_storage_items
    } else if systems.storage_open {
        &systems.storage_items
    } else {
        return None;
    };
    Some(
        records
            .iter()
            .enumerate()
            .map(|(index, record)| StorageItemView {
                index,
                item_id: record.item_id,
                name: GlobalConfigService::item_name_if_loaded(record.item_id).unwrap_or_default(),
                amount: record.amount,
            })
            .collect(),
    )
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
        inventory: character.inventory_iter().map(|(index, item)| inventory_view(server, character, index, item)).collect(),
        storage: storage_view(character),
        skills: Vec::new(),
        party: None,
        party_invitation: None,
        trade: None,
        skill_menu: skill_menu_view(server, state, character, tick),
        crafting: crafting_view(character),
        duel: server.duels().duel_of(char_id),
        duel_invitation: server.duels().invitation_of(char_id).is_some(),
        companions: companions_view(character, tick),
        commerce: commerce_view(state, character),
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
        .map(|other| PlayerView {
            id: other.char_id,
            name: other.name.clone(),
            x: other.x(),
            y: other.y(),
            attackable: server.player_combat_target_allowed(state, character, other.char_id),
            job: job_name(other.status.job),
            dead: other.is_dead(),
            attack_target: other.is_attacking().then(|| other.attack().target),
            party: character.game_systems.party_id != 0 && other.game_systems.party_id == character.game_systems.party_id,
        })
        .collect();
    let Some(instance) = instance else {
        return Ok(view);
    };
    let regions: Option<std::sync::Arc<MapRegions>> = map_regions::of_map(character.current_map_name());
    let map_state = instance.state();
    view.mobs = map_state.mobs().values().filter(|mob| !mob.to_remove).map(mob_view).collect();
    view.npcs = map_state
        .script_skill_state
        .npcs
        .values()
        .filter(|npc| !npc.hidden && npc.sprite != INVISIBLE_NPC_SPRITE)
        .map(|npc| NpcView {
            id: npc.id,
            name: display_name(&npc.script.name),
            x: npc.x,
            y: npc.y,
            region: regions.as_ref().and_then(|regions| regions.landing(npc.x, npc.y)),
        })
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
            region: regions.as_ref().and_then(|regions| regions.of_area(warp.x, warp.y, warp.x_size, warp.y_size)),
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
