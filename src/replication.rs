//! Contains the code for message to be send to the remote server
//! The below given message is then translated into the appropriate rpc.
//! and forwarded to the respective peers.
use jsonrpc_core::Value;
use jsonrpc_lite::{JsonRpc, Params};
use serde_json::to_string;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Message {
    /// Message for setting bits over a given collection
    Set(String, String),

    /// Creats a new collection
    Create(String),

    // Delete a given collection
    Delete(String),

    // Empty message for heartbeat
    None,
}

impl Default for Message {
    fn default() -> Self {
        Message::None
    }
}

impl Into<JsonRpc> for Message {
    fn into(self) -> JsonRpc {
        match &self {
            &Message::Create(ref collection) => JsonRpc::request_with_params(
                1,
                "createCollection",
                Params::Array(vec![Value::String(collection.clone())]),
            ),
            &Message::Set(ref collection, ref bitval) => JsonRpc::request_with_params(
                1,
                "setKey",
                Params::Array(vec![
                    Value::String(collection.clone()),
                    Value::String(bitval.clone()),
                ]),
            ),
            &Message::Delete(ref collection) => JsonRpc::request_with_params(
                1,
                "deleteCollection",
                Params::Array(vec![Value::String(collection.clone())]),
            ),
            &Message::None => JsonRpc::request_with_params(1, "version", Params::Array(vec![])),
        }
    }
}

pub struct ReplicationController {
    inner: mpsc::Receiver<Message>,
    peers: Option<Vec<SocketAddr>>,
}

impl ReplicationController {
    pub fn new(recv: mpsc::Receiver<Message>, peers: Option<Vec<SocketAddr>>) -> Self {
        ReplicationController {
            inner: recv,
            peers: peers,
        }
    }

    pub fn run(&self) -> () {
        let mut connections: Vec<TcpStream> = Vec::new();
        if self.peers.is_some() {
            for peer in self.peers.as_ref().unwrap() {
                connections.push(TcpStream::connect(peer).unwrap())
            }
        }
        loop {
            let msg: Message = self.inner.recv().unwrap_or_default();
            let rpc: JsonRpc = msg.into();
            for connection in connections.iter_mut() {
                if let &JsonRpc::Request(ref _request) = &rpc {
                    let str_value = to_string(&_request).unwrap() + "\n";
                    info!("{}", str_value);
                    connection.write(str_value.as_bytes()).unwrap();
                    connection.flush().unwrap();
                    info!("replication message");
                }
            }
        }
    }
}
