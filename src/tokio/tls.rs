use super::io::{StubbornIo, UnderlyingIo};
use native_tls::TlsConnector;
use std::future::Future;
use std::io;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;

impl<A> UnderlyingIo<A> for TlsStream<TcpStream>
where
    A: Into<String> + Send + Clone + Unpin + 'static,
{
    fn establish(addr: A) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send>> {
        Box::pin(async move {
            let a = addr.into();
            let stream = TcpStream::connect(&a).await?;
            let cx = match TlsConnector::builder().build() {
                Ok(c) => c,
                Err(_) => {
                    return Err(io::Error::from(io::ErrorKind::Other));
                }
            };
            let cx = tokio_native_tls::TlsConnector::from(cx);
            let server_parts: Vec<&str> = a.split(':').collect();
            match cx.connect(&server_parts[0], stream).await {
                Ok(st) => Ok(st),
                Err(_) => Err(io::Error::from(io::ErrorKind::NotConnected)),
            }
        })
    }
}

pub type StubbornTlsStream<A> = StubbornIo<TlsStream<TcpStream>, A>;
