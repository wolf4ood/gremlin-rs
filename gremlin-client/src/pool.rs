use r2d2::ManageConnection;

use crate::connection::Connection;
use crate::connection::ConnectionOptions;
use crate::error::GremlinError;
use crate::message::{
    message_with_args, message_with_args_and_uuid, message_with_args_v1, message_with_args_v2,
    Response,
};
use crate::{GValue, GraphSON, GremlinResult};
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
        let args = self.options.serializer.write(&GValue::from(args))?;

        let message = match self.options.serializer {
            GraphSON::V1 => message_with_args_v1(String::from("eval"), String::default(), args),
            GraphSON::V2 => message_with_args_v2(String::from("eval"), String::default(), args),
            GraphSON::V3 => message_with_args(String::from("eval"), String::default(), args),
        };

        let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;

        let content_type = match self.options.serializer {
            GraphSON::V1 => "application/vnd.gremlin-v1.0+json",
            GraphSON::V2 => "application/vnd.gremlin-v2.0+json",
            GraphSON::V3 => "application/vnd.gremlin-v3.0+json",
        };
        let payload = String::from("") + content_type + &msg;

        let mut binary = payload.into_bytes();
        binary.insert(0, content_type.len() as u8);

        conn.send(binary)?;

        let result = conn.recv()?;
        let response: Response = serde_json::from_slice(&result)?;

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

                    let args = self.options.serializer.write(&GValue::from(args))?;
                    let message = message_with_args_and_uuid(
                        String::from("authentication"),
                        String::from("traversal"),
                        response.request_id,
                        args,
                    );

                    let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;

                    let content_type = match self.options.serializer {
                        GraphSON::V1 => "application/vnd.gremlin-v1.0+json",
                        GraphSON::V2 => "application/vnd.gremlin-v2.0+json",
                        GraphSON::V3 => "application/vnd.gremlin-v3.0+json",
                    };
                    let payload = String::from("") + content_type + &msg;

                    let mut binary = payload.into_bytes();
                    binary.insert(0, content_type.len() as u8);

                    conn.send(binary)?;

                    let result = conn.recv()?;
                    let response: Response = serde_json::from_slice(&result)?;

                    match response.status.code {
                        200 | 206 => Ok(()),
                        204 => Ok(()),
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
