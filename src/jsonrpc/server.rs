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
    pub fn io() -> IoHandler {
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

        io
    }
    
    /// TODO: extend methods.
    pub fn new(conf: Config) -> Self {
        let io = JsonRPC::io();
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

// TODO: 
// test io;
// #[cfg(test)]
// mod tests {
//     use super::JsonRPC;
//     use crate::txpool::TxPool;
//     use jsonrpc_core::futures::Future;
//     
//     //  #[test]
//     fn signature() {
//         let txpool = TxPool::default();
//         txpool.0.clear();
//         txpool.0.flush();
// 
//         let io = JsonRPC::io();
//         let request = r#"{ "jsonrpc": "2.0", "method": "sendRawTransaction", "params": { "code": "CkC3w4fBnydctlE6b7yShjzmaO2bMLsWj9wbNWFNTMVIHyigRRpZ03vjjFleKZwmwvR46FKM5iPphmY46YSYmuYDElYKCW9iamVjdF9pZBAqGixqbm1odHRGZTduZ083eXB3SnY2MkZ6OUZPc1NFWGFRNXV3RW1mRVRZWUNVPSICdG8qADITCgZkb2NfaWQQARoHY29udGVudA==" }, "id": 1 }"#;
//         let response = r#"{ "jsonrpc": "2.0", "result": "true", "id": "1" }"#;
// 
//         assert_eq!(io.handle_request(request).wait().unwrap(), Some(response.to_string()));
//         txpool.0.flush();
//         txpool.0.clear();
//     }
// }
