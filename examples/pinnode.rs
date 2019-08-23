#[macro_use]
extern crate serde_derive;

use std::sync::mpsc::channel;
use crust::Service;
use crust::Uid;
use crust::Config;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct UniqueId([u8; 20]);
impl Uid for UniqueId {}
impl UniqueId {
    pub fn new_random() -> UniqueId {
        let inner = rand::random::<[u8;20]>();
        UniqueId (inner)
    }
}

fn main() {
    let config = Config::default();
    let (channel_sender, channel_receiver) = channel();
    let (category_tx, category_rx) = channel();
    let crust_event_category = ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust;
    let event_sender = ::maidsafe_utilities::event_sender::MaidSafeObserver::new(
        channel_sender,
        crust_event_category,
        category_tx,
    );
    let mut service = Service::with_config(event_sender, config, UniqueId::new_random()).unwrap();
    service.start_listening_tcp().unwrap();
    service.prepare_connection_info(1);
    for event in channel_receiver.iter() {
        println!("{:?}", event);
    }
}

