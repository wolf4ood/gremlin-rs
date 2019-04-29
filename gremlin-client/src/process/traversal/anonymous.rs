use crate::client::GremlinClient;
use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::GraphTraversalSource;

pub struct AnonymousTraversalSource {}

impl AnonymousTraversalSource {
    pub fn with_remote(&self, client: GremlinClient) -> GraphTraversalSource {
        GraphTraversalSource::new(TraversalStrategies::new(vec![])).with_remote(client)
    }
}

pub fn traversal() -> AnonymousTraversalSource {
    AnonymousTraversalSource {}
}
