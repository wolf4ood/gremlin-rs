use std::collections::HashMap;

use common::io::{drop_vertices, expect_janusgraph_client};
use gremlin_client::{
    process::traversal::{traversal, __},
    structure::T,
    GKey, GValue,
};

mod common;

//Custom vertex ids are a feature offered by JanusGraph
//https://docs.janusgraph.org/advanced-topics/custom-vertex-id/

#[test]
fn test_merge_v_custom_id() {
    let client = expect_janusgraph_client();
    let expected_label = "test_merge_v_custom_id";
    drop_vertices(&client, expected_label).expect("Failed to drop vertices");
    let g = traversal().with_remote(client);
    let expected_id = "test_merge_v_custom_id";
    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(T::Id.into(), expected_id.into());
    start_step_map.insert(T::Label.into(), expected_label.into());
    let actual_vertex = g
        .merge_v(start_step_map)
        .next()
        .expect("Should get a response")
        .expect("Should return a vertex");
    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }

    assert_eq!(expected_label, actual_vertex.label());

    //Now try it as a mid-traversal step (inject is the start step)
    let expected_id = "foo";
    let expected_property = "propValue";

    let mut map_to_inject: HashMap<GKey, GValue> = HashMap::new();
    let mut lookup_map: HashMap<GKey, GValue> = HashMap::new();
    lookup_map.insert(T::Id.into(), expected_id.into());
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
        gremlin_client::GID::String(actual) => assert_eq!(expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }

    let actual_property: &String = actual_vertex
        .property("propertyKey")
        .expect("Should have property")
        .get()
        .unwrap();
    assert_eq!(expected_property, actual_property);
}

#[test]
fn test_add_v_custom_id() {
    let client = expect_janusgraph_client();
    let expected_id = "test_add_v_custom_id";
    let test_vertex_label = "test_add_v_custom_id";
    drop_vertices(&client, test_vertex_label).expect("Failed to drop vertices");
    let g = traversal().with_remote(client);
    let actual_vertex = g
        .add_v(test_vertex_label)
        .property(T::Id, expected_id)
        .next()
        .expect("Should get a response")
        .expect("Should return a vertex");
    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }
}
