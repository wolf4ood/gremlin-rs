use crate::message::Response;
use crate::pool::GremlinConnectionManager;
use crate::structure::GValue;
use crate::{GremlinClient, GremlinResult};
use r2d2::PooledConnection;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct GResultSet {
    client: GremlinClient,
    results: VecDeque<GValue>,
    response: Response,
    conn: PooledConnection<GremlinConnectionManager>,
}

impl GResultSet {
    pub(crate) fn new(
        client: GremlinClient,
        results: VecDeque<GValue>,
        response: Response,
        conn: PooledConnection<GremlinConnectionManager>,
    ) -> GResultSet {
        GResultSet {
            client,
            results,
            response,
            conn,
        }
    }

    fn fetch(&mut self) -> GremlinResult<()> {
        if self.results.is_empty() && self.response.status.code == 206 {
            let (response, resuts) = self.client.read_response(&mut self.conn)?;
            self.response = response;
            self.results = resuts;
        }
        Ok(())
    }
}

impl Iterator for GResultSet {
    type Item = GremlinResult<GValue>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.fetch() {
            Ok(_) => self.results.pop_front().map(Ok),
            Err(e) => Some(Err(e)),
        }
    }
}
