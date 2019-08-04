use serde_json::Map;
use jsonrpc_http_server::{Server, ServerBuilder};
use jsonrpc_core::{Params, Value, IoHandler};

use super::config::Config;
use crate::txpool::TxPool;


pub struct JsonRPC {
    server: Server,
    config: Config
}

impl JsonRPC {
    /// TODO: extend methods.
    pub fn new(conf: Config) -> Self {
        let mut io = IoHandler::new();
        io.add_method("sendRawTransaction", | p: Params | {
            // get values
            let d: Map<String, Value> = p.parse().unwrap();
            let code = d.get("code").unwrap().as_str().unwrap();
            let txpool = TxPool::default();

            match txpool.push(String::from(code)) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Ok(Value::String(e.to_string()))
            }
        });

        let lis = conf.listen.as_str();
        let server = ServerBuilder::new(io)
            .threads(conf.threads)
            .start_http(&lis.parse().unwrap())
            .unwrap();

        JsonRPC {
            server: server, config: conf
        }
    }

    pub fn run(self) {
        println!("rpc server listening at: {:?}", self.config.listen);
        self.server.wait();
    }
}

impl std::default::Default for JsonRPC {
    fn default() -> Self {
        JsonRPC::new(Config::default())
    }
}
