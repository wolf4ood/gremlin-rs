use crate::conversion::FromGValue;
use crate::process::bytecode::Bytecode;
use crate::process::p::ToPredicate;
use crate::process::strategies::TraversalStrategies;
use crate::{GValue, GremlinResult};
use std::marker::PhantomData;

pub struct GraphTraversal<S, E: FromGValue> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    strategies: TraversalStrategies,
    bytecode: Bytecode,
}

impl<S, E: FromGValue> GraphTraversal<S, E> {
    pub fn new(strategies: TraversalStrategies, bytecode: Bytecode) -> GraphTraversal<S, E> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            bytecode,
            strategies,
        }
    }
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }

    pub fn has_label(mut self, labels: &[&str]) -> Self {
        self.bytecode.add_step(
            String::from("hasLabel"),
            labels.iter().map(|s| GValue::from(*s)).collect(),
        );
        self
    }

    pub fn has<P>(mut self, key: &str, predicate: &P) -> Self
    where
        P: ToPredicate,
    {
        let p = predicate.to_p();
        self.bytecode.add_step(
            String::from("has"),
            vec![String::from(key).into(), p.into()],
        );
        self
    }

    pub fn to_list(&self) -> GremlinResult<Vec<E>> {
        self.strategies.apply(self)?.collect()
    }
}
