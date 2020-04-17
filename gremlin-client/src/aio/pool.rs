use mobc::Manager;

use crate::aio::connection::Conn;
use crate::connection::ConnectionOptions;
use crate::error::GremlinError;
use async_trait::async_trait;

#[derive(Debug)]
pub(crate) struct GremlinConnectionManager {
    options: ConnectionOptions,
}

impl GremlinConnectionManager {
    pub(crate) fn new(options: ConnectionOptions) -> GremlinConnectionManager {
        GremlinConnectionManager { options }
    }
}

#[async_trait]
impl Manager for GremlinConnectionManager {
    type Connection = Conn;
    type Error = GremlinError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Conn::connect(self.options.clone()).await
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {

    use super::GremlinConnectionManager;
    use crate::ConnectionOptions;

    use mobc::Pool;
    use std::time::Duration;

    #[cfg(feature = "async-std-runtime")]
    mod async_std_use {
        pub use async_std::task;
    }

    #[cfg(feature = "async-std-runtime")]
    use async_std_use::*;

    #[cfg(feature = "tokio-runtime")]
    mod tokio_use {
        pub use tokio::task;
    }

    #[cfg(feature = "tokio-runtime")]
    use tokio_use::*;
    
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn it_should_create_a_connection_pool() {
        let manager = GremlinConnectionManager::new(ConnectionOptions::default());

        let pool = Pool::builder().max_open(16).build(manager);

        let conn = pool.get().await.expect("Failed to get the connection");

        pool.state().await;

        assert_eq!(1, pool.state().await.connections);

        assert_eq!(0, pool.state().await.idle);

        drop(conn);

        task::spawn_blocking(move || {
            std::thread::sleep(Duration::from_millis(200));
        })
        .await;

        assert_eq!(1, pool.state().await.idle);
    }
}
