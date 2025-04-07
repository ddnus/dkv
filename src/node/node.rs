use std::{collections::HashMap, path::Path, sync::{Arc, RwLock}};

use bytes::Bytes;
use crate::{Error, P2pClient};
use super::{kv::KV, Space};

pub struct Node {
    p2p: P2pClient,
    data_path: String,
    kv: Arc<KV>,
    spaces: Arc<RwLock<HashMap<String, Vec<Arc<Space>>>>>,  // Arc<Space> for cheap cloning
}

impl Node {
    
    pub fn new(data_path: &str, p2p: P2pClient) -> Result<Self, Error> {
        let path = Path::new(data_path).join("ddnus@node");

        let kv = KV::new(&path)?;
        Ok(Self {
            p2p,
            data_path: data_path.to_string(),
            kv: Arc::new(kv),
            spaces: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // pub fn get_closest_peers(&self) {
    //     self.p2p.blocking_request(target, request)
    // }

    // fn init_spaces(&self) -> Vec<Arc<Space>> {
    // }

    // Add a space (takes ownership)
    pub fn add_space(&self, identify: &[u8], space: Space) -> Arc<Space> {
        let space = Arc::new(space);
        let identify = String::from_utf8_lossy(&space.identify).into_owned();
        
        let mut spaces = self.spaces.write().unwrap();
        spaces.entry(identify)
            .or_insert_with(Vec::new)
            .push(space.clone());
        
        space
    }

    // 根据 identify 和 name 查找特定 Space
    pub fn identify_space(&self, identity: &[u8], space_name: &[u8]) -> Option<Arc<Space>> {
        let identify = String::from_utf8_lossy(identity);
        self.spaces.read().unwrap()
            .get(identify.as_ref())
            .and_then(|spaces| {
                spaces.iter().find(|space| space.name.as_ref() == space_name)
            })
            .cloned()
    }

    // 获取特定 identify 的所有 Space
    pub fn get_identify_spaces(&self, identity: &[u8]) -> Vec<Arc<Space>> {
        let identify = String::from_utf8_lossy(identity);
        self.spaces.read().unwrap()
            .get(identify.as_ref())
            .map(|spaces| spaces.clone())
            .unwrap_or_default()
    }

    // 获取所有 Space（扁平化处理）
    pub fn get_all_spaces(&self) -> Vec<Arc<Space>> {
        self.spaces.read().unwrap()
            .values()
            .flat_map(|spaces| spaces.iter().cloned())
            .collect()
    }

    // 

}
