use crate::aio::result::GResultSet;
pub(crate) mod remote;
use crate::conversion::FromGValue;
use crate::structure::Traverser;
use crate::GremlinResult;
use async_std::stream::Stream;
use core::task::Context;
use core::task::Poll;
use std::marker::PhantomData;
use std::pin::Pin;

use pin_project_lite::pin_project;

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
impl<T: FromGValue> Stream for RemoteTraversalStream<T> {
    type Item = GremlinResult<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let item = futures::ready!(this.stream.poll_next(cx));

        Poll::Ready(item.map(|e| e.unwrap().take::<Traverser>().unwrap().take::<T>()))
    }
}
