use crate::aio::process::traversal::RemoteTraversalStream;
use crate::aio::GremlinClient;
use crate::conversion::FromGValue;
use crate::process::traversal::remote::Terminator;
use crate::process::traversal::GraphTraversal;
use crate::GremlinResult;
use async_std::prelude::*;
use futures::future::{BoxFuture, FutureExt};

#[derive(Clone)]
pub struct AsyncTerminator {
    client: GremlinClient,
}

impl AsyncTerminator {
    pub fn new(client: GremlinClient) -> AsyncTerminator {
        AsyncTerminator { client }
    }
}

impl<T: FromGValue + std::marker::Send + 'static> Terminator<T> for AsyncTerminator {
    type List = BoxFuture<'static, GremlinResult<Vec<T>>>;
    type Next = BoxFuture<'static, GremlinResult<Option<T>>>;
    type HasNext = BoxFuture<'static, GremlinResult<bool>>;
    type Iter = BoxFuture<'static, GremlinResult<RemoteTraversalStream<T>>>;

    fn to_list<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::List
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            while let Some(item) = stream.next().await {
                vec.push(item?);
            }
            Ok(vec)
        }
        .boxed()
    }

    fn next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Next
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            while let Some(item) = stream.next().await {
                vec.push(item?);
            }
            Ok(vec.pop())
        }
        .boxed()
    }

    fn has_next<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::HasNext
    where
        E: Terminator<T>,
    {
        let iter = self.iter(traversal);

        async move {
            let mut stream = iter.await?;

            let mut vec = vec![];
            while let Some(item) = stream.next().await {
                vec.push(item?);
            }
            Ok(vec.len() > 0)
        }
        .boxed()
    }

    fn iter<S, E>(&self, traversal: &GraphTraversal<S, T, E>) -> Self::Iter
    where
        E: Terminator<T>,
    {
        let client = self.client.clone();
        let bytecode = traversal.bytecode().clone();

        async move {
            let stream = client.submit_traversal(&bytecode).await?;

            Ok(RemoteTraversalStream::new(stream))
        }
        .boxed()
    }
}
