use lightcore::p2p::service::Service;
use lightcore::p2p::config::Config;
use lightcore::p2p::config::NodeInfo;
use base64::encode;

fn main() {
    let mut config = Config::default();
    config.listen = String::from("127.0.0.1:6582");
    let bootstrap = NodeInfo {
        peer_addr:String::from("127.0.0.1:6581"),
        peer_cert_der:String::from("MIIBTzCB9qADAgECAgEqMAoGCCqGSM49BAMCMCExHzAdBgNVBAMMFnJjZ2VuIHNlbGYgc2lnbmVkIGNlcnQwIhgPMTk3NTAxMDEwMDAwMDBaGA80MDk2MDEwMTAwMDAwMFowITEfMB0GA1UEAwwWcmNnZW4gc2VsZiBzaWduZWQgY2VydDBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABH4EmEDgD6/igjtzr825DPlZGHxor+HKEyUUDxecG3u2/KMXKYuFO7TyF9/UWBz8YWHNHts98u/rGwrCiTb9pbGjGzAZMBcGA1UdEQQQMA6CDE1haWRTQUZFLm5ldDAKBggqhkjOPQQDAgNIADBFAiEA4jrlbBh0R+ZisSaHgNnocjCfbGUTPBKwqY9NofZArJoCIFwvFYTeOgidtz2fcJSW7ExvO+Fbt6RuEe/+nHubQQ+2"),
    };
    config.hard_coded_contacts.insert(bootstrap);
    let p2p = Service::new(config);
    println!("{:?}",p2p.our_info.peer_addr);
    println!("{:?}",encode(&p2p.our_info.peer_cert_der));
    p2p.run();
}

