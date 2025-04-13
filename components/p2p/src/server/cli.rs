use libp2p::{PeerId, kad};
use log::{error, info};
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use crate::{Peer, P2pError, PeerIdWithMultiaddr};
use super::Command;

#[derive(Debug, Clone)]
pub struct Client {
    pub cmd_sender: UnboundedSender<Command>,
}


impl Client {
    /// Send a blocking request to the `target` peer.
    pub fn blocking_request(&self, target: PeerId, request: Vec<u8>) -> Result<Vec<u8>, P2pError> {
        let (responder, receiver) = oneshot::channel();
        let _ = self.cmd_sender.send(Command::SendRequest {
            target,
            request,
            responder,
        });
        receiver
            .blocking_recv()?
            .map_err(|_| P2pError::RequestRejected)
    }

    /// Publish a message to the given topic.
    pub fn broadcast(&self, topic: impl Into<String>, message: Vec<u8>) {
        let _ = self.cmd_sender.send(Command::Broadcast {
            topic: topic.into(),
            message,
        });
    }

    /// Get known peers of the node.
    // pub async fn get_known_peers(&self) -> Result<Vec<PeerId>, P2pError> {
    //     let peer_id_list = self.get_peer().await?
    //         .known_peers
    //         .into_keys()
    //         .collect();
    //     Ok(peer_id_list)
    // }

    /// Get status of the node for debugging.
    pub async fn get_peer(&self) -> Result<Peer, P2pError> {
        let (responder, receiver) = oneshot::channel();
        let _ = self.cmd_sender.send(Command::GetStatus(responder));
        let peer = receiver.await;
        peer.map_err(|_| P2pError::InvalidPeerId)
        // receiver.await.map_err(|e| P2pError::InvalidPeerId)
    }

    pub async fn get_closet_peer(&self, key: Vec<u8>) -> Result<Vec<PeerId>, P2pError> {
        let (responder, receiver) = oneshot::channel();
        self.cmd_sender.send(Command::GetClosestPeers { key, responder });
        let query_result = receiver.await?;
        match query_result {
            kad::QueryResult::GetClosestPeers(Ok(ok)) => {
                if ok.peers.is_empty() {
                    return Err(P2pError::EmptyClosestPeers);
                }
                return Ok(ok.peers)
            },
            kad::QueryResult::GetClosestPeers(Err(e)) => return Err(P2pError::QueryClosestPeersTimeout),
            _ => return Err(P2pError::UnknownQueryClosestPeersResult),
        }
    }

    // pub async fn sync_datablock(&self, )

}

