//! `unitwalk`, `unitwalkto` and `unitstopwalk` for players, monsters and NPCs. Players walk through the movement loop
//! like a click on the ground; monsters and NPCs are walked by their map (`map_unit_walk.rs`).

use movement::position::Position;
use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterRequestMove, GameEvent};
use crate::server::model::events::map_event::{ForcedWalk, MapEvent, ScriptMapCommand};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::MapItemType;
use crate::server::model::movement::Movable;
use crate::server::script::ScriptRequest;
use crate::server::script::skill::actor::ScriptSkillActor;
use crate::server::script::unit_data::request_actor;
use crate::server::service::map_instance_service::{mob_can_walk, npc_can_walk_to, walk_path};
use crate::server::service::script_npc_walk::arrival;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::util::tick::get_tick;

pub(crate) fn handles(function: Function) -> bool {
    matches!(function, Function::UnitWalk | Function::UnitWalkTo | Function::UnitStopWalk)
}

impl Server {
    pub(crate) fn script_unit_walk_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let id = |index: usize| -> Result<u32, String> {
            u32::try_from(arguments.get(index).ok_or("Missing unit id")?.number_value()?).map_err(|_| "Invalid unit id".to_string())
        };
        let Some(unit) = request_actor(self, state, context, id(0)?)? else {
            return Ok(Value::Number(0));
        };
        if function == Function::UnitStopWalk {
            self.stop_unit_walk(state, &unit);
            return Ok(Value::default());
        }
        let (destination, label) = if function == Function::UnitWalk {
            let coordinate = |index: usize| -> Result<u16, String> {
                u16::try_from(arguments.get(index).ok_or("Missing coordinate")?.number_value()?).map_err(|_| "unitwalk: coordinate is outside the map".to_string())
            };
            ((coordinate(1)?, coordinate(2)?), arguments.get(3))
        } else {
            let Some(target) = request_actor(self, state, context, id(1)?)?.filter(|target| (&target.map, target.instance) == (&unit.map, unit.instance)) else {
                return Ok(Value::Number(0));
            };
            ((target.x, target.y), arguments.get(2))
        };
        let on_arrival = label.map(Value::text).filter(|label| !label.is_empty()).map(|label| arrival(state, context, &label)).transpose()?;
        let walk = ForcedWalk { map: MapInstanceKey::new(unit.map.clone(), unit.instance), destination, on_arrival };
        Ok(Value::Number(i32::from(self.start_unit_walk(state, &unit, walk))))
    }

    /// False when the unit cannot walk there now: no path, a state that blocks walking, or a `unitwalk` still under way.
    fn start_unit_walk(&self, state: &mut ServerState, unit: &ScriptSkillActor, walk: ForcedWalk) -> bool {
        let Some(instance) = state.get_map_instance(&unit.map, unit.instance) else { return false };
        let (x, y) = walk.destination;
        match unit.object_type {
            MapItemType::Character => {
                let Some(character) = state.characters().get(&unit.id) else { return false };
                let blocked = character.is_dead()
                    || character.status.blocks_movement()
                    || character.game_systems.is_trading()
                    || character.game_systems.vending_store.is_some()
                    || character.game_systems.buying_store.is_some()
                    || (character.game_systems.forced_walk.is_some() && character.is_moving());
                let from = (character.x, character.y);
                if blocked || (from != walk.destination && walk_path(&instance.state(), from, walk.destination).is_empty()) {
                    return false;
                }
                if from == walk.destination {
                    if let Some(callback) = walk.on_arrival {
                        self.queue_player_npc_event(unit.id, callback.npc_id, callback.scope_instance, callback.entry_id);
                    }
                    return true;
                }
                state.characters_mut().get_mut(&unit.id).unwrap().game_systems.forced_walk = Some(walk);
                self.add_to_next_movement_tick(GameEvent::CharacterRequestMove(CharacterRequestMove { char_id: unit.id, destination: Position { x, y, dir: 0 } }));
                true
            }
            MapItemType::Mob => {
                let walkable = {
                    let map = instance.state();
                    map.get_mob(unit.id).is_some_and(|mob| mob_can_walk(mob) && ((mob.x, mob.y) == (x, y) || !walk_path(&map, (mob.x, mob.y), (x, y)).is_empty()))
                };
                if walkable {
                    instance.add_to_next_tick(MapEvent::ScriptMapCommand(ScriptMapCommand::MobWalk { mob_id: unit.id, x, y, on_arrival: walk.on_arrival }));
                }
                walkable
            }
            MapItemType::Npc => {
                let walkable = {
                    let map = instance.state();
                    map.script_skill_state.npcs.get(&unit.id).is_some_and(|npc| {
                        !npc.hidden && !npc.is_dead() && !npc.walk.as_ref().is_some_and(|walk| walk.forced) && npc_can_walk_to(&map, npc, x, y)
                    })
                };
                if walkable {
                    instance.add_to_next_tick(MapEvent::ScriptMapCommand(ScriptMapCommand::NpcWalk { npc_id: unit.id, x, y, on_arrival: walk.on_arrival, forced: true }));
                }
                walkable
            }
            // Pets, homunculi and mercenaries follow their owner.
            _ => false,
        }
    }

    /// The movement loop calls this when a player's path runs out: the `unitwalk` label runs if they stopped on its destination.
    pub(crate) fn character_stopped_walking(&self, character: &mut Character) {
        let Some(walk) = character.game_systems.forced_walk.take() else { return };
        if let Some(callback) = walk.on_arrival.filter(|_| walk.arrived(&character.map_instance_key, (character.x, character.y))) {
            self.queue_player_npc_event(character.char_id, callback.npc_id, callback.scope_instance, callback.entry_id);
        }
    }

    fn stop_unit_walk(&self, state: &mut ServerState, unit: &ScriptSkillActor) {
        let command = match unit.object_type {
            MapItemType::Character => {
                if let Some(character) = state.characters_mut().get_mut(&unit.id) {
                    character.game_systems.forced_walk = None;
                    if character.is_moving() {
                        self.character_service().cancel_movement(character, get_tick());
                    }
                }
                return;
            }
            MapItemType::Mob => ScriptMapCommand::MobStop { mob_id: unit.id },
            MapItemType::Npc => ScriptMapCommand::NpcStop { npc_id: unit.id },
            _ => return,
        };
        if let Some(instance) = state.get_map_instance(&unit.map, unit.instance) {
            instance.add_to_next_tick(MapEvent::ScriptMapCommand(command));
        }
    }
}
