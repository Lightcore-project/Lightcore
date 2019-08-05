use lightcore::p2p::service::Service;
use lightcore::p2p::config::Config;

fn main() {
    let _config = Config::default();
    let service = Service::new(_config);
    tokio::run(service.run());
}

