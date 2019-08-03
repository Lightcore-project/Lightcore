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
use libp2p::{
    Swarm,
    PeerId,
    identity,
    tcp::TcpConfig,
    build_development_transport
};
use futures::future::Future;
use libp2p::tcp;
use libp2p::secio;
use libp2p::yamux;
use libp2p::mplex;
use libp2p::core;
use libp2p::dns;
use libp2p::Transport;
use libp2p::core::{
    transport::TransportError,
    upgrade::{InboundUpgrade, InboundUpgradeExt, OutboundUpgrade, OutboundUpgradeExt}
};
use libp2p::websocket;
use std::io;
use std::error;
use libp2p::multiaddr::{Multiaddr, multiaddr as build_multiaddr};
use libp2p::kad::{Kademlia, KademliaConfig, KademliaEvent};
use libp2p::kad::record::store::MemoryStore;
use std::time::Duration;

/// Implementation of `Transport` that supports the most common protocols.
///
/// The list currently is TCP/IP, DNS, and WebSockets. However this list could change in the
/// future to get new transports.
#[derive(Debug, Clone)]
struct CommonTransport {
    // The actual implementation of everything.
    inner: CommonTransportInner
}

type InnerImplementation = dns::DnsConfig<tcp::TcpConfig>;

#[derive(Debug, Clone)]
struct CommonTransportInner {
    inner: InnerImplementation,
}

impl CommonTransport {
    /// Initializes the `CommonTransport`.
    pub fn new() -> CommonTransport {
        let tcp = tcp::TcpConfig::new().nodelay(true);
        let transport = dns::DnsConfig::new(tcp);

        CommonTransport {
            inner: CommonTransportInner { inner: transport }
        }
    }

}

impl Transport for CommonTransport {
    type Output = <InnerImplementation as Transport>::Output;
    type Error = <InnerImplementation as Transport>::Error;
    type Listener = <InnerImplementation as Transport>::Listener;
    type ListenerUpgrade = <InnerImplementation as Transport>::ListenerUpgrade;
    type Dial = <InnerImplementation as Transport>::Dial;

    fn listen_on(self, addr: Multiaddr) -> Result<Self::Listener, TransportError<Self::Error>> {
        self.inner.inner.listen_on(addr)
    }

    fn dial(self, addr: Multiaddr) -> Result<Self::Dial, TransportError<Self::Error>> {
        self.inner.inner.dial(addr)
    }
}

fn main() {
    env_logger::init();

    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("{:?}", local_peer_id);

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = CommonTransport::new()
        .with_upgrade(secio::SecioConfig::new(local_key))
        .and_then(move |output, endpoint| {
            let peer_id = output.remote_key.into_peer_id();
            let peer_id2 = peer_id.clone();
            let upgrade = core::upgrade::SelectUpgrade::new(yamux::Config::default(), mplex::MplexConfig::new())
                // TODO: use a single `.map` instead of two maps
                .map_inbound(move |muxer| (peer_id, muxer))
                .map_outbound(move |muxer| (peer_id2, muxer));
            core::upgrade::apply(output.stream, upgrade, endpoint)
                .map(|(id, muxer)| (id, core::muxing::StreamMuxerBox::new(muxer)))
        })
        .with_timeout(Duration::from_secs(20));
    // let transport = TcpConfig::new();

    // Create a swarm to manage peers and events.
    let mut swarm = {
        let mut cfg = KademliaConfig::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id.clone());
        let mut behaviour = Kademlia::with_config(local_peer_id.clone(), store, cfg);
        behaviour.add_address(&"QmQ679eN5YTsQFXvi8K7ZyPH6Q4bhZHXJG5LayMefTM6iH".parse().unwrap(), "/ip4/127.0.0.1/tcp/3000".parse().unwrap());
        behaviour.bootstrap();
        Swarm::new(transport, behaviour, local_peer_id)
    };

    libp2p::Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/3001".parse().unwrap()).unwrap();

    let r = swarm.for_each(|stream| {
        println!("{:?}", stream);
        Ok(())
    }).map_err(|err| {
        println!("{:?}",err);
    });


    tokio::run(r);
    
/*     let _trans = libp2p::swarm::Swarm::transport(&mut swarm).clone(); */
    //
    // _trans.dial("/ip4/127.0.0.1/tcp/3000".parse().unwrap()).unwrap();
    //
    // // Kick it off!
    // tokio::run(futures::future::poll_fn(move || {
    //     loop {
    //         match swarm.poll().expect("Error while polling swarm") {
    //             Async::Ready(Some(KademliaEvent::BootstrapResult(res))) => {
    //                 println!("Bootstrap {:?}",res);
    //             },
    //             Async::Ready(Some(res)) => {
    //                 println!("Event: {:?}",res);
    //             },
    //             Async::Ready(None) | Async::NotReady => break,
    //         }
    //     }
    //
    //     Ok(Async::NotReady)
    /* })); */
}
