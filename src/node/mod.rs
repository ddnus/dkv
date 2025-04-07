mod space;
use space::Space;

use std::{ops::Deref, sync::Arc, time::Duration};

mod kv;
mod node;

use bytes::Bytes;
use p2p::PeerIdWithMultiaddr;

use crate::{db::{Db, DbDropGuard}, P2pClient};

#[derive(Debug, Clone)]
pub struct Node {
    inner: Arc<NodeInner>,
}

impl Node {
    pub fn new(
        db_holder: DbDropGuard,
        p2p: P2pClient,
    ) -> Self {
        Self {
            inner: Arc::new(NodeInner {
                db_holder,
                p2p,
            }),
        }
    }
}

impl Deref for Node {
    type Target = NodeInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct NodeInner {
    // A state machine that holds the state of the blockchain.
    db_holder: DbDropGuard,

    p2p: P2pClient,
}

impl Node {
    pub fn db(&self) -> Db {
        self.db_holder.db()
    }

    pub(crate) fn get(&self, key: &Bytes) -> Option<Bytes> {
        self.db().get(key)
    }

    pub(crate) fn set(&self, key: Bytes, value: Bytes, expire: Option<Duration>) {
        self.db().set(key, value, expire)
    }

    // pub fn next_account_nonce(&self, account: &str) -> u64 {
    //     self.state.next_account_nonce(account)
    // }

    pub async fn peer_basic(&self) -> Option<Vec<String>> {
        let known_peers = self.p2p.get_known_peers().await;
        if known_peers.len() > 0 {
            Some(known_peers)
        } else {
            None
        }
    }

}
