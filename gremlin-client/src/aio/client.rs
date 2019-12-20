use crate::aio::pool::GremlinConnectionManager;
use crate::aio::GResultSet;
use crate::io::GraphSON;
use crate::message::{message_with_args, message_with_args_and_uuid, Message, Response};
use crate::process::traversal::Bytecode;
use crate::GValue;
use crate::ToGValue;
use crate::{ConnectionOptions, GremlinError, GremlinResult};
use base64::encode;
use mobc::runtime::DefaultExecutor;
use mobc::{Pool, PooledConnection};
use serde::Serialize;
use std::collections::{HashMap, VecDeque};

use futures::future::{BoxFuture, FutureExt};

#[derive(Clone)]
pub struct GremlinClient {
    pool: Pool<GremlinConnectionManager<DefaultExecutor>>,
    io: GraphSON,
    alias: Option<String>,
    options: ConnectionOptions,
}

impl GremlinClient {
    pub async fn connect<T>(options: T) -> GremlinResult<GremlinClient>
    where
        T: Into<ConnectionOptions>,
    {
        let opts = options.into();
        let pool_size = opts.pool_size;
        let manager = GremlinConnectionManager::new(opts.clone());

        let pool = Pool::builder().max_size(pool_size).build(manager).await?;

        Ok(GremlinClient {
            pool,
            io: GraphSON::V3,
            alias: None,
            options: opts,
        })
    }

    /// Return a cloned client with the provided alias
    pub fn alias<T>(&self, alias: T) -> GremlinClient
    where
        T: Into<String>,
    {
        let mut cloned = self.clone();
        cloned.alias = Some(alias.into());
        cloned
    }

    pub async fn execute<T>(
        &self,
        script: T,
        params: &[(&str, &dyn ToGValue)],
    ) -> GremlinResult<GResultSet>
    where
        T: Into<String>,
    {
        let mut args = HashMap::new();

        args.insert(String::from("gremlin"), GValue::String(script.into()));
        args.insert(
            String::from("language"),
            GValue::String(String::from("gremlin-groovy")),
        );

        let aliases = self
            .alias
            .clone()
            .map(|s| {
                let mut map = HashMap::new();
                map.insert(String::from("g"), GValue::String(s));
                map
            })
            .unwrap_or_else(HashMap::new);

        args.insert(String::from("aliases"), GValue::from(aliases));

        let bindings: HashMap<String, GValue> = params
            .iter()
            .map(|(k, v)| (String::from(*k), v.to_gvalue()))
            .collect();

        args.insert(String::from("bindings"), GValue::from(bindings));

        let args = self.io.write(&GValue::from(args))?;

        let message = message_with_args(String::from("eval"), String::default(), args);

        let conn = self.pool.get().await?;

        self.send_message(conn, message).await
    }

    pub(crate) async fn write_message<T: Serialize>(
        &self,
        conn: &mut PooledConnection<GremlinConnectionManager<DefaultExecutor>>,
        msg: Message<T>,
    ) -> GremlinResult<()> {
        let message = self.build_message(msg)?;

        let content_type = "application/vnd.gremlin-v3.0+json";
        let payload = String::from("") + content_type + &message;

        let mut binary = payload.into_bytes();
        binary.insert(0, content_type.len() as u8);

        conn.send(binary).await?;

        Ok(())
    }

    pub(crate) async fn send_message<T: Serialize>(
        &self,
        mut conn: PooledConnection<GremlinConnectionManager<DefaultExecutor>>,
        msg: Message<T>,
    ) -> GremlinResult<GResultSet> {
        self.write_message(&mut conn, msg).await?;

        let (response, results) = self.read_response(&mut conn).await?;

        Ok(GResultSet::new(self.clone(), results, response, conn))
    }

    pub(crate) async fn submit_traversal(&self, bytecode: &Bytecode) -> GremlinResult<GResultSet> {
        let mut args = HashMap::new();

        args.insert(String::from("gremlin"), GValue::Bytecode(bytecode.clone()));

        let aliases = self
            .alias
            .clone()
            .or_else(|| Some(String::from("g")))
            .map(|s| {
                let mut map = HashMap::new();
                map.insert(String::from("g"), GValue::String(s));
                map
            })
            .unwrap_or_else(HashMap::new);

        args.insert(String::from("aliases"), GValue::from(aliases));

        let args = self.io.write(&GValue::from(args))?;

        let message = message_with_args(String::from("bytecode"), String::from("traversal"), args);

        let conn = self.pool.get().await?;

        self.send_message(conn, message).await
    }
    pub(crate) fn read_response<'a>(
        &'a self,
        conn: &'a mut PooledConnection<GremlinConnectionManager<DefaultExecutor>>,
    ) -> BoxFuture<'a, GremlinResult<(Response, VecDeque<GValue>)>> {
        async move {
            let result = conn.recv().await?;

            let response: Response = serde_json::from_slice(&result)?;

            match response.status.code {
                200 | 206 => {
                    let results: VecDeque<GValue> = self
                        .io
                        .read(&response.result.data)?
                        .map(|v| v.into())
                        .unwrap_or_else(VecDeque::new);

                    Ok((response, results))
                }
                204 => Ok((response, VecDeque::new())),
                407 => match &self.options.credentials {
                    Some(c) => {
                        let mut args = HashMap::new();

                        args.insert(
                            String::from("sasl"),
                            GValue::String(encode(&format!("\0{}\0{}", c.username, c.password))),
                        );

                        let args = self.io.write(&GValue::from(args))?;
                        let message = message_with_args_and_uuid(
                            String::from("authentication"),
                            String::from("traversal"),
                            response.request_id,
                            args,
                        );

                        self.write_message(conn, message).await?;

                        self.read_response(conn).await
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
        .boxed()
    }
    fn build_message<T: Serialize>(&self, msg: Message<T>) -> GremlinResult<String> {
        serde_json::to_string(&msg).map_err(GremlinError::from)
    }
}
