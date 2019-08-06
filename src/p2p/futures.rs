use std::collections::HashMap;
use futures::task::Task;
use libp2p::PeerId;

pub struct FuturesInner<T> {
    pub tasks: HashMap<PeerId, Task>,
    pub results: HashMap<PeerId, T>,
}

impl<T> FuturesInner <T> {
    pub fn new() -> Self {
        FuturesInner {
            tasks: HashMap::new(),
            results: HashMap::new(),
        }
    }
}

