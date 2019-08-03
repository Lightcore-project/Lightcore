use super::protocol;
use futures::prelude::*;
use libp2p::swarm::{
    KeepAlive,
    SubstreamProtocol,
    ProtocolsHandler,
    ProtocolsHandlerUpgrErr,
    ProtocolsHandlerEvent
};
use std::{error::Error, io, fmt, num::NonZeroU32, time::Duration};
use std::collections::VecDeque;
use tokio_io::{AsyncRead, AsyncWrite};
use wasm_timer::{Delay, Instant};
use void::Void;


#[derive(Clone, Debug)]
pub struct PingConfig {
    timeout: Duration,
    interval: Duration,
    /// The maximum number of failed outbound pings before the associated
    /// connection is deemed unhealthy, indicating to the `Swarm` that it
    /// should be closed.
    max_failures: NonZeroU32,
    /// Whether the connection should generally be kept alive unless
    /// `max_failures` occur.
    keep_alive: bool,
}

impl PingConfig {
    /// Creates a new `PingConfig` with the following default settings:
    ///
    ///   * [`PingConfig::with_interval`] 15s
    ///   * [`PingConfig::with_timeout`] 20s
    ///   * [`PingConfig::with_max_failures`] 1
    ///   * [`PingConfig::with_keep_alive`] false
    ///
    /// These settings have the following effect:
    ///
    ///   * A ping is sent every 15 seconds on a healthy connection.
    ///   * Every ping sent must yield a response within 20 seconds in order to
    ///     be successful.
    ///   * A single ping failure is sufficient for the connection to be subject
    ///     to being closed.
    ///   * The connection may be closed at any time as far as the ping protocol
    ///     is concerned, i.e. the ping protocol itself does not keep the
    ///     connection alive.
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(20),
            interval: Duration::from_secs(15),
            max_failures: NonZeroU32::new(1).expect("1 != 0"),
            keep_alive: false
        }
    }

    /// Sets whether the ping protocol itself should keep the connection alive,
    /// apart from the maximum allowed failures.
    ///
    /// By default, the ping protocol itself allows the connection to be closed
    /// at any time, i.e. in the absence of ping failures the connection lifetime
    /// is determined by other protocol handlers.
    ///
    /// If the maximum  number of allowed ping failures is reached, the
    /// connection is always terminated as a result of [`PingHandler::poll`]
    /// returning an error, regardless of the keep-alive setting.
    pub fn with_keep_alive(mut self, b: bool) -> Self {
        self.keep_alive = b;
        self
    }
}

/// The result of an inbound or outbound ping.
pub type PingResult = Result<PingSuccess, PingFailure>;

/// The successful result of processing an inbound or outbound ping.
#[derive(Debug)]
pub enum PingSuccess {
    /// Received a ping and sent back a pong.
    Pong,
    /// Sent a ping and received back a pong.
    ///
    /// Includes the round-trip time.
    Ping { rtt: Duration },
}

/// An outbound ping failure.
#[derive(Debug)]
pub enum PingFailure {
    /// The ping timed out, i.e. no response was received within the
    /// configured ping timeout.
    Timeout,
    /// The ping failed for reasons other than a timeout.
    Other { error: Box<dyn std::error::Error + Send + 'static> }
}

impl fmt::Display for PingFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PingFailure::Timeout => f.write_str("Ping timeout"),
            PingFailure::Other { error } => write!(f, "Ping error: {}", error)
        }
    }
}

impl Error for PingFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PingFailure::Timeout => None,
            PingFailure::Other { error } => Some(&**error)
        }
    }
}

/// Protocol handler that handles pinging the remote at a regular period
/// and answering ping queries.
///
/// If the remote doesn't respond, produces an error that closes the connection.
pub struct PingHandler<TSubstream> {
    /// Configuration options.
    config: PingConfig,
    /// The timer for when to send the next ping.
    next_ping: Delay,
    /// The pending results from inbound or outbound pings, ready
    /// to be `poll()`ed.
    pending_results: VecDeque<PingResult>,
    /// The number of consecutive ping failures that occurred.
    failures: u32,
    _marker: std::marker::PhantomData<TSubstream>
}

impl<TSubstream> PingHandler<TSubstream> {
    /// Builds a new `PingHandler` with the given configuration.
    pub fn new(config: PingConfig) -> Self {
        PingHandler {
            config,
            next_ping: Delay::new(Instant::now()),
            pending_results: VecDeque::with_capacity(2),
            failures: 0,
            _marker: std::marker::PhantomData
        }
    }
}

impl<TSubstream> ProtocolsHandler for PingHandler<TSubstream>
where
    TSubstream: AsyncRead + AsyncWrite,
{
    type InEvent = Void;
    type OutEvent = PingResult;
    type Error = PingFailure;
    type Substream = TSubstream;
    type InboundProtocol = protocol::Ping;
    type OutboundProtocol = protocol::Ping;
    type OutboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<protocol::Ping> {
        println!("[handler]: listen protocol...");
        SubstreamProtocol::new(protocol::Ping)
    }

    fn inject_fully_negotiated_inbound(&mut self, _: ()) {
        println!("[handler]: inject fully negotiated inbound...");
        // A ping from a remote peer has been answered.
        self.pending_results.push_front(Ok(PingSuccess::Pong));
    }

    fn inject_fully_negotiated_outbound(&mut self, rtt: Duration, _info: ()) {
        println!("[handler]: inject fully negotiated outbound...");
        // A ping initiated by the local peer was answered by the remote.
        self.pending_results.push_front(Ok(PingSuccess::Ping { rtt }));
    }

    fn inject_event(&mut self, _: Void) {}

    fn inject_dial_upgrade_error(&mut self, _info: (), error: ProtocolsHandlerUpgrErr<io::Error>) {
        println!("[handler]: inject dial upgrade error...");
        self.pending_results.push_front(
            Err(match error {
                ProtocolsHandlerUpgrErr::Timeout => PingFailure::Timeout,
                e => PingFailure::Other { error: Box::new(e) }
            }))
    }

    fn connection_keep_alive(&self) -> KeepAlive {
        if self.config.keep_alive {
            KeepAlive::Yes
        } else {
            KeepAlive::No
        }
    }

    fn poll(&mut self) -> Poll<ProtocolsHandlerEvent<protocol::Ping, (), PingResult>, Self::Error> {
        println!("[handler]: poll...");
        if let Some(result) = self.pending_results.pop_back() {
            if let Ok(PingSuccess::Ping { .. }) = result {
                let next_ping = Instant::now() + self.config.interval;
                self.failures = 0;
                self.next_ping.reset(next_ping);
            }
            if let Err(e) = result {
                self.failures += 1;
                if self.failures >= self.config.max_failures.get() {
                    return Err(e)
                } else {
                    return Ok(Async::Ready(ProtocolsHandlerEvent::Custom(Err(e))))
                }
            }
            return Ok(Async::Ready(ProtocolsHandlerEvent::Custom(result)))
        }

        match self.next_ping.poll() {
            Ok(Async::Ready(())) => {
                self.next_ping.reset(Instant::now() + self.config.timeout);
                let protocol = SubstreamProtocol::new(protocol::Ping)
                    .with_timeout(self.config.timeout);
                Ok(Async::Ready(ProtocolsHandlerEvent::OutboundSubstreamRequest {
                    protocol,
                    info: (),
                }))
            },
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(PingFailure::Other { error: Box::new(e) })
        }
    }
}
