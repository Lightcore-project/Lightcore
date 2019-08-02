use futures::{ prelude::*, future, try_ready };
use tokio_io::{ io as nio, AsyncRead, AsyncWrite };
use libp2p::core::upgrade::{
    UpgradeInfo, InboundUpgrade, OutboundUpgrade, Negotiated
};

pub struct Hello;
impl UpgradeInfo for Hello {
    type Info = &'static [u8];
    type InfoIter = ::std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        ::std::iter::once(b"hello, world!")
    }
}


impl<TSocket> InboundUpgrade<TSocket> for Hello
where TSocket: AsyncRead + AsyncWrite {
    type Output = &'static [u8];
    type Error = ::std::io::Error;
    type Future = future::FutureResult<Self::Output, ::std::io::Error>;

    fn upgrade_inbound(self, _socket: Negotiated<TSocket>, _: Self::Info) -> Self::Future {
        future::ok(b"hello, upgrade inbound")
    }
}


impl<TSocket> OutboundUpgrade<TSocket> for Hello
where TSocket: AsyncRead + AsyncWrite {
    type Output = &'static [u8];
    type Error = ::std::io::Error;
    type Future = future::FutureResult<Self::Output, ::std::io::Error>;

    fn upgrade_outbound(self, _socket: Negotiated<TSocket>, _: Self::Info) -> Self::Future {
        future::ok(b"hello, upgrade outbound")
    }
}
