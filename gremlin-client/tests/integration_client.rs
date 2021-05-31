mod common;

use std::collections::HashSet;
use std::iter::FromIterator;

use chrono::{offset::TimeZone, DateTime, Utc};
use gremlin_client::{
    ConnectionOptions, GremlinClient, GremlinError, List, TlsOptions, ToGValue,
    TraversalExplanation, TraversalMetrics, VertexProperty,
};
use gremlin_client::{Edge, GValue, Map, Vertex};

use common::io::{create_edge, create_vertex, expect_client, graph};

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
fn test_session_empty_query() {
    let mut graph = graph();
    let mut sessioned_graph = graph
        .create_session("test-session".to_string())
        .expect("It should create a session.");
    assert_eq!(
        0,
        sessioned_graph
            .execute("g.V().hasLabel('Not Found')", &[])
            .expect("It should execute a traversal")
            .count()
    );
    sessioned_graph
        .close_session()
        .expect("It should close the session.");
}

#[test]
fn test_ok_credentials() {
    let client = GremlinClient::connect(
        ConnectionOptions::builder()
            .host("localhost")
            .port(8183)
            .credentials("stephen", "password")
            .ssl(true)
            .tls_options(TlsOptions {
                accept_invalid_certs: true,
            })
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
            .ssl(true)
            .tls_options(TlsOptions {
                accept_invalid_certs: true,
            })
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
fn test_complex_vertex_creation_with_option_none_properties() {
    let graph = graph();
    let properties = graph
        .execute(r#"g.addV('person').valueMap()"#, &[])
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .next()
        .expect("There should be 1 traversal element")
        .expect("The traversal response should not have errored");

    //No properties should be present
    assert_eq!(0, properties.len());

    //Now demonstrate retrieving Option of the various value types
    //All responses should be Ok and be containing a None
    assert!(properties
        .try_get::<&str, Option<String>>("name")
        .unwrap()
        .is_none());
    assert!(properties
        .try_get::<&str, Option<i32>>("age")
        .unwrap()
        .is_none());
    assert!(properties
        .try_get::<&str, Option<i64>>("time")
        .unwrap()
        .is_none());
    assert!(properties
        .try_get::<&str, Option<f32>>("score")
        .unwrap()
        .is_none());
    assert!(properties
        .try_get::<&str, Option<f64>>("score2")
        .unwrap()
        .is_none());
    assert!(properties
        .try_get::<&str, Option<chrono::DateTime<chrono::offset::Utc>>>("date")
        .unwrap()
        .is_none());
    assert!(properties
        .try_get::<&str, Option<uuid::Uuid>>("uuid")
        .unwrap()
        .is_none());
}

#[test]
fn test_complex_vertex_creation_with_option_some_properties() {
    let graph = graph();
    let q = r#"
        g.addV('person')
            .property('name',name)
            .property('age',age)
            .property('time',time)
            .property('score',score)
            .property('score2',score2)
            .property('uuid',uuid)
            .property('date',date)
            .property('bool',true)
            .valueMap()"#;

    let uuid = uuid::Uuid::new_v4();
    let now = Utc.timestamp(5, 0);
    let params: &[(&str, &dyn ToGValue)] = &[
        ("name", &"mark"),
        ("age", &(22 as i32)),
        ("time", &(23 as i64)),
        ("score", &(3.2 as f32)),
        ("score2", &(3.2 as f64)),
        ("uuid", &uuid),
        ("date", &now),
    ];
    let properties = graph
        .execute(q, params)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .next()
        .expect("There should be 1 traversal element")
        .expect("The traversal response should not have errored");

    assert_eq!(8, properties.len());

    //Now demonstrate retrieving Option of the various value types
    //All responses should be Ok contain a Some value
    assert_eq!(
        "mark",
        properties
            .try_get::<&str, Option<String>>("name")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        22,
        properties
            .try_get::<&str, Option<i32>>("age")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        23,
        properties
            .try_get::<&str, Option<i64>>("time")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        3.2 as f32,
        properties
            .try_get::<&str, Option<f32>>("score")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        3.2 as f64,
        properties
            .try_get::<&str, Option<f64>>("score2")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        now,
        properties
            .try_get::<&str, Option<chrono::DateTime<chrono::offset::Utc>>>("date")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        uuid,
        properties
            .try_get::<&str, Option<uuid::Uuid>>("uuid")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        true,
        properties
            .try_get::<&str, Option<bool>>("bool")
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_complex_vertex_creation_with_properties() {
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
        &chrono::Utc.timestamp_millis(1551825863),
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
fn test_inserting_date_with_milisecond_precision() {
    use chrono::offset::TimeZone;
    use chrono::DateTime;
    use chrono::Utc;

    let graph = graph();

    let q = r#"g.addV('person').property('dateTime',dateTime).propertyMap()"#;

    let expected = chrono::Utc.timestamp(1551825863, 0);
    let params: &[(&str, &dyn ToGValue)] = &[("dateTime", &expected)];

    let results = graph
        .execute(q, params)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties = &results[0];

    assert_eq!(1, properties.len());

    assert_eq!(
        &expected,
        properties["dateTime"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<DateTime<Utc>>()
            .unwrap()
    );
}

#[test]
fn test_list_cardinality() {
    let graph = graph();

    //split into 2 queries due to the bindings limit

    let q1 = r#"
        g.addV('person')
            .property(list, 'name','name1')
            .property(list, 'name','name2')
            .property(list, 'name','name2')
            .property(list, 'name','name3')
            .property(list, 'num1', i32_1)
            .property(list, 'num1', i32_2)
            .property(list, 'num1', i32_3)
            .property(list, 'num1', i32_4)
            .property(list, 'num2', i64_1)
            .property(list, 'num2', i64_2)
            .property(list, 'num2', i64_3)
            .property(list, 'num2', i64_4)
            .property(list, 'float1', f32_1)
            .property(list, 'float1', f32_2)
            .property(list, 'float1', f32_3)
            .property(list, 'float1', f32_4)
            .valueMap()"#;

    let q2 = r#"
        g.addV('person')
            .property(list, 'double1', f64_1)
            .property(list, 'double1', f64_2)
            .property(list, 'double1', f64_3)
            .property(list, 'double1', f64_4)
            .property(list, 'date1', date_1)
            .property(list, 'date1', date_2)
            .property(list, 'date1', date_3)
            .property(list, 'date1', date_4)
            .property(list, 'uuid1', uuid_1)
            .property(list, 'uuid1', uuid_2)
            .property(list, 'uuid1', uuid_3)
            .property(list, 'uuid1', uuid_4)
            .property(list, 'bool1', bool_1)
            .property(list, 'bool1', bool_2)
            .property(list, 'bool1', bool_3)
            .property(list, 'bool1', bool_4)
            .valueMap()"#;

    let date_1 = Utc.timestamp(1, 0);
    let date_2 = Utc.timestamp(1, 0);
    let date_3 = Utc.timestamp(2, 0);
    let date_4 = Utc.timestamp(3, 0);

    let uuid_1 = uuid::Uuid::new_v4();
    let uuid_2 = uuid::Uuid::new_v4();
    let uuid_3 = uuid::Uuid::new_v4();

    let params1: &[(&str, &dyn ToGValue)] = &[
        ("i32_1", &(1 as i32)),
        ("i32_2", &(1 as i32)),
        ("i32_3", &(2 as i32)),
        ("i32_4", &(3 as i32)),
        ("i64_1", &(4 as i64)),
        ("i64_2", &(4 as i64)),
        ("i64_3", &(5 as i64)),
        ("i64_4", &(6 as i64)),
        ("f32_1", &(1.1 as f32)),
        ("f32_2", &(1.1 as f32)),
        ("f32_3", &(2.3 as f32)),
        ("f32_4", &(3.4 as f32)),
    ];

    let params2: &[(&str, &dyn ToGValue)] = &[
        ("f64_1", &(4.4 as f64)),
        ("f64_2", &(4.4 as f64)),
        ("f64_3", &(5.5 as f64)),
        ("f64_4", &(6.6 as f64)),
        ("date_1", &date_1),
        ("date_2", &date_2),
        ("date_3", &date_3),
        ("date_4", &date_4),
        ("uuid_1", &uuid_1),
        ("uuid_2", &uuid_1),
        ("uuid_3", &uuid_2),
        ("uuid_4", &uuid_3),
        ("bool_1", &false),
        ("bool_2", &true),
        ("bool_3", &false),
        ("bool_4", &true),
    ];

    let results1 = graph
        .execute(q1, params1)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties1 = &results1[0];
    let string_list = properties1["name"].clone().take::<Vec<String>>().unwrap();
    assert_eq!(string_list, vec!["name1", "name2", "name2", "name3"]);

    let i32_list = properties1["num1"].clone().take::<Vec<i32>>().unwrap();
    assert_eq!(i32_list, vec![1, 1, 2, 3]);

    let i64_list = properties1["num2"].clone().take::<Vec<i64>>().unwrap();
    assert_eq!(i64_list, vec![4, 4, 5, 6]);

    let f32_list = properties1["float1"].clone().take::<Vec<f32>>().unwrap();
    assert_eq!(f32_list, vec![1.1, 1.1, 2.3, 3.4]);

    let results2 = graph
        .execute(q2, params2)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties2 = &results2[0];

    let f64_list = properties2["double1"].clone().take::<Vec<f64>>().unwrap();
    assert_eq!(f64_list, vec![4.4, 4.4, 5.5, 6.6]);

    let date_list = properties2["date1"]
        .clone()
        .take::<Vec<DateTime<Utc>>>()
        .unwrap();
    assert_eq!(date_list, vec![date_1, date_2, date_3, date_4]);

    let uuid_list = properties2["uuid1"]
        .clone()
        .take::<Vec<uuid::Uuid>>()
        .unwrap();
    assert_eq!(uuid_list, vec![uuid_1, uuid_1, uuid_2, uuid_3]);

    let boolean_list = properties2["bool1"].clone().take::<Vec<bool>>().unwrap();
    assert_eq!(boolean_list, vec![false, true, false, true]);
}

#[test]
fn test_set_cardinality() {
    let graph = graph();

    //split into 2 queries due to the bindings limit

    let q1 = r#"
        g.addV('person')
            .property(set, 'name_set','name1')
            .property(set, 'name_set','name2')
            .property(set, 'name_set','name2')
            .property(set, 'name_set','name3')
            .property(set, 'num1_set', i32_1)
            .property(set, 'num1_set', i32_2)
            .property(set, 'num1_set', i32_3)
            .property(set, 'num1_set', i32_4)
            .property(set, 'num2_set', i64_1)
            .property(set, 'num2_set', i64_2)
            .property(set, 'num2_set', i64_3)
            .property(set, 'num2_set', i64_4)
            .valueMap()"#;

    let q2 = r#"
        g.addV('person')
            .property(set, 'date1_set', date_1)
            .property(set, 'date1_set', date_2)
            .property(set, 'date1_set', date_3)
            .property(set, 'date1_set', date_4)
            .property(set, 'uuid1_set', uuid_1)
            .property(set, 'uuid1_set', uuid_2)
            .property(set, 'uuid1_set', uuid_3)
            .property(set, 'uuid1_set', uuid_4)
            .property(set, 'bool1_set', bool_1)
            .property(set, 'bool1_set', bool_2)
            .property(set, 'bool1_set', bool_3)
            .property(set, 'bool1_set', bool_4)
            .valueMap()"#;

    let date_1 = Utc.timestamp(1, 0);
    let date_3 = Utc.timestamp(2, 0);
    let date_4 = Utc.timestamp(3, 0);

    let uuid_1 = uuid::Uuid::new_v4();
    let uuid_2 = uuid::Uuid::new_v4();
    let uuid_3 = uuid::Uuid::new_v4();

    let params1: &[(&str, &dyn ToGValue)] = &[
        ("i32_1", &(1 as i32)),
        ("i32_2", &(1 as i32)),
        ("i32_3", &(2 as i32)),
        ("i32_4", &(3 as i32)),
        ("i64_1", &(4 as i64)),
        ("i64_2", &(4 as i64)),
        ("i64_3", &(5 as i64)),
        ("i64_4", &(6 as i64)),
    ];

    let params2: &[(&str, &dyn ToGValue)] = &[
        ("date_1", &date_1),
        ("date_2", &date_1),
        ("date_3", &date_3),
        ("date_4", &date_4),
        ("uuid_1", &uuid_1),
        ("uuid_2", &uuid_1),
        ("uuid_3", &uuid_2),
        ("uuid_4", &uuid_3),
        ("bool_1", &false),
        ("bool_2", &true),
        ("bool_3", &false),
        ("bool_4", &true),
    ];

    let results1 = graph
        .execute(q1, params1)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties1 = &results1[0];
    let string_set = properties1["name_set"]
        .clone()
        .take::<HashSet<String>>()
        .unwrap();
    assert_eq!(
        string_set,
        HashSet::from_iter(
            vec![
                "name1".to_string(),
                "name2".to_string(),
                "name3".to_string()
            ]
            .iter()
            .cloned()
        )
    );

    let i32_set = properties1["num1_set"]
        .clone()
        .take::<HashSet<i32>>()
        .unwrap();
    assert_eq!(i32_set, HashSet::from_iter(vec![1, 2, 3].iter().cloned()));

    let i64_set = properties1["num2_set"]
        .clone()
        .take::<HashSet<i64>>()
        .unwrap();
    assert_eq!(i64_set, HashSet::from_iter(vec![4, 5, 6].iter().cloned()));

    let results2 = graph
        .execute(q2, params2)
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties2 = &results2[0];

    let date_set = properties2["date1_set"]
        .clone()
        .take::<HashSet<DateTime<Utc>>>()
        .unwrap();
    assert_eq!(
        date_set,
        HashSet::from_iter(vec![date_1, date_3, date_4].iter().cloned())
    );

    let uuid_set = properties2["uuid1_set"]
        .clone()
        .take::<HashSet<uuid::Uuid>>()
        .unwrap();
    assert_eq!(
        uuid_set,
        HashSet::from_iter(vec![uuid_1, uuid_2, uuid_3].iter().cloned())
    );

    let boolean_set = properties2["bool1_set"]
        .clone()
        .take::<HashSet<bool>>()
        .unwrap();
    assert_eq!(
        boolean_set,
        HashSet::from_iter(vec![false, true].iter().cloned())
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
            .round()
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

#[test]
#[cfg(feature = "derive")]
fn test_vertex_mapping() {
    let graph = graph();
    use gremlin_client::derive::FromGValue;
    use std::convert::TryFrom;

    let q = r#"
    g.addV('person')
        .property('id',UUID.randomUUID())
        .property('name',name)
        .property('age',age)
        .property('time',time)
        .property('score',score)
        .property('uuid',uuid)
        .property('date',new Date(date))
        .property('dateTime',dateTime)"#;

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
    let mark = graph
        .execute(q, params)
        .expect("should create a vertex")
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .expect("It should be ok");

    #[derive(Debug, PartialEq, FromGValue)]
    struct Person {
        name: String,
        age: i32,
        time: i64,
        optional: Option<String>,
    }

    assert_eq!("person", mark[0].label());

    let value_map = graph
        .execute("g.V(identity).valueMap()", &[("identity", mark[0].id())])
        .expect("should fetch valueMap with properties")
        .filter_map(Result::ok)
        .map(|f| Person::try_from(f))
        .collect::<Result<Vec<Person>, _>>()
        .expect("It should be ok");

    assert_eq!(1, value_map.len());

    assert_eq!(
        Person {
            name: String::from("mark"),
            age: 22,
            time: 22,
            optional: None
        },
        value_map[0]
    );
}
