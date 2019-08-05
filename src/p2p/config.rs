use std::collections::HashMap;

// #[derive(Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub bootstrap: bool,
    pub bootstrap_list: HashMap<String,String>,
}

impl Config {
    pub fn default () -> Self {
        Config {
            listen: String::from("/ip4/0.0.0.0/tcp/3000"),
            bootstrap: false,
            bootstrap_list: HashMap::new(),
        }
    }
}

