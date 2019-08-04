pub mod config;
pub mod server;

use config::Config;
use crate::protobuf::tx::SignedTransaction;

use jsonrpc_core::IoHandler;
use jsonrpc_core::Params;
use jsonrpc_core::Value;
use serde_json::Map;
use jsonrpc_http_server::Server;
use jsonrpc_http_server::ServerBuilder;
use quick_protobuf::{MessageRead, BytesReader};
use tokio::runtime::{Runtime, TaskExecutor};

pub struct JsonRPCServer {
    pub server: Server,
}

impl std::default::Default for JsonRPCServer {
    fn default() -> Self {
        let rt = Runtime::new().unwrap();
        let conf = Config::default();
        
        JsonRPCServer::new(&conf, rt.executor())
    }
}

impl JsonRPCServer {
    pub fn new(_config: &Config,
               _execute: TaskExecutor
               ) -> Self {
        let mut io = IoHandler::new();
        io.add_method("sendRawTransaction", | p: Params | {
            let d: Map<String, Value> = p.parse().unwrap();
            let code = d.get("code").unwrap().as_str().unwrap();
            let code_bytes = base64::decode(code).unwrap();

            let mut reader = BytesReader::from_bytes(&code_bytes);
            let stx = SignedTransaction::from_reader(&mut reader, &code_bytes).unwrap();
            let sign = base64::encode(&stx.signature).to_string();

            println!("{:?}", &stx);
            Ok(Value::String(sign))
        });

        let lis = _config.listen.as_str();
        let server = ServerBuilder::new(io)
            .event_loop_executor(_execute)
            .start_http(&lis.parse().unwrap()).unwrap();
        JsonRPCServer {
            server,
        }
    }

    pub fn wait(self) {
        self.server.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::JsonRPCServer;
    use tokio::runtime::Runtime;
    use super::Config;
    use tokio::prelude::*;

    #[test]
    fn test_jsonrpc() {
        let rt = Runtime::new().unwrap();
        let config = Config::default();
        let server = JsonRPCServer::new(&config, rt.executor());
        rt.shutdown_on_idle()
            .wait().unwrap();
    //    server.wait();
    }
}
