use crate::jsonrpc::server::JsonRPC;

pub fn rpc() {
    let rpc = JsonRPC::default();
    rpc.run();
}
