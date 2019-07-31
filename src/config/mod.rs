use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Config {
    test: String,
}

impl Config {
    fn default (path: &Path) -> Self {
        let mut file = File::create(path).unwrap();
        let config = Config {
            test: String::from("hello"),
        };
        let serialized = toml::to_string(&config).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
        config
    }

    fn new (path: &Path) -> Self {
        let file = File::open(path);
        match file {
            Ok(mut f) => {
                let mut jsonstr = String::new();
                f.read_to_string(&mut jsonstr).unwrap();
                toml::from_str(jsonstr.as_str()).unwrap()
            },
            Err(_) => {
                Config::default(path)
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Path;

    #[test]
    fn test_new() {
        let config = Config::new(Path::new("/home/tiannian/workspace/Alpaca/lightcore/config.toml"));
        println!("{}", config.test);
    }
}

