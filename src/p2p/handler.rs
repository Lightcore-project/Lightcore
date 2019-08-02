use tokio_io::{ AsyncRead, AsyncWrite };
use libp2p::swarm::{
    ProtocolsHandler,
    ProtocolsHandlerEvent,
    ProtocolsHandlerUpgrErr,
    KeepAlive,
    SubstreamProtocol,
    NetworkBehaviourAction,
};
use void::Void;
use futures::Async;
use std::io::Error;
use std::result::Result;
use std::marker::PhantomData;
use std::collections::VecDeque;
use super::error::NetworkError;
use super::protocol::Hello;

#[allow(dead_code)]
#[derive(Default)]
pub struct Handler<TSubStream> {
    pending_results: VecDeque<Result<&'static [u8], NetworkError>>,
    marker: PhantomData<TSubStream>
}

impl<TSubStream> Handler<TSubStream>
where TSubStream: AsyncRead + AsyncWrite {
    pub fn new() -> Self {
        Handler {
            pending_results: VecDeque::new(),
            marker: PhantomData
        }
    }
}

impl<TSubStream> ProtocolsHandler for Handler<TSubStream>
where TSubStream: AsyncRead + AsyncWrite {
    type InEvent = &'static [u8];
    type OutEvent = &'static [u8];
    type Error = NetworkError;
    type Substream = TSubStream;
    type InboundProtocol = Hello;
    type OutboundProtocol = Hello;
    type OutboundOpenInfo = &'static [u8];

    fn listen_protocol(&self) -> SubstreamProtocol<Hello> {
        SubstreamProtocol::new(Hello)
    }

    fn inject_fully_negotiated_inbound(&mut self, info: &'static [u8]) {
        self.pending_results.push_front(Ok(info))
    }

    fn inject_fully_negotiated_outbound(&mut self, _in: &'static [u8], info: &'static [u8]) {
        self.pending_results.push_front(Ok(info));
    }

    fn inject_event(&mut self, _: &'static [u8]) {
        println!("handler inject event...");
    }

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::Yes
    }

    fn inject_dial_upgrade_error(&mut self, _info: &'static [u8], error: ProtocolsHandlerUpgrErr<Error>) {
        self.pending_results.push_front(
            Err(
                match error {
                    ProtocolsHandlerUpgrErr::Timeout => NetworkError::Timeout,
                    e => NetworkError::Other { error: Box::new(e) }
            })
        )
    }
    
    fn poll(&mut self) -> Result<Async<ProtocolsHandlerEvent<Hello, &'static [u8], &'static [u8]>>, Self::Error> {
        if let Some(_e) = self.pending_results.pop_back() {
            Ok(Async::Ready(ProtocolsHandlerEvent::Custom(b"hello, world")))
        } else {
            Ok(Async::NotReady)
        }
    }
}
