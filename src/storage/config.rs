use std::path::PathBuf;
use std::default::Default;

pub(super) struct Config {
    pub path: PathBuf,
    pub batch: usize,
}

impl Default for Config {
    fn default() -> Self {
        let mut data_dir = dirs::data_dir().unwrap();
        data_dir.push("lightcore");
        if data_dir.exists() == false {
            ::std::fs::create_dir(data_dir.to_owned()).unwrap();
        }

        data_dir.push("storage");
        
        Config {
            path: data_dir,
            batch: 7
        }
    }
}
