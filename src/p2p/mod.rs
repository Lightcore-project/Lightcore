#![allow(dead_code)]
#![allow(unused_imports)]
mod behaviour;
mod handler;
mod error;
mod protocol;

use futures::prelude::*;
use libp2p::{
    Swarm,
    PeerId,
    identity,
    build_development_transport
};
use libp2p::kad::{ Kademlia, KademliaConfig, KademliaEvent, GetClosestPeersError };
use libp2p::kad::record::store::MemoryStore;
use std::time::Duration;

struct Node {
    keypair: identity::Keypair,
    peer_id: PeerId,
}

impl Node {
    fn new() -> Node {
        let kp = identity::Keypair::generate_ed25519();
        println!("{:?}", kp.public().into_peer_id());
        Node {
            keypair: kp.to_owned(),
            peer_id: PeerId::from(kp.public())
        }
    }
}


use behaviour::Behaviour;
pub fn main() {
    env_logger::init();
    
    let node = Node::new();

    // Manage peers and events.
    let mut swarm = {
        let transport = build_development_transport(node.keypair);
        let behaviour = Behaviour::new();

        // let mut cfg = KademliaConfig::default();
        // cfg.set_query_timeout(Duration::from_secs(5 * 60));
        // let store = MemoryStore::new(node.peer_id.clone());
        // let mut behaviour = Kademlia::with_config(node.peer_id.clone(), store, cfg);

        // The only address that currently works.
        // behaviour.add_address(
        //     &"QmZtJZ8i6sdALHTvLn161uETrNEvVYXorHBwDAkMjg2nvD".parse().unwrap(),
        //     "/ip4/127.0.0.1/tcp/3000".parse().unwrap()
        // );

        Swarm::new(transport, behaviour, node.peer_id)
    };

    let addr: libp2p::Multiaddr = "/ip4/127.0.0.1/tcp/3000".parse().unwrap();
    libp2p::swarm::ExpandedSwarm::listen_on(&mut swarm, addr).unwrap();
    println!("listen on 3000...");

    libp2p::swarm::ExpandedSwarm::dial_addr(
        &mut swarm,
        "/ip4/127.0.0.1/tcp/3001".parse().unwrap()
    );
    
    
    // Kick it off!
    tokio::run(futures::future::poll_fn(move || {
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(None) => {
                    println!("not ready");
                },
                Async::NotReady => {
                    // println!("not ready")
                },
                Async::Ready(Some(msg)) => {
                    println!("{:?}", msg);
                },
                _ => {}
                // Async::Ready(Some(KademliaEvent::GetClosestPeersResult(res))) => {
                //     match res {
                //         Ok(ok) => {
                //             if !ok.peers.is_empty() {
                //                 println!("Query finished with closest peers: {:#?}", ok.peers);
                //                 return Ok(Async::Ready(()));
                //             } else {
                //                 // The example is considered failed as there
                //                 // should always be at least 1 reachable peer.
                //                 panic!("Query finished with no closest peers.");
                //             }
                //         }
                //         Err(GetClosestPeersError::Timeout { peers, .. }) => {
                //             if !peers.is_empty() {
                //                 println!("Query timed out with closest peers: {:#?}", peers);
                //                 return Ok(Async::Ready(()));
                //             } else {
                //                 // The example is considered failed as there
                //                 // should always be at least 1 reachable peer.
                //                 panic!("Query timed out with no closest peers.");
                //             }
                //         }
                //     }
                // },
                // Async::Ready(Some(_)) => {},
                // Async::Ready(None) | Async::NotReady => break,
            }
        }

        // Ok(Async::NotReady)
    }));
}
