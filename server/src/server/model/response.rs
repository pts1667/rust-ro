use std::sync::{Arc, RwLock};

use crate::server::model::client_socket::ClientSocket;

pub struct Response {
    socket: Arc<RwLock<ClientSocket>>,
    packet: Vec<u8>,
}

impl Response {
    pub fn new(socket: Arc<RwLock<ClientSocket>>, packet: Vec<u8>) -> Self {
        Self { socket, packet }
    }

    pub fn socket(&self) -> Arc<RwLock<ClientSocket>> {
        self.socket.clone()
    }

    pub fn serialized_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}
