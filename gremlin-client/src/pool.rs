use r2d2::ManageConnection;

use crate::connection::Connection;
use crate::connection::ConnectionOptions;
use crate::error::GremlinError;
use crate::GremlinResult;

#[derive(Debug)]
pub(crate) struct GremlinConnectionManager {
    options: ConnectionOptions,
}

impl GremlinConnectionManager {
    pub(crate) fn new(options: ConnectionOptions) -> GremlinConnectionManager {
        GremlinConnectionManager { options }
    }
}

impl ManageConnection for GremlinConnectionManager {
    type Connection = Connection;
    type Error = GremlinError;

    fn connect(&self) -> GremlinResult<Connection> {
        Connection::connect(self.options.clone())
    }
    fn is_valid(&self, _conn: &mut Connection) -> Result<(), GremlinError> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Connection) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::GremlinConnectionManager;
    use crate::ConnectionOptions;

    use r2d2::Pool;

    #[test]
    fn it_should_create_a_connection_pool() {
        let manager = GremlinConnectionManager::new(ConnectionOptions::default());

        let result = Pool::builder().max_size(16).build(manager);

        let pool = result.unwrap();

        let connection = pool.get();

        assert_eq!(16, pool.state().connections);

        assert_eq!(15, pool.state().idle_connections);

        drop(connection);

        assert_eq!(16, pool.state().idle_connections);
    }
}
