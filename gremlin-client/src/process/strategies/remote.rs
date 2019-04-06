use crate::process::strategies::TraversalStrategy;
use crate::GremlinClient;

pub struct RemoteStrategy {
    client: GremlinClient,
}

impl RemoteStrategy {
    pub fn new(client: GremlinClient) -> RemoteStrategy {
        RemoteStrategy { client }
    }
}

impl TraversalStrategy for RemoteStrategy {}
