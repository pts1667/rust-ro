//! HTTP and WebSocket API that lets external programs, LLM agents in particular, drive characters ("bots").
//!
//! A bot is an ordinary character of its own account, but nothing connects for it: the API puts the character in the game, runs its
//! commands as game loop events, and reads the packets the server addresses to it (NPC dialogues) from the notification thread.
//! `SKILL.md` documents the API for the agents that use it.
mod auth;
mod command;
mod controller;
mod dialog;
mod http;
mod lifecycle;
mod observation;
mod registry;
mod websocket;

use std::sync::Arc;
use std::time::Duration;

pub use command::BotCommand;
pub use registry::BotRegistry;

use crate::server::model::events::game_event::{CharacterLoadedFromClientSide, GameEvent};
use crate::server::Server;

/// A client answers the map change packet once its map is loaded; a bot has no client to do it.
const MAP_CHANGE_PACKET_ID: [u8; 2] = 0x0091_u16.to_le_bytes();
const CLIENT_LOAD_DELAY_MS: u128 = 200;
const SHUTDOWN_POLL: Duration = Duration::from_millis(500);
const API_THREADS: usize = 2;

/// Serves the API until the server stops. Runs on its own runtime, so a slow client can never starve the game.
pub fn serve(server: Arc<Server>) {
    let config = &server.configuration.bots;
    let key = match auth::ApiKey::load_or_create(&config.api_key_path) {
        Ok(key) => Arc::new(key),
        Err(error) => {
            error!("The bot API is disabled, its key file {} is unusable: {error}", config.api_key_path);
            return;
        }
    };
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(API_THREADS)
        .thread_name("bots_api")
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            error!("The bot API is disabled, its runtime failed to start: {error}");
            return;
        }
    };
    runtime.block_on(async {
        let listener = match tokio::net::TcpListener::bind((config.host.as_str(), config.port)).await {
            Ok(listener) => listener,
            Err(error) => {
                error!("The bot API can not listen on {}:{}: {error}", config.host, config.port);
                return;
            }
        };
        info!("Bot API listens on http://{}:{}/bots, its skill is at /bots/SKILL.md", config.host, config.port);
        let app = http::router(Arc::new(controller::BotController::new(server.clone())), key);
        let stopped = {
            let server = server.clone();
            async move {
                while server.is_alive() {
                    tokio::time::sleep(SHUTDOWN_POLL).await;
                }
            }
        };
        // No graceful shutdown: an open WebSocket would keep the server from stopping
        tokio::select! {
            result = axum::serve(listener, app) => {
                if let Err(error) = result {
                    error!("The bot API stopped: {error}");
                }
            }
            _ = stopped => {}
        }
    });
    runtime.shutdown_background();
}

/// Reads what the server sent to a bot, and plays the part of its client for what needs an answer.
pub fn receive_packet(server: &Server, char_id: u32, packet: &[u8]) {
    server.bots().observe_packet(char_id, packet);
    if packet.starts_with(&MAP_CHANGE_PACKET_ID) {
        server.add_to_delayed_tick(
            GameEvent::CharacterLoadedFromClientSide(CharacterLoadedFromClientSide { char_id }),
            CLIENT_LOAD_DELAY_MS,
        );
    }
}
