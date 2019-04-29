mod remote;
use crate::process::traversal::GraphTraversal;
use crate::{GremlinError, GremlinResult};
pub use remote::RemoteStrategy;

use crate::conversion::FromGValue;

#[derive(Clone)]
pub enum TraversalStrategy {
    Remote(RemoteStrategy),
}

impl TraversalStrategy {
    pub(crate) fn apply<S, E: FromGValue>(
        &self,
        traversal: &GraphTraversal<S, E>,
    ) -> GremlinResult<impl Iterator<Item = GremlinResult<E>>> {
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

    pub(crate) fn apply<S, E: FromGValue>(
        &self,
        traversal: &GraphTraversal<S, E>,
    ) -> GremlinResult<impl Iterator<Item = GremlinResult<E>>> {
        let mut iter = None;
        for strategy in self.strategies.iter() {
            iter = Some(strategy.apply(traversal)?);
        }

        iter.map(Ok)
            .unwrap_or_else(|| Err(GremlinError::Generic(String::from("Empty traversal"))))
    }
}
