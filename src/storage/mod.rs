mod config;
pub mod error;

use sled::Db;
use std::result::Result;
use std::path::PathBuf;
use error::Error;
use config::Config;

/// # Storage
/// basic struct for storage models
pub struct Storage(Db);

impl std::default::Default for Storage {
    fn default() -> Self {
        let conf = Config::default();
        Storage(Db::start_default(conf.path).unwrap())
    }
}

impl Storage {
    fn get(&self, key: &'static [u8]) -> Result<Vec<u8>, Error> {
        match self.0.get(key) {
            Ok(e) => Ok(e.unwrap().to_vec()),
            Err(_) => Err(Error::GetError)
        }
    }
    
    pub(crate) fn new(path: PathBuf) -> Self {
        Storage(Db::start_default(path).unwrap())
    }
    
    pub(crate) fn set(&self, key: &'static [u8], value: &'static [u8]) -> Result<(), Error> {
        if self.0.get(key).is_ok() {
            return Err(Error::Repeated);
        }
        
        match self.0.set(key, value) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SetError)
        }
    }

    pub(crate) fn batch(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let mut arr = vec![];
        let conf = Config::default();
        
        for i in self.0.iter().enumerate() {
            if i.to_owned().0 >= conf.batch {
                return Err(Error::BatchError);
            }
            arr.push((i.to_owned().1.unwrap().0, i.1.unwrap().1.to_vec()));
        }
        Ok(arr)
    }
}
