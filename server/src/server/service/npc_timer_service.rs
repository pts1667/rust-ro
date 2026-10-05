use std::sync::Arc;

use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, ScriptNpcEvent};
use crate::server::model::script::Script;
use crate::server::model::script_timer::{NpcTimer, NpcTimerKey, PlayerScriptTimer, ScriptTimerGuard, ScriptTimerOwner};
use crate::server::script::ScriptRequest;
use crate::server::service::npc_event_service::named_npc;
use crate::server::service::script_service::ScriptService;
use crate::server::state::server::ServerState;

const NPC_PRESENCE_CHECK_INTERVAL_MS: u128 = 1000;

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::AddTimer
            | Function::DeleteTimer
            | Function::AddTimerCount
            | Function::InitNpcTimer
            | Function::StartNpcTimer
            | Function::StopNpcTimer
            | Function::SetNpcTimer
            | Function::GetNpcTimer
            | Function::AttachNpcTimer
            | Function::DetachNpcTimer
    )
}

fn owner(state: &ServerState, char_id: u32) -> Result<ScriptTimerOwner, String> {
    let character = state.get_character(char_id).ok_or("Script timer requires an online player")?;
    let session = state
        .find_session(character.account_id)
        .filter(|session| session.char_id == Some(char_id))
        .ok_or("Script timer player session is unavailable")?;
    Ok(ScriptTimerOwner {
        char_id,
        account_id: character.account_id,
        session,
    })
}

fn target(state: &ServerState, context: &ScriptRequest, name: Option<&Value>) -> Result<Arc<Script>, String> {
    if let Some(name) = name {
        return named_npc(state, context, name.string_value()?)?
            .map(|(_, script)| script)
            .ok_or("Timer NPC is unavailable".into());
    }
    let actor = crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC timer requires a source NPC")?;
    let map = state
        .get_map_instance(&actor.map, actor.instance)
        .ok_or("Timer NPC map is unavailable")?;
    let script = map
        .state()
        .script_skill_state
        .npcs
        .get(&actor.id)
        .map(|npc| npc.current_script(map.name()))
        .ok_or("Timer NPC is unavailable".into());
    script
}

fn name_and_flag(arguments: &[Value]) -> Result<(Option<&Value>, bool), String> {
    match arguments {
        [] => Ok((None, false)),
        [Value::String(_)] => Ok((arguments.first(), false)),
        [flag] => Ok((None, flag.number_value()? != 0)),
        [name, flag] => {
            name.string_value()?;
            Ok((Some(name), flag.number_value()? != 0))
        }
        _ => Err("NPC timer expects an optional NPC name and attachment flag".into()),
    }
}

fn timer_selection(state: &ServerState, context: &ScriptRequest, script: &Script) -> (NpcTimerKey, Option<ScriptTimerOwner>) {
    if let Some(key) = context
        .timer_context
        .filter(|key| key.npc_id == script.id && key.scope_instance == script.scope_instance)
    {
        let owner = state.script_timers.npc.get(&key).and_then(|timer| timer.owner.clone());
        return (key, owner);
    }
    let owner = state
        .script_timers
        .attachments
        .get(&(script.id, script.scope_instance))
        .filter(|owner| owner.is_current(state))
        .cloned();
    (
        NpcTimerKey {
            npc_id: script.id,
            scope_instance: script.scope_instance,
            char_id: owner.as_ref().map(|owner| owner.char_id),
        },
        owner,
    )
}

impl Server {
    pub(crate) fn npc_timer_call(
        &self,
        state: &mut ServerState,
        context: &ScriptRequest,
        function: Function,
        arguments: &[Value],
        tick: u128,
    ) -> Reply {
        if matches!(function, Function::AddTimer | Function::DeleteTimer | Function::AddTimerCount) {
            if context.logout_token.is_some() {
                return if function == Function::AddTimer {
                    Err("Cannot start player timers during logout".into())
                } else {
                    Ok(Value::default())
                };
            }
            return self.player_timer_call(state, context, function, arguments, tick);
        }
        let (name, attach_flag) = match function {
            Function::InitNpcTimer | Function::StartNpcTimer | Function::StopNpcTimer => name_and_flag(arguments)?,
            Function::GetNpcTimer | Function::SetNpcTimer if (1..=2).contains(&arguments.len()) => (arguments.get(1), false),
            Function::AttachNpcTimer | Function::DetachNpcTimer if arguments.len() <= 1 => (
                if function == Function::DetachNpcTimer {
                    arguments.first()
                } else {
                    None
                },
                false,
            ),
            _ => return Err("Invalid NPC timer arguments".into()),
        };
        let script = target(state, context, name)?;
        if let Some(current) = state
            .pending_character_logouts
            .get_mut(&context.char_id)
            .and_then(|pending| pending.current.as_mut())
            .filter(|current| {
                context.logout_token == Some(current.token)
                    && context.timer_context == Some(current.key)
                    && script.id == current.key.npc_id
                    && script.scope_instance == current.key.scope_instance
            })
        {
            match function {
                Function::GetNpcTimer => {
                    return Ok(Value::Number(match arguments[0].number_value()? {
                        0 => current.elapsed(tick).min(i32::MAX as u64) as i32,
                        1 => 0,
                        2 => current.label_count.min(i32::MAX as usize) as i32,
                        _ => 0,
                    }));
                }
                Function::SetNpcTimer => {
                    current.elapsed_ms = u64::try_from(arguments[0].number_value()?).map_err(|_| "NPC timer tick must be nonnegative")?;
                    if current.started_at.is_some() {
                        current.started_at = Some(tick);
                    }
                    return Ok(Value::default());
                }
                Function::StopNpcTimer => {
                    current.elapsed_ms = current.elapsed(tick);
                    current.started_at = None;
                    return Ok(Value::default());
                }
                Function::InitNpcTimer | Function::StartNpcTimer => return Err("Cannot restart an attached NPC timer during logout".into()),
                _ => {}
            }
        }
        let attachment_key = (script.id, script.scope_instance);
        if function == Function::AttachNpcTimer {
            let char_id = if let Some(name) = arguments.first() {
                let name = name.string_value()?;
                state
                    .characters()
                    .values()
                    .find(|character| character.name == *name)
                    .map(|character| character.char_id)
                    .ok_or("NPC timer player is unavailable")?
            } else {
                context.char_id
            };
            let owner = owner(state, char_id)?;
            let (key, _) = timer_selection(state, context, &script);
            if state.script_timers.npc.get(&key).is_some_and(|timer| timer.started_at.is_some()) {
                return Err("Stop the NPC timer before attaching a player".into());
            }
            state.script_timers.attachments.insert(attachment_key, owner);
            return Ok(Value::default());
        }
        let pending_attachment = if attach_flag && matches!(function, Function::InitNpcTimer | Function::StartNpcTimer) {
            Some(owner(state, context.char_id)?)
        } else {
            None
        };
        let (key, selected_owner) = if let Some(owner) = &pending_attachment {
            (
                NpcTimerKey {
                    npc_id: script.id,
                    scope_instance: script.scope_instance,
                    char_id: Some(owner.char_id),
                },
                Some(owner.clone()),
            )
        } else {
            timer_selection(state, context, &script)
        };
        if function == Function::DetachNpcTimer {
            if state.script_timers.npc.get(&key).is_some_and(|timer| timer.started_at.is_some()) {
                return Err("Stop the NPC timer before detaching a player".into());
            }
            if state
                .script_timers
                .attachments
                .get(&attachment_key)
                .is_some_and(|owner| key.char_id == Some(owner.char_id))
            {
                state.script_timers.attachments.remove(&attachment_key);
            }
            return Ok(Value::default());
        }
        let elapsed = if function == Function::SetNpcTimer {
            Some(u64::try_from(arguments[0].number_value()?).map_err(|_| "NPC timer tick must be nonnegative")?)
        } else {
            None
        };
        let information = if function == Function::GetNpcTimer {
            Some(arguments[0].number_value()?)
        } else {
            None
        };
        if matches!(function, Function::InitNpcTimer | Function::StartNpcTimer) && key.char_id.is_some() {
            if state.script_timers.npc.iter().any(|(other, timer)| {
                other != &key && other.char_id == key.char_id && timer.started_at.is_some() && timer.next_label < timer.labels.len()
            }) {
                return Err("Player already has a running NPC timer".into());
            }
        }
        let revision = if information.is_none() { state.script_timers.next_id()? } else { 0 };
        if let Some(owner) = pending_attachment {
            state.script_timers.attachments.insert(attachment_key, owner);
        }
        let timer = state.script_timers.npc.entry(key).or_insert_with(|| NpcTimer {
            npc_name: script.name.clone(),
            elapsed_ms: 0,
            started_at: None,
            next_label: 0,
            labels: ScriptService::npc_timer_entries(&script.name),
            revision,
            owner: selected_owner,
        });
        match function {
            Function::GetNpcTimer => {
                return Ok(Value::Number(match information.unwrap() {
                    0 => timer.elapsed(tick).min(i32::MAX as u64) as i32,
                    1 => i32::from(timer.started_at.is_some() && timer.next_label < timer.labels.len()),
                    2 => timer.labels.len().min(i32::MAX as usize) as i32,
                    _ => 0,
                }));
            }
            Function::InitNpcTimer => {
                timer.revision = revision;
                timer.set_elapsed(0, tick);
                timer.started_at = Some(tick);
            }
            Function::StartNpcTimer => {
                if timer.started_at.is_none() {
                    timer.revision = revision;
                    timer.started_at = Some(tick);
                }
            }
            Function::StopNpcTimer => {
                timer.elapsed_ms = timer.elapsed(tick);
                timer.started_at = None;
                timer.revision = revision;
                if attach_flag {
                    state.script_timers.attachments.remove(&attachment_key);
                }
            }
            Function::SetNpcTimer => {
                timer.set_elapsed(elapsed.unwrap(), tick);
                timer.revision = revision;
            }
            _ => return Err("Unknown NPC timer operation".into()),
        }
        Ok(Value::default())
    }

    fn player_timer_call(
        &self,
        state: &mut ServerState,
        context: &ScriptRequest,
        function: Function,
        arguments: &[Value],
        tick: u128,
    ) -> Reply {
        let owner = owner(state, context.char_id)?;
        let (delay, label) = match (function, arguments) {
            (Function::DeleteTimer, [label]) => (0, label.string_value()?),
            (Function::AddTimer | Function::AddTimerCount, [delay, label]) => (delay.number_value()?, label.string_value()?),
            _ => return Err("Player timer expects a delay and event label".into()),
        };
        if function == Function::AddTimer {
            if delay < 0 {
                return Err("Player timer delay must be nonnegative".into());
            }
            let (name, event_label) = label.split_once("::").ok_or("Player timer must name an NPC and On label")?;
            if name.is_empty() || !event_label.starts_with("On") || event_label.len() > 128 || label.as_bytes().contains(&0) {
                return Err("Invalid player timer event label".into());
            }
            let (actor, script) = named_npc(state, context, name)?.ok_or("Player timer NPC is unavailable")?;
            let entry_id = ScriptService::event_entry(label).ok_or("Player timer has no compiled event entry")?;
            if state
                .script_timers
                .player
                .values()
                .filter(|timer| timer.owner.char_id == owner.char_id && Arc::ptr_eq(&timer.owner.session, &owner.session))
                .count()
                >= 32
            {
                return Err("Player script timer limit exceeded".into());
            }
            let id = state.script_timers.next_id()?;
            state.script_timers.player.insert(id, PlayerScriptTimer {
                event: ScriptNpcEvent {
                    npc_id: actor.id,
                    scope_instance: script.scope_instance,
                    entry_id,
                    char_id: Some(owner.char_id),
                    depth: 0,
                    queued_until: 0,
                    args: None,
                    timer_guard: None,
                },
                owner,
                label: label.into(),
                due_at: tick.saturating_add(delay as u128),
                queued: false,
                revision: id,
            });
        } else {
            let id = state
                .script_timers
                .player
                .iter()
                .filter(|(_, timer)| {
                    timer.label == *label && timer.owner.char_id == owner.char_id && Arc::ptr_eq(&timer.owner.session, &owner.session)
                })
                .map(|(id, _)| *id)
                .min();
            if let Some(id) = id {
                if function == Function::DeleteTimer {
                    state.script_timers.player.remove(&id);
                } else {
                    let revision = state.script_timers.next_id()?;
                    let timer = state.script_timers.player.get_mut(&id).unwrap();
                    timer.due_at = if delay >= 0 {
                        timer.due_at.saturating_add(delay as u128)
                    } else {
                        timer.due_at.saturating_sub(u128::from(delay.unsigned_abs()))
                    };
                    timer.revision = revision;
                    timer.queued = false;
                }
            }
        }
        Ok(Value::default())
    }

    fn npc_present(state: &ServerState, npc_id: u32, scope_instance: u8) -> bool {
        state
            .map_instances()
            .values()
            .flatten()
            .any(|instance| instance.key().map_instance() == scope_instance && instance.get_script(npc_id).is_some())
    }

    pub(crate) fn tick_script_timers(&self, state: &mut ServerState, tick: u128) {
        let mut timers = std::mem::take(&mut state.script_timers);
        timers
            .player
            .retain(|_, timer| timer.owner.is_current(state) && (!timer.queued || tick < timer.event.queued_until));
        timers
            .npc
            .retain(|_, timer| timer.owner.as_ref().is_none_or(|owner| owner.is_current(state)));
        timers.attachments.retain(|_, owner| owner.is_current(state));
        if tick % NPC_PRESENCE_CHECK_INTERVAL_MS < 40 {
            timers.npc.retain(|key, _| Self::npc_present(state, key.npc_id, key.scope_instance));
            timers.attachments.retain(|(npc_id, scope), _| Self::npc_present(state, *npc_id, *scope));
        }
        let queued_until = tick.saturating_add(u128::from(self.configuration.scripting.conversation_timeout_secs.max(1)) * 2000);
        let mut player_ids = timers.player.keys().copied().collect::<Vec<_>>();
        player_ids.sort_unstable();
        for id in player_ids {
            let timer = timers.player.get_mut(&id).unwrap();
            if !timer.queued && tick >= timer.due_at {
                timer.queued = true;
                timer.event.queued_until = queued_until;
                timer.event.timer_guard = Some(ScriptTimerGuard::Player {
                    id,
                    revision: timer.revision,
                });
                self.add_to_next_tick(GameEvent::ScriptNpcEvent(timer.event.clone()));
            }
        }
        for (key, timer) in &mut timers.npc {
            if timer.started_at.is_none() {
                continue;
            }
            let elapsed = timer.elapsed(tick);
            while let Some((time, entry_id)) = timer.labels.get(timer.next_label).copied().filter(|(time, _)| *time <= elapsed) {
                timer.next_label += 1;
                self.add_to_next_tick(GameEvent::ScriptNpcEvent(ScriptNpcEvent {
                    npc_id: key.npc_id,
                    scope_instance: key.scope_instance,
                    entry_id,
                    char_id: key.char_id,
                    depth: 0,
                    queued_until,
                    args: None,
                    timer_guard: Some(ScriptTimerGuard::Npc {
                        key: *key,
                        revision: timer.revision,
                    }),
                }));
                if timer.owner.is_some() && timer.next_label == timer.labels.len() {
                    timer.elapsed_ms = time;
                    timer.started_at = None;
                }
            }
        }
        state.script_timers = timers;
    }
}
