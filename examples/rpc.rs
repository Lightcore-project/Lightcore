extern crate lightcore;

use lightcore::jsonrpc::server::JsonRPC;

fn main() {
    let server = JsonRPC::default();

    server.run()
}
