use crate::client::GremlinClient;
use crate::conversion::FromGValue;
use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::RemoteTraversalIterator;
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

impl Default for MockTerminator {
    fn default() -> Self {
        MockTerminator {}
    }
}

impl MockTerminator {
    pub fn new() -> Self {
        MockTerminator {}
    }
}
impl<T: FromGValue> Terminator<T> for MockTerminator {
    type List = ();
    type Next = ();
    type HasNext = ();
    type Iter = ();

    fn to_list<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }

    fn next<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }

    fn has_next<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }

    fn iter<S, E>(&self, _traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>,
    {
        unimplemented!()
    }
}
pub trait Terminator<T: FromGValue>: Clone {
    type List;
    type Next;
    type HasNext;
    type Iter;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>;

    fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>;

    fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>;

    fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>;
}

#[derive(Clone)]
pub struct SyncTerminator {
    strategies: TraversalStrategies,
}

impl SyncTerminator {
    pub fn new(strategies: TraversalStrategies) -> SyncTerminator {
        SyncTerminator { strategies }
    }
}

impl<T: FromGValue> Terminator<T> for SyncTerminator {
    type List = GremlinResult<Vec<T>>;
    type Next = GremlinResult<Option<T>>;
    type HasNext = GremlinResult<bool>;
    type Iter = GremlinResult<RemoteTraversalIterator<T>>;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        self.strategies.apply(traversal)?.collect()
    }

    fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>,
    {
        let results: GremlinResult<Vec<T>> = self.strategies.apply(traversal)?.collect();

        Ok(results?.into_iter().next())
    }

    fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>,
    {
        let results: GremlinResult<Vec<T>> = self.strategies.apply(traversal)?.collect();

        Ok(results?.iter().next().is_some())
    }

    fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>,
    {
        self.strategies.apply(traversal)
    }
}
