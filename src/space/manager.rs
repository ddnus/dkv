use std::{collections::HashMap, path::Path, sync::{Arc, RwLock}};

use p2p::Peer;

use crate::error::Error;

use super::Space;

pub struct SpaceManager {
    path: Box<Path>,
    spaces: Arc<RwLock<HashMap<String, Vec<Arc<Space>>>>>,  // Arc<Space> for cheap cloning
}

impl SpaceManager {

    pub fn new(path: String) -> Self {
        Self { 
            path: Path::new(&path).join(".space"),
            spaces: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn check_exist(&self, peer: Peer, space_no: u32) -> Result<bool, Error> {
        // check if the space exist
        let path = self.path.join(peer.id.to_string()).join(space_no.to_string());
        if path.exists() {
            return Ok(true);
        }
        Err(Error::SpaceNotFound(format!("Space not found: {}", path.display())))
    }

    // pub fn get_closest_peers(&self) {
    //     self.p2p.blocking_request(target, request)
    // }

    // fn init_spaces(&self) -> Vec<Arc<Space>> {
    // }

    // Add a space (takes ownership)
    pub fn add_space(&self, identify: &[u8], space: Space) -> Arc<Space> {
        let space = Arc::new(space);
        let identify: String = String::from_utf8_lossy(&space.identify).into_owned();
        
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

}