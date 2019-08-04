use serde_json::Map;
use jsonrpc_http_server::{Server, ServerBuilder};
use jsonrpc_core::{Params, Value, IoHandler};
use quick_protobuf::{MessageRead, BytesReader};
use super::config::Config;
use crate::protobuf::tx::SignedTransaction;
use crate::crypto::Signature;


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
            let pk = d.get("pk").unwrap().as_str().unwrap();
            let code = d.get("code").unwrap().as_str().unwrap();

            // decode stx
            let code_bytes = base64::decode(code).unwrap();
            let mut reader = BytesReader::from_bytes(&code_bytes);
            let stx = SignedTransaction::from_reader(&mut reader, &code_bytes).unwrap();

            // verify signature
            let verify_sig = Signature::verify_tx(pk.to_string(), stx);

            match verify_sig {
                true => Ok(Value::Bool(true)),
                false => Ok(Value::Bool(false))
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
