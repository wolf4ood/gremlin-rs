extern crate gremlin_client;

use gremlin_client::{Edge, GValue, Map, Vertex};
use gremlin_client::{GremlinClient, GremlinError, GremlinResult, ToGValue, VertexProperty};

fn connect() -> GremlinResult<GremlinClient> {
    GremlinClient::connect(("localhost", 8182))
}

fn expect_client() -> GremlinClient {
    connect().expect("It should connect")
}

fn graph() -> GremlinClient {
    let client = expect_client();

    client
}

#[test]
fn test_client_connection_ok() {
    expect_client();
}

#[test]
fn test_empty_query() {
    assert_eq!(
        0,
        graph()
            .execute("g.V().hasLabel('NotFound')", &[])
            .expect("It should execute a traversal")
            .count()
    )
}

#[test]
fn test_wrong_query() {
    let error = graph()
        .execute("g.V", &[])
        .expect_err("it should return an error");

    match error {
        GremlinError::Request((code, message)) => {
            assert_eq!(597, code);
            assert_eq!("No such property: V for class: org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource",message)
        }
        _ => panic!("wrong error type"),
    }
}

#[test]
fn test_wrong_alias() {
    let error = graph()
        .alias("foo")
        .execute("g.V()", &[])
        .expect_err("it should return an error");

    match error {
        GremlinError::Request((code, message)) => {
            assert_eq!(499, code);
            assert_eq!("Could not alias [g] to [foo] as [foo] not in the Graph or TraversalSource global bindings",message)
        }
        _ => panic!("wrong error type"),
    }
}

#[test]

fn test_vertex_query() {
    let graph = graph();
    let vertices = graph
        .execute(
            "g.V().hasLabel('person').has('name',name)",
            &[("name", &"marko")],
        )
        .expect("it should execute a query")
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .expect("It should be ok");

    assert_eq!("person", vertices[0].label());
}
#[test]
fn test_edge_query() {
    let graph = graph();
    let edges = graph
        .execute("g.E().limit(1)", &[])
        .expect("it should execute a query")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .expect("It should be ok");

    assert_eq!("knows", edges[0].label());
}

fn create_vertex(graph: &GremlinClient, name: &str) -> Vertex {
    graph
        .execute("g.addV('person').property('name',name)", &[("name", &name)])
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .expect("It should be ok")
        .pop()
        .expect("It should contain 1 element")
}

fn create_edge(graph: &GremlinClient, v: &Vertex, v1: &Vertex, name: &str) -> Edge {
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
#[test]
fn test_vertex_creation() {
    let graph = graph();
    let mark = create_vertex(&graph, "mark");

    assert_eq!("person", mark.label());

    let value_map = graph
        .execute("g.V(identity).valueMap()", &[("identity", mark.id())])
        .expect("should fetch valueMap with properties")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    assert_eq!(1, value_map.len());

    assert_eq!(
        Some(&GValue::List(vec![String::from("mark").into()])),
        value_map[0].get("name")
    );
}

#[test]
fn test_complex_vertex_creation_with_properties() {
    use chrono::offset::TimeZone;

    let graph = graph();

    let q = r#"
        g.addV('person')
            .property('id',UUID.randomUUID())
            .property('name',name)
            .property('age',age)
            .property('score',score)
            .property('date',new Date(date))
            .properties()"#;

    let params: &[(&str, &dyn ToGValue)] = &[
        ("age", &22),
        ("name", &"mark"),
        ("score", &3.2),
        ("date", &1551825863),
    ];
    let mut properties = graph
        .execute(q, params)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<VertexProperty>())
        .collect::<Result<Vec<VertexProperty>, _>>()
        .expect("It should be ok");

    properties.sort_by(|a, b| a.label().cmp(b.label()));

    assert_eq!(5, properties.len());

    assert_eq!(
        22,
        properties[0]
            .clone()
            .take::<i32>()
            .expect("It should be aa integer")
    );

    assert_eq!(
        chrono::Utc.timestamp(1551825863, 0),
        properties[1]
            .clone()
            .take::<chrono::DateTime<chrono::Utc>>()
            .expect("It should be a Date")
    );

    assert_ne!(
        uuid::Uuid::new_v4(),
        properties[2]
            .clone()
            .take::<uuid::Uuid>()
            .expect("It should be an integer")
    );

    assert_eq!(
        String::from("mark"),
        properties[3]
            .clone()
            .take::<String>()
            .expect("It should be a string")
    );

    assert_eq!(
        3.2,
        properties[4]
            .clone()
            .take::<f64>()
            .expect("It should be a double")
    );
}

#[test]
fn test_edge_creation() {
    let graph = graph();
    let mark = create_vertex(&graph, "mark");
    let frank = create_vertex(&graph, "frank");

    let edge = create_edge(&graph, &mark, &frank, "knows");

    assert_eq!("knows", edge.label());

    assert_eq!(&mark, edge.out_v());
    assert_eq!(&frank, edge.in_v());

    let edges = graph
        .execute("g.V(identity).outE()", &[("identity", mark.id())])
        .expect("should fetch edge")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .expect("It should be ok");

    assert_eq!(1, edges.len());

    let edge = &edges[0];

    assert_eq!("knows", edge.label());

    assert_eq!(&mark, edge.out_v());
    assert_eq!(&frank, edge.in_v());
}
