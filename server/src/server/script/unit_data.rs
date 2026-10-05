use std::fmt;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use models::enums::EnumWithNumberValue;
use models::enums::element::Element as DefenseElement;
use models::enums::look::LookType;
use models::enums::mob::MobRace;
use script_sdk::{Function, Reply, Request, Value};

use super::ScriptRequest;
use super::skill::actor::{NpcSkillState, ScriptSkillActor};
use crate::server::Server;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::MapItemType;
use crate::server::model::session::Session;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

#[derive(Clone)]
pub struct UnitDataContext {
    pub request: ScriptRequest,
    pub session: Option<Arc<Session>>,
}

impl fmt::Debug for UnitDataContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("UnitDataContext").field("request", &self.request).finish()
    }
}

impl PartialEq for UnitDataContext {
    fn eq(&self, other: &Self) -> bool {
        self.request == other.request
            && match (&self.session, &other.session) {
                (Some(first), Some(second)) => Arc::ptr_eq(first, second),
                (None, None) => true,
                _ => false,
            }
    }
}

impl UnitDataContext {
    pub fn active(&self) -> bool {
        self.session.as_ref().map_or(
            self.request.background && self.request.char_id == 0 && self.request.account_id == 0,
            |session| {
                session.account_id == self.request.account_id
                    && session.char_id == Some(self.request.char_id)
                    && (self.request.background || session.script_generation.load(Ordering::Acquire) == self.request.generation)
            },
        ) && self
            .request
            .response
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(|response| !response.is_closed())
    }

    pub fn reply(&self, reply: Reply) {
        if let Some(response) = self.request.response.lock().unwrap().take() {
            let _ = response.send(reply);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapUnitDataRequest {
    pub context: UnitDataContext,
    pub actor_id: u32,
    pub actor_scope_instance: u8,
    pub map_id: i32,
    pub operation: UnitDataOperation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitDataOperation {
    Read,
    Write {
        field: NpcUnitDataField,
        value: Value,
        destination: Option<MapInstanceKey>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptNpcTransfer {
    pub context: UnitDataContext,
    pub npc: NpcSkillState,
    pub original_position: (u16, u16),
    pub origin: MapInstanceKey,
    pub destination: MapInstanceKey,
    pub error: Option<String>,
    pub phase: NpcTransferPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcTransferPhase {
    Install,
    Rollback,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum NpcUnitDataField {
    Level,
    Hp,
    MaxHp,
    Map,
    X,
    Y,
    Direction,
    Str,
    Agi,
    Vit,
    Int,
    Dex,
    Luk,
    AllStats,
    DamageImmune,
    AttackRange,
    AttackMin,
    AttackMax,
    MatkMin,
    MatkMax,
    Def,
    Mdef,
    Hit,
    Flee,
    PerfectDodge,
    Critical,
    Race,
    Element,
    ElementLevel,
    AttackMotion,
    AttackDelay,
    DamageMotion,
    Sex,
    Class,
    Hair,
    HairColor,
    HeadBottom,
    HeadMiddle,
    HeadTop,
    ClothesColor,
    Shield,
    Weapon,
    Robe,
    Body,
    DeadSit,
    GroupId,
}

const NPC_FIELDS: [(&str, NpcUnitDataField); 46] = [
    ("UNPC_LEVEL", NpcUnitDataField::Level),
    ("UNPC_HP", NpcUnitDataField::Hp),
    ("UNPC_MAXHP", NpcUnitDataField::MaxHp),
    ("UNPC_MAPID", NpcUnitDataField::Map),
    ("UNPC_X", NpcUnitDataField::X),
    ("UNPC_Y", NpcUnitDataField::Y),
    ("UNPC_LOOKDIR", NpcUnitDataField::Direction),
    ("UNPC_STR", NpcUnitDataField::Str),
    ("UNPC_AGI", NpcUnitDataField::Agi),
    ("UNPC_VIT", NpcUnitDataField::Vit),
    ("UNPC_INT", NpcUnitDataField::Int),
    ("UNPC_DEX", NpcUnitDataField::Dex),
    ("UNPC_LUK", NpcUnitDataField::Luk),
    ("UNPC_PLUSALLSTAT", NpcUnitDataField::AllStats),
    ("UNPC_DMGIMMUNE", NpcUnitDataField::DamageImmune),
    ("UNPC_ATKRANGE", NpcUnitDataField::AttackRange),
    ("UNPC_ATKMIN", NpcUnitDataField::AttackMin),
    ("UNPC_ATKMAX", NpcUnitDataField::AttackMax),
    ("UNPC_MATKMIN", NpcUnitDataField::MatkMin),
    ("UNPC_MATKMAX", NpcUnitDataField::MatkMax),
    ("UNPC_DEF", NpcUnitDataField::Def),
    ("UNPC_MDEF", NpcUnitDataField::Mdef),
    ("UNPC_HIT", NpcUnitDataField::Hit),
    ("UNPC_FLEE", NpcUnitDataField::Flee),
    ("UNPC_PDODGE", NpcUnitDataField::PerfectDodge),
    ("UNPC_CRIT", NpcUnitDataField::Critical),
    ("UNPC_RACE", NpcUnitDataField::Race),
    ("UNPC_ELETYPE", NpcUnitDataField::Element),
    ("UNPC_ELELEVEL", NpcUnitDataField::ElementLevel),
    ("UNPC_AMOTION", NpcUnitDataField::AttackMotion),
    ("UNPC_ADELAY", NpcUnitDataField::AttackDelay),
    ("UNPC_DMOTION", NpcUnitDataField::DamageMotion),
    ("UNPC_SEX", NpcUnitDataField::Sex),
    ("UNPC_CLASS", NpcUnitDataField::Class),
    ("UNPC_HAIRSTYLE", NpcUnitDataField::Hair),
    ("UNPC_HAIRCOLOR", NpcUnitDataField::HairColor),
    ("UNPC_HEADBOTTOM", NpcUnitDataField::HeadBottom),
    ("UNPC_HEADMIDDLE", NpcUnitDataField::HeadMiddle),
    ("UNPC_HEADTOP", NpcUnitDataField::HeadTop),
    ("UNPC_CLOTHCOLOR", NpcUnitDataField::ClothesColor),
    ("UNPC_SHIELD", NpcUnitDataField::Shield),
    ("UNPC_WEAPON", NpcUnitDataField::Weapon),
    ("UNPC_ROBE", NpcUnitDataField::Robe),
    ("UNPC_BODY2", NpcUnitDataField::Body),
    ("UNPC_DEADSIT", NpcUnitDataField::DeadSit),
    ("UNPC_GROUP_ID", NpcUnitDataField::GroupId),
];

pub fn constant(name: &str) -> Option<Value> {
    NPC_FIELDS
        .iter()
        .position(|(field, _)| *field == name)
        .map(|index| Value::Number(index as i32))
}

impl NpcUnitDataField {
    fn from_index(index: i32) -> Result<Self, String> {
        usize::try_from(index)
            .ok()
            .and_then(|index| NPC_FIELDS.get(index))
            .map(|(_, field)| *field)
            .ok_or_else(|| "Unknown NPC unit-data field".into())
    }

    pub fn look(self) -> Option<LookType> {
        Some(match self {
            Self::Hair => LookType::Hair,
            Self::HairColor => LookType::HairColor,
            Self::HeadBottom => LookType::HeadBottom,
            Self::HeadMiddle => LookType::HeadMid,
            Self::HeadTop => LookType::HeadTop,
            Self::ClothesColor => LookType::ClothesColor,
            Self::Shield => LookType::Shield,
            Self::Weapon => LookType::Weapon,
            Self::Robe => LookType::Robe,
            Self::Body => LookType::Body,
            _ => return None,
        })
    }
}

fn map_names(state: &ServerState) -> Vec<String> {
    let mut configured = GlobalConfigService::instance().maps().keys().cloned().collect::<Vec<_>>();
    configured.sort_unstable();
    let mut extra = state
        .map_instances()
        .keys()
        .filter(|name| !configured.contains(name))
        .cloned()
        .collect::<Vec<_>>();
    extra.sort_unstable();
    configured.extend(extra);
    configured
}

pub(crate) fn script_actor(state: &ServerState, request: &ScriptRequest) -> Result<Option<ScriptSkillActor>, String> {
    let mut actors = state.map_instances().values().flatten().filter_map(|instance| {
        let map = instance.state();
        map.script_skill_state
            .npcs
            .get(&request.npc_id)
            .filter(|npc| npc.script.scope_instance == request.npc_scope_instance && npc.script.entry_id == request.npc_entry)
            .map(|npc| npc.actor(instance.key().map_name().clone(), instance.id()))
    });
    let actor = actors.next();
    if actors.next().is_some() {
        return Err("NPC script scope is ambiguous".into());
    }
    Ok(actor)
}

pub(crate) fn request_actor(
    server: &Server,
    state: &ServerState,
    request: &ScriptRequest,
    actor_id: u32,
) -> Result<Option<ScriptSkillActor>, String> {
    let actor_id = state
        .characters()
        .values()
        .find(|character| character.account_id == actor_id)
        .map_or(actor_id, |character| character.char_id);
    let caller = script_actor(state, request)?;
    if actor_id == request.npc_id {
        return Ok(caller);
    }
    let (map, instance) = if let Some(npc) = caller.as_ref() {
        (npc.map.as_str(), npc.instance)
    } else {
        let character = state.get_character(request.char_id).ok_or("Script player disconnected")?;
        (character.current_map_name().as_str(), request.map_instance)
    };
    let service = server.script_skill_service();
    match service.find_script_skill_actor_in(state, actor_id, Some(map), Some(instance))? {
        Some(actor) => Ok(Some(actor)),
        None => service.find_script_skill_actor_in(state, actor_id, None, None),
    }
}

pub fn handle_request(server: &Server, state: &mut ServerState, request: &ScriptRequest) -> bool {
    let Request::Call {
        function: Function::GetUnitData | Function::SetUnitData,
        arguments,
    } = &request.request
    else {
        return false;
    };
    let result = (|| -> Result<(), String> {
        let session = if request.logout_token.is_some() {
            if !server.valid_logout_request(state, request) { return Err("Logout callback is no longer active".into()); }
            state.pending_character_logouts.get(&request.char_id).map(|pending| pending.owner.session.clone())
        } else {
            if state.pending_character_logouts.contains_key(&request.char_id) { return Err("Character is leaving the game".into()); }
            state.find_session(request.account_id)
        };
        let background = request.background && request.char_id == 0 && request.account_id == 0;
        let context = UnitDataContext {
            request: request.clone(),
            session,
        };
        if !context.active()
            || if background {
                script_actor(state, request)?.is_none()
            } else {
                !state.characters().contains_key(&request.char_id)
            }
        {
            return Err("Conversation is no longer active".into());
        }
        let actor_id = arguments.first().ok_or("Unit-data actor ID is required")?.number_value()? as u32;
        let actor = request_actor(server, state, request, actor_id)?;
        let Some(actor) = actor else {
            context.reply(Ok((-1).into()));
            return Ok(());
        };
        if actor.object_type != MapItemType::Npc {
            return Err("Unit-data operations for this actor type are not implemented".into());
        }
        let names = map_names(state);
        let map_id = names
            .iter()
            .position(|name| name == actor.map.trim_end_matches(".gat"))
            .ok_or("Unit-data map has no ID")? as i32;
        let operation = if matches!(request.request, Request::Call {
            function: Function::GetUnitData,
            ..
        }) {
            if arguments.len() != 1 {
                return Err("GetUnitData expects one actor ID".into());
            }
            UnitDataOperation::Read
        } else {
            if arguments.len() != 3 {
                return Err("SetUnitData expects actor ID, field, and value".into());
            }
            let field = NpcUnitDataField::from_index(arguments[1].number_value()?)?;
            let value = arguments[2].clone();
            let destination = if field == NpcUnitDataField::Map {
                let name = match &value {
                    Value::String(name) => name.trim_end_matches(".gat").to_owned(),
                    Value::Number(index) => usize::try_from(*index)
                        .ok()
                        .and_then(|index| names.get(index))
                        .cloned()
                        .ok_or("Invalid destination map ID")?,
                    _ => return Err("NPC destination map must be a name or map ID".into()),
                };
                let instance_id = if state.get_map_instance(&name, actor.instance).is_some() {
                    actor.instance
                } else {
                    0
                };
                let destination = if let Some(instance) = state.get_map_instance(&name, instance_id) {
                    instance
                } else {
                    let map = GlobalConfigService::instance()
                        .find_map(&name)
                        .ok_or("NPC destination map is unavailable")?;
                    server.server_service().create_map_instance(state, map, instance_id)
                };
                if !destination.is_alive() {
                    return Err("NPC destination map is stopped".into());
                }
                Some(destination.key().clone())
            } else {
                None
            };
            UnitDataOperation::Write { field, value, destination }
        };
        let instance = state
            .get_map_instance(&actor.map, actor.instance)
            .filter(|instance| instance.is_alive())
            .ok_or("NPC map is unavailable")?;
        let actor_scope_instance = instance
            .state()
            .script_skill_state
            .npcs
            .get(&actor_id)
            .map(|npc| npc.script.scope_instance);
        let Some(actor_scope_instance) = actor_scope_instance else {
            context.reply(Ok((-1).into()));
            return Ok(());
        };
        instance.add_to_next_tick(MapEvent::UnitData(MapUnitDataRequest {
            context,
            actor_id,
            actor_scope_instance,
            map_id,
            operation,
        }));
        Ok(())
    })();
    if let Err(error) = result {
        if let Some(response) = request.response.lock().unwrap().take() {
            let _ = response.send(Err(error));
        }
    }
    true
}

pub fn route_transfer(server: &Server, state: &ServerState, mut transfer: ScriptNpcTransfer) {
    if transfer.phase == NpcTransferPhase::Complete {
        if let Some(origin) = state.get_map_instance(transfer.origin.map_name(), transfer.origin.map_instance()) {
            origin.add_to_next_tick(MapEvent::ReleaseScriptNpc(transfer.npc.id));
        }
        return;
    }
    if transfer.phase == NpcTransferPhase::Install && !transfer.context.active() {
        transfer.error = Some("Conversation is no longer active".into());
        transfer.phase = NpcTransferPhase::Rollback;
        transfer.destination = transfer.origin.clone();
    }
    if let Some(instance) = state
        .get_map_instance(transfer.destination.map_name(), transfer.destination.map_instance())
        .filter(|instance| instance.is_alive())
    {
        instance.add_to_next_tick(MapEvent::InstallScriptNpc(transfer));
    } else if transfer.error.is_none() {
        transfer.error = Some("NPC destination map stopped during transfer".into());
        transfer.phase = NpcTransferPhase::Rollback;
        transfer.destination = transfer.origin.clone();
        route_transfer(server, state, transfer);
    } else {
        transfer
            .context
            .reply(Err("NPC origin and destination maps are unavailable".into()));
        error!(
            "NPC {} cannot return to stopped map {}",
            transfer.npc.id,
            transfer.origin.map_name()
        );
    }
}

impl NpcSkillState {
    pub fn unit_data(&self, map_id: i32) -> Value {
        use NpcUnitDataField::*;
        let status = self.snapshot();
        Value::Array(
            NPC_FIELDS
                .iter()
                .map(|(_, field)| {
                    Value::Number(match field {
                        Level => self.level as i32,
                        Hp => self.hp as i32,
                        MaxHp => status.max_hp() as i32,
                        Map => map_id,
                        X => i32::from(self.x),
                        Y => i32::from(self.y),
                        Direction => i32::from(self.dir),
                        Str => i32::from(status.str()),
                        Agi => i32::from(status.agi()),
                        Vit => i32::from(status.vit()),
                        Int => i32::from(status.int()),
                        Dex => i32::from(status.dex()),
                        Luk => i32::from(status.luk()),
                        AllStats => self.stat_point as i32,
                        DamageImmune => i32::from(self.damage_immune),
                        AttackRange => i32::from(self.attack_range),
                        AttackMin => i32::from(self.attack_min),
                        AttackMax => i32::from(self.attack_max),
                        MatkMin => i32::from(status.matk_min()),
                        MatkMax => i32::from(status.matk_max()),
                        Def => i32::from(status.def()),
                        Mdef => i32::from(status.mdef()),
                        Hit => i32::from(status.hit()),
                        Flee => i32::from(status.flee()),
                        PerfectDodge => (status.perfect_dodge() * 10.0) as i32,
                        Critical => (status.crit() * 10.0) as i32,
                        Race => status.race().value() as i32,
                        Element => status.element().value() as i32,
                        ElementLevel => i32::from(status.element_level()),
                        AttackMotion => i32::from(self.attack_motion),
                        AttackDelay => i32::from(self.attack_delay),
                        DamageMotion => i32::from(self.damage_motion),
                        Sex => i32::from(self.sex),
                        Class => i32::from(self.sprite),
                        DeadSit => i32::from(self.dead_sit),
                        GroupId => self.group_id,
                        field => i32::from(self.looks[field.look().unwrap().value() as usize]),
                    })
                })
                .collect(),
        )
    }

    pub fn set_unit_data(&mut self, field: NpcUnitDataField, value: &Value) -> Result<(), String> {
        use NpcUnitDataField::*;
        let value = value.number_value()?;
        let unsigned = || u16::try_from(value).map_err(|_| "NPC unit-data value is outside the unsigned 16-bit range".to_owned());
        let signed = || i16::try_from(value).map_err(|_| "NPC unit-data value is outside the signed 16-bit range".to_owned());
        let mut next = self.clone();
        if next.hp == 0 {
            next.initialize_for_cast();
        }
        match field {
            Level => {
                next.level = u32::try_from(value).map_err(|_| "NPC level cannot be negative")?;
                next.recalculate_misc();
            }
            Hp => {
                next.hp = u32::try_from(value)
                    .map_err(|_| "NPC HP cannot be negative")?
                    .min(next.snapshot().max_hp())
            }
            MaxHp => {
                next.max_hp = u32::try_from(value)
                    .ok()
                    .filter(|value| *value > 0)
                    .ok_or("NPC maximum HP must be positive")?;
                next.hp = next.snapshot().max_hp();
            }
            Str | Agi | Vit | Int | Dex | Luk => {
                next.parameters[field as usize - Str as usize] = unsigned()?;
                next.recalculate_misc();
            }
            AllStats => {
                next.stat_point = u32::from(unsigned()?);
                next.recalculate_misc();
            }
            DamageImmune => {
                if !(0..=1).contains(&value) {
                    return Err("NPC damage immunity must be zero or one".into());
                }
                next.damage_immune = value != 0;
            }
            AttackRange => next.attack_range = unsigned()?,
            AttackMin => next.attack_min = unsigned()?,
            AttackMax => next.attack_max = unsigned()?,
            MatkMin => next.base_status.set_matk_min(unsigned()?),
            MatkMax => next.base_status.set_matk_max(unsigned()?),
            Def => next.base_status.set_def(signed()?),
            Mdef => next.base_status.set_mdef(signed()?),
            Hit => next.base_status.set_hit(signed()?),
            Flee => next.base_status.set_flee(signed()?),
            PerfectDodge => next.base_status.set_perfect_dodge(f32::from(signed()?) / 10.0),
            Critical => next.base_status.set_crit(f32::from(signed()?) / 10.0),
            Race => next
                .base_status
                .set_race(MobRace::try_from_value(unsigned()? as usize).map_err(|_| "Invalid NPC race")?),
            Element => {
                if !(0..=9).contains(&value) {
                    return Err("Invalid NPC element".into());
                }
                next.base_status.set_element(DefenseElement::from_value(value as usize));
            }
            ElementLevel => {
                if !(1..=4).contains(&value) {
                    return Err("Invalid NPC element level".into());
                }
                next.base_status.set_element_level(value as u8);
            }
            AttackMotion => next.attack_motion = signed()?,
            AttackDelay => next.attack_delay = signed()?,
            DamageMotion => next.damage_motion = signed()?,
            Direction => {
                if !(0..=7).contains(&value) {
                    return Err("Invalid NPC direction".into());
                }
                next.dir = value as u16;
            }
            Sex => {
                if !(0..=2).contains(&value) {
                    return Err("Invalid NPC sex".into());
                }
                next.sex = value as u8;
            }
            Class => next.sprite = unsigned()?,
            DeadSit => {
                if !(0..=3).contains(&value) {
                    return Err("Invalid NPC posture".into());
                }
                next.dead_sit = value as u8;
            }
            GroupId => next.group_id = value,
            Map | X | Y => return Err("NPC position changes require the map loop".into()),
            field => next.looks[field.look().ok_or("Unknown NPC appearance field")?.value() as usize] = unsigned()?,
        }
        if self.hp == 0 && self.max_hp > 0 && next.hp > 0 {
            next.dead_sit = 0;
        }
        *self = next;
        Ok(())
    }
}
