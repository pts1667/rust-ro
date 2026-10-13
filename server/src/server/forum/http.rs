use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{ConnectInfo, Extension, Path, Query, Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::websocket;
use super::{Author, ErrorKind, Forum, ForumError, now};
use crate::server::bots::{ApiKey, require_key};

const SKILL: &str = include_str!("SKILL.md");
/// The character a request carrying the bot API key writes as.
const ACTING_AS: &str = "x-forum-as";

impl IntoResponse for ForumError {
    fn into_response(self) -> Response {
        let body = json!({ "error": self.message });
        match self.kind {
            ErrorKind::NotFound => (StatusCode::NOT_FOUND, Json(body)).into_response(),
            ErrorKind::Invalid => (StatusCode::BAD_REQUEST, Json(body)).into_response(),
            ErrorKind::Unauthorized => (StatusCode::UNAUTHORIZED, [(header::WWW_AUTHENTICATE, "Bearer")], Json(body)).into_response(),
            ErrorKind::RateLimited(wait) => (
                StatusCode::TOO_MANY_REQUESTS,
                [(header::RETRY_AFTER, wait.to_string())],
                Json(json!({ "error": self.message, "retry_after": wait })),
            )
                .into_response(),
            ErrorKind::Unavailable => (StatusCode::SERVICE_UNAVAILABLE, Json(body)).into_response(),
        }
    }
}

type ForumState = State<Arc<Forum>>;
type Guard = (Arc<Forum>, Arc<ApiKey>);
type Reply = Result<Json<Value>, ForumError>;

#[derive(Deserialize)]
struct ThreadsQuery {
    before: Option<u64>,
    limit: Option<usize>,
}

#[derive(Deserialize)]
struct AfterQuery {
    #[serde(default)]
    after: u64,
}

#[derive(Deserialize)]
struct SignIn {
    account: String,
    password: String,
    character: String,
}

#[derive(Deserialize)]
struct NewThreadRequest {
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct NewCommentRequest {
    body: String,
    parent: Option<u64>,
}

/// Runs blocking storage work off the async workers.
async fn off_thread<T: Send + 'static>(work: impl FnOnce() -> Result<T, ForumError> + Send + 'static) -> Result<T, ForumError> {
    tokio::task::spawn_blocking(work).await.unwrap_or_else(|error| Err(ForumError::unavailable(error.to_string())))
}

async fn run<T: Serialize + Send + 'static>(work: impl FnOnce() -> Result<T, ForumError> + Send + 'static) -> Reply {
    Ok(Json(serde_json::to_value(off_thread(work).await?).unwrap_or(Value::Null)))
}

fn bearer(headers: &HeaderMap) -> Option<&str> {
    headers.get(header::AUTHORIZATION)?.to_str().ok()?.strip_prefix("Bearer ").map(str::trim)
}

fn client_ipv4(address: IpAddr) -> Ipv4Addr {
    match address {
        IpAddr::V4(address) => address,
        IpAddr::V6(address) => address.to_ipv4_mapped().unwrap_or(Ipv4Addr::LOCALHOST),
    }
}

/// Lets through a request that carries a token of a sign-in, or the bot API key with the character it acts as. The handlers
/// then get that character as an `Author`.
async fn require_member(State((forum, key)): State<Guard>, mut request: Request, next: Next) -> Response {
    let headers = request.headers().clone();
    let author = if key.accepts(&headers) {
        match headers.get(ACTING_AS).and_then(|value| value.to_str().ok()).map(str::to_string) {
            Some(name) => off_thread(move || forum.act_as(&name)).await,
            None => Err(ForumError::invalid(format!("With the bot key, name the character to write as in the {ACTING_AS} header"))),
        }
    } else {
        bearer(&headers)
            .and_then(|token| forum.session(token, now()))
            .ok_or_else(|| ForumError::unauthorized("Sign in at /forum/auth and send the token as `Authorization: Bearer <token>`"))
    };
    match author {
        Ok(author) => {
            request.extensions_mut().insert(author);
            next.run(request).await
        }
        Err(error) => error.into_response(),
    }
}

async fn skill() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/markdown; charset=utf-8")], SKILL)
}

async fn sign_in(State(forum): ForumState, ConnectInfo(address): ConnectInfo<SocketAddr>, Json(request): Json<SignIn>) -> Reply {
    let client = client_ipv4(address.ip());
    run(move || forum.authenticate(&request.account, &request.password, &request.character, client, now())).await
}

async fn threads(State(forum): ForumState, Query(query): Query<ThreadsQuery>) -> Reply {
    run(move || forum.threads(query.before, query.limit)).await
}

async fn create_thread(State(forum): ForumState, Extension(author): Extension<Author>, Json(request): Json<NewThreadRequest>) -> Reply {
    run(move || forum.create_thread(&author, &request.title, &request.body)).await
}

async fn thread(State(forum): ForumState, Path(id): Path<u64>) -> Reply {
    run(move || forum.thread(id)).await
}

async fn comment(State(forum): ForumState, Extension(author): Extension<Author>, Path(thread_id): Path<u64>, Json(request): Json<NewCommentRequest>) -> Reply {
    run(move || forum.comment(&author, thread_id, &request.body, request.parent)).await
}

async fn socket(State(forum): ForumState, Extension(author): Extension<Author>, Query(query): Query<AfterQuery>, upgrade: WebSocketUpgrade) -> Response {
    upgrade.on_upgrade(move |socket| websocket::serve(socket, forum, author, query.after))
}

/// `/forum/SKILL.md` is open. `/forum/auth` takes the bot API key. Everything else takes a token, or the key and a name.
pub fn router(forum: Arc<Forum>, key: Arc<ApiKey>) -> Router {
    let guard: Guard = (forum.clone(), key.clone());
    let members = Router::new()
        .route("/forum/threads", get(threads).post(create_thread))
        .route("/forum/threads/{id}", get(thread))
        .route("/forum/threads/{id}/comments", post(comment))
        .route("/forum/ws", get(socket))
        .layer(middleware::from_fn_with_state(guard, require_member));
    let signing_in = Router::new().route("/forum/auth", post(sign_in)).layer(middleware::from_fn_with_state(key, require_key));
    Router::new().route("/forum/SKILL.md", get(skill)).merge(signing_in).merge(members).with_state(forum)
}

#[cfg(test)]
mod tests {
    use axum::body::{Body, to_bytes};
    use axum::http::{Method, Request};
    use configuration::account_config::LoginConfig;
    use database::CharacterCreation;
    use database::model::{AccountRecord, CharacterRecord};
    use tower::ServiceExt;

    use super::*;
    use crate::repository::{CharServerRepository, LoginRepository, SledRepository};

    const BOT_KEY: &str = "bot-key";

    fn key_file() -> String {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let number = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("forum-test-key-{}-{number}", std::process::id()));
        std::fs::write(&path, BOT_KEY).unwrap();
        path.to_string_lossy().into_owned()
    }

    /// A forum with the account `alice` (password `secret`) owning the character `Hero`.
    fn forum() -> Arc<Forum> {
        let repository = Arc::new(SledRepository::temporary().unwrap());
        let account_id = repository.account_create(AccountRecord::new(0, "alice", "secret")).unwrap();
        repository
            .char_create(&CharacterCreation {
                character: CharacterRecord { account_id: account_id as i32, name: "Hero".into(), inventory_slots: 100, ..Default::default() },
                items: Vec::new(),
                slot_limit: 12,
                case_sensitive_names: false,
            })
            .unwrap();
        Arc::new(Forum::new(repository, LoginConfig::default()))
    }

    fn app(forum: &Arc<Forum>) -> Router {
        let key = ApiKey::load_or_create(&key_file()).unwrap();
        router(forum.clone(), Arc::new(key))
    }

    /// Sends one request and gives its status and its JSON body.
    async fn send(app: &Router, method: Method, uri: &str, headers: &[(&str, &str)], body: Option<&str>) -> (StatusCode, Value) {
        let mut request = Request::builder().method(method).uri(uri);
        for (name, value) in headers {
            request = request.header(*name, *value);
        }
        if body.is_some() {
            request = request.header("content-type", "application/json");
        }
        let mut request = request.body(body.map_or(Body::empty(), |text| Body::from(text.to_string()))).unwrap();
        request.extensions_mut().insert(ConnectInfo(SocketAddr::from(([10, 0, 0, 7], 5000))));
        let response = app.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
    }

    async fn sign_in_as_hero(app: &Router) -> String {
        let (status, issued) = send(app, Method::POST, "/forum/auth", &[("x-api-key", BOT_KEY)], Some(r#"{"account":"alice","password":"secret","character":"hero"}"#)).await;
        assert_eq!(status, StatusCode::OK);
        issued["token"].as_str().unwrap().to_string()
    }

    #[tokio::test]
    async fn a_signed_in_player_writes_as_the_character_and_the_token_is_the_credential() {
        let forum = forum();
        let app = app(&forum);
        let token = sign_in_as_hero(&app).await;
        let bearer = format!("Bearer {token}");
        let (status, thread) = send(&app, Method::POST, "/forum/threads", &[("authorization", &bearer)], Some(r#"{"title":"Hi","body":"Post"}"#)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(thread["author"], "Hero");
        let id = thread["id"].as_u64().unwrap();
        let (status, read) = send(&app, Method::GET, &format!("/forum/threads/{id}"), &[("authorization", &bearer)], None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(read["title"], "Hi");
    }

    #[tokio::test]
    async fn the_bot_key_writes_as_the_character_it_names_and_must_name_one() {
        let forum = forum();
        let app = app(&forum);
        let (status, _) = send(&app, Method::POST, "/forum/threads", &[("x-api-key", BOT_KEY)], Some(r#"{"title":"Hi","body":"Post"}"#)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let (status, thread) = send(&app, Method::POST, "/forum/threads", &[("x-api-key", BOT_KEY), ("x-forum-as", "HERO")], Some(r#"{"title":"Hi","body":"Post"}"#)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(thread["author"], "Hero");
        let (status, _) = send(&app, Method::POST, "/forum/threads", &[("x-api-key", BOT_KEY), ("x-forum-as", "Nobody")], Some(r#"{"title":"Hi","body":"Post"}"#)).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn a_second_thread_of_the_same_character_waits_and_says_for_how_long() {
        let forum = forum();
        let app = app(&forum);
        let as_hero = [("x-api-key", BOT_KEY), ("x-forum-as", "Hero")];
        assert_eq!(send(&app, Method::POST, "/forum/threads", &as_hero, Some(r#"{"title":"One","body":"Post"}"#)).await.0, StatusCode::OK);
        let (status, refused) = send(&app, Method::POST, "/forum/threads", &as_hero, Some(r#"{"title":"Two","body":"Post"}"#)).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
        assert!(refused["retry_after"].as_u64().is_some_and(|wait| wait > 0 && wait <= 900));
    }

    #[tokio::test]
    async fn the_members_need_a_token_or_the_key_and_a_wrong_password_is_refused() {
        let forum = forum();
        let app = app(&forum);
        assert_eq!(send(&app, Method::GET, "/forum/threads", &[], None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(send(&app, Method::GET, "/forum/threads", &[("authorization", "Bearer not-a-token")], None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(send(&app, Method::GET, "/forum/threads", &[("x-api-key", BOT_KEY), ("x-forum-as", "Hero")], None).await.0, StatusCode::OK);
        let (status, _) = send(&app, Method::POST, "/forum/auth", &[("x-api-key", BOT_KEY)], Some(r#"{"account":"alice","password":"wrong","character":"Hero"}"#)).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(send(&app, Method::POST, "/forum/auth", &[], Some(r#"{"account":"alice","password":"secret","character":"Hero"}"#)).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(send(&app, Method::GET, "/forum/SKILL.md", &[], None).await.0, StatusCode::OK);
    }

    #[tokio::test]
    async fn a_missing_thread_is_404_and_a_parent_outside_it_is_400() {
        let forum = forum();
        let app = app(&forum);
        let as_hero = [("x-api-key", BOT_KEY), ("x-forum-as", "Hero")];
        assert_eq!(send(&app, Method::GET, "/forum/threads/999", &as_hero, None).await.0, StatusCode::NOT_FOUND);
        assert_eq!(send(&app, Method::POST, "/forum/threads/999/comments", &as_hero, Some(r#"{"body":"Hi"}"#)).await.0, StatusCode::NOT_FOUND);
    }
}
