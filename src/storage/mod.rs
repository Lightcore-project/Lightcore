mod config;
pub mod error;

use sled::Db;
use std::result::Result;
use std::path::PathBuf;
use error::Error;
use config::Config;

/// # Storage
/// basic struct for txpool and chain
///
/// ```
///   use lightcore::storage::Storage;
/// 
///   let store = Storage::default();
///   store.set(vec![0], vec![1]).unwrap();
///
///   let v = store.get(vec![0]).unwrap();
///   assert_eq!(v, &[1]);
///
///   store.clear();
///   store.flush();
/// ```
pub struct Storage(Db);

impl std::default::Default for Storage {
    fn default() -> Self {
        let conf = Config::default();
        Storage(Db::start_default(conf.path).unwrap())
    }
}

impl Storage {
    pub fn new(path: PathBuf) -> Self {
        Storage(Db::start_default(path).unwrap())
    }
    
    pub fn set(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Error> {
        if self.0.get(key.to_owned()).unwrap() != None {
            return Err(Error::Repeated);
        }
        
        match self.0.set(key, value) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SetError)
        }
    }

    pub fn get(&self, key: Vec<u8>) -> Result<Vec<u8>, Error> {
        match self.0.get(key) {
            Ok(e) => Ok(e.unwrap().to_vec()),
            Err(_) => Err(Error::GetError)
        }
    }

    pub fn del(&self, key: Vec<u8>) -> Result<(), Error> {
        match self.0.del(key) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::DeleteError)
        }
    }

    pub fn batch(&self, n: usize) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let mut arr = vec![];
        
        for i in self.0.iter().enumerate() {
            if i.to_owned().0 >= n {
                return Ok(arr)
            }
            arr.push((i.to_owned().1.unwrap().0, i.1.unwrap().1.to_vec()));
        }
        Ok(arr)
    }

    pub fn flush(&self) {
        self.0.flush().unwrap();
    }

    pub fn clear(&self) {
        self.0.clear().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::Storage;
    
    #[test]
    fn test_sgbdc() {
        let store = Storage::default();
        store.clear();
        store.flush();
        
        store.set(vec![0], vec![1]).unwrap();
        store.set(vec![1], vec![1]).unwrap();
        store.set(vec![2], vec![1]).unwrap();

        let v = store.get(vec![0]);
        assert_eq!(v.unwrap(), &[1]);

        let n = store.batch(3);
        assert_eq!(n.unwrap().len(), 3);

        let r = store.del(vec![0]);
        assert_eq!(r.is_ok(), true);

        store.clear();
        store.flush();
    }
}
