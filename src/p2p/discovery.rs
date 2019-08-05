use futures::task;
use futures::prelude::Future;
use libp2p::kad::GetClosestPeersOk;
use libp2p::kad::GetClosestPeersError;
use libp2p::kad::GetClosestPeersResult;
use futures::prelude::Async;
use std::result::Result;
use std::option::Option;

pub struct DiscoveryFuture {
    result: Option<GetClosestPeersResult>,
}

impl DiscoveryFuture {
    pub fn new() -> Self {
        DiscoveryFuture {
            result:None,
        }
    }

    pub fn tigger(&mut self,result: GetClosestPeersResult) {
        self.result = Some(result);
        let handler = task::current();
        handler.notify();
    }
}

impl futures::future::Future for DiscoveryFuture {
    type Item = GetClosestPeersOk;
    type Error = GetClosestPeersError;

    fn poll (&mut self) -> Result<Async<GetClosestPeersOk>, GetClosestPeersError> {
        match self.result.to_owned() {
            Some(some) => {
                match some {
                    Ok(r) => {
                        Ok(Async::Ready(r))
                    },
                    Err(r) => {
                        Err(r)
                    }
                }
            },
            None => {
                Ok(Async::NotReady)
            }
        }
    }
}

