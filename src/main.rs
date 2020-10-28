#[macro_use]
extern crate log;
extern crate bit_vec;
extern crate fasthash;
extern crate jsonrpc_core;
extern crate jsonrpc_derive;
extern crate jsonrpc_lite;
extern crate jsonrpc_tcp_server;
extern crate serde_json;
extern crate structopt;

pub mod bloom;
pub mod handler;
pub mod replication;
pub mod rpc;

use jsonrpc_tcp_server::jsonrpc_core::*;
use jsonrpc_tcp_server::*;
use rpc::BloomRPC;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lagoon")]
struct Opt {
    addr: SocketAddr,
    #[structopt(short = "p", long = "peers")]
    peers: Option<Vec<SocketAddr>>,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            addr: FromStr::from_str("0.0.0.0:3030").unwrap(),
            peers: None,
        }
    }
}

fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    let (tx, rx) = channel::<replication::Message>();
    let storage_rpc = handler::BloomFilter::new(tx);
    let repc = replication::ReplicationController::new(rx, opt.peers);
    thread::spawn(move || {
        info!("starting the background control loop");
        repc.run();
    });
    let mut io = IoHandler::new();
    io.extend_with(storage_rpc.to_delegate());
    info!("Server starting up!!");
    let server = ServerBuilder::new(io)
        .start(&opt.addr)
        .expect("Server must start with no issues");
    server.wait()
}
