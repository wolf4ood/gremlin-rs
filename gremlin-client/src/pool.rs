use r2d2::ManageConnection;

use crate::connection::Connection;
use crate::connection::ConnectionOptions;
use crate::error::GremlinError;
use crate::message::Response;
use crate::{GValue, GremlinResult, IoProtocol};
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

impl ManageConnection for GremlinConnectionManager {
    type Connection = Connection;
    type Error = GremlinError;

    fn connect(&self) -> GremlinResult<Connection> {
        Connection::connect(self.options.clone())
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), GremlinError> {
        let mut args = HashMap::new();

        args.insert(
            String::from("gremlin"),
            GValue::String("g.inject(0)".into()),
        );
        args.insert(
            String::from("language"),
            GValue::String(String::from("gremlin-groovy")),
        );

        let (_, message) = self
            .options
            .serializer
            .build_message("eval", "", args, None)?;
        conn.send(message)?;

        let result = conn.recv()?;
        let response = self.options.deserializer.read_response(result)?;

        match response.status.code {
            200 | 206 => Ok(()),
            204 => Ok(()),
            407 => match &self.options.credentials {
                Some(c) => {
                    let mut args = HashMap::new();

                    args.insert(
                        String::from("sasl"),
                        GValue::String(encode(&format!("\0{}\0{}", c.username, c.password))),
                    );

                    let (_, message) = self.options.serializer.build_message(
                        "authentication",
                        "traversal",
                        args,
                        Some(
                            response
                                .request_id
                                .expect("Auth challenge requires response id"),
                        ),
                    )?;
                    conn.send(message)?;

                    let result = conn.recv()?;
                    let response = self.options.deserializer.read_response(result)?;

                    match response.status.code {
                        200 | 206 => Ok(()),
                        204 => Ok(()),
                        401 => Ok(()),
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

    fn has_broken(&self, conn: &mut Connection) -> bool {
        conn.is_broken()
    }
}

#[cfg(test)]
mod tests {

    use super::GremlinConnectionManager;
    use crate::ConnectionOptions;

    use r2d2::Pool;

    #[test]
    fn it_should_create_a_connection_pool() {
        let manager = GremlinConnectionManager::new(ConnectionOptions::default());

        let result = Pool::builder().max_size(16).build(manager);

        let pool = result.unwrap();

        let connection = pool.get();

        assert_eq!(16, pool.state().connections);

        assert_eq!(15, pool.state().idle_connections);

        drop(connection);

        assert_eq!(16, pool.state().idle_connections);
    }
}
