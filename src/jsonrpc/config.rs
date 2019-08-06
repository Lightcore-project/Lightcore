#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub threads: usize,
    pub allow_cors: bool,
}

impl Config {
    pub fn default () -> Self {
        Config {
            listen: String::from("127.0.0.1:9432"),
            threads: 3_usize,
            allow_cors: true,
        }
    }
}
