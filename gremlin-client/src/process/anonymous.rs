use crate::client::GremlinClient;
use crate::process::graph_traversal_source::GraphTraversalSource;
use crate::process::strategies::TraversalStrategies;

pub struct AnonymousTraversalSource {}

impl AnonymousTraversalSource {
    pub fn with_remote(&self, client: GremlinClient) -> GraphTraversalSource {
        GraphTraversalSource::new(TraversalStrategies::new(vec![])).with_remote(client)
    }
}

pub fn traversal() -> AnonymousTraversalSource {
    AnonymousTraversalSource {}
}
