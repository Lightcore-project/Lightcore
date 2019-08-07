use quic_p2p::Config;
use quic_p2p::Builder;
use crossbeam_channel as mpmc;
use quic_p2p::OurType;
use base64::encode;
use quic_p2p::Event;
use quic_p2p::Peer;
use bytes::Bytes;

fn main() {
    let mut config = Config::read_or_construct_default(Option::None).unwrap();
    config.port = Some(5438);
    config.ip = Some("192.168.0.104".parse().unwrap());
    config.our_type = OurType::default();
    // println!("{:?}",config.clone());
    let (ev_rx, ev_tx) = mpmc::unbounded();
    let mut qp2p = Builder::new(ev_rx)
        .with_config(config)
        .build().unwrap();
    let our_conn_info = qp2p.our_connection_info().unwrap();
    // println!("{:?}", our_conn_info);
    println!("{}", encode(&our_conn_info.peer_cert_der));
    for event in ev_tx.iter() {
        match event {
            Event::ConnectedTo{ peer } => {
                println!("{:?}", peer);
                qp2p.send(peer,Bytes::from("hello"),1);
            },
            _ => {
                println!("other event {:?}", event);
            }
        }
    }
}

