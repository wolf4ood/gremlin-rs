mod remote;
use crate::conversion::FromGValue;
use crate::process::traversal::remote::Terminator;
use crate::process::traversal::GraphTraversal;
use crate::process::traversal::RemoteTraversalIterator;
use crate::{GremlinError, GremlinResult};
pub use remote::RemoteStrategy;

#[derive(Clone)]
pub enum TraversalStrategy {
    Remote(RemoteStrategy),
}

impl TraversalStrategy {
    pub(crate) fn apply<S, E: FromGValue, A>(
        &self,
        traversal: &GraphTraversal<S, E, A>,
    ) -> GremlinResult<RemoteTraversalIterator<E>>
    where
        A: Terminator<E>,
    {
        match self {
            TraversalStrategy::Remote(x) => x.apply(traversal),
        }
    }
}

#[derive(Clone)]
pub struct TraversalStrategies {
    strategies: Vec<TraversalStrategy>,
}

impl TraversalStrategies {
    pub(crate) fn new(strategies: Vec<TraversalStrategy>) -> TraversalStrategies {
        TraversalStrategies { strategies }
    }
    pub(crate) fn add_strategy(&mut self, strategy: TraversalStrategy) {
        self.strategies.push(strategy);
    }

    pub(crate) fn apply<S, E: FromGValue, A>(
        &self,
        traversal: &GraphTraversal<S, E, A>,
    ) -> GremlinResult<RemoteTraversalIterator<E>>
    where
        A: Terminator<E>,
    {
        let mut iter = None;
        for strategy in self.strategies.iter() {
            iter = Some(strategy.apply(traversal)?);
        }

        iter.map(Ok)
            .unwrap_or_else(|| Err(GremlinError::Generic(String::from("Empty traversal"))))
    }
}
