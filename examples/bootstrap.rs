use quic_p2p::Config;
use quic_p2p::Builder;
use crossbeam_channel as mpmc;
use quic_p2p::OurType;
use base64::decode;
use base64::encode;
use quic_p2p::NodeInfo;

fn main() {
    let mut config = Config::read_or_construct_default(Option::None).unwrap();
    config.port = Some(5439);
    config.ip = Some("192.168.0.104".parse().unwrap());
    config.our_type = OurType::default();
    let pinnode = NodeInfo {
        peer_addr:"192.168.0.104:5438".parse().unwrap(),
        peer_cert_der:decode("MIIBTjCB9qADAgECAgEqMAoGCCqGSM49BAMCMCExHzAdBgNVBAMMFnJjZ2VuIHNlbGYgc2lnbmVkIGNlcnQwIhgPMTk3NTAxMDEwMDAwMDBaGA80MDk2MDEwMTAwMDAwMFowITEfMB0GA1UEAwwWcmNnZW4gc2VsZiBzaWduZWQgY2VydDBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABKbDw6CCxNmL+mbxdawxX1Hwyh/ey2NNAVIpG2bbysEJpATNVBn9Z+Gm1DwKZEe8IUDpLx48JxKf053fd9xJCVijGzAZMBcGA1UdEQQQMA6CDE1haWRTQUZFLm5ldDAKBggqhkjOPQQDAgNHADBEAiBqFWu6hZigOThoaXxkVvJpt5TOMLEsMk0bs857bfKg4QIgYihUbFlDkariJSk4gTIXxlmXcw9L3gYIW7SkAU+70MA=").unwrap(),
    };
    config.hard_coded_contacts.insert(pinnode);
    // println!("{:?}",config.clone());
    let (ev_rx, ev_tx) = mpmc::unbounded();
    let mut qp2p = Builder::new(ev_rx)
        .with_config(config)
        .build().unwrap();
    let our_conn_info = qp2p.our_connection_info().unwrap();
    // println!("{:?}", our_conn_info);
    println!("{}", encode(&our_conn_info.peer_cert_der));
    for event in ev_tx.iter() {
        println!("{:?}", event);
    }
}

