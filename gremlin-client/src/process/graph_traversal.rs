use crate::process::bytecode::Bytecode;
use crate::process::strategies::TraversalStrategy;
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
}
