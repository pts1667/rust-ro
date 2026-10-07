use std::sync::Arc;

use models::enums::EnumWithMaskValueU32;
use models::enums::script::BroadcastFlag;
use packets::packets::{Packet, PacketZcCloseDialog};
use script_sdk::{Function, Reply, Value};
use tokio::sync::mpsc;

use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptNpcEvent};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map::Map;
use crate::server::model::map_flags::MapFlag;
use crate::server::model::script::Script;
use crate::server::model::session::Session;
use crate::server::script::skill::actor::{NpcSkillState, ScriptSkillActor};
use crate::server::script::{NpcScriptHost, ScriptRequest};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_service::ScriptService;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::server::ServerState;
use crate::util::tick::get_tick;

pub(crate) fn npcs(state: &ServerState) -> Vec<(ScriptSkillActor, Arc<Script>)> {
    state
        .map_instances()
        .values()
        .flatten()
        .filter(|instance| instance.is_alive())
        .flat_map(|instance| {
            instance
                .state()
                .script_skill_state
                .npcs
                .values()
                .map(|npc| {
                    (
                        npc.actor(instance.key().map_name().clone(), instance.id()),
                        npc.current_script(instance.name()),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub(crate) fn event_npc(state: &ServerState, npc_id: u32, scope_instance: u8) -> Option<(ScriptSkillActor, Arc<Script>)> {
    for instance in state.map_instances().values().flatten().filter(|instance| instance.is_alive()) {
        let map_state = instance.state();
        if let Some(npc) = map_state.script_skill_state.npcs.get(&npc_id).filter(|npc| npc.script.scope_instance == scope_instance) {
            return Some((npc.actor(instance.key().map_name().clone(), instance.id()), npc.current_script(instance.name())));
        }
    }
    None
}

pub(crate) fn named_npc(state: &ServerState, context: &ScriptRequest, name: &str) -> Result<Option<(ScriptSkillActor, Arc<Script>)>, String> {
    let mut candidates = npcs(state)
        .into_iter()
        .filter(|(_, script)| script.name == name)
        .collect::<Vec<_>>();
    if let Some(caller) = crate::server::script::unit_data::script_actor(state, context)? {
        let nearby = candidates
            .iter()
            .filter(|(actor, _)| actor.map == caller.map && actor.instance == caller.instance)
            .cloned()
            .collect::<Vec<_>>();
        if !nearby.is_empty() {
            candidates = nearby;
        }
    }
    if candidates.len() > 1 {
        candidates.retain(|(_, script)| script.scope_instance == context.npc_scope_instance);
    }
    if candidates.len() > 1 {
        return Err("NPC name is ambiguous in this instance".into());
    }
    Ok(candidates.pop())
}

/// True when a step from `from` to `to` enters the trigger area centred on the NPC.
pub(crate) fn touch_area_entered(npc: (u16, u16), area: (u16, u16), from: (u16, u16), to: (u16, u16)) -> bool {
    let inside = |(x, y): (u16, u16)| x.abs_diff(npc.0) <= area.0 && y.abs_diff(npc.1) <= area.1;
    (area.0 > 0 || area.1 > 0) && inside(to) && !inside(from)
}

/// `(npc id, scope instance, OnTouch entry)` of the NPCs whose area the step enters; without an `OnTouch` label the NPC is clicked instead.
pub(crate) fn touched_npc_events(map: &MapInstanceState, from: (u16, u16), to: (u16, u16)) -> Vec<(u32, u8, Option<u32>)> {
    map.script_skill_state
        .npcs
        .values()
        .filter(|npc: &&NpcSkillState| {
            !npc.hidden && touch_area_entered((npc.x, npc.y), (npc.script.x_size, npc.script.y_size), from, to)
        })
        .map(|npc| (npc.id, npc.script.scope_instance, ScriptService::event_entry(&format!("{}::OnTouch", npc.script.name))))
        .collect()
}

static LAST_CLOCK_TIME: std::sync::Mutex<Option<chrono::NaiveDateTime>> = std::sync::Mutex::new(None);

/// The clock labels rathena fires when the wall clock goes from `previous` to `now`.
fn clock_events(previous: chrono::NaiveDateTime, now: chrono::NaiveDateTime) -> Vec<String> {
    use chrono::{Datelike, Timelike};
    const WEEKDAYS: [&str; 7] = ["OnMon", "OnTue", "OnWed", "OnThu", "OnFri", "OnSat", "OnSun"];
    let mut events = Vec::new();
    if now.minute() != previous.minute() {
        events.push(format!("OnMinute{:02}", now.minute()));
        events.push(format!("OnClock{:02}{:02}", now.hour(), now.minute()));
        events.push(format!("{}{:02}{:02}", WEEKDAYS[now.weekday().num_days_from_monday() as usize], now.hour(), now.minute()));
    }
    if now.hour() != previous.hour() {
        events.push(format!("OnHour{:02}", now.hour()));
    }
    if now.day() != previous.day() {
        events.push(format!("OnDay{:02}{:02}", now.month(), now.day()));
    }
    events
}

impl Server {
    pub(crate) fn schedule_npc_initialization(&self, state: &ServerState) {
        self.broadcast_npc_event(state, "OnInit");
        self.broadcast_npc_event(state, "OnAgitInit");
        self.add_to_next_tick(GameEvent::CastleLifecycle(crate::server::model::events::game_event::CastleLifecycle::Init));
    }

    /// Fires the `OnMinute`, `OnClock`, weekday, `OnHour` and `OnDay` labels the wall clock just reached.
    pub(crate) fn tick_npc_clock(&self, state: &ServerState) {
        let now = chrono::Local::now().naive_local();
        let Some(previous) = LAST_CLOCK_TIME.lock().unwrap().replace(now) else { return };
        for event in clock_events(previous, now) {
            self.broadcast_npc_event(state, &event);
        }
    }

    /// Queues `OnPC...Event` for every NPC that has the label, with the player attached and `killedrid`/`killerrid` readable as `@` variables.
    pub(crate) fn broadcast_player_npc_event(&self, state: &ServerState, char_id: u32, event: &str, related_id: Option<(&str, i32)>) {
        let targets: Vec<_> = npcs(state).into_iter()
            .filter_map(|(actor, script)| ScriptService::event_entry(&format!("{}::{event}", script.name)).map(|entry_id| (actor.id, script.scope_instance, entry_id)))
            .collect();
        if targets.is_empty() {
            return;
        }
        if let Some((name, value)) = related_id {
            self.script_service().install_temporary_variables(char_id, &[script_sdk::Variable { scope: script_sdk::VariableScope::CharacterTemporary, name: name.into(), index: 0, value: Value::Number(value) }]);
        }
        for (npc_id, scope_instance, entry_id) in targets {
            self.queue_player_npc_event(char_id, npc_id, scope_instance, entry_id);
        }
    }

    pub(crate) fn broadcast_npc_event(&self, state: &ServerState, event: &str) {
        for (actor, script) in npcs(state) {
            if let Some(entry_id) = ScriptService::event_entry(&format!("{}::{event}", script.name)) {
                self.add_to_next_tick(GameEvent::ScriptNpcEvent(ScriptNpcEvent {
                    npc_id: actor.id,
                    scope_instance: script.scope_instance,
                    entry_id,
                    char_id: None,
                    depth: 0,
                    queued_until: 0,
                    args: None,
                    timer_guard: None,
                }));
            }
        }
    }

    pub(crate) fn npc_event_call(&self, state: &ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        if function == Function::GetNpcId {
            let kind = arguments.first().ok_or("NPC identifier type is required")?.number_value()?;
            if kind != 0 || arguments.len() > 2 {
                return Ok(0.into());
            }
            let id = if let Some(name) = arguments.get(1) {
                named_npc(state, context, name.string_value()?)?.map(|(actor, _)| actor.id)
            } else {
                crate::server::script::unit_data::script_actor(state, context)?.map(|actor| actor.id)
            };
            return Ok(Value::Number(id.unwrap_or(0) as i32));
        }
        if arguments.len() != 1 {
            return Err("NPC event expects one label".into());
        }
        if context.event_depth >= 8 {
            return Err("NPC event nesting limit exceeded".into());
        }
        let label = arguments[0].string_value()?;
        let (name, event) = label.split_once("::").ok_or("NPC event must name an NPC and an On label")?;
        if !event.starts_with("On") || event.len() > 128 || label.as_bytes().contains(&0) {
            return Err("Invalid NPC event label".into());
        }
        let attached = function == Function::DoEvent;
        if attached && context.char_id == 0 {
            return Err("doevent requires an attached player".into());
        }
        let targets = if name.is_empty() && !attached {
            npcs(state)
        } else if name.is_empty() {
            return Err("doevent requires a named NPC".into());
        } else {
            named_npc(state, context, name)?.into_iter().collect()
        };
        let mut count = 0;
        for (actor, script) in targets {
            let Some(entry_id) = ScriptService::event_entry(&format!("{}::{event}", script.name)) else {
                continue;
            };
            self.add_to_next_tick(GameEvent::ScriptNpcEvent(ScriptNpcEvent {
                npc_id: actor.id,
                scope_instance: script.scope_instance,
                entry_id,
                char_id: attached.then_some(context.char_id),
                depth: context.event_depth + 1,
                queued_until: get_tick() + u128::from(self.configuration.scripting.conversation_timeout_secs.max(1)) * 2000,
                args: None,
                timer_guard: None,
            }));
            count += 1;
        }
        Ok(Value::Number(count))
    }

    /// Queues `NpcName::OnLabel` as a background event; false when no compiled NPC owns the label.
    pub(crate) fn trigger_npc_event(&self, state: &ServerState, label: &str) -> bool {
        let Some((name, _)) = label.split_once("::") else { return false };
        let Some(entry_id) = ScriptService::event_entry(label) else { return false };
        let Some((actor, script)) = npcs(state).into_iter().find(|(_, script)| script.name == name) else { return false };
        self.add_to_next_tick(GameEvent::ScriptNpcEvent(ScriptNpcEvent {
            npc_id: actor.id,
            scope_instance: script.scope_instance,
            entry_id,
            char_id: None,
            depth: 1,
            queued_until: 0,
            args: None,
            timer_guard: None,
        }));
        true
    }

    pub(crate) fn queue_player_npc_event(&self, char_id: u32, npc_id: u32, scope_instance: u8, entry_id: u32) {
        self.add_to_next_tick(GameEvent::ScriptNpcEvent(ScriptNpcEvent {
            npc_id,
            scope_instance,
            entry_id,
            char_id: Some(char_id),
            depth: 1,
            queued_until: get_tick() + u128::from(self.configuration.scripting.conversation_timeout_secs.max(1)) * 2000,
            args: None,
            timer_guard: None,
        }));
    }

    /// Runs every compiled `OnPCLoadMapEvent` for a player entering a map that has the `loadevent` flag.
    pub(crate) fn trigger_map_load_events(&self, state: &ServerState, char_id: u32) {
        let Some(character) = state.get_character(char_id) else { return };
        if !state.map_flags(&character.map_instance_key).enabled(MapFlag::LoadEvent) {
            return;
        }
        for (actor, script) in npcs(state) {
            if let Some(entry_id) = ScriptService::event_entry(&format!("{}::OnPCLoadMapEvent", script.name)) {
                self.queue_player_npc_event(char_id, actor.id, script.scope_instance, entry_id);
            }
        }
    }

    /// Queues `NpcName::OnLabel` with the character attached; false when no compiled NPC owns the label.
    pub(crate) fn trigger_player_npc_event(&self, state: &ServerState, char_id: u32, label: &str) -> bool {
        let Some((name, _)) = label.split_once("::") else { return false };
        let Some(entry_id) = ScriptService::event_entry(label) else { return false };
        let Some((actor, script)) = npcs(state).into_iter().find(|(_, script)| script.name == name) else { return false };
        self.add_to_next_tick(GameEvent::ScriptNpcEvent(ScriptNpcEvent {
            npc_id: actor.id,
            scope_instance: script.scope_instance,
            entry_id,
            char_id: Some(char_id),
            depth: 1,
            queued_until: get_tick() + u128::from(self.configuration.scripting.conversation_timeout_secs.max(1)) * 2000,
            args: None,
            timer_guard: None,
        }));
        true
    }

    pub(crate) fn run_npc_event(&self, state: &mut ServerState, event: ScriptNpcEvent, tick: u128) -> Result<(), String> {
        if event.depth > 8 {
            return Err("NPC event nesting limit exceeded".into());
        }
        if event.queued_until != 0 && tick >= event.queued_until {
            if let Some(guard) = event.timer_guard { state.script_timers.consume(guard); }
            return Ok(());
        }
        if event.timer_guard.is_some_and(|guard| !state.script_timers.is_valid(state, guard)) {
            return Ok(());
        }
        let target = event_npc(state, event.npc_id, event.scope_instance);
        let Some((actor, script)) = target else {
            if event.timer_guard.is_some() {
                self.add_to_delayed_tick(GameEvent::ScriptNpcEvent(event), crate::server::game_loop::GAME_TICK_RATE);
                return Ok(());
            }
            return Err("NPC event target is unavailable".into());
        };
        let server = self.shared().ok_or("NPC runtime is not bound")?;
        if event.char_id.is_some_and(|char_id| state.pending_character_logouts.contains_key(&char_id)) { return Ok(()); }
        let background = event.char_id.is_none();
        let (session, generation, inputs) = if let Some(char_id) = event.char_id {
            let character = state.get_character(char_id).ok_or("Event player disconnected")?;
            let session = state
                .find_session(character.account_id)
                .filter(|session| session.char_id == Some(char_id))
                .ok_or("Event session expired")?;
            if session.script_handler_channel_sender.lock().unwrap().is_some() {
                self.add_to_delayed_tick(GameEvent::ScriptNpcEvent(event), crate::server::game_loop::GAME_TICK_RATE);
                return Ok(());
            }
            let (sender, inputs) = mpsc::channel(4);
            let generation = session.set_script_handler_channel_sender(sender);
            (session, generation, inputs)
        } else {
            let session = Arc::new(Session::create_empty(0, 0, 0, self.packetver()).recreate_with_character(0));
            let (_, inputs) = mpsc::channel(1);
            (session, 0, inputs)
        };
        let host = NpcScriptHost {
            server,
            session: session.clone(),
            script,
            inputs,
            notifications: self.server_service().notification_sender(),
            generation,
            map_instance: actor.instance,
            background,
            event_depth: event.depth,
            event_arguments: event.args,
            timer_context: match event.timer_guard {
                Some(crate::server::model::script_timer::ScriptTimerGuard::Npc { key, .. }) => Some(key),
                _ => None,
            },
            logout_token: None,
            error: None,
        };
        if let Some(guard) = event.timer_guard { state.script_timers.consume(guard); }
        let vm = self.script_service().vm.clone();
        let timeout = std::time::Duration::from_secs(self.configuration.scripting.conversation_timeout_secs.max(1));
        let notifications = self.server_service().notification_sender();
        let packetver = self.packetver();
        self.runtime().spawn(async move {
            let error = match tokio::time::timeout(timeout, vm.execute(host, "run_event", event.entry_id)).await {
                Ok((host, result)) => result.err().map(|error| host.error.unwrap_or(error)),
                Err(_) => Some("NPC event timed out".into()),
            };
            if let Some(error) = error {
                script_debug!("NPC event failed: npc={} entry={} char={}: {error}", event.npc_id, event.entry_id, session.char_id());
                if !background && session.script_generation.load(std::sync::atomic::Ordering::Acquire) == generation {
                    let mut packet = PacketZcCloseDialog::new(packetver);
                    packet.naid = event.npc_id;
                    packet.fill_raw();
                    let _ = notifications.try_send(Notification::Char(CharNotification::new(session.char_id(), packet.raw)));
                }
            }
            if !background {
                session.finish_script(generation);
            }
        });
        Ok(())
    }

    /// Spawns a monster on a script-named map, binding its callback event to the compiled NPC that owns it.
    pub(crate) fn spawn_script_monster(
        &self,
        state: &mut ServerState,
        context: &ScriptRequest,
        map_name: &str,
        mut request: crate::server::model::events::map_event::ScriptSpawn,
    ) -> Result<Option<u32>, String> {
        let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC source is unavailable")?;
        if let Some((name, _)) = request.event.split_once("::") {
            let (target, script) = named_npc(state, context, name)?.ok_or("Monster callback NPC is unavailable")?;
            request.event_npc = Some(crate::server::model::events::map_event::ScriptNpcCallback {
                npc_id: target.id,
                scope_instance: script.scope_instance,
                entry_id: ScriptService::event_entry(&request.event).ok_or("Monster callback has no compiled event entry")?,
            });
        }
        let (name, resolved) = if map_name == "this" { (Map::name_without_ext(&npc.map), None) } else { self.resolve_script_map(context.npc_scope_instance, map_name) };
        let id = resolved.unwrap_or(if name == Map::name_without_ext(&npc.map) { npc.instance } else { 0 });
        let map = if let Some(map) = state.get_map_instance(&name, id) {
            map
        } else if id != 0 && crate::server::model::instance::is_memorial_map(&name) {
            return Err("Monster instance map is gone".into());
        } else {
            let definition = GlobalConfigService::instance().find_map(&name).ok_or("Monster map is unavailable")?;
            self.server_service().create_map_instance(state, definition, id)
        };
        request.reserved_id = (request.amount == 1).then(|| map.state().reserve_map_item_id());
        let reserved_id = request.reserved_id;
        map.add_to_next_tick(MapEvent::ScriptSpawn(request));
        Ok(reserved_id)
    }

    pub(crate) fn npc_background_call(
        &self,
        state: &mut ServerState,
        context: &ScriptRequest,
        function: Function,
        arguments: &[Value],
    ) -> Reply {
        let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC source is unavailable")?;
        if function == Function::Monster {
            let request = self.item_service().spawn_request(arguments, context.char_id)?;
            self.spawn_script_monster(state, context, arguments[0].string_value()?, request)?;
            return Ok(Value::default());
        }
        if function == Function::AreaMonster {
            if arguments.len() < 8 {
                return Err("areamonster needs a map, an area, a name, a class and an amount".into());
            }
            let corner = |index: usize| arguments[index].number_value();
            let monster_arguments: Vec<Value> = arguments[..3].iter().chain(&arguments[5..]).cloned().collect();
            let mut request = self.item_service().spawn_request(&monster_arguments, context.char_id)?;
            request.area_end = Some((corner(3)?, corner(4)?));
            self.spawn_script_monster(state, context, arguments[0].string_value()?, request)?;
            return Ok(Value::default());
        }
        let packet = super::script_presentation_service::announcement_packet(arguments)?;
        let scope = arguments[1].number_value()? as u32
            & (BroadcastFlag::Map.as_flag() | BroadcastFlag::Area.as_flag() | BroadcastFlag::ReservedTarget.as_flag());
        if scope == BroadcastFlag::Map.as_flag() | BroadcastFlag::Area.as_flag() {
            return Err("Self announcement requires an attached player".into());
        }
        let recipients = state
            .characters()
            .values()
            .filter(|character| match scope {
                0 => true,
                value if value == BroadcastFlag::Map.as_flag() => {
                    character.current_map_name() == &npc.map && character.current_map_instance() == npc.instance
                }
                value if value == BroadcastFlag::Area.as_flag() => {
                    character.current_map_name() == &npc.map
                        && character.current_map_instance() == npc.instance
                        && character.x.abs_diff(npc.x) <= crate::server::PLAYER_FOV
                        && character.y.abs_diff(npc.y) <= crate::server::PLAYER_FOV
                }
                _ => false,
            })
            .map(|character| character.char_id)
            .collect::<Vec<_>>();
        self.map_notifications.extend(
            recipients
                .into_iter()
                .map(|char_id| Notification::Char(CharNotification::new(char_id, packet.clone()))),
        );
        self.drain_map_notifications();
        Ok(Value::default())
    }
}

#[cfg(test)]
mod tests {
    use super::{clock_events, touch_area_entered};
    use chrono::NaiveDate;

    #[test]
    fn clock_labels_follow_the_wall_clock() {
        let at = |day: u32, hour: u32, minute: u32| NaiveDate::from_ymd_opt(2026, 3, day).unwrap().and_hms_opt(hour, minute, 0).unwrap();
        assert!(clock_events(at(1, 8, 4), at(1, 8, 4)).is_empty());
        assert_eq!(clock_events(at(1, 8, 4), at(1, 8, 5)), ["OnMinute05", "OnClock0805", "OnSun0805"]);
        assert_eq!(clock_events(at(1, 23, 59), at(2, 0, 0)), ["OnMinute00", "OnClock0000", "OnMon0000", "OnHour00", "OnDay0302"]);
    }

    #[test]
    fn touch_fires_only_when_a_step_enters_the_area() {
        let npc = (50, 50);
        let area = (2, 1);
        assert!(touch_area_entered(npc, area, (47, 50), (48, 50)));
        assert!(touch_area_entered(npc, area, (48, 52), (48, 51)));
        assert!(!touch_area_entered(npc, area, (48, 50), (49, 50)), "stepping inside the area is not an entry");
        assert!(!touch_area_entered(npc, area, (49, 50), (47, 50)), "leaving the area never fires");
        assert!(!touch_area_entered(npc, area, (40, 40), (41, 40)));
        assert!(!touch_area_entered(npc, (0, 0), (49, 50), (50, 50)), "an NPC without an area never fires");
    }
}
