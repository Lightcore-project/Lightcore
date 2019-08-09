use quic_p2p::QuicP2p;
use quic_p2p::NodeInfo;
use std::collections::HashMap;
use quic_p2p::Peer;
use quic_p2p::Builder;
use quic_p2p::Event;
use crossbeam_channel as mpmc;
use crossbeam_channel::Receiver;

use super::config::Config;

pub struct Service {
    p2p: QuicP2p,
    connected_peer: HashMap<NodeInfo, Peer>,
    our_info: NodeInfo,
    ev_rx: Receiver<Event>,
}

impl Service {
    pub fn new(_config: Config) -> Self {
        let qpconfig = _config.to_quic_p2p_config();

        let (ev_tx, ev_rx) = mpmc::unbounded();
        let mut p2p = Builder::new(ev_tx)
            .with_config(qpconfig)
            .build().unwrap();
        let our_info = p2p.our_connection_info().unwrap();
        Service {
            p2p,
            our_info,
            connected_peer: HashMap::new(),
            ev_rx,
        }
    }
    
    pub fn run(self) {
        for event in self.ev_rx.iter() {
            println!("{:?}", event);
        }
    }
}


