
pub mod config;
pub mod error;

mod server;
pub use server::*;

mod protocol;

pub use config::*;
pub use error::P2pError;

use libp2p::{Multiaddr, PeerId};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub peer: Peer,
    pub known_peers_count: usize,
    pub known_peers: HashMap<PeerId, Vec<Multiaddr>>,
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub address: Vec<Multiaddr>,
}

impl Peer {
    pub fn new(peer_id: PeerId, address: Vec<Multiaddr>) -> Self {
        Peer { id: peer_id, address}
    }
}