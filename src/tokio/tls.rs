use super::io::{StubbornIo, UnderlyingIo};
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::{client::TlsStream, rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};

impl<A> UnderlyingIo<A> for TlsStream<TcpStream>
where
    A: Into<String> + Send + Clone + Unpin + 'static,
{
    fn establish(addr: A) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send>> {
        Box::pin(async move {
            let a = addr.into();
            let mut config = ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let config = TlsConnector::from(Arc::new(config));
            let server_parts: Vec<&str> = a.split(':').collect();
            let dnsname = DNSNameRef::try_from_ascii_str(&server_parts[0]).unwrap();
            let stream = TcpStream::connect(&a).await?;
            match config.connect(dnsname, stream).await {
                Ok(st) => Ok(st),
                Err(_) => Err(io::Error::from(io::ErrorKind::NotConnected)),
            }
        })
    }
}

pub type StubbornTlsStream<A> = StubbornIo<TlsStream<TcpStream>, A>;
