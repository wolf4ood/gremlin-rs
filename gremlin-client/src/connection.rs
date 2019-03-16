use crate::{GremlinError, GremlinResult};
use std::net::TcpStream;
use websocket::{sync::Client, ClientBuilder, OwnedMessage};

enum ConnectionStream {
    TCP(Client<TcpStream>),
    #[allow(dead_code)]
    SSL,
}

impl std::fmt::Debug for ConnectionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Connection")
    }
}

impl ConnectionStream {
    fn connect(options: ConnectionOptions) -> GremlinResult<Self> {
        let address = format!("ws://{}:{}/gremlin", options.host, options.port);

        let client = ClientBuilder::new(&address)
            .map_err(|e| GremlinError::Generic(e.to_string()))?
            .connect_insecure()?;

        Ok(ConnectionStream::TCP(client))
    }

    fn send(&mut self, payload: Vec<u8>) -> GremlinResult<()> {
        match self {
            ConnectionStream::TCP(s) => s
                .send_message(&OwnedMessage::Binary(payload))
                .map_err(GremlinError::from),
            ConnectionStream::SSL => unimplemented!(),
        }
    }

    fn recv(&mut self) -> GremlinResult<Vec<u8>> {
        let message = match self {
            ConnectionStream::TCP(s) => s.recv_message()?,
            ConnectionStream::SSL => unimplemented!(),
        };

        match message {
            OwnedMessage::Binary(binary) => Ok(binary),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Connection {
    stream: ConnectionStream,
}

impl Into<ConnectionOptions> for (&str, u16) {
    fn into(self) -> ConnectionOptions {
        ConnectionOptions {
            host: String::from(self.0),
            port: self.1,
            ..Default::default()
        }
    }
}

impl Into<ConnectionOptions> for &str {
    fn into(self) -> ConnectionOptions {
        ConnectionOptions {
            host: String::from(self),
            ..Default::default()
        }
    }
}

pub struct ConnectionOptionsBuilder(ConnectionOptions);

impl ConnectionOptionsBuilder {
    pub fn host<T>(mut self, host: T) -> Self
    where
        T: Into<String>,
    {
        self.0.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.0.port = port;
        self
    }

    pub fn pool_size(mut self, pool_size: u32) -> Self {
        self.0.pool_size = pool_size;
        self
    }

    pub fn build(self) -> ConnectionOptions {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionOptions {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) pool_size: u32,
}

impl Default for ConnectionOptions {
    fn default() -> ConnectionOptions {
        ConnectionOptions {
            host: String::from("localhost"),
            port: 8182,
            pool_size: 10,
        }
    }
}

impl ConnectionOptions {
    pub fn builder() -> ConnectionOptionsBuilder {
        ConnectionOptionsBuilder(ConnectionOptions::default())
    }
}

impl Connection {
    pub fn connect<T>(options: T) -> GremlinResult<Connection>
    where
        T: Into<ConnectionOptions>,
    {
        Ok(Connection {
            stream: ConnectionStream::connect(options.into())?,
        })
    }

    pub fn send(&mut self, payload: Vec<u8>) -> GremlinResult<()> {
        self.stream.send(payload)
    }

    pub fn recv(&mut self) -> GremlinResult<Vec<u8>> {
        self.stream.recv()
    }
}

#[cfg(test)]
mod tests {

    use super::Connection;

    #[test]
    fn it_should_connect() {
        Connection::connect(("localhost", 8182)).unwrap();
    }
}
