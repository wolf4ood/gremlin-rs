use crate::client::GremlinClient;
use crate::conversion::FromGValue;
use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::{GraphTraversal, GraphTraversalSource};
use crate::GremlinResult;

pub struct RemoteTraversalSource {}

impl RemoteTraversalSource {
    pub fn with_remote(&self, client: GremlinClient) -> GraphTraversalSource<SyncTerminator> {
        GraphTraversalSource::<MockTerminator>::new(MockTerminator {}).with_remote(client)
    }
}

pub fn traversal() -> RemoteTraversalSource {
    RemoteTraversalSource {}
}

#[derive(Clone)]
pub struct MockTerminator {}

impl MockTerminator {
    pub fn new() -> Self {
        MockTerminator {}
    }
}
impl<T: FromGValue> Terminator<T> for MockTerminator {
    type List = ();

    fn to_list<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }
}
pub trait Terminator<T: FromGValue>: Clone {
    type List;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>;
}

#[derive(Clone)]
pub struct SyncTerminator {
    strategies: TraversalStrategies,
}

impl SyncTerminator {
    pub fn new(strategies: TraversalStrategies) -> SyncTerminator {
        SyncTerminator {
            strategies: strategies,
        }
    }
}

impl<T: FromGValue> Terminator<T> for SyncTerminator {
    type List = GremlinResult<Vec<T>>;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        self.strategies.apply(traversal)?.collect()
    }
}
