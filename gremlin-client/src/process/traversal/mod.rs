use crate::conversion::FromGValue;
use crate::structure::Traverser;
use crate::GResultSet;
use crate::GremlinResult;
use std::marker::PhantomData;

mod anonymous_traversal_source;
mod builder;
mod bytecode;
mod graph_traversal;
mod graph_traversal_source;
mod order;
pub(crate) mod remote;
mod scope;
mod step;
mod strategies;
pub use order::Order;
pub use remote::{traversal, SyncTerminator, Terminator};

pub use builder::TraversalBuilder;
pub use bytecode::{Bytecode, Instruction, WRITE_OPERATORS};
pub use graph_traversal::GraphTraversal;
pub use graph_traversal_source::GraphTraversalSource;
pub use scope::Scope;

pub use anonymous_traversal_source::AnonymousTraversalSource;

use lazy_static::lazy_static;

pub use step::*;

pub trait Traversal<S, E> {
    fn bytecode(&self) -> &Bytecode;
}

pub struct RemoteTraversalIterator<T: FromGValue> {
    data: PhantomData<T>,
    result: GResultSet,
}

impl<T: FromGValue> RemoteTraversalIterator<T> {
    pub fn new(result: GResultSet) -> RemoteTraversalIterator<T> {
        RemoteTraversalIterator {
            result,
            data: PhantomData,
        }
    }
}

impl RemoteTraversalIterator<crate::structure::Null> {
    pub fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.next() {
            //consume the entire iterator, returning any errors
            response?;
        }
        Ok(())
    }
}

impl<T: FromGValue> Iterator for RemoteTraversalIterator<T> {
    type Item = GremlinResult<T>;

    // todo remove unwrap
    fn next(&mut self) -> Option<Self::Item> {
        self.result
            .next()
            .map(|e| e.unwrap().take::<Traverser>())
            .map(|t| t.unwrap().take::<T>())
    }
}

lazy_static! {
    pub static ref __: AnonymousTraversalSource = AnonymousTraversalSource::new();
}
