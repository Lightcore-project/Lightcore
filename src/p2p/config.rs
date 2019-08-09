use quic_p2p::Config as P2PConfig;
use std::net::SocketAddr;
use base64::decode;
use quic_p2p::NodeInfo as QPNodeInfo;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub peer_addr: String,
    pub peer_cert_der: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub hard_coded_contacts: HashSet<NodeInfo>,
}

impl Config {
    pub fn default () -> Self {
        Config {
            listen: String::from("127.0.0.1:6581"),
            hard_coded_contacts: HashSet::new(),
        }
    }

    pub fn to_quic_p2p_config (self) -> P2PConfig {
        let mut config = P2PConfig::with_default_cert();

        // Listen address
        let listen: SocketAddr = self.listen.parse().unwrap();
        config.ip = Some(listen.ip());
        config.port = Some(listen.port());

        // Hard code nodes
        for info in self.hard_coded_contacts.iter() {
            let peer_addr = info.peer_addr.parse().unwrap();
            let peer_cert_der = decode(&info.peer_cert_der).unwrap();
            let i = QPNodeInfo {
                peer_addr,
                peer_cert_der,
            };
            config.hard_coded_contacts.insert(i);
        }
        
        config
    }
}
