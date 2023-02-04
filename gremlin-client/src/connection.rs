use std::net::TcpStream;

use crate::{GraphSON, GremlinError, GremlinResult};
use native_tls::TlsConnector;
use tungstenite::{
    client::{uri_mode, IntoClientRequest},
    client_tls_with_config,
    stream::{MaybeTlsStream, Mode, NoDelay},
    Connector, Message, WebSocket,
};

struct ConnectionStream(WebSocket<MaybeTlsStream<TcpStream>>);

impl std::fmt::Debug for ConnectionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Connection")
    }
}

impl ConnectionStream {
    fn connect(options: ConnectionOptions) -> GremlinResult<Self> {
        let connector = match options.tls_options.as_ref() {
            Some(option) => Some(Connector::NativeTls(
                option
                    .tls_connector()
                    .map_err(|e| GremlinError::Generic(e.to_string()))?,
            )),
            _ => None,
        };

        // TcpStream::connect(addr)
        let request = options
            .websocket_url()
            .into_client_request()
            .map_err(|e| GremlinError::Generic(e.to_string()))?;
        let uri = request.uri();
        let mode = uri_mode(uri).map_err(|e| GremlinError::Generic(e.to_string()))?;
        let host = request
            .uri()
            .host()
            .ok_or_else(|| GremlinError::Generic("No Hostname".into()))?;
        let port = uri.port_u16().unwrap_or(match mode {
            Mode::Plain => 80,
            Mode::Tls => 443,
        });
        let mut stream = TcpStream::connect((host, port))
            .map_err(|e| GremlinError::Generic(format!("Unable to connect {e:?}")))?;
        NoDelay::set_nodelay(&mut stream, true)
            .map_err(|e| GremlinError::Generic(e.to_string()))?;

        let (client, _response) =
            client_tls_with_config(options.websocket_url(), stream, None, connector)
                .map_err(|e| GremlinError::Generic(e.to_string()))?;

        Ok(ConnectionStream(client))
    }

    fn send(&mut self, payload: Vec<u8>) -> GremlinResult<()> {
        self.0
            .write_message(Message::Binary(payload))
            .map_err(GremlinError::from)
    }

    fn recv(&mut self) -> GremlinResult<Vec<u8>> {
        match self.0.read_message()? {
            Message::Binary(binary) => Ok(binary),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Connection {
    stream: ConnectionStream,
    broken: bool,
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

    pub fn credentials(mut self, username: &str, password: &str) -> Self {
        self.0.credentials = Some(Credentials {
            username: String::from(username),
            password: String::from(password),
        });
        self
    }

    pub fn ssl(mut self, ssl: bool) -> Self {
        self.0.ssl = ssl;
        self
    }

    pub fn tls_options(mut self, options: TlsOptions) -> Self {
        self.0.tls_options = Some(options);
        self
    }

    pub fn serializer(mut self, serializer: GraphSON) -> Self {
        self.0.serializer = serializer;
        self
    }

    pub fn deserializer(mut self, deserializer: GraphSON) -> Self {
        self.0.deserializer = deserializer;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionOptions {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) pool_size: u32,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) ssl: bool,
    pub(crate) tls_options: Option<TlsOptions>,
    pub(crate) serializer: GraphSON,
    pub(crate) deserializer: GraphSON,
}

#[derive(Clone, Debug)]
pub(crate) struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Clone, Debug)]
pub struct TlsOptions {
    pub accept_invalid_certs: bool,
}

impl Default for ConnectionOptions {
    fn default() -> ConnectionOptions {
        ConnectionOptions {
            host: String::from("localhost"),
            port: 8182,
            pool_size: 10,
            credentials: None,
            ssl: false,
            tls_options: None,
            serializer: GraphSON::V3,
            deserializer: GraphSON::V3,
        }
    }
}

impl ConnectionOptions {
    pub fn builder() -> ConnectionOptionsBuilder {
        ConnectionOptionsBuilder(ConnectionOptions::default())
    }

    pub fn websocket_url(&self) -> String {
        let protocol = if self.ssl { "wss" } else { "ws" };
        format!("{}://{}:{}/gremlin", protocol, self.host, self.port)
    }
}

impl Connection {
    pub fn connect<T>(options: T) -> GremlinResult<Connection>
    where
        T: Into<ConnectionOptions>,
    {
        Ok(Connection {
            stream: ConnectionStream::connect(options.into())?,
            broken: false,
        })
    }

    pub fn send(&mut self, payload: Vec<u8>) -> GremlinResult<()> {
        self.stream.send(payload).map_err(|e| {
            if let GremlinError::WebSocket(_) = e {
                self.broken = true;
            }
            e
        })
    }

    pub fn recv(&mut self) -> GremlinResult<Vec<u8>> {
        self.stream.recv().map_err(|e| {
            if let GremlinError::WebSocket(_) = e {
                self.broken = true
            }
            e
        })
    }

    pub fn is_broken(&self) -> bool {
        self.broken
    }
}

impl TlsOptions {
    pub(crate) fn tls_connector(&self) -> native_tls::Result<TlsConnector> {
        TlsConnector::builder()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_connect() {
        Connection::connect(("localhost", 8182)).unwrap();
    }

    #[test]
    fn connection_option_build_url() {
        let options = ConnectionOptions {
            host: "localhost".into(),
            port: 8182,
            ssl: false,
            ..Default::default()
        };

        assert_eq!(options.websocket_url(), "ws://localhost:8182/gremlin");

        let options = ConnectionOptions {
            host: "localhost".into(),
            port: 8182,
            ssl: true,
            ..Default::default()
        };

        assert_eq!(options.websocket_url(), "wss://localhost:8182/gremlin");
    }
}
