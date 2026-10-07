use std::sync::Arc;

use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, header};
use axum::middleware;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};

use super::auth::{ApiKey, require_key};
use super::controller::{Action, BotController, BotError, CreateBot, ErrorKind};
use super::websocket;

const SKILL: &str = include_str!("SKILL.md");

impl IntoResponse for BotError {
    fn into_response(self) -> Response {
        let status = match self.kind {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Invalid => StatusCode::BAD_REQUEST,
            ErrorKind::Conflict => StatusCode::CONFLICT,
            ErrorKind::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        };
        (status, Json(json!({ "error": self.message }))).into_response()
    }
}

type Controller = State<Arc<BotController>>;
type Reply = Result<Json<Value>, BotError>;

#[derive(Deserialize)]
struct MapQuery {
    x: Option<u16>,
    y: Option<u16>,
    radius: Option<u16>,
}

async fn skill() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/markdown; charset=utf-8")], SKILL)
}

async fn list(State(controller): Controller) -> Json<Value> {
    Json(controller.list())
}

async fn create(State(controller): Controller, Json(request): Json<CreateBot>) -> Reply {
    let (new_bot, connect) = request.into_parts();
    controller.create(new_bot, connect).await.map(Json)
}

async fn show(State(controller): Controller, Path(name): Path<String>) -> Reply {
    controller.status(&name).await.map(Json)
}

async fn connect(State(controller): Controller, Path(name): Path<String>) -> Reply {
    controller.connect(&name).await.map(Json)
}

async fn disconnect(State(controller): Controller, Path(name): Path<String>) -> Reply {
    controller.disconnect(&name).await.map(Json)
}

async fn observation(State(controller): Controller, Path(name): Path<String>) -> Reply {
    controller.observe(&name).await.map(Json)
}

async fn map(State(controller): Controller, Path(name): Path<String>, Query(query): Query<MapQuery>) -> Reply {
    let center = match (query.x, query.y) {
        (Some(x), Some(y)) => Some((x, y, query.radius.unwrap_or(20))),
        (None, None) => None,
        _ => return Err(BotError { kind: ErrorKind::Invalid, message: "Give both x and y, or neither".into() }),
    };
    controller.map(&name, center).await.map(Json)
}

async fn action(State(controller): Controller, Path(name): Path<String>, Json(action): Json<Action>) -> Reply {
    controller.act(&name, action).await.map(Json)
}

async fn socket(State(controller): Controller, upgrade: WebSocketUpgrade) -> Response {
    upgrade.on_upgrade(move |socket| websocket::serve(socket, controller))
}

/// Everything below `/bots` needs the API key, except the skill that explains how to use it.
pub fn router(controller: Arc<BotController>, key: Arc<ApiKey>) -> Router {
    let protected = Router::new()
        .route("/bots", get(list).post(create))
        .route("/bots/ws", get(socket))
        .route("/bots/{name}", get(show))
        .route("/bots/{name}/connect", post(connect))
        .route("/bots/{name}/disconnect", post(disconnect))
        .route("/bots/{name}/observation", get(observation))
        .route("/bots/{name}/map", get(map))
        .route("/bots/{name}/actions", post(action))
        .layer(middleware::from_fn_with_state(key, require_key));
    Router::new().route("/bots/SKILL.md", get(skill)).merge(protected).with_state(controller)
}
