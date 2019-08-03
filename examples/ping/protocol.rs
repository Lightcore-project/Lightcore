use futures::{prelude::*, future, try_ready};
use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo, upgrade::Negotiated};
use log::debug;
use rand::{distributions, prelude::*};
use std::{io, iter, time::Duration};
use tokio_io::{io as nio, AsyncRead, AsyncWrite};
use wasm_timer::Instant;

#[derive(Default, Debug, Copy, Clone)]
pub struct Ping;

impl UpgradeInfo for Ping {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        println!("[protocol]: protocol info...");
        iter::once(b"/ipfs/ping/1.0.0")
    }
}

type RecvPing<T> = nio::ReadExact<Negotiated<T>, [u8; 32]>;
type SendPong<T> = nio::WriteAll<Negotiated<T>, [u8; 32]>;
type Flush<T> = nio::Flush<Negotiated<T>>;
type Shutdown<T> = nio::Shutdown<Negotiated<T>>;

impl<TSocket> InboundUpgrade<TSocket> for Ping
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Output = ();
    type Error = io::Error;
    type Future = future::Map<
        future::AndThen<
        future::AndThen<
        future::AndThen<
            RecvPing<TSocket>,
            SendPong<TSocket>, fn((Negotiated<TSocket>, [u8; 32])) -> SendPong<TSocket>>,
            Flush<TSocket>, fn((Negotiated<TSocket>, [u8; 32])) -> Flush<TSocket>>,
            Shutdown<TSocket>, fn(Negotiated<TSocket>) -> Shutdown<TSocket>>,
    fn(Negotiated<TSocket>) -> ()>;

    #[inline]
    fn upgrade_inbound(self, socket: Negotiated<TSocket>, _: Self::Info) -> Self::Future {
        println!("[protocol]: upgrade_inbound...");
        nio::read_exact(socket, [0; 32])
            .and_then::<fn(_) -> _, _>(|(sock, buf)| nio::write_all(sock, buf))
            .and_then::<fn(_) -> _, _>(|(sock, _)| nio::flush(sock))
            .and_then::<fn(_) -> _, _>(|sock| nio::shutdown(sock))
            .map(|_| ())
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for Ping
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Output = Duration;
    type Error = io::Error;
    type Future = PingDialer<Negotiated<TSocket>>;

    #[inline]
    fn upgrade_outbound(self, socket: Negotiated<TSocket>, _: Self::Info) -> Self::Future {
        println!("[protocol]: upgrade_outbound...");
        let payload: [u8; 32] = thread_rng().sample(distributions::Standard);
        debug!("Preparing ping payload {:?}", payload);

        PingDialer {
            state: PingDialerState::Write {
                inner: nio::write_all(socket, payload),
            },
        }
    }
}

/// A `PingDialer` is a future that sends a ping and expects to receive a pong.
pub struct PingDialer<TSocket> {
    state: PingDialerState<TSocket>
}

enum PingDialerState<TSocket> {
    Write {
        inner: nio::WriteAll<TSocket, [u8; 32]>,
    },
    Flush {
        inner: nio::Flush<TSocket>,
        payload: [u8; 32],
    },
    Read {
        inner: nio::ReadExact<TSocket, [u8; 32]>,
        payload: [u8; 32],
        started: Instant,
    },
    Shutdown {
        inner: nio::Shutdown<TSocket>,
        rtt: Duration,
    },
}

impl<TSocket> Future for PingDialer<TSocket>
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Item = Duration;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            self.state = match self.state {
                PingDialerState::Write { ref mut inner } => {
                    let (socket, payload) = try_ready!(inner.poll());
                    PingDialerState::Flush {
                        inner: nio::flush(socket),
                        payload,
                    }
                },
                PingDialerState::Flush { ref mut inner, payload } => {
                    let socket = try_ready!(inner.poll());
                    let started = Instant::now();
                    PingDialerState::Read {
                        inner: nio::read_exact(socket, [0; 32]),
                        payload,
                        started,
                    }
                },
                PingDialerState::Read { ref mut inner, payload, started } => {
                    let (socket, payload_received) = try_ready!(inner.poll());
                    let rtt = started.elapsed();
                    if payload_received != payload {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData, "Ping payload mismatch"));
                    }
                    PingDialerState::Shutdown {
                        inner: nio::shutdown(socket),
                        rtt,
                    }
                },
                PingDialerState::Shutdown { ref mut inner, rtt } => {
                    try_ready!(inner.poll());
                    return Ok(Async::Ready(rtt));
                },
            }
        }
    }
}
