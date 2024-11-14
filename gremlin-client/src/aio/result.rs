use crate::aio::GremlinClient;
use crate::message::Response;
use crate::structure::GValue;
use crate::GremlinResult;
use futures::Stream;

use core::task::Context;
use core::task::Poll;
use futures::channel::mpsc::Receiver;
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::pin::Pin;

pin_project! {
    pub struct GResultSet {
        client: GremlinClient,
        results: VecDeque<GValue>,
        response: Response,
        #[pin]
        receiver: Receiver<GremlinResult<Response>>,
    }
}

impl std::fmt::Debug for GResultSet {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "GResultSet {{ response: {:?}, resuls: {:?} }}",
            self.response, self.results
        )
    }
}

impl GResultSet {
    pub(crate) fn new(
        client: GremlinClient,
        results: VecDeque<GValue>,
        response: Response,
        receiver: Receiver<GremlinResult<Response>>,
    ) -> GResultSet {
        GResultSet {
            client,
            results,
            response,
            receiver,
        }
    }
}

impl Stream for GResultSet {
    type Item = GremlinResult<GValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            match this.results.pop_front() {
                Some(r) => return Poll::Ready(Some(Ok(r))),
                None => {
                    if this.response.status.code == 206 {
                        match futures::ready!(this.receiver.as_mut().poll_next(cx)) {
                            Some(Ok(response)) => {
                                let results: VecDeque<GValue> = response
                                    .result
                                    .data
                                    .clone()
                                    .map(Into::into)
                                    .unwrap_or_else(VecDeque::new);
                                *this.results = results;
                                *this.response = response;
                            }
                            Some(Err(e)) => {
                                return Poll::Ready(Some(Err(e)));
                            }
                            None => {
                                return Poll::Ready(None);
                            }
                        }
                    } else {
                        return Poll::Ready(None);
                    }
                }
            }
        }
    }
}
