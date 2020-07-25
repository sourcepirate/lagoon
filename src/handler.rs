// use jsonrpc_derive::rpc;
use super::bloom::{BloomCollection, BloomNode};
use super::rpc::BloomRPC;
use jsonrpc_core::{Error, ErrorCode, Result};
use std::sync::{Arc, Mutex};

pub struct BloomFilter {
    inner: Arc<Mutex<BloomCollection>>,
}

impl BloomRPC for BloomFilter {
    fn protocol_version(&self) -> Result<String> {
        Ok("v1".into())
    }

    fn create(&self, collection: String) -> Result<bool> {
        let data = self.inner.clone();
        let mut guard = data.lock().unwrap();
        match guard.create(collection, BloomNode::max_bits(), BloomNode::max_hash()) {
            Ok(_) => Ok(true),
            Err(_e) => Err(Error::new(ErrorCode::ServerError(_e.code()))),
        }
    }

    fn has_key(&self, collection: String, val: String) -> Result<bool> {
        let data = self.inner.clone();
        let guard = data.lock().unwrap();
        match guard.exist(collection, val) {
            Ok(flag) => Ok(flag),
            Err(_e) => Err(Error::new(ErrorCode::ServerError(_e.code()))),
        }
    }

    fn set_key(&self, collection: String, val: String) -> Result<bool> {
        let data = self.inner.clone();
        let guard = data.lock().unwrap();
        match guard.set(collection, val) {
            Ok(_) => Ok(true),
            Err(_e) => Err(Error::new(ErrorCode::ServerError(_e.code()))),
        }
    }
}

impl BloomFilter {
    pub fn new() -> Self {
        BloomFilter {
            inner: Arc::new(Mutex::new(BloomCollection::new())),
        }
    }
}
