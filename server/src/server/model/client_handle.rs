use std::fmt;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use std::time::Duration;

use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::sync::{Notify, mpsc, watch};

use crate::server::websocket;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Tcp,
    WebSocket,
}

/// What a client may have queued and not yet taken before it is dropped, as rathena caps a session's write buffer: a client
/// that stopped reading only costs this much memory, and never holds back the others.
pub const MAX_QUEUED_BYTES: usize = 1024 * 1024;
/// How long one write may wait for a client that does not read.
pub const WRITE_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_BATCH_BYTES: usize = 64 * 1024;

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

enum Outgoing {
    Packet(Vec<u8>),
    Control(u8, Vec<u8>),
    Close,
}

struct Connection {
    id: u64,
    peer_addr: SocketAddr,
    local_addr: SocketAddr,
    transport: Transport,
    queue: mpsc::UnboundedSender<Outgoing>,
    queued_bytes: AtomicUsize,
    dropped: AtomicBool,
    abort_write: Notify,
    closing: watch::Sender<bool>,
}

/// One client connection, cloned into its session and its requests. Sending queues the packet for the connection's writer
/// task and never waits on the network.
#[derive(Clone)]
pub struct ClientHandle(Arc<Connection>);

/// The receiving end of a connection's queue, drained by its writer task.
pub struct Outbox {
    queue: mpsc::UnboundedReceiver<Outgoing>,
    connection: Weak<Connection>,
    transport: Transport,
}

impl ClientHandle {
    pub fn new(peer_addr: SocketAddr, local_addr: SocketAddr, transport: Transport) -> (Self, Outbox) {
        let (queue, receiver) = mpsc::unbounded_channel();
        let connection = Arc::new(Connection {
            id: NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed),
            peer_addr,
            local_addr,
            transport,
            queue,
            queued_bytes: AtomicUsize::new(0),
            dropped: AtomicBool::new(false),
            abort_write: Notify::new(),
            closing: watch::Sender::new(false),
        });
        let outbox = Outbox {
            queue: receiver,
            connection: Arc::downgrade(&connection),
            transport,
        };
        (Self(connection), outbox)
    }

    pub fn id(&self) -> u64 {
        self.0.id
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.0.peer_addr
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.0.local_addr
    }

    pub fn transport(&self) -> Transport {
        self.0.transport
    }

    /// Queues one or more whole packets. Returns false when the connection is closing, or when the client is that far behind
    /// and is dropped instead.
    pub fn send(&self, packet: Vec<u8>) -> bool {
        if packet.is_empty() {
            return true;
        }
        if self.is_closing() {
            return false;
        }
        let queued = self.0.queued_bytes.fetch_add(packet.len(), Ordering::AcqRel) + packet.len();
        if queued > MAX_QUEUED_BYTES {
            warn!("Disconnected {}, it has not taken {queued} bytes of packets", self.peer_addr());
            self.0.dropped.store(true, Ordering::Release);
            self.0.abort_write.notify_one();
            self.close();
            return false;
        }
        self.0.queue.send(Outgoing::Packet(packet)).is_ok()
    }

    pub(crate) fn send_control(&self, opcode: u8, payload: Vec<u8>) {
        if !self.is_closing() {
            let _ = self.0.queue.send(Outgoing::Control(opcode, payload));
        }
    }

    /// Stops reading from the client at once; what was queued before is still written, then the connection is closed.
    pub fn close(&self) {
        if !self.0.closing.send_replace(true) {
            let _ = self.0.queue.send(Outgoing::Close);
        }
    }

    pub fn is_closing(&self) -> bool {
        *self.0.closing.borrow()
    }

    pub(crate) fn closing(&self) -> watch::Receiver<bool> {
        self.0.closing.subscribe()
    }
}

impl PartialEq for ClientHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ClientHandle {}

impl fmt::Debug for ClientHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClientHandle")
            .field("id", &self.0.id)
            .field("peer_addr", &self.0.peer_addr)
            .field("transport", &self.0.transport)
            .finish()
    }
}

impl Outbox {
    /// Writes the queued packets until the handle is closed or the client stops taking them. What is queued together is
    /// written at once, and a WebSocket message only ever holds whole packets.
    pub async fn write_to<W: AsyncWrite + Unpin>(mut self, mut writer: W) {
        let mut batch = Vec::with_capacity(4096);
        let mut message = Vec::new();
        while let Some(first) = self.queue.recv().await {
            batch.clear();
            let mut taken = 0;
            let mut finished = false;
            let mut next = Some(first);
            while let Some(outgoing) = next.take() {
                match outgoing {
                    Outgoing::Packet(packet) => {
                        taken += packet.len();
                        match self.transport {
                            Transport::Tcp => batch.extend_from_slice(&packet),
                            Transport::WebSocket => message.extend_from_slice(&packet),
                        }
                    }
                    Outgoing::Control(opcode, payload) => {
                        Self::end_message(&mut batch, &mut message);
                        batch.extend_from_slice(&websocket::encode_frame(opcode, &payload));
                    }
                    Outgoing::Close => {
                        finished = true;
                        break;
                    }
                }
                if batch.len() + message.len() < MAX_BATCH_BYTES {
                    next = self.queue.try_recv().ok();
                }
            }
            Self::end_message(&mut batch, &mut message);
            let Some(connection) = self.connection.upgrade() else {
                break;
            };
            if connection.dropped.load(Ordering::Acquire) {
                break;
            }
            let written = tokio::select! {
                written = tokio::time::timeout(WRITE_TIMEOUT, writer.write_all(&batch)) => written,
                () = connection.abort_write.notified() => break,
            };
            connection.queued_bytes.fetch_sub(taken, Ordering::AcqRel);
            match written {
                Ok(Ok(())) => {}
                Ok(Err(error)) => {
                    debug!("Could not write to {}: {error}", connection.peer_addr);
                    break;
                }
                Err(_) => {
                    warn!("Disconnected {}, it took no packet for {WRITE_TIMEOUT:?}", connection.peer_addr);
                    break;
                }
            }
            if finished {
                break;
            }
        }
        if let Some(connection) = self.connection.upgrade() {
            ClientHandle(connection).close();
        }
        let _ = tokio::time::timeout(Duration::from_secs(1), writer.shutdown()).await;
    }

    fn end_message(batch: &mut Vec<u8>, message: &mut Vec<u8>) {
        if !message.is_empty() {
            batch.extend_from_slice(&websocket::encode_frame(websocket::OPCODE_BINARY, message));
            message.clear();
        }
    }

    /// The next queued packet, for tests that check what a handle was sent without a writer.
    #[cfg(test)]
    pub fn try_next_packet(&mut self) -> Option<Vec<u8>> {
        loop {
            match self.queue.try_recv().ok()? {
                Outgoing::Packet(packet) => return Some(packet),
                Outgoing::Control(..) => {}
                Outgoing::Close => return None,
            }
        }
    }
}

#[cfg(test)]
pub fn test_handle() -> (ClientHandle, Outbox) {
    let address = SocketAddr::from(([127, 0, 0, 1], 0));
    ClientHandle::new(address, address, Transport::Tcp)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use tokio::io::AsyncReadExt;
    use tokio::net::{TcpListener, TcpStream};

    use super::*;

    async fn connected_pair(transport: Transport) -> (ClientHandle, TcpStream, tokio::task::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = TcpStream::connect(listener.local_addr().unwrap()).await.unwrap();
        let (server, peer_addr) = listener.accept().await.unwrap();
        let (handle, outbox) = ClientHandle::new(peer_addr, server.local_addr().unwrap(), transport);
        let writer = tokio::spawn(outbox.write_to(server));
        (handle, client, writer)
    }

    #[tokio::test]
    async fn packets_arrive_in_the_order_they_were_queued() {
        let (handle, mut client, _writer) = connected_pair(Transport::Tcp).await;
        for packet in 0..200u8 {
            assert!(handle.send(vec![packet; 3]));
        }
        let mut received = vec![0u8; 600];
        client.read_exact(&mut received).await.unwrap();
        let expected: Vec<u8> = (0..200u8).flat_map(|packet| [packet; 3]).collect();
        assert_eq!(received, expected);
    }

    #[tokio::test]
    async fn a_client_that_stops_reading_is_dropped_without_blocking_the_sender() {
        let (handle, _client, writer) = connected_pair(Transport::Tcp).await;
        let started = Instant::now();
        let chunk = vec![0u8; 16 * 1024];
        let refused = (0..1000).find(|_| !handle.send(chunk.clone()));
        assert!(refused.is_some(), "a client that never reads falls behind and is dropped");
        assert!(started.elapsed() < Duration::from_secs(1), "queueing never waits on the network");
        assert!(handle.is_closing());
        tokio::time::timeout(Duration::from_secs(5), writer).await.expect("the writer stops").unwrap();
    }

    #[tokio::test]
    async fn closing_writes_what_was_queued_then_ends_the_connection() {
        let (handle, mut client, _writer) = connected_pair(Transport::Tcp).await;
        handle.send(vec![1, 2, 3]);
        handle.close();
        assert!(!handle.send(vec![4]), "nothing is queued after the close");
        let mut received = Vec::new();
        client.read_to_end(&mut received).await.unwrap();
        assert_eq!(received, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn websocket_packets_queued_together_share_one_binary_message() {
        let (handle, mut client, _writer) = connected_pair(Transport::WebSocket).await;
        handle.send_control(websocket::OPCODE_PING, vec![]);
        handle.send(vec![1, 2]);
        handle.send(vec![3]);
        handle.close();
        let mut received = Vec::new();
        client.read_to_end(&mut received).await.unwrap();
        assert_eq!(received, [0x89, 0, 0x82, 3, 1, 2, 3]);
    }
}
