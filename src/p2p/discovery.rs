use futures::task;
use futures::prelude::Future;
use libp2p::kad::GetClosestPeersOk;
use libp2p::kad::GetClosestPeersError;
use libp2p::kad::GetClosestPeersResult;
use futures::prelude::Async;
use std::result::Result;
use std::option::Option;

#[derive(Debug)]
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

// #[cfg(test)]
// mod tests {
//     use super::DiscoveryFuture;
//     use futures::future::{Future, Shared};
//     use std::fmt::Debug;
// 
//     fn run_one<F>(f: F) -> Result<F::Item, F::Error>
//     where
//         F: IntoFuture,
//         F::Future: Send + 'static,
//         F::Item: Send + 'static,
//         F::Error: Send + 'static,
//     {
//         let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
//         runtime.block_on(f.into_future())
//     }
//     
//     #[test]
//     fn shared() {
//         let df = DiscoveryFuture::new();
//         let dfs = Future::shared(df);
//         let _dfs = dfs.clone();
// 
//         assert_eq!(
//             _dfs.peek().unwrap().unwrap().fmt("{:?}"),
//             df.fmt("{:?}")
//         );
//     }
// }
