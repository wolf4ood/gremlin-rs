use crate::client::GremlinClient;
use crate::process::graph_traversal_source::GraphTraversalSource;

pub struct AnonymousTraversalSource {}

impl AnonymousTraversalSource {
    pub fn with_remote(client: GremlinClient) -> GraphTraversalSource {
        GraphTraversalSource::new(vec![]).with_remote(client)
    }
}

pub fn traversal() -> AnonymousTraversalSource {
    AnonymousTraversalSource {}
}
