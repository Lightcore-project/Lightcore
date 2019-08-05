use futures::prelude::*;
use libp2p::{
	InboundUpgradeExt, OutboundUpgradeExt, PeerId, Transport,
	mplex, identity, secio, yamux, bandwidth, tcp, websocket, dns
};
use libp2p::core::{self, transport::boxed::Boxed, transport::OptionalTransport, muxing::StreamMuxerBox};
use std::{io, time::Duration, usize};

pub use self::bandwidth::BandwidthSinks;

pub fn build_transport(
	keypair: identity::Keypair,
	memory_only: bool,
) -> Boxed<(PeerId, StreamMuxerBox), io::Error> {
	// Build configuration objects for encryption mechanisms.
	let secio_config = secio::SecioConfig::new(keypair);

	// Build configuration objects for multiplexing mechanisms.
	let mut mplex_config = mplex::MplexConfig::new();
	mplex_config.max_buffer_len_behaviour(mplex::MaxBufferBehaviour::Block);
	mplex_config.max_buffer_len(usize::MAX);
	let yamux_config = yamux::Config::default();

	// Build the base layer of the transport.
    let desktop_trans = tcp::TcpConfig::new();
    let desktop_trans = websocket::WsConfig::new(desktop_trans.clone())
        .or_transport(desktop_trans);
    let transport = OptionalTransport::some(dns::DnsConfig::new(desktop_trans));

	let transport = transport.or_transport(if memory_only {
		OptionalTransport::some(libp2p::core::transport::MemoryTransport::default())
	} else {
		OptionalTransport::none()
	});

	// let (transport, sinks) = bandwidth::BandwidthLogging::new(transport, Duration::from_secs(5));

	let transport = transport.and_then(move |stream, endpoint| {
		core::upgrade::apply(stream, secio_config, endpoint)
			.and_then(|out| Ok((out.stream, out.remote_key.into_peer_id())))
	});

	// Multiplexing
	let transport = transport.and_then(move |(stream, peer_id), endpoint| {
			let peer_id2 = peer_id.clone();
			let upgrade = core::upgrade::SelectUpgrade::new(yamux_config, mplex_config)
				.map_inbound(move |muxer| (peer_id, muxer))
				.map_outbound(move |muxer| (peer_id2, muxer));

			core::upgrade::apply(stream, upgrade, endpoint)
				.map(|(id, muxer)| (id, core::muxing::StreamMuxerBox::new(muxer)))
		})

		.with_timeout(Duration::from_secs(20))
		.map_err(|err| io::Error::new(io::ErrorKind::Other, err))
		.boxed();

	transport
}
