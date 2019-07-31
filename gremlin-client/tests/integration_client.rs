use gremlin_client::{
    ConnectionOptions, GremlinClient, GremlinError, List, ToGValue, TraversalExplanation,
    TraversalMetrics, VertexProperty,
};
use gremlin_client::{Edge, GValue, Map, Vertex};

mod common;

use common::{create_edge, create_vertex, expect_client, graph};

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
fn test_ok_credentials() {
    let client = GremlinClient::connect(
        ConnectionOptions::builder()
            .host("localhost")
            .port(8183)
            .credentials("stephen", "password")
            .build(),
    )
    .expect("Cannot connect");

    let result = client.execute("g.V().limit(1)", &[]);
    assert!(result.is_ok(), format!("{:?}", result));
}

#[test]
fn test_ko_credentials() {
    let client = GremlinClient::connect(
        ConnectionOptions::builder()
            .host("localhost")
            .port(8183)
            .credentials("stephen", "pwd")
            .build(),
    )
    .expect("Cannot connect");

    let result = client.execute("g.V().limit(1)", &[]);
    assert!(result.is_err(), format!("{:?}", result));
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
        .execute("g.E().hasLabel('knows').limit(1)", &[])
        .expect("it should execute a query")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .expect("It should be ok");

    assert_eq!("knows", edges[0].label());
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
        Some(&GValue::List(vec![String::from("mark").into()].into())),
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
            .property('time',time)
            .property('score',score)
            .property('uuid',uuid)
            .property('date',new Date(date))
            .property('dateTime',dateTime)
            .propertyMap()"#;

    let uuid = uuid::Uuid::new_v4();
    let params: &[(&str, &dyn ToGValue)] = &[
        ("age", &22),
        ("time", &(22 as i64)),
        ("name", &"mark"),
        ("score", &3.2),
        ("uuid", &uuid),
        ("dateTime", &chrono::Utc.timestamp(1551825863, 0)),
        ("date", &(1551825863 as i64)),
    ];
    let results = graph
        .execute(q, params)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties = &results[0];

    assert_eq!(8, properties.len());

    assert_eq!(
        &22,
        properties["age"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<i32>()
            .unwrap()
    );

    assert_eq!(
        &22,
        properties["time"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<i64>()
            .unwrap()
    );

    assert_eq!(
        &chrono::Utc.timestamp(1551825863, 0),
        properties["date"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<chrono::DateTime<chrono::Utc>>()
            .unwrap()
    );

    assert!(properties["id"].get::<List>().unwrap()[0]
        .get::<VertexProperty>()
        .unwrap()
        .get::<uuid::Uuid>()
        .is_ok());

    assert_eq!(
        &uuid,
        properties["uuid"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<uuid::Uuid>()
            .unwrap()
    );

    assert_eq!(
        &String::from("mark"),
        properties["name"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );

    assert_eq!(
        &3.2,
        properties["score"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<f64>()
            .unwrap()
    );

    assert_eq!(
        &chrono::Utc.timestamp(1551825863, 0),
        properties["dateTime"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<chrono::DateTime<chrono::Utc>>()
            .unwrap()
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

#[test]
fn test_profile() {
    let graph = graph();

    let metrics = graph
        .execute("g.V().limit(1).profile()", &[])
        .expect("should return a profile")
        .filter_map(Result::ok)
        .map(|f| f.take::<TraversalMetrics>())
        .collect::<Result<Vec<_>, _>>()
        .expect("It should be ok");

    assert_eq!(1, metrics.len());

    let t = &metrics[0];

    assert_eq!(true, t.duration() > &0.0);

    let steps = t.metrics();

    assert_ne!(0, steps.len());

    assert_eq!(
        100.0,
        steps
            .iter()
            .map(|s| s.perc_duration())
            .fold(0.0, |acc, x| acc + x)
    );
}

#[test]
fn test_explain() {
    let graph = graph();

    let metrics = graph
        .execute("g.V().limit(1).explain()", &[])
        .expect("should return a profile")
        .filter_map(Result::ok)
        .map(|f| f.take::<TraversalExplanation>())
        .collect::<Result<Vec<_>, _>>()
        .expect("It should be ok");

    assert_eq!(1, metrics.len());

    let t = &metrics[0];

    assert_eq!(
        &vec![
            String::from("GraphStep(vertex,[])"),
            String::from("RangeGlobalStep(0,1)")
        ],
        t.original()
    );

    assert_eq!(
        &vec![
            String::from("TinkerGraphStep(vertex,[])"),
            String::from("RangeGlobalStep(0,1)"),
            String::from("ReferenceElementStep")
        ],
        t.final_t()
    );
}

#[test]

fn test_group_count_vertex() {
    let graph = graph();
    let mark = create_vertex(&graph, "mark");
    let frank = create_vertex(&graph, "frank");

    create_edge(&graph, &mark, &frank, "knows");

    let map = graph
        .execute(
            "g.V(identity).out().groupCount()",
            &[("identity", mark.id())],
        )
        .expect("should fetch a groupCount")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    assert_eq!(1, map.len());

    let first = &map[0];

    assert_eq!(1, first.len());

    let count = first.get(&frank);

    assert_eq!(Some(&GValue::Int64(1)), count);
}

#[test]

fn test_group_count_edge() {
    let graph = graph();
    let mark = create_vertex(&graph, "mark");
    let frank = create_vertex(&graph, "frank");

    let edge = create_edge(&graph, &mark, &frank, "knows");

    let map = graph
        .execute(
            "g.V(identity).outE().groupCount()",
            &[("identity", mark.id())],
        )
        .expect("should fetch a groupCount")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    assert_eq!(1, map.len());

    let first = &map[0];

    assert_eq!(1, first.len());

    let count = first.get(&edge);

    assert_eq!(Some(&GValue::Int64(1)), count);
}
