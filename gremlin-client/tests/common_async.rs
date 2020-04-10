use gremlin_client::aio::GremlinClient;

use gremlin_client::{ConnectionOptions, Edge, GraphSON, GremlinResult, Vertex};

use async_std::prelude::*;

#[allow(dead_code)]
pub async fn connect() -> GremlinClient {
    GremlinClient::connect(("localhost", 8182))
        .await
        .expect("It should connect")
}

#[allow(dead_code)]
pub async fn connect_serializer(serializer: GraphSON) -> GremlinClient {
    let port = match serializer {
        GraphSON::V1 => 8186,
        GraphSON::V2 => 8184,
        GraphSON::V3 => 8182,
    };
    GremlinClient::connect(
        ConnectionOptions::builder()
            .host("localhost")
            .port(port)
            .serializer(serializer.clone())
            .deserializer(serializer)
            .build(),
    )
    .await
    .expect("It should connect")
}

#[allow(dead_code)]
pub async fn create_vertex(graph: &GremlinClient, name: &str) -> Vertex {
    create_vertex_with_label(graph, "person", name).await
}

pub async fn create_vertex_with_label(graph: &GremlinClient, label: &str, name: &str) -> Vertex {
    graph
        .execute(
            "g.addV(_label).property('name',name)",
            &[("_label", &label), ("name", &name)],
        )
        .await
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .await
        .expect("It should be ok")
        .pop()
        .expect("It should contain 1 element")
}

#[allow(dead_code)]
pub async fn drop_vertices(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
    graph
        .execute("g.V().hasLabel(_label).drop()", &[("_label", &label)])
        .await
        .map(|_| ())
}

#[allow(dead_code)]
pub async fn drop_edges(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
    graph
        .execute("g.E().hasLabel(_label).drop()", &[("_label", &label)])
        .await
        .map(|_| ())
}

#[allow(dead_code)]
pub async fn create_edge(graph: &GremlinClient, v: &Vertex, v1: &Vertex, name: &str) -> Edge {
    graph
        .execute(
            "g.V(v1).as('a').V(v2).as('b').addE(rel).from('a').to('b')",
            &[("rel", &name), ("v1", v.id()), ("v2", v1.id())],
        )
        .await
        .expect("it should execute addE")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .await
        .expect("It should be ok")
        .pop()
        .expect("It should contain 1 element")
}
