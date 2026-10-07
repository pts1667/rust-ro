use std::path::Path;
use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use rand::RngCore;

const KEY_BYTES: usize = 32;
const KEY_HEADER: &str = "x-api-key";
const BEARER: &str = "Bearer ";

/// The secret that opens the bot API.
pub struct ApiKey(String);

impl ApiKey {
    /// Reads the key file, or writes a new random key to it when it does not exist yet.
    pub fn load_or_create(path: &str) -> std::io::Result<Self> {
        match std::fs::read_to_string(path) {
            Ok(content) if !content.trim().is_empty() => return Ok(Self(content.trim().to_string())),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        let mut bytes = [0u8; KEY_BYTES];
        rand::rngs::OsRng.fill_bytes(&mut bytes);
        let key: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        if let Some(directory) = Path::new(path).parent() {
            std::fs::create_dir_all(directory)?;
        }
        std::fs::write(path, &key)?;
        info!("Created the bot API key in {path}");
        Ok(Self(key))
    }

    /// Compares without stopping at the first difference, so that the time taken tells nothing about the key.
    fn matches(&self, candidate: &str) -> bool {
        let (key, candidate) = (self.0.as_bytes(), candidate.as_bytes());
        key.len() == candidate.len() && key.iter().zip(candidate).fold(0u8, |difference, (left, right)| difference | (left ^ right)) == 0
    }

    fn accepts(&self, headers: &HeaderMap) -> bool {
        let bearer = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix(BEARER));
        let custom = headers.get(KEY_HEADER).and_then(|value| value.to_str().ok());
        bearer.or(custom).is_some_and(|candidate| self.matches(candidate.trim()))
    }
}

pub async fn require_key(State(key): State<Arc<ApiKey>>, request: Request, next: Next) -> Response {
    if key.accepts(request.headers()) {
        return next.run(request).await;
    }
    (StatusCode::UNAUTHORIZED, [(header::WWW_AUTHENTICATE, "Bearer")], "Send the API key as `Authorization: Bearer <key>` or `X-API-Key: <key>`\n")
        .into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    fn headers(name: &'static str, value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(name, HeaderValue::from_str(value).unwrap());
        headers
    }

    #[test]
    fn the_key_is_accepted_as_bearer_token_or_custom_header_only() {
        let key = ApiKey("secret".into());
        assert!(key.accepts(&headers("authorization", "Bearer secret")));
        assert!(key.accepts(&headers("x-api-key", "secret")));
        assert!(!key.accepts(&headers("authorization", "Bearer secre")));
        assert!(!key.accepts(&headers("authorization", "secret")));
        assert!(!key.accepts(&HeaderMap::new()));
    }
}
