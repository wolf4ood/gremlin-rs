use gremlin_client::{GremlinClient, GremlinResult};

pub fn connect() -> GremlinResult<GremlinClient> {
    GremlinClient::connect(("localhost", 8182))
}

pub fn expect_client() -> GremlinClient {
    connect().expect("It should connect")
}

pub fn graph() -> GremlinClient {
    let client = expect_client();

    client
}
