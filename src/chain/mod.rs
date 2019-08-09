mod block;
mod merkle;
mod config;

use config::Config;
use ed25519_dalek::SecretKey;
use quick_protobuf::{serialize_into_vec, deserialize_from_slice};
use crate::txpool::TxPool;
use crate::storage::{Storage, error::Error};
use crate::protobuf::tx::SignedTransaction;


/// # TODO:
/// + block iter
pub struct Chain(pub Storage);

impl std::default::Default for Chain {
    fn default() -> Self {
        let conf = Config::default();
        Chain(Storage::new(conf.path))
    }
}

impl Chain {
    fn new_block(&self, sk: SecretKey) -> Result<(), Error> {
        let txpool = TxPool::default();
        let stxs = txpool.pack().unwrap();

        let mut stxs_vec: Vec<SignedTransaction> = vec![];
        for i in stxs.iter() {
            stxs_vec.push(deserialize_from_slice(&i.1).unwrap());
        }
        
        let block = block::new_block(sk, stxs_vec);
        match self.0.set(block.id.to_vec(), serialize_into_vec(&block).unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => { Err(e) }
        }
    }
    
    fn batch(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, ()> {
        let conf = Config::default();
        
        match self.0.batch(conf.batch) {
            Ok(e) => Ok(e),
            Err(_) => Err(())
        }
    }
}
