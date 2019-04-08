use crate::process::bytecode::Bytecode;
use crate::process::p::{ToPredicate, P};
use crate::process::strategies::TraversalStrategy;
use crate::GValue;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct GraphTraversal<S, E> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    strategies: Vec<Arc<TraversalStrategy>>,
    bytecode: Bytecode,
}

impl<S, E> GraphTraversal<S, E> {
    pub fn new(
        strategies: Vec<Arc<TraversalStrategy>>,
        bytecode: Bytecode,
    ) -> GraphTraversal<S, E> {
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
}
