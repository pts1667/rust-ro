//! The custom packets that carry plugin UI on the map connection. They continue the custom id range of the login
//! packets (`docs/custom-login-packets.md`); the stock client knows none of them, so the server only sends the
//! `ZC_*` ones to a session that announced [`FEATURE_PLUGIN_UI`]. Variable packets are `id, total length (u16), body`,
//! little endian, with JSON bodies.

use serde::{Deserialize, Serialize};

use crate::{Overhead, UiEvent, Window};

/// Client to map, 6 bytes: `id, flags (u32)`. Sent once after entering the map.
pub const CZ_CLIENT_FEATURES: u16 = 0xC3E4;
/// Map to client, variable: `id, length, op (u8), JSON`. Op 0 opens or replaces ([`WindowOpen`]), 1 closes
/// ([`WindowClose`]).
pub const ZC_PLUGIN_WINDOW: u16 = 0xC3E5;
/// Client to map, variable: `id, length, JSON` ([`EventMessage`]).
pub const CZ_PLUGIN_UI_EVENT: u16 = 0xC3E6;
/// Map to client, variable: `id, length, unit id (u32), JSON` ([`OverheadMessage`]).
pub const ZC_PLUGIN_OVERHEAD: u16 = 0xC3E7;

pub const FEATURE_PLUGIN_UI: u32 = 1;

pub const CLIENT_FEATURES_LENGTH: usize = 6;
const HEADER: usize = 4;
const MAX_PACKET: usize = u16::MAX as usize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowOpen {
    pub plugin: String,
    pub window: Window,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowClose {
    pub plugin: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    pub plugin: String,
    pub event: UiEvent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverheadMessage {
    pub plugin: String,
    pub overhead: Overhead,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerMessage {
    Open(WindowOpen),
    Close(WindowClose),
    Overhead { unit_id: u32, message: OverheadMessage },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientMessage {
    Features(u32),
    Event(EventMessage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameLength {
    Fixed(usize),
    /// The length is the u16 after the id.
    Variable,
}

/// The framing of the packets the client sends.
pub fn client_frame_length(id: u16) -> Option<FrameLength> {
    match id {
        CZ_CLIENT_FEATURES => Some(FrameLength::Fixed(CLIENT_FEATURES_LENGTH)),
        CZ_PLUGIN_UI_EVENT => Some(FrameLength::Variable),
        _ => None,
    }
}

/// The framing of the packets the server sends.
pub fn server_frame_length(id: u16) -> Option<FrameLength> {
    match id {
        ZC_PLUGIN_WINDOW | ZC_PLUGIN_OVERHEAD => Some(FrameLength::Variable),
        _ => None,
    }
}

pub fn encode_features(flags: u32) -> Vec<u8> {
    let mut raw = CZ_CLIENT_FEATURES.to_le_bytes().to_vec();
    raw.extend_from_slice(&flags.to_le_bytes());
    raw
}

pub fn encode_open(message: &WindowOpen) -> Result<Vec<u8>, String> {
    variable(ZC_PLUGIN_WINDOW, &[0], message)
}

pub fn encode_close(message: &WindowClose) -> Result<Vec<u8>, String> {
    variable(ZC_PLUGIN_WINDOW, &[1], message)
}

pub fn encode_event(message: &EventMessage) -> Result<Vec<u8>, String> {
    variable(CZ_PLUGIN_UI_EVENT, &[], message)
}

pub fn encode_overhead(unit_id: u32, message: &OverheadMessage) -> Result<Vec<u8>, String> {
    variable(ZC_PLUGIN_OVERHEAD, &unit_id.to_le_bytes(), message)
}

fn variable(id: u16, prefix: &[u8], body: &impl Serialize) -> Result<Vec<u8>, String> {
    let json = serde_json::to_vec(body).map_err(|error| error.to_string())?;
    let length = HEADER + prefix.len() + json.len();
    if length > MAX_PACKET {
        return Err(format!("packet 0x{id:04X} would be {length} bytes"));
    }
    let mut raw = Vec::with_capacity(length);
    raw.extend_from_slice(&id.to_le_bytes());
    raw.extend_from_slice(&(length as u16).to_le_bytes());
    raw.extend_from_slice(prefix);
    raw.extend_from_slice(&json);
    Ok(raw)
}

fn split(raw: &[u8]) -> Result<(u16, &[u8]), String> {
    if raw.len() < 2 {
        return Err("packet shorter than its id".into());
    }
    let id = u16::from_le_bytes([raw[0], raw[1]]);
    if id == CZ_CLIENT_FEATURES {
        return Ok((id, &raw[2..]));
    }
    if raw.len() < HEADER {
        return Err(format!("packet 0x{id:04X} shorter than its header"));
    }
    let length = u16::from_le_bytes([raw[2], raw[3]]) as usize;
    if length != raw.len() {
        return Err(format!("packet 0x{id:04X} says {length} bytes, has {}", raw.len()));
    }
    Ok((id, &raw[HEADER..]))
}

fn json<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|error| error.to_string())
}

pub fn decode_client(raw: &[u8]) -> Result<ClientMessage, String> {
    match split(raw)? {
        (CZ_CLIENT_FEATURES, body) if body.len() == 4 => Ok(ClientMessage::Features(u32::from_le_bytes([body[0], body[1], body[2], body[3]]))),
        (CZ_PLUGIN_UI_EVENT, body) => Ok(ClientMessage::Event(json(body)?)),
        (id, _) => Err(format!("0x{id:04X} is not a plugin UI client packet")),
    }
}

pub fn decode_server(raw: &[u8]) -> Result<ServerMessage, String> {
    match split(raw)? {
        (ZC_PLUGIN_WINDOW, [0, body @ ..]) => Ok(ServerMessage::Open(json(body)?)),
        (ZC_PLUGIN_WINDOW, [1, body @ ..]) => Ok(ServerMessage::Close(json(body)?)),
        (ZC_PLUGIN_OVERHEAD, [a, b, c, d, body @ ..]) => Ok(ServerMessage::Overhead { unit_id: u32::from_le_bytes([*a, *b, *c, *d]), message: json(body)? }),
        (id, _) => Err(format!("0x{id:04X} is not a plugin UI server packet")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Node, Overhead};

    #[test]
    fn every_packet_decodes_to_what_was_encoded() {
        let open = WindowOpen { plugin: "p".into(), window: Window::new("w", "W", 120, Node::text("hi")) };
        assert_eq!(decode_server(&encode_open(&open).unwrap()), Ok(ServerMessage::Open(open)));
        let close = WindowClose { plugin: "p".into(), id: "w".into() };
        assert_eq!(decode_server(&encode_close(&close).unwrap()), Ok(ServerMessage::Close(close)));
        let overhead = OverheadMessage { plugin: "p".into(), overhead: Overhead::text("up", None, 500) };
        assert_eq!(decode_server(&encode_overhead(150_000, &overhead).unwrap()), Ok(ServerMessage::Overhead { unit_id: 150_000, message: overhead }));
        let event = EventMessage { plugin: "p".into(), event: UiEvent::click("w", "b") };
        assert_eq!(decode_client(&encode_event(&event).unwrap()), Ok(ClientMessage::Event(event)));
        assert_eq!(decode_client(&encode_features(FEATURE_PLUGIN_UI)), Ok(ClientMessage::Features(1)));
    }

    #[test]
    fn lengths_match_the_framing() {
        assert_eq!(encode_features(0).len(), CLIENT_FEATURES_LENGTH);
        let raw = encode_event(&EventMessage { plugin: "p".into(), event: UiEvent::closed("w") }).unwrap();
        assert_eq!(u16::from_le_bytes([raw[2], raw[3]]) as usize, raw.len());
        assert_eq!(client_frame_length(CZ_PLUGIN_UI_EVENT), Some(FrameLength::Variable));
        assert_eq!(server_frame_length(ZC_PLUGIN_OVERHEAD), Some(FrameLength::Variable));
        assert_eq!(client_frame_length(ZC_PLUGIN_WINDOW), None);
    }

    #[test]
    fn truncated_packets_are_errors() {
        let raw = encode_event(&EventMessage { plugin: "p".into(), event: UiEvent::closed("w") }).unwrap();
        assert!(decode_client(&raw[..raw.len() - 1]).is_err());
        assert!(decode_client(&[0xE4]).is_err());
    }
}
