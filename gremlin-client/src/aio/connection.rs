use crate::{GremlinError, GremlinResult, WebSocketOptions};

use crate::connection::ConnectionOptions;

use crate::message::Response;

#[cfg(feature = "async-std-runtime")]
mod async_std_use {
    pub use async_std::net::TcpStream;
    pub use async_std::task;
    pub use async_tls::client::TlsStream;
}

#[cfg(feature = "async-std-runtime")]
use async_std_use::*;

#[cfg(feature = "tokio-runtime")]
mod tokio_use {
    pub use tokio::net::TcpStream;
    pub use tokio::task;
    pub use tokio_native_tls::TlsStream;
}

use futures::TryFutureExt;
use log::{error, info, warn};
#[cfg(feature = "tokio-runtime")]
use tokio_use::*;

#[cfg(feature = "async-std-runtime")]
use async_tungstenite::async_std::connect_async_with_tls_connector_and_config;

#[cfg(feature = "tokio-runtime")]
use async_tungstenite::tokio::{connect_async_with_tls_connector_and_config, TokioAdapter};

use async_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};
use async_tungstenite::WebSocketStream;
use async_tungstenite::{self, stream};
use futures::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

use futures::channel::mpsc::{channel, Receiver, Sender};
use std::collections::HashMap;
use std::sync::Arc;
use url;
use uuid::Uuid;

#[cfg(feature = "async-std-runtime")]
type WSStream = WebSocketStream<stream::Stream<TcpStream, TlsStream<TcpStream>>>;

#[cfg(feature = "tokio-runtime")]
type WSStream =
    WebSocketStream<stream::Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Cmd {
    Msg((Sender<GremlinResult<Response>>, Uuid, Vec<u8>)),
    Pong(Vec<u8>),
    Shutdown,
}

pub(crate) struct Conn {
    sender: Sender<Cmd>,
    valid: bool,
}

impl std::fmt::Debug for Conn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Conn")
    }
}

#[cfg(feature = "async-std-runtime")]
mod tls {

    use std::time::SystemTime;

    use crate::connection::ConnectionOptions;
    use rustls::{Certificate, ServerName};
    pub struct NoCertificateVerification {}

    impl rustls::client::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &Certificate,
            _intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::TLSError> {
            Ok(rustls::client::ServerCertVerified::assertion())
        }
    }

    pub fn connector(opts: &ConnectionOptions) -> Option<async_tls::TlsConnector> {
        use rustls::ClientConfig;
        use std::sync::Arc;
        if opts
            .tls_options
            .as_ref()
            .map(|tls| tls.accept_invalid_certs)
            .unwrap_or(false)
        {
            let config = ClientConfig::builder()
                .with_safe_defaults()
                .with_custom_certificate_verifier(Arc::new(NoCertificateVerification {}))
                .with_no_client_auth();

            Some(async_tls::TlsConnector::from(Arc::new(config)))
        } else {
            Some(async_tls::TlsConnector::new())
        }
    }
}

#[cfg(feature = "tokio-runtime")]
mod tls {

    use crate::connection::ConnectionOptions;
    use tokio_native_tls::TlsConnector;

    pub fn connector(opts: &ConnectionOptions) -> Option<TlsConnector> {
        opts.tls_options
            .as_ref()
            .and_then(|tls| tls.tls_connector().map(TlsConnector::from).ok())
    }
}
impl Conn {
    pub async fn connect<T>(options: T) -> GremlinResult<Conn>
    where
        T: Into<ConnectionOptions>,
    {
        let opts = options.into();
        let url = url::Url::parse(&opts.websocket_url()).expect("failed to parse url");

        let websocket_config = opts.websocket_options.as_ref().map(WebSocketConfig::from);
        info!("Openning websocket connection");

        #[cfg(feature = "async-std-runtime")]
        let (client, _) = {
            connect_async_with_tls_connector_and_config(
                url,
                tls::connector(&opts),
                websocket_config,
            )
            .map_err(|e| Arc::new(e))
            .await?
        };
        #[cfg(feature = "tokio-runtime")]
        let (client, _) = {
            connect_async_with_tls_connector_and_config(
                url,
                tls::connector(&opts),
                websocket_config,
            )
            .map_err(|e| Arc::new(e))
            .await?
        };

        info!("Opened websocket connection");
        let (sink, stream) = client.split();
        let (sender, receiver) = channel(20);
        let requests = Arc::new(Mutex::new(HashMap::new()));

        sender_loop(sink, requests.clone(), receiver);

        receiver_loop(stream, requests.clone(), sender.clone());

        Ok(Conn {
            sender,
            valid: true,
        })
    }

    pub async fn send(
        &mut self,
        id: Uuid,
        payload: Vec<u8>,
    ) -> GremlinResult<(Response, Receiver<GremlinResult<Response>>)> {
        let (sender, mut receiver) = channel(1);

        self.sender
            .send(Cmd::Msg((sender, id, payload)))
            .await
            .map_err(|e| {
                error!("Marking websocket connection invalid on send error");
                self.valid = false;
                e
            })?;

        receiver
            .next()
            .await
            .expect("It should contain the response")
            .map(|r| (r, receiver))
            .map_err(|e| {
                //If there's been an websocket layer error, mark the connection as invalid
                match e {
                    GremlinError::WebSocket(_)
                    | GremlinError::WebSocketAsync(_)
                    | GremlinError::WebSocketPoolAsync(_) => {
                        error!("Marking websocket connection invalid on received error");
                        self.valid = false;
                    }
                    _ => {}
                }
                e
            })
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        warn!("Websocket connection instance dropped");
        send_shutdown(self);
    }
}

fn send_shutdown(conn: &mut Conn) {
    warn!("Websocket connection instance shutting down channel");
    conn.sender.close_channel();
}

fn sender_loop(
    mut sink: SplitSink<WSStream, Message>,
    requests: Arc<Mutex<HashMap<Uuid, Sender<GremlinResult<Response>>>>>,
    mut receiver: Receiver<Cmd>,
) {
    task::spawn(async move {
        loop {
            match receiver.next().await {
                Some(item) => match item {
                    Cmd::Msg(msg) => {
                        let mut guard = requests.lock().await;
                        guard.insert(msg.1, msg.0);
                        if let Err(e) = sink.send(Message::Binary(msg.2)).await {
                            error!("Sink sending error occured");
                            let mut sender = guard.remove(&msg.1).unwrap();
                            sender
                                .send(Err(GremlinError::from(Arc::new(e))))
                                .await
                                .expect("Failed to send error");
                        }
                        drop(guard);
                    }
                    Cmd::Pong(data) => {
                        info!("Sending Pong");
                        sink.send(Message::Pong(data))
                            .await
                            .expect("Failed to send pong message.");
                    }
                    Cmd::Shutdown => {
                        warn!("Shuting down connection");
                        let mut guard = requests.lock().await;
                        guard.clear();
                    }
                },
                None => {
                    warn!("Sending loop breaking");
                    break;
                }
            }
        }
        warn!("Sending loop closing sink");
        let _ = sink.close().await;
    });
}

fn receiver_loop(
    mut stream: SplitStream<WSStream>,
    requests: Arc<Mutex<HashMap<Uuid, Sender<GremlinResult<Response>>>>>,
    mut sender: Sender<Cmd>,
) {
    task::spawn(async move {
        loop {
            match stream.next().await {
                Some(Err(error)) => {
                    let mut guard = requests.lock().await;
                    let error = Arc::new(error);
                    error!("Receiver loop error");
                    for s in guard.values_mut() {
                        match s.send(Err(error.clone().into())).await {
                            Ok(_r) => {}
                            Err(_e) => {}
                        }
                    }
                    guard.clear();
                }
                Some(Ok(item)) => match item {
                    Message::Binary(data) => {
                        let response: Response = serde_json::from_slice(&data).unwrap();
                        let mut guard = requests.lock().await;
                        if response.status.code != 206 {
                            let item = guard.remove(&response.request_id);
                            drop(guard);
                            if let Some(mut s) = item {
                                match s.send(Ok(response)).await {
                                    Ok(_r) => {}
                                    Err(_e) => {}
                                };
                            }
                        } else {
                            let item = guard.get_mut(&response.request_id);
                            if let Some(s) = item {
                                match s.send(Ok(response)).await {
                                    Ok(_r) => {}
                                    Err(_e) => {}
                                };
                            }
                            drop(guard);
                        }
                    }
                    Message::Ping(data) => {
                        info!("Received Ping");
                        let _ = sender.send(Cmd::Pong(data)).await;
                    }
                    _ => {}
                },
                None => {
                    warn!("Receiver loop breaking");
                    break;
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn it_should_connect() {
        Conn::connect(("localhost", 8182)).await.unwrap();
    }
}
