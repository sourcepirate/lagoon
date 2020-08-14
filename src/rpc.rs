use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc(server)]
pub trait BloomRPC {
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "createCollection")]
    fn create(&self, collection: String) -> Result<bool>;

    #[rpc(name = "hasKey")]
    fn has_key(&self, collection: String, val: String) -> Result<bool>;

    #[rpc(name = "setKey")]
    fn set_key(&self, collection: String, val: String) -> Result<bool>;

    #[rpc(name = "deleteCollection")]
    fn delete(&self, collection: String) -> Result<bool>;

    #[rpc(name = "hasCollection")]
    fn has_collection(&self, collection: String) -> Result<bool>;
}
