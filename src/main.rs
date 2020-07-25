extern crate jsonrpc_derive;
extern crate jsonrpc_core;
extern crate jsonrpc_tcp_server;
extern crate bit_vec;
extern crate fasthash;

pub mod handler;
pub mod rpc;
pub mod bloom;


use jsonrpc_tcp_server::*;
use jsonrpc_tcp_server::jsonrpc_core::*;
use rpc::BloomRPC;





fn main() {
    let storage_rpc = handler::BloomFilter::new();
    let mut io = IoHandler::new();
    io.extend_with(storage_rpc.to_delegate());
    println!("Server starting up!!");
    let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait()
}
