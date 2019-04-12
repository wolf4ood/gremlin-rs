use crate::conversion::FromGValue;
use crate::process::bytecode::Bytecode;
use crate::structure::Traverser;
use crate::GResultSet;
use crate::GremlinResult;
use std::marker::PhantomData;

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
