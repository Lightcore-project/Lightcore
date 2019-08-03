mod error;
mod config;

use sled::Db;
use std::result::Result;
use std::default::Default;
use error::Error;
use config::Config;

trait StorageTrait {
    fn set(&self, key: &'static [u8], value: &'static [u8]) -> Result<(), Error>;
    fn get(&self, key: &'static [u8]) -> Result<Vec<u8>, Error>;
    fn batch(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error>;
}

struct Storage(Db);

impl Default for Storage {
    fn default() -> Self {
        let conf = Config::default();
        Storage(Db::start_default(conf.path).unwrap())
    }
}

#[allow(unused_variables)]
impl StorageTrait for Storage {
    fn set(&self, key: &'static [u8], value: &'static [u8]) -> Result<(), Error> {
        if self.0.get(key).is_ok() {
            return Err(Error::Repeated);
        }
        
        match self.0.set(key, value) {
            Ok(e) => Ok(()),
            Err(e) => Err(Error::SetError)
        }
    }

    fn get(&self, key: &'static [u8]) -> Result<Vec<u8>, Error> {
        match self.0.get(key) {
            Ok(e) => Ok(e.unwrap().to_vec()),
            Err(e) => Err(Error::GetError)
        }
    }

    fn batch(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
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
