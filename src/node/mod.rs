
use std::{ops::Deref, sync::Arc, time::{Duration, SystemTime}};

use bytes::Bytes;
use p2p::Peer;

use crate::{db::{Db, DbDropGuard}, error::Error, space::{Accounter, Account}, P2pClient};

#[derive(Debug, Clone)]
pub struct Node {
    inner: Arc<NodeInner>,
}

impl Node {
    pub fn new(
        db_holder: DbDropGuard,
        p2p: P2pClient,
        peer: Peer,
        accounter: Accounter,
    ) -> Self {
        Self {
            inner: Arc::new(NodeInner {
                db_holder,
                p2p,
                peer,
                accounter,
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
    peer: Peer,
    accounter: Accounter,
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
        // 同步到其他节点
    }

    // pub fn next_account_nonce(&self, account: &str) -> u64 {
    //     self.state.next_account_nonce(account)
    // }

    // 注册帐号
    pub async fn register_account(&self, identity: Vec<u8>) -> Result<Vec<u8>, Error> {
        // 获取离帐号最近的一批节点
        let peers = self.p2p.get_closet_peer(self.peer.id.to_bytes()).await?;
        // 依此检测是否已经注册该帐号的space0
        peers.iter().for_each(|peer_id| {
            // 发送请求，获取帐号的space0
            
            // 如果存在，返回space0
        });
        // 如果不存在，取最近的n个节点

        // 初始化帐号信息
        // 依此初始化space信息，返回写入成功信息
        // 
        Ok(Vec::new())
    }

    pub async fn get_account(&self, name: Vec<u8>) -> Result<Account, Error> {
        // self.db().get_account(self.peer.id.to_bytes())
        self.accounter.get_account(name)
    }

    pub async fn login_account(&self, identity: Vec<u8>) {
        // 获取离帐号最近的一批节点
        // 依此获取帐号最新版本的系统空间
    }

    // 申请空间
    pub async fn apply_space(&self, identity: Vec<u8>) {
        // 获取离帐号最近的一批节点
        // 依此获取帐号最新版本的系统空间
        
        // 判断新空间spaceX是否存在
        // 如果不存在，取离空间最近的n个节点
        // 依此写入space信息，返回写入成功信息
    }
    // 

    

}
