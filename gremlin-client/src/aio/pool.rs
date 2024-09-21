use futures::StreamExt;
use mobc::Manager;

use crate::aio::connection::Conn;
use crate::connection::ConnectionOptions;
use crate::error::GremlinError;
use crate::message::{message_with_args, message_with_args_and_uuid, message_with_args_v2};
use crate::{GValue, GraphSON};
use async_trait::async_trait;
use base64::encode;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct GremlinConnectionManager {
    options: ConnectionOptions,
}

impl GremlinConnectionManager {
    pub(crate) fn new(options: ConnectionOptions) -> GremlinConnectionManager {
        GremlinConnectionManager { options }
    }
}

#[async_trait]
impl Manager for GremlinConnectionManager {
    type Connection = Conn;
    type Error = GremlinError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Conn::connect(self.options.clone()).await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        let mut args = HashMap::new();

        args.insert(
            String::from("gremlin"),
            GValue::String("g.inject(0)".into()),
        );
        args.insert(
            String::from("language"),
            GValue::String(String::from("gremlin-groovy")),
        );
        let args = self.options.serializer.write(&GValue::from(args))?;

        let message = match self.options.serializer {
            GraphSON::V2 => message_with_args_v2(String::from("eval"), String::default(), args),
            GraphSON::V3 => message_with_args(String::from("eval"), String::default(), args),
        };

        let id = message.id().clone();
        let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;

        let content_type = self.options.serializer.content_type();

        let payload = String::from("") + content_type + &msg;
        let mut binary = payload.into_bytes();
        binary.insert(0, content_type.len() as u8);

        let response = conn
            .send(id, binary)
            .await?
            .next()
            .await
            .expect("Should have received response")?;

        match response.status.code {
            200 | 206 => Ok(conn),
            204 => Ok(conn),
            407 => match &self.options.credentials {
                Some(c) => {
                    let mut args = HashMap::new();

                    args.insert(
                        String::from("sasl"),
                        GValue::String(encode(&format!("\0{}\0{}", c.username, c.password))),
                    );

                    let args = self.options.serializer.write(&GValue::from(args))?;
                    let message = message_with_args_and_uuid(
                        String::from("authentication"),
                        String::from("traversal"),
                        response.request_id,
                        args,
                    );

                    let id = message.id().clone();
                    let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;

                    let content_type = self.options.serializer.content_type();
                    let payload = String::from("") + content_type + &msg;

                    let mut binary = payload.into_bytes();
                    binary.insert(0, content_type.len() as u8);

                    let response = conn
                        .send(id, binary)
                        .await?
                        .next()
                        .await
                        .expect("Should have received response")?;

                    match response.status.code {
                        200 | 206 => Ok(conn),
                        204 => Ok(conn),
                        401 => Ok(conn),
                        // 401 is actually a username/password incorrect error, but if not
                        // not returned as okay, the pool loops infinitely trying
                        // to authenticate.
                        _ => Err(GremlinError::Request((
                            response.status.code,
                            response.status.message,
                        ))),
                    }
                }
                None => Err(GremlinError::Request((
                    response.status.code,
                    response.status.message,
                ))),
            },
            _ => Err(GremlinError::Request((
                response.status.code,
                response.status.message,
            ))),
        }
    }

    fn validate(&self, conn: &mut Self::Connection) -> bool {
        conn.is_valid()
    }
}

#[cfg(test)]
mod tests {

    use super::GremlinConnectionManager;
    use crate::ConnectionOptions;

    use mobc::Pool;
    use std::time::Duration;

    #[cfg(feature = "async-std-runtime")]
    mod async_std_use {
        pub use async_std::task;
    }

    #[cfg(feature = "async-std-runtime")]
    use async_std_use::*;

    #[cfg(feature = "tokio-runtime")]
    mod tokio_use {
        pub use tokio::task;
    }

    #[cfg(feature = "tokio-runtime")]
    use tokio_use::*;

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    #[allow(unused_must_use)]
    async fn it_should_create_a_connection_pool() {
        let manager = GremlinConnectionManager::new(ConnectionOptions::default());

        let pool = Pool::builder().max_open(16).build(manager);

        let conn = pool.get().await.expect("Failed to get the connection");

        pool.state().await;

        assert_eq!(1, pool.state().await.connections);

        assert_eq!(0, pool.state().await.idle);

        drop(conn);

        task::spawn_blocking(move || {
            std::thread::sleep(Duration::from_millis(200));
        })
        .await;

        assert_eq!(1, pool.state().await.idle);
    }
}
