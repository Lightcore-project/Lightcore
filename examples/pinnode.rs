use lightcore::p2p::service::Service;
use lightcore::p2p::config::Config;
use base64::encode;

fn main() {
    let mut config = Config::default();
    config.listen = String::from("127.0.0.1:6581");
    let p2p = Service::new(config);
    println!("{:?}",p2p.our_info.peer_addr);
    println!("{:?}",encode(&p2p.our_info.peer_cert_der));
    p2p.run();
}

