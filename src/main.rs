#[macro_use] extern crate log;
extern crate bit_vec;
extern crate fasthash;
extern crate jsonrpc_core;
extern crate jsonrpc_derive;
extern crate jsonrpc_tcp_server;

pub mod bloom;
pub mod handler;
pub mod rpc;

use jsonrpc_tcp_server::jsonrpc_core::*;
use jsonrpc_tcp_server::*;
use rpc::BloomRPC;

fn main() {
    env_logger::init();
    let storage_rpc = handler::BloomFilter::new();
    let mut io = IoHandler::new();
    io.extend_with(storage_rpc.to_delegate());
    info!("Server starting up!!");
    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:3030".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait()
}
