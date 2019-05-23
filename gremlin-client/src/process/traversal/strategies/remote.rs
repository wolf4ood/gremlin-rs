use crate::conversion::FromGValue;

use crate::process::traversal::remote::Terminator;
use crate::{
    process::traversal::GraphTraversal, process::traversal::RemoteTraversalIterator, GremlinClient,
    GremlinResult,
};

#[derive(Clone)]
pub struct RemoteStrategy {
    client: GremlinClient,
}

impl RemoteStrategy {
    pub fn new(client: GremlinClient) -> RemoteStrategy {
        RemoteStrategy { client }
    }

    pub(crate) fn apply<S, E: FromGValue, A>(
        &self,
        traversal: &GraphTraversal<S, E, A>,
    ) -> GremlinResult<impl Iterator<Item = GremlinResult<E>>>
    where
        A: Terminator<E>,
    {
        let result = self.client.submit_traversal(traversal.bytecode())?;

        Ok(RemoteTraversalIterator::new(result))
    }
}
