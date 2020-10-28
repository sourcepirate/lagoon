// use jsonrpc_derive::rpc;
use super::bloom::{BloomCollection, BloomNode};
use super::replication::Message;
use super::rpc::BloomRPC;
use jsonrpc_core::{Error, ErrorCode, Result};
use std::sync::{mpsc, Arc, Mutex};

pub struct BloomFilter {
    inner: Arc<Mutex<BloomCollection>>,
    sender: Arc<Mutex<mpsc::Sender<Message>>>,
}

impl BloomRPC for BloomFilter {
    fn protocol_version(&self) -> Result<String> {
        Ok("v1".into())
    }

    fn create(&self, collection: String) -> Result<bool> {
        debug!("Create Collection -- {}", collection);
        let data = self.inner.clone();
        let sender_clone = self.sender.clone();
        let mut guard = data.lock().unwrap();
        let send_guard = sender_clone.lock().unwrap();
        match guard.create(
            collection.clone(),
            BloomNode::max_bits(),
            BloomNode::max_hash(),
        ) {
            Ok(_) => {
                send_guard.send(Message::Create(collection)).unwrap();
                Ok(true)
            }
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
        info!("receving message");
        let data = self.inner.clone();
        let sender_clone = self.sender.clone();
        let guard = data.lock().unwrap();
        let send_guard = sender_clone.lock().unwrap();
        match guard.set(collection.clone(), val.clone()) {
            Ok(_) => {
                send_guard.send(Message::Set(collection, val)).unwrap();
                Ok(true)
            }
            Err(_e) => Err(Error::new(ErrorCode::ServerError(_e.code()))),
        }
    }

    fn delete(&self, collection: String) -> Result<bool> {
        debug!("Delete collection -- {}", collection);
        let data = self.inner.clone();
        let sender_clone = self.sender.clone();
        let mut guard = data.lock().unwrap();
        let send_guard = sender_clone.lock().unwrap();
        match guard.delete(collection.clone()) {
            Ok(_) => {
                send_guard.send(Message::Delete(collection)).unwrap();
                Ok(true)
            }
            Err(_e) => Err(Error::new(ErrorCode::ServerError(_e.code()))),
        }
    }

    fn has_collection(&self, collection: String) -> Result<bool> {
        debug!("Has Collection -- {}", collection);
        let data = self.inner.clone();
        let mut guard = data.lock().unwrap();
        match guard.has_collection(collection) {
            Ok(flag) => Ok(flag),
            Err(_e) => Err(Error::new(ErrorCode::ServerError(_e.code()))),
        }
    }
}

impl BloomFilter {
    pub fn new(tx: mpsc::Sender<Message>) -> Self {
        BloomFilter {
            inner: Arc::new(Mutex::new(BloomCollection::new())),
            sender: Arc::new(Mutex::new(tx.clone())),
        }
    }
}
