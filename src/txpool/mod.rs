mod config;

use config::Config;
use sha3::{Digest, Sha3_256};
use quick_protobuf::{MessageRead, BytesReader, serialize_into_vec};
use crate::storage::{Storage, error::Error};
use crate::protobuf::tx::{Transaction, SignedTransaction};

/// # TxPool
/// txpool based on storage.
pub struct TxPool(pub Storage);

impl std::default::Default for TxPool {
    fn default() -> Self {
        let conf = Config::default();
        TxPool(Storage::new(conf.path))
    }
}

impl TxPool {
    fn verify(&self, stx: SignedTransaction) -> bool {
        let mut reader = BytesReader::from_bytes(&stx.tx);
        let tx = Transaction::from_reader(&mut reader, &stx.tx);
        
        let sig = ed25519_dalek::Signature::from_bytes(
            &stx.to_owned().signature.to_vec()
        ).unwrap();
                
        let public_key = ed25519_dalek::PublicKey::from_bytes(
            &base64::decode(&tx.unwrap().from).unwrap()
        ).unwrap();

        public_key.verify(&stx.tx.to_vec(), &sig).is_ok()
    }

    /// # todo:
    /// + verify signature
    /// + verify tx_id
    pub fn push(&self, code: String) -> std::result::Result<(), Error> {
        // decode stx
        let code_bytes = base64::decode(&code).unwrap();
        let mut reader = BytesReader::from_bytes(&code_bytes);
        let stx = SignedTransaction::from_reader(&mut reader, &code_bytes).unwrap();

        // if sig not pair return error.
        if !self.verify(stx.to_owned()) { return Err(Error::Illegal); }
        
        // gen hash
        let mut hasher = Sha3_256::new();
        hasher.input(&stx.tx.to_vec());
        
        // set in txpool
        let tx_id = hasher.result().to_vec();
        
        match self.0.set(tx_id, serialize_into_vec(&stx).unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => { Err(e) }
        }
    }

    pub fn pack(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, ()> {
        let conf = Config::default();

        match self.0.batch(conf.batch) {
            Ok(e) => Ok(e),
            Err(_) => Err(())
        }
    }
}
