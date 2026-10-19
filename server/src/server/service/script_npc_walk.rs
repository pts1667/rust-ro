//! `npcwalkto`, `npcstop` and `npcspeed`: the walk itself is stepped by the NPC's map (`map_unit_walk.rs`).

use std::sync::Arc;

use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::events::map_event::{MapEvent, ScriptMapCommand, ScriptNpcCallback};
use crate::server::model::map_instance::MapInstance;
use crate::server::script::ScriptRequest;
use crate::server::script::skill::actor::NPC_WALK_SPEEDS;
use crate::server::service::map_instance_service::npc_can_walk_to;
use crate::server::service::npc_event_service::{named_npc, npc_by_name_or_id};
use crate::server::service::script_service::ScriptService;
use crate::server::state::server::ServerState;

pub(crate) fn handles(function: Function) -> bool {
    matches!(function, Function::NpcSpeed | Function::NpcWalkTo | Function::NpcStop)
}

impl Server {
    pub(crate) fn script_npc_walk_call(&self, state: &ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let command = match function {
            Function::NpcSpeed => {
                let speed = number(0)?;
                let speed = u16::try_from(speed)
                    .ok()
                    .filter(|speed| NPC_WALK_SPEEDS.contains(speed))
                    .ok_or_else(|| format!("npcspeed: {speed} is outside {}..={}", NPC_WALK_SPEEDS.start(), NPC_WALK_SPEEDS.end()))?;
                let (npc_id, instance) = target_npc(state, context, arguments.get(1))?;
                (ScriptMapCommand::NpcSpeed { npc_id, speed }, instance)
            }
            Function::NpcStop => {
                let (npc_id, instance) = target_npc(state, context, arguments.first())?;
                (ScriptMapCommand::NpcStop { npc_id }, instance)
            }
            Function::NpcWalkTo => {
                let coordinate = |index: usize| u16::try_from(number(index)?).map_err(|_| "npcwalkto: coordinate is outside the map".to_string());
                let (x, y) = (coordinate(0)?, coordinate(1)?);
                let (npc_id, instance) = target_npc(state, context, arguments.get(2))?;
                let on_arrival = arguments.get(3).map(Value::text).filter(|label| !label.is_empty()).map(|label| arrival(state, context, &label)).transpose()?;
                let reachable = {
                    let map = instance.state();
                    map.script_skill_state.npcs.get(&npc_id).is_some_and(|npc| !npc.hidden && !npc.is_dead() && npc_can_walk_to(&map, npc, x, y))
                };
                if !reachable {
                    return Ok(Value::Number(0));
                }
                instance.add_to_next_tick(MapEvent::ScriptMapCommand(ScriptMapCommand::NpcWalk { npc_id, x, y, on_arrival, forced: false }));
                return Ok(Value::Number(1));
            }
            _ => return Err(format!("{function:?} is not an NPC walk call")),
        };
        command.1.add_to_next_tick(MapEvent::ScriptMapCommand(command.0));
        Ok(Value::default())
    }
}

/// The NPC named (or given by unit id) by `npc`, or the one running the script when it is absent or empty.
pub(crate) fn target_npc(state: &ServerState, context: &ScriptRequest, npc: Option<&Value>) -> Result<(u32, Arc<MapInstance>), String> {
    let actor = match npc.filter(|npc| !npc.text().is_empty()) {
        Some(npc) => npc_by_name_or_id(state, context, npc)?.ok_or_else(|| format!("NPC {npc} is not found"))?.0,
        None => crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC walking needs an NPC")?,
    };
    let instance = state.get_map_instance(&actor.map, actor.instance).ok_or("NPC map is unavailable")?;
    Ok((actor.id, instance))
}

pub(crate) fn arrival(state: &ServerState, context: &ScriptRequest, label: &str) -> Result<ScriptNpcCallback, String> {
    let (name, _) = label.split_once("::").ok_or("npcwalkto: the arrival event must be \"Npc::OnLabel\"")?;
    let entry_id = ScriptService::event_entry(label).ok_or_else(|| format!("npcwalkto: no NPC has the label {label}"))?;
    let (actor, script) = named_npc(state, context, name)?.ok_or_else(|| format!("npcwalkto: NPC {name} is not placed"))?;
    Ok(ScriptNpcCallback { npc_id: actor.id, scope_instance: script.scope_instance, entry_id })
}
