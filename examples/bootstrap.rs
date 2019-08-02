// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Demonstrates how to perform Kademlia queries on the IPFS network.
//!
//! You can pass as parameter a base58 peer ID to search for. If you don't pass any parameter, a
//! peer ID will be generated randomly.

use futures::prelude::*;
use futures::future::Future;
use libp2p::{
    Swarm,
    PeerId,
    identity,
    build_development_transport
};
use libp2p::kad::{Kademlia, KademliaConfig, KademliaEvent};
use libp2p::kad::record::store::MemoryStore;
use libp2p::Transport;
use std::time::Duration;

fn main() {
    env_logger::init();

    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("{:?}", local_peer_id);

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = build_development_transport(local_key);

    // Create a swarm to manage peers and events.
    let mut swarm = {
        let mut cfg = KademliaConfig::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id.clone());
        let mut behaviour = Kademlia::with_config(local_peer_id.clone(), store, cfg);
        behaviour.add_address(&"QmY6GjFhaQAyNeVtoafaNVyWt7oN2oZvfj8G5kx2q1kXXU".parse().unwrap(), "/ip4/0.0.0.0/tcp/3000".parse().unwrap());
        behaviour.bootstrap();
        Swarm::new(transport, behaviour, local_peer_id)
    };

    libp2p::Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/3001".parse().unwrap()).unwrap();

    let _trans = libp2p::swarm::ExpandedSwarm::transport(&mut swarm);


    // Kick it off!
    tokio::run(futures::future::poll_fn(move || {
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(KademliaEvent::BootstrapResult(res))) => {
                    println!("bootstrap {:?}",res);
                },
                Async::Ready(Some(res)) => {
                    // transport.dial();
                    println!("Event: {:?}",res);
                },
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        Ok(Async::NotReady)
    }));
}
