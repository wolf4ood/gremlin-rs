use mobc::AnyFuture;
use mobc::ConnectionManager;

use mobc::futures::prelude::*;
use mobc::runtime::DefaultExecutor;
use mobc::Executor;
use mobc::Pool;

use crate::aio::connection::Connection;
use crate::connection::ConnectionOptions;
use crate::error::GremlinError;
use crate::GremlinResult;

pub(crate) type GremlinAsyncPool<T> = Pool<GremlinConnectionManager<T>>;

#[derive(Debug)]
pub(crate) struct GremlinConnectionManager<T> {
    options: ConnectionOptions,
    executor: T,
}

impl GremlinConnectionManager<DefaultExecutor> {
    pub(crate) fn new(options: ConnectionOptions) -> GremlinConnectionManager<DefaultExecutor> {
        GremlinConnectionManager {
            options,
            executor: DefaultExecutor::current(),
        }
    }
}

impl<T> ConnectionManager for GremlinConnectionManager<T>
where
    T: Executor,
{
    type Connection = Connection;
    type Error = GremlinError;
    type Executor = T;

    fn connect(&self) -> AnyFuture<Self::Connection, Self::Error> {
        Box::pin(Connection::connect(self.options.clone()))
    }

    fn get_executor(&self) -> Self::Executor {
        self.executor.clone()
    }

    fn is_valid(&self, conn: Self::Connection) -> AnyFuture<Self::Connection, Self::Error> {
        unimplemented!()
    }

    fn has_broken(&self, conn: &mut Option<Self::Connection>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::GremlinConnectionManager;
    use crate::ConnectionOptions;

    use async_std::task;
    use mobc::Pool;

    #[test]
    fn it_should_create_a_connection_pool() {
        task::block_on(async {
            let manager = GremlinConnectionManager::new(ConnectionOptions::default());

            let pool = Pool::builder()
                .max_size(16)
                .build(manager)
                .await
                .expect("Failed to create the pool");

            let conn = pool.get().await.expect("Failed to get the connection");

            pool.state().await;

            assert_eq!(16, pool.state().await.connections);

            assert_eq!(15, pool.state().await.idle_connections);

            drop(conn);

            task::spawn_blocking(move || {
                std::thread::sleep_ms(200);
            })
            .await;

            assert_eq!(16, pool.state().await.idle_connections);
        });
    }
}
