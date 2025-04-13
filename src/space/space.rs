use std::sync::Arc;

use bytes::Bytes;

use crate::error::Error;

use super::kv::KV;


pub struct Space {
    pub meta: SpaceMeta,
    pub kv: Arc<KV>,
}

// 如何保护空间的写入权限
pub struct SpaceMeta {
    pub name: String, // 空间名
    pub id: u32, // 空间编号
    pub version: u64, // 更新版本
    pub identity: Vec<u8>, // 非对称加密
    pub authorization_code: Vec<u8>, // 授权码code
    pub expiration_time: u64, // 授权码过期时间
    pub read_level: u8, // 读级别
    pub write_level: u8, // 写级别
}


impl Space {

    fn new() -> Space {
        Space::default()
    }

    fn check_permission(&self, authorization_code: Vec<u8>) -> Result<bool, Error> {
        Ok(true)
    }

    fn get(&self, key: Bytes) -> Result<Option<Bytes>, Error> {
        Ok(None)
    }

    fn set(&self, key: Bytes, val: Bytes) -> Result<Option<Bytes>, Error> {
        Ok(None)
    }

    fn del(&self, key: Bytes) -> Result<bool, Error> {

        Ok(true)
    }

    fn get_size(&self) -> u64 {
        self.size
    }


}