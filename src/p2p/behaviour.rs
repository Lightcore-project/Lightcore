use libp2p::PeerId;
use libp2p::swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters};
use libp2p::core::{Multiaddr, ConnectedPoint};
use void::Void;
use futures::Async;
use tokio_io::{AsyncRead, AsyncWrite};
use super::handler::Handler;

pub struct Behaviour<TSubStream> {
    events: std::collections::VecDeque<&'static [u8]>,
    marker: ::std::marker::PhantomData<TSubStream>
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
        Vec::new()
    }

    fn inject_connected(&mut self, _peer: PeerId, _: ConnectedPoint) {}

    fn inject_disconnected(&mut self, _peer: &PeerId, _: ConnectedPoint) {}

    fn inject_node_event(&mut self, _peer: PeerId, _: Void) {}

    fn poll(&mut self, _: &mut impl PollParameters) -> Async<NetworkBehaviourAction<Void, &'static [u8]>> {
        if let Some(e) = self.events.pop_back() {
            Async::Ready(NetworkBehaviourAction::GenerateEvent(e))
        } else {
            Async::NotReady
        }
    }
}

