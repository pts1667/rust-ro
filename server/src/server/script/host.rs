use std::sync::atomic::Ordering;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value};
use tokio::sync::{mpsc, oneshot};

use crate::server::Server;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::script::Script;
use crate::server::model::session::Session;

#[derive(Debug, Clone)]
pub enum PlayerInput {
    Next,
    Selection(u8),
    Number(i32),
    Text(String),
    DealType(u8),
    Purchases(Vec<(u32, i16)>),
    Sales(Vec<(usize, i16)>),
}

#[derive(Debug, Clone)]
pub struct ScriptRequest {
    pub char_id: u32,
    pub account_id: u32,
    pub npc_id: u32,
    pub npc_entry: u32,
    pub npc_scope_instance: u8,
    pub map_instance: u8,
    pub generation: u64,
    pub background: bool,
    pub event_depth: u8,
    pub timer_context: Option<crate::server::model::script_timer::NpcTimerKey>,
    pub logout_token: Option<u64>,
    pub request: Request,
    pub response: Arc<Mutex<Option<oneshot::Sender<Reply>>>>,
}

impl PartialEq for ScriptRequest {
    fn eq(&self, rhs: &Self) -> bool {
        self.char_id == rhs.char_id
            && self.account_id == rhs.account_id
            && self.npc_id == rhs.npc_id
            && self.npc_scope_instance == rhs.npc_scope_instance
            && self.generation == rhs.generation
            && self.timer_context == rhs.timer_context
            && self.logout_token == rhs.logout_token
            && self.request == rhs.request
    }
}

pub struct NpcScriptHost {
    pub server: Arc<Server>,
    pub session: Arc<Session>,
    pub script: Arc<Script>,
    pub inputs: mpsc::Receiver<PlayerInput>,
    pub notifications: SyncSender<Notification>,
    pub generation: u64,
    pub map_instance: u8,
    pub background: bool,
    pub event_depth: u8,
    pub event_arguments: Option<Vec<Value>>,
    pub timer_context: Option<crate::server::model::script_timer::NpcTimerKey>,
    pub logout_token: Option<u64>,
    /// The client shows a dialogue window the script has not closed yet.
    pub dialog_open: bool,
    pub error: Option<String>,
}

impl NpcScriptHost {
    pub fn current(&self) -> bool {
        self.background || self.session.script_generation.load(Ordering::Acquire) == self.generation
    }

    pub async fn forward(&self, request: Request) -> Reply {
        if !self.current() {
            return Err("Conversation cancelled".into());
        }
        let (sender, receiver) = oneshot::channel();
        self.server.add_to_next_tick(GameEvent::ScriptRequest(ScriptRequest {
            char_id: self.session.char_id(),
            account_id: self.session.account_id,
            npc_id: self.script.id,
            npc_entry: self.script.entry_id,
            npc_scope_instance: self.script.scope_instance,
            map_instance: self.map_instance,
            generation: self.generation,
            background: self.background,
            event_depth: self.event_depth,
            timer_context: self.timer_context,
            logout_token: self.logout_token,
            request,
            response: Arc::new(Mutex::new(Some(sender))),
        }));
        receiver.await.map_err(|_| "Game loop stopped".to_string())?
    }

    pub async fn receive(&mut self) -> Result<PlayerInput, String> {
        if !self.current() {
            return Err("Conversation cancelled".into());
        }
        self.inputs
            .recv()
            .await
            .ok_or_else(|| "Player disconnected or started another conversation".into())
    }
}

#[async_trait]
impl Host for NpcScriptHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        if !self.current() {
            return Err("Conversation cancelled".into());
        }
        if self.logout_token.is_some() {
            match &request {
                Request::Call { function: Function::Mes | Function::Close | Function::Message | Function::DispBottom | Function::Cutin, .. } => return Ok(Value::default()),
                Request::Call { function: Function::Shop | Function::Next | Function::Select | Function::InputNumber | Function::InputString, .. } => return Err("Logout callback cannot wait for client input".into()),
                Request::Purchase(_) | Request::Sale(_) => return Err("Logout callback cannot process client shop input".into()),
                _ => {},
            }
        }
        if self.session.char_id == Some(0) && matches!(&request, Request::Call { function: Function::Shop | Function::Mes | Function::Close | Function::Next | Function::Select | Function::InputNumber | Function::InputString | Function::Message | Function::DispBottom | Function::Cutin, .. }) {
            return Err("NPC interaction requires an attached player".into());
        }
        match request {
            Request::Constant(name) => super::item_script_handler::constant(&name),
            Request::Arguments => Ok(Value::Array(self.event_arguments.as_ref().unwrap_or(&self.script.constructor_args).clone())),
            Request::Call {
                function: Function::Shop, ..
            } => self.shop().await,
            Request::Call { function, arguments }
                if crate::server::service::npc_timer_service::handles(function) =>
            {
                let changes_attachment = match function {
                    Function::AttachNpcTimer | Function::DetachNpcTimer => true,
                    Function::InitNpcTimer | Function::StartNpcTimer | Function::StopNpcTimer => {
                        arguments.last().is_some_and(|flag| flag.number_value().is_ok_and(|flag| flag != 0))
                    }
                    _ => false,
                };
                let named_target = if function == Function::AttachNpcTimer { None } else {
                    arguments.first().filter(|value| value.is_string())
                };
                let current_npc = named_target.is_none_or(|name| name.string_value().is_ok_and(|name| name == &self.script.name));
                let reply = self.forward(Request::Call { function, arguments }).await;
                if reply.is_ok() && changes_attachment && current_npc {
                    self.timer_context = match function {
                        Function::AttachNpcTimer => None,
                        Function::DetachNpcTimer | Function::StopNpcTimer => Some(crate::server::model::script_timer::NpcTimerKey {
                            npc_id: self.script.id, scope_instance: self.script.scope_instance, char_id: None,
                        }),
                        _ => Some(crate::server::model::script_timer::NpcTimerKey {
                            npc_id: self.script.id, scope_instance: self.script.scope_instance, char_id: self.session.char_id,
                        }),
                    };
                }
                reply
            }
            Request::Call { function, arguments }
                if matches!(
                    function,
                    Function::Mes
                        | Function::Close
                        | Function::Next
                        | Function::Select
                        | Function::InputNumber
                        | Function::InputString
                        | Function::Message
                        | Function::DispBottom
                        | Function::Cutin
                ) =>
            {
                self.interaction(function, arguments).await
            }
            Request::ReportError(error) => {
                self.error = Some(error);
                Ok(Value::default())
            }
            request => self.forward(request).await,
        }
    }
}
