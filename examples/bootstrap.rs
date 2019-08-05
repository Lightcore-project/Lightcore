use lightcore::p2p::service::Service;
use lightcore::p2p::config::Config;

fn main() {
    let mut _config = Config::default();
    _config.listen = String::from("/ip4/0.0.0.0/tcp/3001");
    _config.bootstrap = true;
    _config.bootstrap_list.insert(
        String::from("QmbKnY86ZUMjiFmWjUdNXKD3rrppVPPAQkTtir4Wsjr8vp"), String::from("/ip4/127.0.0.1/tcp/3000")
    );
    let mut service = Service::new(_config);
    service.dial("QmbKnY86ZUMjiFmWjUdNXKD3rrppVPPAQkTtir4Wsjr8vp".parse().unwrap());
    tokio::run(service.run());
}

