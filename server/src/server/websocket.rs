use std::io::{self, ErrorKind};
use std::time::Duration;

use base64::Engine;
use sha1::{Digest, Sha1};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const HANDSHAKE_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_HANDSHAKE_BYTES: usize = 8192;
const MAX_FRAME_PAYLOAD: u64 = 64 * 1024;
const MAX_CONTROL_PAYLOAD: u64 = 125;

pub const OPCODE_CONTINUATION: u8 = 0x0;
pub const OPCODE_TEXT: u8 = 0x1;
pub const OPCODE_BINARY: u8 = 0x2;
pub const OPCODE_CLOSE: u8 = 0x8;
pub const OPCODE_PING: u8 = 0x9;
pub const OPCODE_PONG: u8 = 0xA;

const CLOSE_PROTOCOL_ERROR: u16 = 1002;
const CLOSE_UNSUPPORTED_DATA: u16 = 1003;
const CLOSE_MESSAGE_TOO_BIG: u16 = 1009;

const BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
const UPGRADE_REQUIRED: &str = "HTTP/1.1 426 Upgrade Required\r\nSec-WebSocket-Version: 13\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";

pub fn accept_key(client_key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(client_key.trim().as_bytes());
    hasher.update(HANDSHAKE_GUID.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
}

/// Server frames are never masked, and each frame is a single unfragmented message.
pub fn encode_frame(opcode: u8, payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(payload.len() + 10);
    frame.push(0x80 | opcode);
    match payload.len() {
        length if length < 126 => frame.push(length as u8),
        length if length <= usize::from(u16::MAX) => {
            frame.push(126);
            frame.extend_from_slice(&(length as u16).to_be_bytes());
        }
        length => {
            frame.push(127);
            frame.extend_from_slice(&(length as u64).to_be_bytes());
        }
    }
    frame.extend_from_slice(payload);
    frame
}

/// How a connection opened, with the bytes read past that opening.
#[derive(Debug, PartialEq, Eq)]
pub enum Opening {
    Raw(Vec<u8>),
    WebSocket(Vec<u8>),
}

/// RO clients send a packet id first, never `GET `, so the first bytes are enough to tell the protocols apart. A client that
/// sends nothing for a while is a raw one.
pub async fn open<S: AsyncRead + AsyncWrite + Unpin>(stream: &mut S) -> io::Result<Opening> {
    const METHOD: &[u8] = b"GET ";
    let mut head = Vec::new();
    let mut chunk = [0u8; 1024];
    let upgrade = tokio::time::timeout(HANDSHAKE_TIMEOUT, async {
        loop {
            let known = head.len().min(METHOD.len());
            if !METHOD.starts_with(&head[..known]) {
                return Ok::<_, io::Error>(false);
            }
            if known == METHOD.len() {
                return Ok(true);
            }
            let count = stream.read(&mut chunk).await?;
            if count == 0 {
                return Ok(false);
            }
            head.extend_from_slice(&chunk[..count]);
        }
    })
    .await
    .unwrap_or(Ok(false))?;
    if !upgrade {
        return Ok(Opening::Raw(head));
    }
    let end = tokio::time::timeout(HANDSHAKE_TIMEOUT, async {
        loop {
            if let Some(end) = head.windows(4).position(|window| window == b"\r\n\r\n") {
                return Ok(end + 4);
            }
            if head.len() >= MAX_HANDSHAKE_BYTES {
                return Err(io::Error::new(ErrorKind::InvalidData, "websocket handshake is too large"));
            }
            let count = stream.read(&mut chunk).await?;
            if count == 0 {
                return Err(ErrorKind::UnexpectedEof.into());
            }
            head.extend_from_slice(&chunk[..count]);
        }
    })
    .await
    .map_err(|_| io::Error::from(ErrorKind::TimedOut))??;
    let after_head = head.split_off(end);
    let request = String::from_utf8(head).map_err(|_| io::Error::new(ErrorKind::InvalidData, "websocket handshake is not UTF-8"))?;
    match negotiate(&request) {
        Ok(response) => {
            stream.write_all(response.as_bytes()).await?;
            Ok(Opening::WebSocket(after_head))
        }
        Err(rejection) => {
            stream.write_all(rejection.as_bytes()).await?;
            Err(io::Error::new(ErrorKind::InvalidData, "invalid websocket handshake"))
        }
    }
}

/// Builds the `101` response, or the HTTP error to send instead.
fn negotiate(request: &str) -> Result<String, &'static str> {
    let mut lines = request.split("\r\n");
    if !lines.next().unwrap_or_default().starts_with("GET ") {
        return Err(BAD_REQUEST);
    }
    let headers: Vec<(String, &str)> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim()))
        .collect();
    let header = |name: &str| headers.iter().find(|(header, _)| header == name).map(|(_, value)| *value);
    let lists = |name: &str, token: &str| {
        header(name).is_some_and(|value| value.split(',').any(|candidate| candidate.trim().eq_ignore_ascii_case(token)))
    };
    if !lists("upgrade", "websocket") || !lists("connection", "upgrade") {
        return Err(BAD_REQUEST);
    }
    if header("sec-websocket-version") != Some("13") {
        return Err(UPGRADE_REQUIRED);
    }
    let key = header("sec-websocket-key").filter(|key| !key.is_empty()).ok_or(BAD_REQUEST)?;
    // Browsers drop the connection unless the server selects one of the sub-protocols they offered.
    let protocol = header("sec-websocket-protocol").and_then(|offered| {
        let offered: Vec<&str> = offered.split(',').map(str::trim).filter(|protocol| !protocol.is_empty()).collect();
        offered.iter().find(|protocol| protocol.eq_ignore_ascii_case("binary")).or(offered.first()).copied()
    });
    let mut response = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n",
        accept_key(key)
    );
    if let Some(protocol) = protocol {
        response.push_str(&format!("Sec-WebSocket-Protocol: {protocol}\r\n"));
    }
    response.push_str("\r\n");
    Ok(response)
}

/// What a run of incoming bytes held: the payloads of binary frames, as the same byte stream a raw TCP client sends, and the
/// control frames to send back.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Decoded {
    pub payload: Vec<u8>,
    pub replies: Vec<(u8, Vec<u8>)>,
    pub closed: bool,
}

/// A frame the protocol forbids, with the close status to answer it with.
#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    pub code: u16,
    pub reason: &'static str,
}

impl Violation {
    fn new(code: u16, reason: &'static str) -> Self {
        Self { code, reason }
    }
}

/// Reads frames out of the bytes of a WebSocket connection, keeping a frame that is not whole yet for the next bytes.
#[derive(Default)]
pub struct WebSocketDecoder {
    pending: Vec<u8>,
    inside_fragmented_message: bool,
}

impl WebSocketDecoder {
    pub fn push(&mut self, bytes: &[u8]) -> Result<Decoded, Violation> {
        self.pending.extend_from_slice(bytes);
        let mut decoded = Decoded::default();
        let mut consumed = 0;
        while !decoded.closed {
            match self.next_frame(consumed, &mut decoded)? {
                Some(length) => consumed += length,
                None => break,
            }
        }
        self.pending.drain(..consumed);
        Ok(decoded)
    }

    /// Handles the frame at `start`, or returns `None` while it is not whole.
    fn next_frame(&mut self, start: usize, decoded: &mut Decoded) -> Result<Option<usize>, Violation> {
        let buffer = &self.pending[start..];
        if buffer.len() < 2 {
            return Ok(None);
        }
        let fin = buffer[0] & 0x80 != 0;
        let reserved = buffer[0] & 0x70;
        let opcode = buffer[0] & 0x0F;
        let masked = buffer[1] & 0x80 != 0;
        let (length, header) = match buffer[1] & 0x7F {
            126 if buffer.len() < 4 => return Ok(None),
            126 => (u64::from(u16::from_be_bytes([buffer[2], buffer[3]])), 4),
            127 if buffer.len() < 10 => return Ok(None),
            127 => (u64::from_be_bytes(buffer[2..10].try_into().unwrap()), 10),
            short => (u64::from(short), 2),
        };
        let control = opcode & 0x8 != 0;
        if reserved != 0 || !masked {
            return Err(Violation::new(CLOSE_PROTOCOL_ERROR, "websocket frame is unmasked or uses extensions"));
        }
        if control && (!fin || length > MAX_CONTROL_PAYLOAD) {
            return Err(Violation::new(CLOSE_PROTOCOL_ERROR, "invalid websocket control frame"));
        }
        if length > MAX_FRAME_PAYLOAD {
            return Err(Violation::new(CLOSE_MESSAGE_TOO_BIG, "websocket frame is too large"));
        }
        let total = header + 4 + length as usize;
        if buffer.len() < total {
            return Ok(None);
        }
        let mask = [buffer[header], buffer[header + 1], buffer[header + 2], buffer[header + 3]];
        let payload: Vec<u8> = buffer[header + 4..total].iter().enumerate().map(|(index, byte)| byte ^ mask[index % 4]).collect();
        match opcode {
            OPCODE_BINARY | OPCODE_CONTINUATION => {
                if (opcode == OPCODE_CONTINUATION) != self.inside_fragmented_message {
                    return Err(Violation::new(CLOSE_PROTOCOL_ERROR, "unexpected websocket fragment order"));
                }
                self.inside_fragmented_message = !fin;
                decoded.payload.extend_from_slice(&payload);
            }
            OPCODE_TEXT => return Err(Violation::new(CLOSE_UNSUPPORTED_DATA, "websocket text frames are not supported")),
            OPCODE_PING => decoded.replies.push((OPCODE_PONG, payload)),
            OPCODE_PONG => {}
            OPCODE_CLOSE => {
                decoded.replies.push((OPCODE_CLOSE, payload));
                decoded.closed = true;
            }
            _ => return Err(Violation::new(CLOSE_PROTOCOL_ERROR, "unknown websocket opcode")),
        }
        Ok(Some(total))
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};

    use super::*;

    const MASK: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
    const UPGRADE_REQUEST: &str = "GET / HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\nConnection: keep-alive, Upgrade\r\n\
                                   Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\
                                   Sec-WebSocket-Protocol: chat, binary\r\n\r\n";

    fn client_frame(opcode: u8, fin: bool, payload: &[u8]) -> Vec<u8> {
        let mut frame = vec![if fin { 0x80 } else { 0 } | opcode, 0x80 | payload.len() as u8];
        frame.extend_from_slice(&MASK);
        frame.extend(payload.iter().enumerate().map(|(index, byte)| byte ^ MASK[index % 4]));
        frame
    }

    async fn open_with(first_bytes: &[u8]) -> (DuplexStream, io::Result<Opening>) {
        let (mut client, mut server) = tokio::io::duplex(16 * 1024);
        client.write_all(first_bytes).await.unwrap();
        let opening = open(&mut server).await;
        (client, opening)
    }

    async fn read_head(client: &mut DuplexStream) -> String {
        let mut head = Vec::new();
        while !head.ends_with(b"\r\n\r\n") {
            head.push(client.read_u8().await.unwrap());
        }
        String::from_utf8(head).unwrap()
    }

    #[test]
    fn accept_key_matches_the_rfc_example() {
        assert_eq!(accept_key("dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }

    #[test]
    fn frames_use_the_shortest_length_encoding() {
        assert_eq!(&encode_frame(OPCODE_BINARY, &[7; 5])[..2], &[0x82, 5]);
        assert_eq!(&encode_frame(OPCODE_BINARY, &[0; 300])[..4], &[0x82, 126, 0x01, 0x2C]);
        let large = encode_frame(OPCODE_BINARY, &vec![0; 70_000]);
        assert_eq!(&large[..10], &[0x82, 127, 0, 0, 0, 0, 0, 1, 0x11, 0x70]);
        assert_eq!(large.len(), 70_010);
    }

    #[test]
    fn handshake_selects_binary_protocol_and_rejects_bad_requests() {
        let response = negotiate(UPGRADE_REQUEST).unwrap();
        assert!(response.starts_with("HTTP/1.1 101 Switching Protocols\r\n"));
        assert!(response.contains("Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n"));
        assert!(response.contains("Sec-WebSocket-Protocol: binary\r\n"));
        assert_eq!(negotiate("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"), Err(BAD_REQUEST));
        assert_eq!(negotiate(&UPGRADE_REQUEST.replace("Version: 13", "Version: 8")), Err(UPGRADE_REQUIRED));
        assert_eq!(negotiate(&UPGRADE_REQUEST.replace("Sec-WebSocket-Key", "X-Other")), Err(BAD_REQUEST));
    }

    #[test]
    fn fragmented_binary_frames_become_one_byte_stream_even_when_split_across_reads() {
        let mut bytes = client_frame(OPCODE_BINARY, false, &[1, 2]);
        bytes.extend(client_frame(OPCODE_CONTINUATION, true, &[3, 4, 5]));
        let mut decoder = WebSocketDecoder::default();
        let first = decoder.push(&bytes[..5]).unwrap();
        assert!(first.payload.is_empty(), "a frame is only read once whole");
        assert_eq!(decoder.push(&bytes[5..]).unwrap().payload, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn ping_is_answered_with_a_pong_carrying_the_same_payload() {
        let mut bytes = client_frame(OPCODE_PING, true, b"hi");
        bytes.extend(client_frame(OPCODE_BINARY, true, &[6]));
        let decoded = WebSocketDecoder::default().push(&bytes).unwrap();
        assert_eq!(decoded.payload, [6]);
        assert_eq!(decoded.replies, [(OPCODE_PONG, b"hi".to_vec())]);
    }

    #[test]
    fn close_frame_ends_the_stream_and_is_echoed() {
        let mut bytes = client_frame(OPCODE_CLOSE, true, &1000u16.to_be_bytes());
        bytes.extend(client_frame(OPCODE_BINARY, true, &[6]));
        let decoded = WebSocketDecoder::default().push(&bytes).unwrap();
        assert!(decoded.closed);
        assert!(decoded.payload.is_empty(), "nothing after the close is read");
        assert_eq!(decoded.replies, [(OPCODE_CLOSE, 1000u16.to_be_bytes().to_vec())]);
    }

    #[test]
    fn protocol_violations_carry_their_close_status() {
        let code = |bytes: &[u8]| WebSocketDecoder::default().push(bytes).unwrap_err().code;
        assert_eq!(code(&[0x82, 0x01, 0x05]), 1002);
        assert_eq!(code(&client_frame(OPCODE_TEXT, true, b"text")), 1003);
        assert_eq!(code(&client_frame(OPCODE_CONTINUATION, true, &[1])), 1002);
        assert_eq!(code(&[0x82, 0xFF, 0, 0, 0, 0, 0, 0x10, 0, 0]), 1009, "refused from its header, before the payload arrives");
    }

    #[tokio::test]
    async fn an_upgrade_keeps_the_bytes_sent_after_the_request() {
        let mut first_bytes = UPGRADE_REQUEST.as_bytes().to_vec();
        first_bytes.extend(client_frame(OPCODE_BINARY, true, &[9]));
        let (mut client, opening) = open_with(&first_bytes).await;
        assert_eq!(opening.unwrap(), Opening::WebSocket(client_frame(OPCODE_BINARY, true, &[9])));
        assert!(read_head(&mut client).await.starts_with("HTTP/1.1 101"));
    }

    #[tokio::test]
    async fn plain_http_request_is_refused() {
        let (mut client, opening) = open_with(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
        assert!(opening.is_err());
        assert!(read_head(&mut client).await.starts_with("HTTP/1.1 400"));
    }

    #[tokio::test]
    async fn raw_tcp_clients_keep_their_first_packet() {
        let login = [0x64, 0x00, 0x01, 0x02, 0x03];
        let (_client, opening) = open_with(&login).await;
        assert_eq!(opening.unwrap(), Opening::Raw(login.to_vec()));
    }
}
