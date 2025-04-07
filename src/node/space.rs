use std::sync::{Arc, RwLock};

use bytes::Bytes;
use crate::Error;

#[derive(Debug, Clone, Default)]
pub struct Space {
    pub identify: Bytes,
    pub name: Bytes,
    number: u16,
    space_no: Bytes,
    read_level: Bytes,
    write_level: Bytes,
    size: u64,
}

impl Space {

    fn new() -> Space {
        Space::default()
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