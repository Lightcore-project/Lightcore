use futures::task;
use futures::prelude::Future;
use libp2p::kad::GetClosestPeersOk;
use libp2p::kad::GetClosestPeersError;
use libp2p::kad::GetClosestPeersResult;
use libp2p::PeerId;
use futures::task::Task;
use futures::prelude::Async;
use std::result::Result;
use std::option::Option;
use std::collections::HashMap;

use super::futures::FuturesInner;

pub struct DiscoveryFuture<'a> {
    peer_id: PeerId,
    inner: &'a mut FuturesInner<GetClosestPeersResult>,
}

impl<'a> DiscoveryFuture<'a> {
    pub fn new(peer_id: PeerId, inner: &'a mut FuturesInner<GetClosestPeersResult>) -> Self {
        DiscoveryFuture {
            inner,
            peer_id,
        }
    }
}

impl<'a> Future for DiscoveryFuture<'a> {
    type Item = GetClosestPeersOk;
    type Error = GetClosestPeersError;

    fn poll (&mut self) -> Result<Async<GetClosestPeersOk>, GetClosestPeersError> {
        let _task = task::current();
        self.inner.tasks.insert(self.peer_id.clone(),_task);
        match self.inner.results.get(&self.peer_id) {
            Some(res) => {
                // Need remove
                match res {
                    Ok(r) => {
                        Ok(Async::Ready(r.clone()))
                    },
                    Err(r) => {
                        Err(r.clone())
                    }
                }
            },
            None => {
                Ok(Async::NotReady)
            }
        }
    }
}

