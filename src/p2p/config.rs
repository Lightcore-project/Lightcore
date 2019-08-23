use std::collections::HashSet;

pub struct Config {
    pub hard_coded_contacts: Option<Vec<String>>,
    pub service_discovery_port: Option<u16>,
    pub bootstrap_cache_name: Option<String>,
    pub whitelisted_node_ips: Option<HashSet<String>>,
    pub whitelisted_client_ips: Option<HashSet<String>>,
    pub network_name: Option<String>,
}

impl Config {
    pub fn convert() -> crust::Config {
        let config = crust::Config::default();

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hard_coded_contacts: Some(Vec::new()),
            service_discovery_port: Some(9438),
            bootstrap_cache_name: Some(String::from("~/.lightcore/cache.dat")),
            whitelisted_node_ips: Some(HashSet::new()),
            whitelisted_client_ips: Some(HashSet::new()),
            network_name: Some(String::from("lightcore-mainnet")),
        }
    }
}

