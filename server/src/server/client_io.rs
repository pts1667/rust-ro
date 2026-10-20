//! The game port. Every client connection is a reader task and a writer task on a runtime of its own, so a client that is
//! slow or gone only holds up itself. The packets of a connection are handled one at a time on the runtime's blocking pool,
//! where a handler may still wait on the game loop or the database.
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::net::IpAddr;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use packets::packets::{Packet, PacketUnknown};
use packets::packets_parser::parse;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpListener, TcpStream};

use crate::server::model::character_lifecycle::ClientDisconnected;
use crate::server::model::client_handle::{ClientHandle, Transport};
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::request::Request;
use crate::server::request_handler::framing::ClientFrames;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::websocket::{self, Opening, WebSocketDecoder};
use crate::server::{PACKETVER, Server, request_handler, service};
use crate::util::packet::{PacketDirection, print_packet};

/// A WebSocket client is pinged this often while it sends nothing; the idle timeout is checked as often.
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(20);
const READ_BUFFER_BYTES: usize = 4096;

/// What a connection needs from the server, so the networking runs without one in its tests.
trait Requests: Clone + Send + Sync + 'static {
    /// Runs on the blocking pool.
    fn handle(&self, client: &ClientHandle, packet: Vec<u8>);
    fn disconnected(&self, client: ClientHandle);
    fn is_alive(&self) -> bool;
}

#[derive(Clone)]
struct ServerRequests {
    server: Arc<Server>,
    notifications: SyncSender<Notification>,
}

impl Requests for ServerRequests {
    fn handle(&self, client: &ClientHandle, frame: Vec<u8>) {
        let packetver = self.server.packetver();
        let id = u16::from_le_bytes([frame[0], frame[1]]);
        let raw = service::script_world_service::world_frame_length(id, packetver).is_some()
            || request_handler::script_operations::frame_length(id, packetver).is_some()
            || request_handler::char_requests::frame_length(id, packetver).is_some();
        let packet: Box<dyn Packet> = if raw {
            Box::new(PacketUnknown {
                raw: frame,
                packet_id: format!("0x{id:04x}"),
            })
        } else {
            parse(&frame, packetver)
        };
        if GlobalConfigService::instance().config().server.trace_packet {
            print_packet(&None, None, PacketDirection::Forward, &packet);
        }
        let context = Request::new(self.server.configuration, None, packetver, client.clone(), packet.as_ref(), self.notifications.clone());
        request_handler::handle(self.server.clone(), context);
    }

    fn disconnected(&self, client: ClientHandle) {
        self.server.add_to_next_tick(GameEvent::ClientDisconnected(ClientDisconnected { socket: client }));
    }

    fn is_alive(&self) -> bool {
        self.server.is_alive()
    }
}

#[derive(Clone, Copy)]
struct ConnectionSettings {
    websocket: bool,
    idle_timeout: Option<Duration>,
    packetver: u32,
}

/// Accepts clients on `listener` until the server shuts down.
pub fn serve(server: Arc<Server>, listener: std::net::TcpListener, notifications: SyncSender<Notification>) {
    let packetver = server.packetver();
    let threads = std::thread::available_parallelism().map_or(2, |threads| threads.get().clamp(2, 4));
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .thread_name("client_io")
        .enable_all()
        .on_thread_start(move || PACKETVER.with(|ver| *ver.borrow_mut() = packetver))
        .build()
        .expect("Failed to start the client connection runtime");
    let config = &server.configuration.server;
    let settings = ConnectionSettings {
        websocket: config.enable_websocket,
        idle_timeout: config.client_idle_timeout(),
        packetver,
    };
    let limits = ConnectionLimits::new(config.max_connections, config.max_connections_per_ip);
    let requests = ServerRequests { server, notifications };
    runtime.block_on(async {
        listener.set_nonblocking(true).expect("Failed to make the client listener non-blocking");
        let listener = TcpListener::from_std(listener).expect("Failed to register the client listener");
        accept(listener, settings, limits, requests).await;
    });
    runtime.shutdown_timeout(Duration::from_secs(5));
}

async fn accept<R: Requests>(listener: TcpListener, settings: ConnectionSettings, limits: ConnectionLimits, requests: R) {
    loop {
        let accepted = listener.accept().await;
        if !requests.is_alive() {
            break;
        }
        let (stream, peer_addr) = match accepted {
            Ok(accepted) => accepted,
            Err(error) => {
                // Out of file handles, or reset before it was accepted: the listener itself is still usable.
                warn!("Could not accept a client connection: {error}");
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
        };
        let Some(slot) = limits.admit(peer_addr.ip()) else {
            warn!("Refused a connection from {peer_addr}, too many clients are connected");
            continue;
        };
        tokio::spawn(connection(stream, settings, slot, requests.clone()));
    }
    info!("Shutdown client connections");
}

async fn connection<R: Requests>(mut stream: TcpStream, settings: ConnectionSettings, _slot: ConnectionSlot, requests: R) {
    let _ = stream.set_nodelay(true);
    let (Ok(peer_addr), Ok(local_addr)) = (stream.peer_addr(), stream.local_addr()) else {
        return;
    };
    debug!("Received new connection from {peer_addr}");
    let opening = if settings.websocket {
        websocket::open(&mut stream).await
    } else {
        Ok(Opening::Raw(Vec::new()))
    };
    let (transport, first_bytes) = match opening {
        Ok(Opening::Raw(bytes)) => (Transport::Tcp, bytes),
        Ok(Opening::WebSocket(bytes)) => (Transport::WebSocket, bytes),
        Err(error) => {
            warn!("Client connection from {peer_addr} rejected: {error}");
            return;
        }
    };
    let (reader, writer) = stream.into_split();
    let (client, outbox) = ClientHandle::new(peer_addr, local_addr, transport);
    let writing = tokio::spawn(outbox.write_to(writer));
    read(&client, reader, first_bytes, settings, &requests).await;
    client.close();
    requests.disconnected(client);
    let _ = writing.await;
}

async fn read<R: Requests>(client: &ClientHandle, mut reader: OwnedReadHalf, first_bytes: Vec<u8>, settings: ConnectionSettings, requests: &R) {
    let mut inbound = Inbound {
        websocket: (client.transport() == Transport::WebSocket).then(WebSocketDecoder::default),
        frames: ClientFrames::new(settings.packetver),
    };
    if !first_bytes.is_empty() && !receive(client, &mut inbound, &first_bytes, requests).await {
        return;
    }
    let wait = settings.idle_timeout.map_or(KEEPALIVE_INTERVAL, |idle| idle.min(KEEPALIVE_INTERVAL));
    let mut closing = client.closing();
    let mut buffer = vec![0u8; READ_BUFFER_BYTES];
    let mut last_received = Instant::now();
    loop {
        let read = tokio::select! {
            _ = closing.wait_for(|closing| *closing) => return,
            read = tokio::time::timeout(wait, reader.read(&mut buffer)) => read,
        };
        match read {
            Ok(Ok(0)) => return,
            Ok(Ok(count)) => {
                if !receive(client, &mut inbound, &buffer[..count], requests).await {
                    return;
                }
                // After the handlers: the client is not read while they wait on the game loop.
                last_received = Instant::now();
            }
            Ok(Err(error)) => {
                debug!("Connection from {} ended: {error}", client.peer_addr());
                return;
            }
            Err(_) => {
                if settings.idle_timeout.is_some_and(|idle| last_received.elapsed() >= idle) {
                    info!("Disconnected {}, it sent nothing for {:?}", client.peer_addr(), last_received.elapsed());
                    return;
                }
                if client.transport() == Transport::WebSocket {
                    client.send_control(websocket::OPCODE_PING, Vec::new());
                }
            }
        }
    }
}

/// Handles what the client sent. Returns false once the connection is to end.
async fn receive<R: Requests>(client: &ClientHandle, inbound: &mut Inbound, bytes: &[u8], requests: &R) -> bool {
    let (packets, open) = match inbound.push(client, bytes) {
        Ok(received) => received,
        Err(error) => {
            warn!("Invalid client packet stream from {}: {error}", client.peer_addr());
            return false;
        }
    };
    if !packets.is_empty() {
        let (task_client, task_requests) = (client.clone(), requests.clone());
        let handled = tokio::task::spawn_blocking(move || {
            for packet in packets {
                if task_client.is_closing() || !task_requests.is_alive() {
                    break;
                }
                task_requests.handle(&task_client, packet);
            }
        })
        .await;
        if let Err(error) = handled {
            error!("Disconnected {}, a request handler failed: {error}", client.peer_addr());
            return false;
        }
    }
    open && !client.is_closing()
}

/// Splits what a client sends into packets, out of its WebSocket frames first when it uses them.
struct Inbound {
    websocket: Option<WebSocketDecoder>,
    frames: ClientFrames,
}

impl Inbound {
    /// The packets completed by `bytes`, and whether the client keeps the connection open.
    fn push(&mut self, client: &ClientHandle, bytes: &[u8]) -> Result<(Vec<Vec<u8>>, bool), String> {
        let Some(decoder) = self.websocket.as_mut() else {
            return Ok((self.frames.push(bytes)?, true));
        };
        match decoder.push(bytes) {
            Ok(decoded) => {
                for (opcode, payload) in decoded.replies {
                    client.send_control(opcode, payload);
                }
                Ok((self.frames.push(&decoded.payload)?, !decoded.closed))
            }
            Err(violation) => {
                client.send_control(websocket::OPCODE_CLOSE, violation.code.to_be_bytes().to_vec());
                Err(violation.reason.to_string())
            }
        }
    }
}

/// Caps the open connections, in total and per address (0 for no limit per address).
#[derive(Clone)]
struct ConnectionLimits(Arc<Limits>);

struct Limits {
    max_total: usize,
    max_per_ip: usize,
    open: Mutex<OpenConnections>,
}

#[derive(Default)]
struct OpenConnections {
    total: usize,
    by_ip: HashMap<IpAddr, usize>,
}

/// Held by a connection's task; frees its place when the connection ends.
struct ConnectionSlot {
    limits: Arc<Limits>,
    ip: IpAddr,
}

impl ConnectionLimits {
    fn new(max_total: usize, max_per_ip: usize) -> Self {
        Self(Arc::new(Limits {
            max_total,
            max_per_ip,
            open: Mutex::default(),
        }))
    }

    fn admit(&self, ip: IpAddr) -> Option<ConnectionSlot> {
        let mut open = self.0.open.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let from_ip = open.by_ip.get(&ip).copied().unwrap_or(0);
        if open.total >= self.0.max_total || (self.0.max_per_ip > 0 && from_ip >= self.0.max_per_ip) {
            return None;
        }
        open.total += 1;
        *open.by_ip.entry(ip).or_default() += 1;
        Some(ConnectionSlot {
            limits: self.0.clone(),
            ip,
        })
    }
}

impl Drop for ConnectionSlot {
    fn drop(&mut self) {
        let mut open = self.limits.open.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        open.total -= 1;
        if let Entry::Occupied(mut from_ip) = open.by_ip.entry(self.ip) {
            *from_ip.get_mut() -= 1;
            if *from_ip.get() == 0 {
                from_ip.remove();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use packets::packets::PacketCaLogin;
    use tokio::io::AsyncWriteExt;

    use super::*;

    const PACKETVER: u32 = 20120307;

    /// Answers every packet with `[0x69, 0x00]`, or fails on it when `panics`.
    #[derive(Clone, Default)]
    struct Recorder {
        packets: Arc<Mutex<Vec<Vec<u8>>>>,
        disconnected: Arc<Mutex<Vec<u64>>>,
        panics: bool,
    }

    impl Requests for Recorder {
        fn handle(&self, client: &ClientHandle, packet: Vec<u8>) {
            assert!(!self.panics, "the handler fails");
            self.packets.lock().unwrap().push(packet);
            client.send(vec![0x69, 0x00]);
        }

        fn disconnected(&self, client: ClientHandle) {
            self.disconnected.lock().unwrap().push(client.id());
        }

        fn is_alive(&self) -> bool {
            true
        }
    }

    fn login_packet() -> Vec<u8> {
        let mut packet = PacketCaLogin::new(PACKETVER);
        packet.fill_raw();
        packet.raw().clone()
    }

    async fn start(idle_timeout: Option<Duration>, max_per_ip: usize) -> (std::net::SocketAddr, Recorder) {
        start_with(idle_timeout, max_per_ip, Recorder::default()).await
    }

    async fn start_with(idle_timeout: Option<Duration>, max_per_ip: usize, recorder: Recorder) -> (std::net::SocketAddr, Recorder) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let settings = ConnectionSettings {
            websocket: true,
            idle_timeout,
            packetver: PACKETVER,
        };
        tokio::spawn(accept(listener, settings, ConnectionLimits::new(16, max_per_ip), recorder.clone()));
        (address, recorder)
    }

    async fn read_some(client: &mut TcpStream) -> Vec<u8> {
        let mut buffer = vec![0u8; 256];
        let count = tokio::time::timeout(Duration::from_secs(5), client.read(&mut buffer)).await.unwrap().unwrap();
        buffer.truncate(count);
        buffer
    }

    async fn ended(client: &mut TcpStream) -> bool {
        let mut buffer = [0u8; 64];
        matches!(tokio::time::timeout(Duration::from_secs(5), client.read(&mut buffer)).await, Ok(Ok(0) | Err(_)))
    }

    async fn wait_for_disconnection(recorder: &Recorder, count: usize) {
        let waited = tokio::time::timeout(Duration::from_secs(5), async {
            while recorder.disconnected.lock().unwrap().len() < count {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        waited.await.expect("the disconnection is reported");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_tcp_client_is_answered_and_its_disconnection_reported() {
        let (address, recorder) = start(None, 0).await;
        let mut client = TcpStream::connect(address).await.unwrap();
        let login = login_packet();
        client.write_all(&login[..10]).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        client.write_all(&login[10..]).await.unwrap();
        assert_eq!(read_some(&mut client).await, [0x69, 0x00]);
        assert_eq!(*recorder.packets.lock().unwrap(), [login], "a packet split across reads is handled once whole");
        drop(client);
        wait_for_disconnection(&recorder, 1).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_websocket_client_sends_and_receives_binary_messages() {
        let (address, recorder) = start(None, 0).await;
        let mut client = TcpStream::connect(address).await.unwrap();
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\
                       Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n";
        let login = login_packet();
        let mut frame = vec![0x82, 0x80 | login.len() as u8, 0, 0, 0, 0];
        frame.extend_from_slice(&login);
        let mut first_bytes = request.as_bytes().to_vec();
        first_bytes.extend(frame);
        client.write_all(&first_bytes).await.unwrap();
        let mut received = Vec::new();
        while !received.ends_with(&[0x82, 2, 0x69, 0x00]) {
            received.extend(read_some(&mut client).await);
        }
        assert!(received.starts_with(b"HTTP/1.1 101"));
        assert_eq!(*recorder.packets.lock().unwrap(), [login]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_client_that_sends_nothing_is_disconnected_after_the_idle_timeout() {
        let (address, recorder) = start(Some(Duration::from_millis(200)), 0).await;
        let mut client = TcpStream::connect(address).await.unwrap();
        client.write_all(&login_packet()).await.unwrap();
        assert_eq!(read_some(&mut client).await, [0x69, 0x00]);
        assert!(ended(&mut client).await);
        wait_for_disconnection(&recorder, 1).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn an_invalid_packet_stream_ends_the_connection() {
        let (address, recorder) = start(None, 0).await;
        let mut client = TcpStream::connect(address).await.unwrap();
        client.write_all(&[0xFF, 0xFF, 0, 0]).await.unwrap();
        assert!(ended(&mut client).await);
        wait_for_disconnection(&recorder, 1).await;
        assert!(recorder.packets.lock().unwrap().is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn connections_over_the_limit_per_address_are_refused_until_one_closes() {
        let (address, recorder) = start(None, 1).await;
        let mut first = TcpStream::connect(address).await.unwrap();
        first.write_all(&login_packet()).await.unwrap();
        assert_eq!(read_some(&mut first).await, [0x69, 0x00]);
        let mut second = TcpStream::connect(address).await.unwrap();
        assert!(ended(&mut second).await);
        drop(first);
        wait_for_disconnection(&recorder, 1).await;
        let reconnected = async {
            loop {
                let mut third = TcpStream::connect(address).await.unwrap();
                third.write_all(&login_packet()).await.unwrap();
                let mut reply = [0u8; 2];
                if third.read_exact(&mut reply).await.is_ok() {
                    return reply;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        };
        assert_eq!(tokio::time::timeout(Duration::from_secs(5), reconnected).await.unwrap(), [0x69, 0x00]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_failing_handler_ends_its_connection_only() {
        let (address, recorder) = start_with(None, 0, Recorder { panics: true, ..Recorder::default() }).await;
        let mut client = TcpStream::connect(address).await.unwrap();
        client.write_all(&login_packet()).await.unwrap();
        assert!(ended(&mut client).await);
        wait_for_disconnection(&recorder, 1).await;
        let mut next = TcpStream::connect(address).await.unwrap();
        next.write_all(&[0xFF, 0xFF]).await.unwrap();
        assert!(ended(&mut next).await, "the server still accepts connections");
    }

    #[test]
    fn a_slot_is_freed_when_its_connection_ends() {
        let limits = ConnectionLimits::new(2, 0);
        let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let first = limits.admit(ip).unwrap();
        let _second = limits.admit(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))).unwrap();
        assert!(limits.admit(ip).is_none(), "the total is reached");
        drop(first);
        assert!(limits.admit(ip).is_some());
    }
}
