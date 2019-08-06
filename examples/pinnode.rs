use crust::read_config_file;
use safe_crypto::gen_sign_keypair;
use std::sync::mpsc::channel;
use crust::Service;
use crust::PeerId;
use safe_crypto::SecretEncryptKey;

fn new_peer_id() -> (PeerId, SecretEncryptKey) {
    let (enc_pk, enc_sk) = gen_encrypt_keypair();
    let (sign_pk, _sign_sk) = gen_sign_keypair();
    let id = PeerId {
        pub_sign_key: sign_pk,
        pub_enc_key: enc_pk,
    };
    (id, enc_sk)
}

fn main() {
    let config = read_config_file().unwrap();
    let (pk, sk) = gen_sign_keypair();
    println!("{:?}",pk);
    let (channel_sender, channel_receiver) = channel();
    let (category_tx, category_rx) = channel();
    let crust_event_category = ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust;
    let event_sender = ::maidsafe_utilities::event_sender::MaidSafeObserver::new(
        channel_sender,
        crust_event_category,
        category_tx,
    );
    let service = Service::with_config(event_sender, config, pk);
}

