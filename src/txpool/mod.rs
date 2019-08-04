mod config;

use crate::storage::{Storage, error::Error};
use config::Config;

struct TxPool(Storage);

impl std::default::Default for TxPool {
    fn default() -> Self {
        let conf = Config::default();
        TxPool(Storage::new(conf.path))
    }
}

impl TxPool {
    /// # todo:
    /// + verify signature
    /// + verify tx_id
    fn push(&self, hash: &'static [u8], tx: &'static [u8]) -> std::result::Result<(), Error> {
        match self.0.set(hash, tx) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn pack(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        match self.0.batch() {
            Ok(e) => Ok(e),
            Err(e) => Err(e)
        }
    }
}
