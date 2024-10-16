use crate::aio::result::GResultSet;
pub(crate) mod remote;
use crate::conversion::FromGValue;
use crate::structure::Traverser;
use crate::GremlinResult;
use core::task::Context;
use core::task::Poll;
use futures::Stream;
use futures::StreamExt;
use std::marker::PhantomData;
use std::pin::Pin;

use pin_project_lite::pin_project;

pub use remote::AsyncTerminator;

pin_project! {
    pub struct RemoteTraversalStream<T> {
        phantom: PhantomData<T>,
        #[pin]
        stream: GResultSet,
    }
}

impl<T> RemoteTraversalStream<T> {
    fn new(stream: GResultSet) -> Self {
        RemoteTraversalStream {
            phantom: PhantomData,
            stream,
        }
    }
}

impl RemoteTraversalStream<crate::structure::Null> {
    pub async fn iterate(&mut self) -> GremlinResult<()> {
        while let Some(response) = self.next().await {
            //consume the entire stream, returning any errors
            response?;
        }
        Ok(())
    }
}

impl<T: FromGValue> Stream for RemoteTraversalStream<T> {
    type Item = GremlinResult<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let item = futures::ready!(this.stream.poll_next(cx));

        Poll::Ready(item.map(|e| {
            e.expect("Failed to take an item from the result set")
                .take::<Traverser>()
                .expect("Failed to convert the item to a Traverser")
                .take::<T>()
        }))
    }
}
