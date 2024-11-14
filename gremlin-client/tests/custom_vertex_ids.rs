use std::collections::HashMap;

use common::io::expect_janusgraph_client;
use gremlin_client::{
    process::traversal::{traversal, __},
    structure::Edge,
    structure::T,
    GKey, GValue, IoProtocol, Property,
};

use rstest::*;
use rstest_reuse::apply;
use serial_test::serial;
use uuid::Uuid;

mod common;

//Custom vertex ids are a feature offered by JanusGraph
//https://docs.janusgraph.org/advanced-topics/custom-vertex-id/

#[apply(common::serializers)]
#[serial(jg_throttle)]
#[cfg(feature = "derive")]
fn test_mapping_custom_vertex_id(protocol: IoProtocol) {
    if protocol == IoProtocol::GraphSONV2 {
        //GraphSONV2 doesn't support the non-string key of the merge step,
        //so skip it in testing
        return;
    }
    let client = expect_janusgraph_client(protocol);
    use chrono::{DateTime, TimeZone, Utc};
    use gremlin_client::derive::FromGMap;
    use gremlin_client::process::traversal::{Bytecode, TraversalBuilder};
    use std::convert::TryFrom;

    let g = traversal().with_remote(client);

    let uuid = uuid::Uuid::new_v4();
    let mark_id = create_novel_vertex_id();
    let mark = g
        .add_v("test_mapping_custom_vertex_id")
        .property(T::Id, mark_id.as_str())
        .property("name", "Mark")
        .property("age", 22)
        .property("time", 22 as i64)
        .property("score", 3.2)
        .property("uuid", uuid.clone())
        .property("datetime", chrono::Utc.timestamp(1551825863, 0))
        .property("date", 1551825863 as i64)
        .value_map(true)
        .by(TraversalBuilder::new(Bytecode::new()).unfold())
        .next()
        .expect("Should get a response");

    #[derive(Debug, PartialEq, FromGMap)]
    struct Person {
        id: String,
        label: String,
        name: String,
        age: i32,
        time: i64,
        datetime: DateTime<Utc>,
        uuid: uuid::Uuid,
        optional: Option<String>,
    }
    let person = Person::try_from(mark.unwrap()).expect("Should get person");

    assert_eq!(
        Person {
            id: mark_id,
            label: String::from("test_mapping_custom_vertex_id"),
            name: String::from("Mark"),
            age: 22,
            time: 22,
            datetime: chrono::Utc.timestamp(1551825863, 0),
            uuid: uuid,
            optional: None
        },
        person
    );
}

#[apply(common::serializers)]
#[serial(jg_throttle)]
fn test_jg_add_e(protocol: IoProtocol) {
    //It seems JanusGraph differs from the standard Tinkerpop Gremlin Server in how it returns edges
    //JG returns edge properties in the same call if the edge is the terminal type
    let client = expect_janusgraph_client(protocol);
    let g = traversal().with_remote(client.clone());
    let v1 = g
        .add_v("test_jg_add_e")
        .property(T::Id, create_novel_vertex_id())
        .property("name", "foo")
        .next()
        .expect("Should to get response")
        .expect("Should have gotten a vertex");

    let v2 = g
        .add_v("test_jg_add_e")
        .property(T::Id, create_novel_vertex_id())
        .next()
        .expect("Should get response")
        .expect("Should have gotten a vertex");

    let created_edge = g
        .v(v1.id().clone())
        .out_e("knows")
        .where_(__.in_v().id().is(v2.id().clone()))
        .fold()
        .coalesce::<Edge, _>([
            __.unfold(),
            __.add_e("knows")
                .from(__.v(v1.id().clone()))
                .to(__.v(v2.id().clone())),
        ])
        .property("someKey", "someValue")
        .next()
        .expect("Should get response")
        .expect("Should get edge");
    let actual_property = created_edge
        .property("someKey")
        .expect("Should have had property");
    let actual_property = actual_property
        .get::<String>()
        .expect("Should have had property with expected type");
    assert_eq!(actual_property, "someValue");
}

#[apply(common::serializers)]
#[serial(jg_throttle)]
fn test_merge_v_custom_id(protocol: IoProtocol) {
    if protocol == IoProtocol::GraphSONV2 {
        //GraphSONV2 doesn't support the non-string key of the merge step,
        //so skip it in testing
        return;
    }

    let client = expect_janusgraph_client(protocol);
    let g = traversal().with_remote(client);
    let expected_id = create_novel_vertex_id();
    let expected_label = "test_merge_v_custom_id";
    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(T::Id.into(), expected_id.clone().into());
    start_step_map.insert(T::Label.into(), expected_label.into());
    let actual_vertex = g
        .merge_v(start_step_map)
        .next()
        .expect("Should get a response")
        .expect("Should return a vertex");
    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(&expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }

    assert_eq!(expected_label, actual_vertex.label());

    //Now try it as a mid-traversal step (inject is the start step)
    let expected_id = create_novel_vertex_id();
    let expected_property = "propValue";

    let mut map_to_inject: HashMap<GKey, GValue> = HashMap::new();
    let mut lookup_map: HashMap<GKey, GValue> = HashMap::new();
    lookup_map.insert(T::Id.into(), expected_id.clone().into());
    lookup_map.insert(T::Label.into(), "myvertexlabel".into());
    let mut property_map: HashMap<GKey, GValue> = HashMap::new();
    property_map.insert("propertyKey".into(), expected_property.into());
    map_to_inject.insert("lookup".into(), lookup_map.into());
    map_to_inject.insert("properties".into(), property_map.into());

    let actual_vertex = g
        .inject(vec![map_to_inject.into()])
        .unfold()
        .as_("payload")
        .merge_v(__.select("lookup"))
        .property(
            "propertyKey",
            __.select("payload")
                .select("properties")
                .select("propertyKey"),
        )
        .next()
        .expect("Should get response")
        .expect("Should have returned a vertex");

    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(&expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }

    let actual_property: &String = actual_vertex
        .property("propertyKey")
        .expect("Should have property")
        .get()
        .unwrap();
    assert_eq!(expected_property, actual_property);
}

#[apply(common::serializers)]
#[serial(jg_throttle)]
fn test_add_v_custom_id(protocol: IoProtocol) {
    let client = expect_janusgraph_client(protocol);
    let expected_id = create_novel_vertex_id();
    let g = traversal().with_remote(client);
    let actual_vertex = g
        .add_v("test_add_v_custom_id")
        .property(T::Id, &expected_id)
        .next()
        .expect("Should get a response")
        .expect("Should return a vertex");
    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(&expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }
}

fn create_novel_vertex_id() -> String {
    //JanusGraph by default treats "-" as a reserved character
    //we can override it, but to stay closer to the default nature of JG just map the character to "_"
    //in our generated vertex ids
    Uuid::new_v4().to_string().replace("-", "_")
}
