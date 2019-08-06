#![allow(unused_imports)]
use libp2p::identity::Keypair;
use libp2p::PeerId;
use libp2p::kad;
use libp2p::kad::record::store::MemoryStore;
use std::time::Duration;
use libp2p::Swarm;
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::boxed::Boxed;
use std::io;
use libp2p::kad::Kademlia;
use libp2p::core::muxing::SubstreamRef;
use std::sync::Arc;
use futures::prelude::Future;
use futures::prelude::Stream;
use libp2p::kad::KademliaEvent;
use std::collections::HashMap;
use libp2p::kad::GetClosestPeersError;
use libp2p::kad::GetClosestPeersOk;
use libp2p::kad::GetClosestPeersResult;
use futures::task::Task;

use super::transport::build_transport;
use super::discovery::DiscoveryFuture;
use super::config::Config;
use super::futures::FuturesInner;

pub struct Service{
    key_pair: Keypair,
    peer_id: PeerId,
    swarm: Swarm<Boxed<(PeerId, StreamMuxerBox), io::Error>, Kademlia<SubstreamRef<Arc<StreamMuxerBox>>, MemoryStore>>,
    discoverys: FuturesInner<GetClosestPeersResult>,
}

impl Service {
    pub fn new(_config: Config) -> Self {
        let key_pair = Keypair::generate_ed25519();
        let public = key_pair.public();
        let peer_id = PeerId::from(public);

        let transport = build_transport(key_pair.clone(), false);

        let mut swarm = {
            let mut cfg = kad::KademliaConfig::default();
            cfg.set_query_timeout(Duration::from_secs(5*60));
            let store = MemoryStore::new(peer_id.clone());
            let behaviour = kad::Kademlia::with_config(peer_id.clone(), store, cfg);
            Swarm::new(transport, behaviour, peer_id.clone())
        };

        libp2p::swarm::Swarm::listen_on(&mut swarm, _config.listen.parse().unwrap()).unwrap();

        if _config.bootstrap {
            let swarm1 = &mut swarm;
            for (id, addr) in _config.bootstrap_list.iter() {
                swarm1.add_address(&id.parse().unwrap(), addr.parse().unwrap());
            }
            swarm1.bootstrap();
        }

        println!("Peer id is: {:?}", peer_id);
        println!("Listen on: {:?}", _config.listen);

        Service {
            key_pair,
            peer_id,
            swarm,
            discoverys: FuturesInner::new(),
        }
    }

    pub fn run(self) -> impl Future<Item = (), Error = ()> {
        self.swarm.for_each(| event | {
            match event {
                KademliaEvent::GetClosestPeersResult(res) => {
                    println!("got closest peer result {:?}", res);
                },
                _ => {
                    println!("match other event {:?}", event);
                }
            }
            Ok(())
        }).map_err(| err | {
            println!("Error result: {:?}",err)
        })
    }

    pub fn dial(&mut self ,peer_id: PeerId) -> DiscoveryFuture {
        let peer_id1 = peer_id.clone();
        self.swarm.get_closest_peers(peer_id);
        let df = DiscoveryFuture::new(peer_id1, &mut (self.discoverys));
        df
    }
}

