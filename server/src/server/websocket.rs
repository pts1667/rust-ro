use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use base64::Engine;
use sha1::{Digest, Sha1};

use crate::server::model::client_socket::ClientSocket;

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

/// RO clients send a packet id first, never `GET `, so the first bytes are enough to tell the protocols apart.
pub fn is_upgrade_request(stream: &TcpStream) -> io::Result<bool> {
    const METHOD: &[u8] = b"GET ";
    stream.set_read_timeout(Some(HANDSHAKE_TIMEOUT))?;
    let mut head = [0u8; METHOD.len()];
    let upgrade = loop {
        match stream.peek(&mut head) {
            Ok(0) => break false,
            Ok(count) if !METHOD.starts_with(&head[..count]) => break false,
            Ok(count) if count == METHOD.len() => break true,
            Ok(_) => thread::sleep(Duration::from_millis(1)),
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => break false,
            Err(error) => return Err(error),
        }
    };
    stream.set_read_timeout(None)?;
    Ok(upgrade)
}

pub fn handshake(stream: &mut TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(HANDSHAKE_TIMEOUT))?;
    let request = read_request_head(stream)?;
    let outcome = match negotiate(&request) {
        Ok(response) => stream.write_all(response.as_bytes()),
        Err(rejection) => {
            stream.write_all(rejection.as_bytes())?;
            Err(io::Error::new(ErrorKind::InvalidData, "invalid websocket handshake"))
        }
    };
    stream.set_read_timeout(None)?;
    outcome
}

// One byte at a time so nothing after the headers is consumed.
fn read_request_head(stream: &mut TcpStream) -> io::Result<String> {
    let mut head = Vec::new();
    let mut byte = [0u8; 1];
    while !head.ends_with(b"\r\n\r\n") {
        if head.len() >= MAX_HANDSHAKE_BYTES {
            return Err(io::Error::new(ErrorKind::InvalidData, "websocket handshake is too large"));
        }
        if stream.read(&mut byte)? == 0 {
            return Err(ErrorKind::UnexpectedEof.into());
        }
        head.push(byte[0]);
    }
    String::from_utf8(head).map_err(|_| io::Error::new(ErrorKind::InvalidData, "websocket handshake is not UTF-8"))
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

/// Presents the payloads of incoming binary frames as the same byte stream a raw TCP client sends.
pub struct WebSocketReader {
    stream: TcpStream,
    writer: Arc<RwLock<ClientSocket>>,
    pending: Vec<u8>,
    position: usize,
    inside_fragmented_message: bool,
}

impl WebSocketReader {
    pub fn new(stream: TcpStream, writer: Arc<RwLock<ClientSocket>>) -> Self {
        Self {
            stream,
            writer,
            pending: Vec::new(),
            position: 0,
            inside_fragmented_message: false,
        }
    }

    fn send_control(&self, opcode: u8, payload: &[u8]) {
        let _ = self.writer.write().unwrap().send_control(opcode, payload);
    }

    fn fail(&self, code: u16, reason: &str) -> io::Result<bool> {
        self.send_control(OPCODE_CLOSE, &code.to_be_bytes());
        Err(io::Error::new(ErrorKind::InvalidData, reason.to_string()))
    }

    /// Handles one frame. `false` means the connection is over.
    fn read_frame(&mut self) -> io::Result<bool> {
        let mut head = [0u8; 2];
        match self.stream.read_exact(&mut head) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => return Ok(false),
            Err(error) => return Err(error),
        }
        let fin = head[0] & 0x80 != 0;
        let reserved = head[0] & 0x70;
        let opcode = head[0] & 0x0F;
        let masked = head[1] & 0x80 != 0;
        let length = match head[1] & 0x7F {
            126 => {
                let mut bytes = [0u8; 2];
                self.stream.read_exact(&mut bytes)?;
                u64::from(u16::from_be_bytes(bytes))
            }
            127 => {
                let mut bytes = [0u8; 8];
                self.stream.read_exact(&mut bytes)?;
                u64::from_be_bytes(bytes)
            }
            short => u64::from(short),
        };
        let control = opcode & 0x8 != 0;
        if reserved != 0 || !masked {
            return self.fail(CLOSE_PROTOCOL_ERROR, "websocket frame is unmasked or uses extensions");
        }
        if control && (!fin || length > MAX_CONTROL_PAYLOAD) {
            return self.fail(CLOSE_PROTOCOL_ERROR, "invalid websocket control frame");
        }
        if length > MAX_FRAME_PAYLOAD {
            return self.fail(CLOSE_MESSAGE_TOO_BIG, "websocket frame is too large");
        }
        let mut mask = [0u8; 4];
        self.stream.read_exact(&mut mask)?;
        let mut payload = vec![0u8; length as usize];
        self.stream.read_exact(&mut payload)?;
        for (index, byte) in payload.iter_mut().enumerate() {
            *byte ^= mask[index % 4];
        }
        match opcode {
            OPCODE_BINARY | OPCODE_CONTINUATION => {
                if (opcode == OPCODE_CONTINUATION) != self.inside_fragmented_message {
                    return self.fail(CLOSE_PROTOCOL_ERROR, "unexpected websocket fragment order");
                }
                self.inside_fragmented_message = !fin;
                self.pending = payload;
                self.position = 0;
                Ok(true)
            }
            OPCODE_TEXT => self.fail(CLOSE_UNSUPPORTED_DATA, "websocket text frames are not supported"),
            OPCODE_PING => {
                self.send_control(OPCODE_PONG, &payload);
                Ok(true)
            }
            OPCODE_PONG => Ok(true),
            OPCODE_CLOSE => {
                self.send_control(OPCODE_CLOSE, &payload);
                Ok(false)
            }
            _ => self.fail(CLOSE_PROTOCOL_ERROR, "unknown websocket opcode"),
        }
    }
}

impl Read for WebSocketReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        while self.position == self.pending.len() {
            if !self.read_frame()? {
                return Ok(0);
            }
        }
        let count = buffer.len().min(self.pending.len() - self.position);
        buffer[..count].copy_from_slice(&self.pending[self.position..self.position + count]);
        self.position += count;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::server::model::client_socket::{ClientConnection, Transport};

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

    fn connect(websocket_enabled: bool, first_bytes: &[u8]) -> (TcpStream, std::io::Result<ClientConnection>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        client.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let (server_stream, _) = listener.accept().unwrap();
        let accepting = thread::spawn(move || ClientConnection::accept(server_stream, websocket_enabled));
        client.write_all(first_bytes).unwrap();
        (client, accepting.join().unwrap())
    }

    fn read_head(client: &mut TcpStream) -> String {
        let mut head = Vec::new();
        let mut byte = [0u8; 1];
        while !head.ends_with(b"\r\n\r\n") {
            client.read_exact(&mut byte).unwrap();
            head.push(byte[0]);
        }
        String::from_utf8(head).unwrap()
    }

    fn websocket_pair() -> (TcpStream, ClientConnection) {
        let (mut client, connection) = connect(true, UPGRADE_REQUEST.as_bytes());
        assert!(read_head(&mut client).starts_with("HTTP/1.1 101"));
        (client, connection.unwrap())
    }

    fn read_server_frame(client: &mut TcpStream) -> (u8, Vec<u8>) {
        let mut head = [0u8; 2];
        client.read_exact(&mut head).unwrap();
        let mut payload = vec![0u8; usize::from(head[1] & 0x7F)];
        client.read_exact(&mut payload).unwrap();
        (head[0], payload)
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
    fn websocket_client_gets_a_byte_stream_and_framed_replies() {
        let (mut client, mut connection) = websocket_pair();
        assert_eq!(connection.socket.read().unwrap().transport(), Transport::WebSocket);
        client.write_all(&client_frame(OPCODE_BINARY, false, &[1, 2])).unwrap();
        client.write_all(&client_frame(OPCODE_CONTINUATION, true, &[3, 4, 5])).unwrap();
        let mut received = [0u8; 5];
        connection.reader.read_exact(&mut received).unwrap();
        assert_eq!(received, [1, 2, 3, 4, 5]);

        connection.socket.write().unwrap().write_all(&[9, 8, 7]).unwrap();
        assert_eq!(read_server_frame(&mut client), (0x82, vec![9, 8, 7]));
    }

    #[test]
    fn ping_is_answered_with_a_pong_carrying_the_same_payload() {
        let (mut client, mut connection) = websocket_pair();
        client.write_all(&client_frame(OPCODE_PING, true, b"hi")).unwrap();
        client.write_all(&client_frame(OPCODE_BINARY, true, &[6])).unwrap();
        let mut received = [0u8; 1];
        connection.reader.read_exact(&mut received).unwrap();
        assert_eq!(received, [6]);
        assert_eq!(read_server_frame(&mut client), (0x8A, b"hi".to_vec()));
    }

    #[test]
    fn close_frame_ends_the_stream_and_is_echoed() {
        let (mut client, mut connection) = websocket_pair();
        client.write_all(&client_frame(OPCODE_CLOSE, true, &1000u16.to_be_bytes())).unwrap();
        assert_eq!(connection.reader.read(&mut [0u8; 8]).unwrap(), 0);
        assert_eq!(read_server_frame(&mut client), (0x88, 1000u16.to_be_bytes().to_vec()));
    }

    #[test]
    fn protocol_violations_close_the_connection_with_a_status_code() {
        let (mut client, mut connection) = websocket_pair();
        client.write_all(&[0x82, 0x01, 0x05]).unwrap();
        assert!(connection.reader.read(&mut [0u8; 8]).is_err());
        assert_eq!(read_server_frame(&mut client), (0x88, 1002u16.to_be_bytes().to_vec()));

        let (mut client, mut connection) = websocket_pair();
        client.write_all(&client_frame(OPCODE_TEXT, true, b"text")).unwrap();
        assert!(connection.reader.read(&mut [0u8; 8]).is_err());
        assert_eq!(read_server_frame(&mut client), (0x88, 1003u16.to_be_bytes().to_vec()));

        let (mut client, mut connection) = websocket_pair();
        client.write_all(&client_frame(OPCODE_CONTINUATION, true, &[1])).unwrap();
        assert!(connection.reader.read(&mut [0u8; 8]).is_err());
        assert_eq!(read_server_frame(&mut client), (0x88, 1002u16.to_be_bytes().to_vec()));
    }

    #[test]
    fn plain_http_request_is_refused() {
        let (mut client, connection) = connect(true, b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(connection.is_err());
        assert!(read_head(&mut client).starts_with("HTTP/1.1 400"));
    }

    #[test]
    fn raw_tcp_clients_are_not_framed() {
        let login = [0x64, 0x00, 0x01, 0x02, 0x03];
        let (mut client, connection) = connect(true, &login);
        let mut connection = connection.unwrap();
        assert_eq!(connection.socket.read().unwrap().transport(), Transport::Tcp);
        let mut received = [0u8; 5];
        connection.reader.read_exact(&mut received).unwrap();
        assert_eq!(received, login);

        connection.socket.write().unwrap().write_all(&[0x69, 0x00]).unwrap();
        let mut reply = [0u8; 2];
        client.read_exact(&mut reply).unwrap();
        assert_eq!(reply, [0x69, 0x00]);
    }

    #[test]
    fn disabled_websocket_treats_an_http_request_as_raw_bytes() {
        let (_client, connection) = connect(false, UPGRADE_REQUEST.as_bytes());
        let mut connection = connection.unwrap();
        assert_eq!(connection.socket.read().unwrap().transport(), Transport::Tcp);
        let mut received = [0u8; 4];
        connection.reader.read_exact(&mut received).unwrap();
        assert_eq!(&received, b"GET ");
    }
}
