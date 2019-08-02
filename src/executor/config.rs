#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
}

impl Config {
    pub fn default () -> Self {
        Config {
            path: String::from("state.db"),
        }
    }
}

