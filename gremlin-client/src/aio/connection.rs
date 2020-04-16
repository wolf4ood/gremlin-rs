use crate::{GremlinError, GremlinResult};

use crate::connection::ConnectionOptions;

use crate::message::Response;
use async_std::net::TcpStream;
use async_std::task;
use async_tls::client::TlsStream;
use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
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

type WSStream = WebSocketStream<stream::Stream<TcpStream, TlsStream<TcpStream>>>;

#[derive(Debug)]
pub enum Cmd {
    Msg((Sender<GremlinResult<Response>>, Uuid, Vec<u8>)),
    Pong(Vec<u8>),
    Shutdown,
}

pub(crate) struct Conn {
    sender: Sender<Cmd>,
}

impl std::fmt::Debug for Conn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Conn")
    }
}
impl Conn {
    pub async fn connect<T>(options: T) -> GremlinResult<Conn>
    where
        T: Into<ConnectionOptions>,
    {
        let opts = options.into();
        let url = url::Url::parse(&opts.websocket_url()).expect("failed to pars url");
        let (client, _) = connect_async(url).await?;

        let (sink, stream) = client.split();
        let (sender, receiver) = channel(20);
        let requests = Arc::new(Mutex::new(HashMap::new()));

        sender_loop(sink, requests.clone(), receiver);

        receiver_loop(stream, requests.clone(), sender.clone());

        Ok(Conn { sender })
    }

    pub async fn send(
        &mut self,
        id: Uuid,
        payload: Vec<u8>,
    ) -> GremlinResult<(Response, Receiver<GremlinResult<Response>>)> {
        let (sender, mut receiver) = channel(1);

        self.sender.send(Cmd::Msg((sender, id, payload))).await?;

        receiver
            .next()
            .await
            .expect("It should contain the response")
            .map(|r| (r, receiver))
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        task::block_on(async {
            match self.sender.send(Cmd::Shutdown).await {
                Ok(_e) => {}
                Err(_e) => {}
            }
        });
    }
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
                            let mut sender = guard.remove(&msg.1).unwrap();
                            sender
                                .send(Err(GremlinError::from(e)))
                                .await
                                .expect("Failed to send error");
                        }
                        drop(guard);
                    }
                    Cmd::Pong(data) => {
                        sink.send(Message::Pong(data))
                            .await
                            .expect("Failed to send pong message.");
                    }
                    Cmd::Shutdown => {
                        let mut guard = requests.lock().await;
                        guard.clear();
                    }
                },
                None => {}
            }
        }
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
                    for s in guard.values_mut() {
                        match s.send(Err(GremlinError::from(&error))).await {
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
                    Message::Ping(data) => sender.send(Cmd::Pong(data)).await.unwrap(),
                    _ => {}
                },
                None => {}
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_std::task;

    #[test]
    fn it_should_connect() {
        task::block_on(async {
            Conn::connect(("localhost", 8182)).await.unwrap();
        });
    }
}
