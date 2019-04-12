use crate::conversion::FromGValue;

use crate::{
    process::traversal::RemoteTraversalIterator, process::GraphTraversal, GremlinClient,
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

    pub(crate) fn apply<S, E: FromGValue>(
        &self,
        traversal: &GraphTraversal<S, E>,
    ) -> GremlinResult<impl Iterator<Item = GremlinResult<E>>> {
        let result = self.client.submit_traversal(traversal.bytecode())?;

        Ok(RemoteTraversalIterator::new(result))
    }
}
