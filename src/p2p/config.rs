use quic_p2p::Config as P2PConfig;
use std::net::SocketAddr;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub peer_addr: String,
    pub peer_cert_der: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub hard_coded_contacts: Vec<NodeInfo>,
}

impl Config {
    pub fn default () -> Self {
        Config {
            listen: String::from("127.0.0.1:6581"),
            hard_coded_contacts: Vec::new(),
        }
    }

    pub fn to_quic_p2p_config (self) -> P2PConfig {
        let mut config = P2PConfig::read_or_construct_default(Option::None).unwrap();

        // Listen address
        let listen: SocketAddr = self.listen.parse().unwrap();
        config.ip = Some(listen.ip());
        config.port = Some(listen.port());

        // Hard code nodes
        for info in self.hard_coded_contacts.iter() {

        }
        
        config
    }
}
