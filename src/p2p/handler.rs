use tokio_io::{AsyncRead, AsyncWrite};
use libp2p::swarm::{
    KeepAlive,
    ProtocolsHandler,
    ProtocolsHandlerEvent,
    ProtocolsHandlerUpgrErr,
    SubstreamProtocol
};
use void::Void;
use super::error::NetworkError;
use super::protocol::Hello;

#[allow(dead_code)]
#[derive(Default)]
pub struct Handler<TSubStream> {
    pending_results: std::collections::VecDeque<std::result::Result<&'static [u8], NetworkError>>,
    marker: std::marker::PhantomData<TSubStream>
}

impl<TSubStream> Handler<TSubStream>
where TSubStream: AsyncRead + AsyncWrite {
    pub fn new() -> Self {
        Handler {
            pending_results: std::collections::VecDeque::new(),
            marker: std::marker::PhantomData
        }
    }
}

impl<TSubStream> ProtocolsHandler for Handler<TSubStream>
where TSubStream: AsyncRead + AsyncWrite {
    type InEvent = Void;
    type OutEvent = Void;
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

    fn inject_event(&mut self, _: Void) {}

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::Yes
    }

    fn inject_dial_upgrade_error(&mut self, _info: &'static [u8], error: ProtocolsHandlerUpgrErr<std::io::Error>) {
        self.pending_results.push_front(
            Err(
                match error {
                    ProtocolsHandlerUpgrErr::Timeout => NetworkError::Timeout,
                    e => NetworkError::Other { error: Box::new(e) }
            })
        )
    }
    
    fn poll(&mut self) -> std::result::Result<futures::Async<ProtocolsHandlerEvent<Hello, &'static [u8], Void>>, Self::Error> {
        unimplemented!();
    }
}
