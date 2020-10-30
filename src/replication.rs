//! Contains the code for message to be send to the remote server
//! The below given message is then translated into the appropriate rpc.
//! and forwarded to the respective peers.
use super::bloom::BloomCollection;
use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};

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

pub struct GossipController {
    listener: TcpListener,
    store: Arc<Mutex<BloomCollection>>,
}

pub struct ReplicationController {
    inner: mpsc::Receiver<Message>,
    peers: Option<Vec<SocketAddr>>,
    store: Arc<Mutex<BloomCollection>>,
}

impl ReplicationController {
    pub fn new(
        recv: mpsc::Receiver<Message>,
        store: Arc<Mutex<BloomCollection>>,
        peers: Option<Vec<SocketAddr>>,
    ) -> Self {
        ReplicationController {
            inner: recv,
            peers: peers,
            store: store,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        loop {
            let msg: Message = self.inner.recv().unwrap_or_default();
        }
    }
}

impl GossipController {
    pub fn new(store: Arc<Mutex<BloomCollection>>, gossip: u16) -> io::Result<Self> {
        Ok(GossipController {
            store,
            listener: TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), gossip))?,
        })
    }

    pub fn listen(&self) -> io::Result<()> {
        for incoming in self.listener.incoming() {
            println!("{:?}", incoming?);
        }
        Ok(())
    }
}
