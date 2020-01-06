use crate::aio::pool::GremlinConnectionManager;
use crate::aio::GremlinClient;
use crate::message::Response;
use crate::structure::GValue;
use crate::GremlinResult;
use async_std::stream::Stream;
use core::task::Context;
use core::task::Poll;
use mobc::Connection;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

pub struct GResultSet {
    client: GremlinClient,
    results: VecDeque<GValue>,
    response: Response,
    conn: Option<Connection<GremlinConnectionManager>>,
    state: GResultState,
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
enum GResultState {
    Looping,
    NextPage(
        Box<
            dyn Future<
                    Output = GremlinResult<(
                        Response,
                        VecDeque<GValue>,
                        Connection<GremlinConnectionManager>,
                    )>,
                > + Send,
        >,
    ),
}

impl GResultSet {
    pub(crate) fn new(
        client: GremlinClient,
        results: VecDeque<GValue>,
        response: Response,
        conn: Connection<GremlinConnectionManager>,
    ) -> GResultSet {
        GResultSet {
            client,
            results,
            response,
            conn: Some(conn),
            state: GResultState::Looping,
        }
    }
}

impl Stream for GResultSet {
    type Item = GremlinResult<GValue>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match &mut self.state {
                GResultState::Looping => match self.results.pop_front() {
                    Some(r) => return Poll::Ready(Some(Ok(r))),
                    None => {
                        if self.response.status.code == 206 {
                            let client = self.client.clone();
                            let mut conn = self
                                .conn
                                .take()
                                .expect("It should have the connection at this point");
                            let future = async move {
                                let (response, results) = client.read_response(&mut conn).await?;
                                Ok((response, results, conn))
                            };

                            self.state = GResultState::NextPage(Box::new(future));
                        } else {
                            return Poll::Ready(None);
                        }
                    }
                },
                GResultState::NextPage(page) => {
                    let (response, resuts, conn) =
                        futures::ready!(unsafe { Pin::new_unchecked(page.as_mut()) }.poll(cx))
                            .expect("Failed to fetch the next page");

                    self.conn = Some(conn);
                    self.response = response;
                    self.results = resuts;
                    self.state = GResultState::Looping;
                }
            }
        }
    }
}
