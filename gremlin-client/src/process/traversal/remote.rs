use crate::client::GremlinClient;
use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::GraphTraversalSource;

pub struct RemoteTraversalSource {}

impl RemoteTraversalSource {
    pub fn with_remote(&self, client: GremlinClient) -> GraphTraversalSource {
        GraphTraversalSource::new(TraversalStrategies::new(vec![])).with_remote(client)
    }
}

pub fn traversal() -> RemoteTraversalSource {
    RemoteTraversalSource {}
}
