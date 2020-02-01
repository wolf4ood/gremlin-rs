use gremlin_client::{ConnectionOptions, Edge, GremlinClient, GremlinResult, Version, Vertex};

#[allow(dead_code)]
pub fn connect() -> GremlinResult<GremlinClient> {
    GremlinClient::connect(("localhost", 8182))
}

#[allow(dead_code)]
pub fn connect_version(version: Version) -> GremlinResult<GremlinClient> {
    let port = match version {
        Version::V2 => 8184,
        Version::V3 => 8182,
    };
    GremlinClient::connect(
        ConnectionOptions::builder()
            .host("localhost")
            .port(port)
            .version(version)
            .build(),
    )
}

#[allow(dead_code)]
pub fn expect_client() -> GremlinClient {
    connect().expect("It should connect")
}

#[allow(dead_code)]
pub fn expect_client_version(version: Version) -> GremlinClient {
    connect_version(version).expect("It should connect")
}

#[allow(dead_code)]
pub fn graph() -> GremlinClient {
    let client = expect_client();

    client
}

#[allow(dead_code)]
pub fn graph_version(version: Version) -> GremlinClient {
    let client = expect_client_version(version);

    client
}

pub fn create_vertex(graph: &GremlinClient, name: &str) -> Vertex {
    create_vertex_with_label(graph, "person", name)
}

#[allow(dead_code)]
pub fn drop_vertices(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
    graph
        .execute("g.V().hasLabel(_label).drop()", &[("_label", &label)])
        .map(|_| ())
}

#[allow(dead_code)]
pub fn drop_edges(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
    graph
        .execute("g.E().hasLabel(_label).drop()", &[("_label", &label)])
        .map(|_| ())
}

pub fn create_vertex_with_label(graph: &GremlinClient, label: &str, name: &str) -> Vertex {
    graph
        .execute(
            "g.addV(_label).property('name',name)",
            &[("_label", &label), ("name", &name)],
        )
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .expect("It should be ok")
        .pop()
        .expect("It should contain 1 element")
}

pub fn create_edge(graph: &GremlinClient, v: &Vertex, v1: &Vertex, name: &str) -> Edge {
    graph
        .execute(
            "g.V(v1).as('a').V(v2).as('b').addE(rel).from('a').to('b')",
            &[("rel", &name), ("v1", v.id()), ("v2", v1.id())],
        )
        .expect("it should execute addE")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .expect("It should be ok")
        .pop()
        .expect("It should contain 1 element")
}
