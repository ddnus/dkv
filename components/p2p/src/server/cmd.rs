use libp2p::PeerId;

use crate::{Peer, protocol::ResponseType};
use super::Server;
use super::*;

#[derive(Debug)]
pub enum Command {
    SendRequest {
        target: PeerId,
        request: Vec<u8>,
        responder: oneshot::Sender<ResponseType>,
    },
    Broadcast {
        topic: String,
        message: Vec<u8>,
    },
    GetKnownPeers {
        responder: oneshot::Sender<Vec<Peer>>,
    },
    GetClosestPeers {
        key: Vec<u8>,
        responder: oneshot::Sender<QueryResult>,
    },
    GetPeerInfo {
        peer_id: PeerId,
        responder: oneshot::Sender<Peer>,
    },
    GetStatus(oneshot::Sender<Peer>),
    // GetClosestPeers()
}

impl <E: EventHandler> Server<E> {
    // Process the next command coming from `Client`.
    pub fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command::SendRequest {
                target,
                request,
                responder,
            } => self.handle_outbound_request(target, request, responder),
            Command::Broadcast { topic, message } => self.handle_outbound_broadcast(topic, message),
            // Command::GetStatus(responder) => {
            //     responder.send(self.get_status()).unwrap()
            // },
            Command::GetClosestPeers { key, responder } => self.handle_get_closest_peers(key, responder),
            Command::GetKnownPeers { responder } => self.handle_get_known_peers(responder),
            _ => {
                println!("=============unknown command===========");
            },
        }
    }

    // Store the request_id with the responder so that we can send the response later.
    fn handle_outbound_request(
        &mut self,
        target: PeerId,
        request: Vec<u8>,
        responder: oneshot::Sender<ResponseType>,
    ) {
        let req_id = self
            .swarm
            .behaviour_mut()
            .send_request(&target, request);
        self.pending_outbound_requests.insert(req_id, responder);
    }

    // fn get_status(&mut self) -> Peer {
    //     let known_peers = self.swarm.behaviour_mut().known_peers();
    //     Peer {
    //         local_peer_id: self.local_peer_id,
    //         listened_addresses: self.listened_addresses.clone(),
    //         known_peers_count: known_peers.len(),
    //         known_peers,
    //     }
    // }
    

    // Broadcast a message to all peers subscribed to the given topic.
    fn handle_outbound_broadcast(&mut self, topic: String, message: Vec<u8>) {
        let _ = self
            .swarm
            .behaviour_mut()
            .broadcast(topic, message);
    }

    /// get closest peers
    fn handle_get_closest_peers(&mut self, key: Vec<u8>, responder: oneshot::Sender<QueryResult>) {
        let query_id = self.swarm.behaviour_mut().get_closest_peers(key);
        self.p2p_query_requests.insert(query_id, responder);
    }

    fn handle_get_known_peers(&mut self, responder: oneshot::Sender<Vec<Peer>>) {
        let peers = self.swarm.behaviour_mut().known_peers();
        let mut peer_list = vec![];
        for (peer_id, peer) in peers.iter() {
            let peer = Peer::new(*peer_id, peer.address.clone());
            peer_list.push(peer);
        }
        let _ = responder.send(peer_list);
    }
    
}

