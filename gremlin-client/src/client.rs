use crate::message::Response;
use crate::pool::GremlinConnectionManager;
use crate::process::traversal::Bytecode;
use crate::ToGValue;
use crate::{ConnectionOptions, GremlinError, GremlinResult};
use crate::{GResultSet, GValue};
use base64::encode;
use r2d2::Pool;
use std::collections::{HashMap, VecDeque};

type SessionedClient = GremlinClient;

impl SessionedClient {
    pub fn close_session(&mut self) -> GremlinResult<GResultSet> {
        if let Some(session_name) = self.session.take() {
            let mut args = HashMap::new();
            args.insert(String::from("session"), GValue::from(session_name.clone()));

            let (_, message) = self
                .options
                .serializer
                .build_message("close", "session", args, None)?;

            let conn = self.pool.get()?;

            self.send_message(conn, message)
        } else {
            Err(GremlinError::Generic("No session to close".to_string()))
        }
    }
}

#[derive(Clone, Debug)]
pub struct GremlinClient {
    pool: Pool<GremlinConnectionManager>,
    session: Option<String>,
    alias: Option<String>,
    options: ConnectionOptions,
}

impl GremlinClient {
    pub fn connect<T>(options: T) -> GremlinResult<GremlinClient>
    where
        T: Into<ConnectionOptions>,
    {
        let opts = options.into();
        let pool_size = opts.pool_size;
        let manager = GremlinConnectionManager::new(opts.clone());

        let mut pool_builder = Pool::builder().max_size(pool_size);

        if let Some(get_connection_timeout) = opts.pool_get_connection_timeout {
            pool_builder = pool_builder.connection_timeout(get_connection_timeout);
        }

        Ok(GremlinClient {
            pool: pool_builder.build(manager)?,
            session: None,
            alias: None,
            options: opts,
        })
    }

    pub fn create_session(&mut self, name: String) -> GremlinResult<SessionedClient> {
        let manager = GremlinConnectionManager::new(self.options.clone());
        Ok(SessionedClient {
            pool: Pool::builder().max_size(1).build(manager)?,
            session: Some(name),
            alias: None,
            options: self.options.clone(),
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

    pub fn execute<T>(
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

        if let Some(session_name) = &self.session {
            args.insert(String::from("session"), GValue::from(session_name.clone()));
        }

        let processor = if self.session.is_some() {
            "session"
        } else {
            ""
        };

        let (_, message) = self
            .options
            .serializer
            .build_message("eval", processor, args, None)?;

        let conn = self.pool.get()?;

        self.send_message(conn, message)
    }

    pub(crate) fn send_message(
        &self,
        mut conn: r2d2::PooledConnection<GremlinConnectionManager>,
        msg: Vec<u8>,
    ) -> GremlinResult<GResultSet> {
        conn.send(msg)?;

        let (response, results) = self.read_response(&mut conn)?;

        Ok(GResultSet::new(self.clone(), results, response, conn))
    }

    pub(crate) fn submit_traversal(&self, bytecode: &Bytecode) -> GremlinResult<GResultSet> {
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

        let mut args = HashMap::new();
        args.insert(String::from("gremlin"), GValue::Bytecode(bytecode.clone()));
        args.insert(String::from("aliases"), GValue::from(aliases));

        let (_, message) =
            self.options
                .serializer
                .build_message("bytecode", "traversal", args, None)?;
        let conn = self.pool.get()?;

        self.send_message(conn, message)
    }

    pub(crate) fn read_response(
        &self,
        conn: &mut r2d2::PooledConnection<GremlinConnectionManager>,
    ) -> GremlinResult<(Response, VecDeque<GValue>)> {
        let result = conn.recv()?;
        let response = self.options.deserializer.read_response(result)?;

        match response.status.code {
            200 | 206 => {
                let results: VecDeque<GValue> = response
                    .result
                    .data
                    .clone()
                    .map(Into::into)
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

                    self.read_response(conn)
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
}
