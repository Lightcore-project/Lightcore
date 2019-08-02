use void::Void;
use futures::Async;
use tokio_io::{ AsyncRead, AsyncWrite };
use libp2p::PeerId;
use libp2p::core::{ Multiaddr, ConnectedPoint };
use libp2p::swarm::{ NetworkBehaviour, NetworkBehaviourAction, PollParameters };
use std::collections::VecDeque;
use std::marker::PhantomData;
use super::handler::Handler;

pub struct Behaviour<TSubStream> {
    events: VecDeque<&'static [u8]>,
    marker: PhantomData<TSubStream>
}

impl<TSubStream> Behaviour<TSubStream>
where TSubStream: AsyncRead + AsyncWrite {
    pub fn new() -> Self {
        Behaviour {
            events: std::collections::VecDeque::new(),
            marker: std::marker::PhantomData
        }
    }
}

impl<TSubStream> NetworkBehaviour for Behaviour<TSubStream>
where TSubStream: AsyncRead + AsyncWrite {
    type ProtocolsHandler = Handler<TSubStream>;
    type OutEvent = &'static [u8];

    fn new_handler(&mut self) -> Handler<TSubStream> {
        Handler::new()
    }

    fn addresses_of_peer(&mut self, _peer_id: &PeerId) -> Vec<Multiaddr> {
        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/3001".parse().unwrap();
        vec![addr]
    }

    fn inject_connected(&mut self, peer_id: PeerId, _: ConnectedPoint) {
        println!("{:?} connect", peer_id);
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId, _: ConnectedPoint) {
        println!("{:?} disconnect", peer_id);
    }

    fn inject_node_event(&mut self, peer_id: PeerId, _: &'static [u8]) {
        println!("inject node {:?}", peer_id);
    }

    fn inject_dial_failure(&mut self, peer_id: &PeerId) {
        println!("dial failed, {:?}", peer_id);
    }

    fn poll(&mut self, _: &mut impl PollParameters) -> Async<NetworkBehaviourAction<&'static [u8], &'static [u8]>> {
        if let Some(e) = self.events.pop_back() {
            Async::Ready(NetworkBehaviourAction::GenerateEvent(e))
        } else {
            Async::NotReady
        }
    }
}
