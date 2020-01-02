use crate::{GremlinError, GremlinResult};

use crate::connection::ConnectionOptions;

use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use async_tungstenite::tungstenite::protocol::Message;
use async_tungstenite::{self, stream};
use async_tungstenite::{connect_async, WebSocketStream};
use futures::{SinkExt, StreamExt};
use url;

type WSStream = WebSocketStream<stream::Stream<TcpStream, TlsStream<TcpStream>>>;
struct ConnectionStream(WSStream);

impl std::fmt::Debug for ConnectionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Connection")
    }
}

impl ConnectionStream {
    async fn connect(options: ConnectionOptions) -> GremlinResult<Self> {
        let url = url::Url::parse(&options.websocket_url()).expect("failed to pars url");

        let (client, _) = connect_async(url).await?;

        Ok(ConnectionStream(client))
    }

    async fn send(&mut self, payload: Vec<u8>) -> GremlinResult<()> {
        self.0
            .send(Message::Binary(payload))
            .await
            .map_err(GremlinError::from)
    }

    async fn recv(&mut self) -> GremlinResult<Vec<u8>> {
        match self.0.next().await.ok_or_else(|| {
            GremlinError::Generic(String::from("No message received in from the network"))
        })?? {
            Message::Binary(binary) => Ok(binary),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Connection {
    stream: ConnectionStream,
}

impl Connection {
    pub async fn connect<T>(options: T) -> GremlinResult<Connection>
    where
        T: Into<ConnectionOptions>,
    {
        Ok(Connection {
            stream: ConnectionStream::connect(options.into()).await?,
        })
    }

    pub async fn send(&mut self, payload: Vec<u8>) -> GremlinResult<()> {
        self.stream.send(payload).await
    }

    pub async fn recv(&mut self) -> GremlinResult<Vec<u8>> {
        self.stream.recv().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_std::task;

    #[test]
    fn it_should_connect() {
        task::block_on(async {
            Connection::connect(("localhost", 8182)).await.unwrap();
        });
    }
}
