mod ping;
mod protocol;
mod handler;

use ping::{ Ping, PingConfig };

use futures::{prelude::*, future};
use libp2p::{ identity, PeerId, Swarm };
use std::env;

fn main() {
    env_logger::init();

    // Create a random PeerId.
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Create a transport.
    let transport = libp2p::build_development_transport(id_keys);

    // Create a ping network behaviour.
    //
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    // Create a Swarm that establishes connections through the given transport
    // and applies the ping behaviour on each connection.
    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = env::args().nth(1) {
        let remote_addr = addr.clone();
        match addr.parse() {
            Ok(remote) => {
                match Swarm::dial_addr(&mut swarm, remote) {
                    Ok(()) => println!("Dialed {:?}", remote_addr),
                    Err(e) => println!("Dialing {:?} failed with: {:?}", remote_addr, e)
                }
            },
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    // Tell the swarm to listenp on all interfaces and a random, OS-assigned port.
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Use tokio to drive the `Swarm`.
    let mut listening = false;
    tokio::run(future::poll_fn(move || -> Result<_, ()> {
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(e)) => println!("{:?}", e),
                Async::Ready(None) | Async::NotReady => {
                    if !listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {:?}", a);
                            listening = true;
                        }
                    }
                    return Ok(Async::NotReady)
                }
            }
        }
    }));
}
