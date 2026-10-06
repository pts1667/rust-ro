use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::{Arc, RwLock};

use crate::server::websocket::{self, WebSocketReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Tcp,
    WebSocket,
}

/// Write half of a client connection. Every `write` on a WebSocket client is sent as one binary message.
#[derive(Debug)]
pub struct ClientSocket {
    stream: TcpStream,
    transport: Transport,
}

impl ClientSocket {
    pub fn tcp(stream: TcpStream) -> Self {
        Self {
            stream,
            transport: Transport::Tcp,
        }
    }

    pub fn websocket(stream: TcpStream) -> Self {
        Self {
            stream,
            transport: Transport::WebSocket,
        }
    }

    pub fn transport(&self) -> Transport {
        self.transport
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.stream.local_addr()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.stream.shutdown(how)
    }

    pub(crate) fn send_control(&mut self, opcode: u8, payload: &[u8]) -> io::Result<()> {
        self.stream.write_all(&websocket::encode_frame(opcode, payload))
    }
}

impl Write for ClientSocket {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        match self.transport {
            Transport::Tcp => self.stream.write(data),
            Transport::WebSocket => {
                if !data.is_empty() {
                    self.stream.write_all(&websocket::encode_frame(websocket::OPCODE_BINARY, data))?;
                }
                Ok(data.len())
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

pub enum ClientReader {
    Tcp(TcpStream),
    WebSocket(WebSocketReader),
}

impl Read for ClientReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self {
            ClientReader::Tcp(stream) => stream.read(buffer),
            ClientReader::WebSocket(reader) => reader.read(buffer),
        }
    }
}

pub struct ClientConnection {
    pub reader: ClientReader,
    pub socket: Arc<RwLock<ClientSocket>>,
}

impl ClientConnection {
    /// Raw TCP clients are used as is; a client opening with an HTTP upgrade request is switched to WebSocket.
    pub fn accept(mut stream: TcpStream, websocket_enabled: bool) -> io::Result<Self> {
        let upgrade = websocket_enabled && websocket::is_upgrade_request(&stream)?;
        if upgrade {
            websocket::handshake(&mut stream)?;
        }
        let writer = stream.try_clone()?;
        let socket = Arc::new(RwLock::new(if upgrade {
            ClientSocket::websocket(writer)
        } else {
            ClientSocket::tcp(writer)
        }));
        let reader = if upgrade {
            ClientReader::WebSocket(WebSocketReader::new(stream, socket.clone()))
        } else {
            ClientReader::Tcp(stream)
        };
        Ok(Self { reader, socket })
    }
}
