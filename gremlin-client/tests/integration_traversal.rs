use std::collections::HashMap;
use std::convert::TryInto;

use common::assert_map_property;
use gremlin_client::process::traversal::{traversal, Order, __};
use gremlin_client::structure::{
    Cardinality, Column, List, Map, Pop, TextP, Vertex, VertexProperty, P, T,
};

use gremlin_client::{utils, GKey, GValue};

mod common;

use common::io::{
    create_edge, create_vertex, create_vertex_with_label, drop_edges, drop_vertices, graph,
};

#[cfg(feature = "merge_tests")]
mod merge_tests {
    use super::*;
    use gremlin_client::{
        process::traversal::{GraphTraversalSource, SyncTerminator},
        structure::{Direction, Merge},
        Edge, GValue, ToGValue,
    };
    use std::collections::HashMap;

    #[test]
    fn test_merge_v_no_options() {
        let client = graph();
        let test_vertex_label = "test_merge_v_no_options";
        drop_vertices(&client, test_vertex_label)
            .expect("Failed to drop vertices in case of rerun");
        let g = traversal().with_remote(client);

        let mut injection_map: HashMap<GKey, GValue> = HashMap::new();
        let mut lookup_map: HashMap<GKey, GValue> = HashMap::new();
        lookup_map.insert(T::Label.into(), test_vertex_label.into());
        let mut property_map: HashMap<GKey, GValue> = HashMap::new();
        property_map.insert("propertyKey".into(), "propertyValue".into());
        injection_map.insert("lookup".into(), lookup_map.into());
        injection_map.insert("properties".into(), property_map.into());

        let vertex_properties = g
            .inject(vec![injection_map.into()])
            .unfold()
            .as_("payload")
            .merge_v(__.select("lookup"))
            .property(
                "propertyKey",
                __.select("payload")
                    .select("properties")
                    .select("propertyKey"),
            )
            .element_map(())
            .next()
            .expect("Should get response")
            .expect("Should have returned a vertex");

        assert_map_property(&vertex_properties, "propertyKey", "propertyValue");
    }

    #[test]
    fn test_merge_v_options() {
        let client = graph();
        let expected_label = "test_merge_v_options";
        drop_vertices(&client, expected_label).expect("Failed to drop vertices");
        let g = traversal().with_remote(client);
        let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
        start_step_map.insert(T::Label.into(), expected_label.into());
        start_step_map.insert("identifing_prop".into(), "some_Value".into());

        let prop_key = "some_prop";
        let mut on_create_map: HashMap<GKey, GValue> = HashMap::new();
        let expected_on_create_prop_value = "on_create_value";
        on_create_map.insert(prop_key.into(), expected_on_create_prop_value.into());

        let mut on_match_map: HashMap<GKey, GValue> = HashMap::new();
        let expected_on_match_prop_value = "on_match_value";
        on_match_map.insert(prop_key.into(), expected_on_match_prop_value.into());

        let on_create_vertex_map = g
            .merge_v(start_step_map.clone())
            .option((Merge::OnCreate, on_create_map.clone()))
            .option((Merge::OnMatch, on_match_map.clone()))
            .element_map(())
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        assert_map_property(&on_create_vertex_map, "label", expected_label);

        assert_map_property(
            &on_create_vertex_map,
            prop_key,
            expected_on_create_prop_value,
        );

        //Now run the traversal again, and confirm the OnMatch applied this time
        let on_match_vertex_map = g
            .merge_v(start_step_map)
            .option((Merge::OnCreate, on_create_map.clone()))
            .option((Merge::OnMatch, on_match_map.clone()))
            .element_map(())
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        assert_map_property(&on_match_vertex_map, "label", expected_label);

        assert_map_property(&on_match_vertex_map, prop_key, expected_on_match_prop_value);
    }

    #[test]
    fn test_merge_v_start_step() {
        let client = graph();
        let expected_label = "test_merge_v_start_step";
        drop_vertices(&client, &expected_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);
        let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
        start_step_map.insert(T::Label.into(), expected_label.into());
        let actual_vertex = g
            .merge_v(start_step_map)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        assert_eq!(expected_label, actual_vertex.label())
    }

    #[test]
    fn test_merge_v_anonymous_traversal() {
        let client = graph();
        let expected_label = "test_merge_v_anonymous_traversal";
        drop_vertices(&client, &expected_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);
        let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
        start_step_map.insert(T::Label.into(), expected_label.into());
        let actual_vertex = g
            .inject(1)
            .unfold()
            .coalesce::<Vertex, _>([__.merge_v(start_step_map)])
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");
        assert_eq!(expected_label, actual_vertex.label())
    }

    #[test]
    fn test_merge_e_start_step() {
        let client = graph();
        let expected_vertex_label = "test_merge_e_start_step_vertex";
        let expected_edge_label = "test_merge_e_start_step_edge";
        let expected_edge_property_key = "test_merge_e_start_step_edge_prop";
        let expected_edge_property_value = "test_merge_e_start_step_edge_value";
        drop_vertices(&client, &expected_vertex_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);

        let vertex_a = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let vertex_b = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
        start_step_map.insert(Direction::In.into(), vertex_a.id().into());
        start_step_map.insert(Direction::Out.into(), vertex_b.id().into());
        start_step_map.insert(T::Label.into(), expected_edge_label.into());
        start_step_map.insert(
            expected_edge_property_key.into(),
            expected_edge_property_value.into(),
        );
        let merged_edge_properties = g
            .merge_e(start_step_map)
            .element_map(())
            .next()
            .expect("Should get a response")
            .expect("Should return a edge properties");

        assert_map_property(&merged_edge_properties, "label", expected_edge_label);

        assert_map_property(
            &merged_edge_properties,
            expected_edge_property_key,
            expected_edge_property_value,
        );

        let incoming_vertex: &Map = merged_edge_properties
            .get(Direction::In)
            .expect("Should have returned incoming vertex info")
            .get()
            .unwrap();

        let incoming_vertex_id = incoming_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(incoming_vertex_id, &vertex_a.id().to_gvalue());

        let outgoing_vertex: &Map = merged_edge_properties
            .get(Direction::Out)
            .expect("Should have returned outgoing vertex info")
            .get()
            .unwrap();
        let outgoing_vertex_id = outgoing_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(outgoing_vertex_id, &vertex_b.id().to_gvalue());
    }

    #[test]
    fn test_merge_e_no_options() {
        let client = graph();
        let expected_vertex_label = "test_merge_e_no_options_vertex";
        let expected_edge_label = "test_merge_e_no_options_edge";
        let expected_edge_property_key = "test_merge_e_no_options_edge_prop";
        let expected_edge_property_value = "test_merge_e_no_options_edge_value";
        drop_vertices(&client, &expected_vertex_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);

        let vertex_a = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let vertex_b = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let mut assignment_map: HashMap<GKey, GValue> = HashMap::new();
        assignment_map.insert(Direction::In.into(), vertex_a.id().into());
        assignment_map.insert(Direction::Out.into(), vertex_b.id().into());
        assignment_map.insert(T::Label.into(), expected_edge_label.into());
        assignment_map.insert(
            expected_edge_property_key.into(),
            expected_edge_property_value.into(),
        );

        let merged_edge_properties = g
            .inject(vec![assignment_map.into()])
            .unfold()
            .as_("payload")
            .merge_e(__.select("payload"))
            .element_map(())
            .next()
            .expect("Should get a response")
            .expect("Should return edge properties");

        assert_map_property(&merged_edge_properties, "label", expected_edge_label);
        assert_map_property(
            &merged_edge_properties,
            expected_edge_property_key,
            expected_edge_property_value,
        );

        let incoming_vertex: &Map = merged_edge_properties
            .get(Direction::In)
            .expect("Should have returned incoming vertex info")
            .get()
            .unwrap();
        let incoming_vertex_id = incoming_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(incoming_vertex_id, &vertex_a.id().to_gvalue());

        let outgoing_vertex: &Map = merged_edge_properties
            .get(Direction::Out)
            .expect("Should have returned outgoing vertex info")
            .get()
            .unwrap();
        let outgoing_vertex_id = outgoing_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(outgoing_vertex_id, &vertex_b.id().to_gvalue());
    }

    #[test]
    fn test_merge_e_options() {
        let client = graph();
        let expected_vertex_label = "test_merge_e_options_vertex";
        let expected_edge_label = "test_merge_e_options_edge";
        let expected_edge_property_key = "test_merge_e_options_edge_prop";
        drop_vertices(&client, &expected_vertex_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);

        let vertex_a = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let vertex_b = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let mut assignment_map: HashMap<GKey, GValue> = HashMap::new();
        assignment_map.insert(Direction::In.into(), vertex_a.id().into());
        assignment_map.insert(Direction::Out.into(), vertex_b.id().into());
        assignment_map.insert(T::Label.into(), expected_edge_label.into());

        let mut on_create_map: HashMap<GKey, GValue> = HashMap::new();
        on_create_map.insert(expected_edge_property_key.into(), "on_create".into());

        let mut on_match_map: HashMap<GKey, GValue> = HashMap::new();
        on_match_map.insert(expected_edge_property_key.into(), "on_match".into());

        let mut injection_map: HashMap<GKey, GValue> = HashMap::new();
        injection_map.insert("merge_params".into(), assignment_map.into());
        injection_map.insert("create_params".into(), on_create_map.into());
        injection_map.insert("match_params".into(), on_match_map.into());

        let do_merge_edge = |g: GraphTraversalSource<SyncTerminator>| -> Map {
            g.inject(vec![injection_map.clone().into()])
                .unfold()
                .as_("payload")
                .merge_e(__.select("payload").select("merge_params"))
                .option((
                    Merge::OnCreate,
                    __.select("payload").select("create_params"),
                ))
                .option((Merge::OnMatch, __.select("payload").select("match_params")))
                .element_map(())
                .next()
                .expect("Should get a response")
                .expect("Should return a edge properties")
        };

        let on_create_edge_properties = do_merge_edge(g.clone());

        //Initially the edge should be the on create value
        assert_map_property(
            &on_create_edge_properties,
            expected_edge_property_key,
            "on_create",
        );

        let on_match_edge_properties = do_merge_edge(g);
        assert_map_property(
            &on_match_edge_properties,
            expected_edge_property_key,
            "on_match",
        );
    }

    #[test]
    fn test_merge_e_anonymous_traversal() {
        let client = graph();
        let expected_vertex_label = "test_merge_e_options_vertex";
        let expected_edge_label = "test_merge_e_options_edge";
        drop_vertices(&client, &expected_vertex_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);

        let vertex_a = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let vertex_b = g
            .add_v(expected_vertex_label)
            .next()
            .expect("Should get a response")
            .expect("Should return a vertex");

        let mut assignment_map: HashMap<GKey, GValue> = HashMap::new();
        assignment_map.insert(Direction::In.into(), vertex_a.id().into());
        assignment_map.insert(Direction::Out.into(), vertex_b.id().into());
        assignment_map.insert(T::Label.into(), expected_edge_label.into());

        let anonymous_merge_e_properties = g
            .inject(1)
            .unfold()
            .coalesce::<Edge, _>([__.merge_e(assignment_map)])
            .element_map(())
            .next()
            .expect("Should get a response")
            .expect("Should return a edge properties");

        let incoming_vertex: &Map = anonymous_merge_e_properties
            .get(Direction::In)
            .expect("Should have returned incoming vertex info")
            .get()
            .unwrap();
        let incoming_vertex_id = incoming_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(incoming_vertex_id, &vertex_a.id().to_gvalue());

        let outgoing_vertex: &Map = anonymous_merge_e_properties
            .get(Direction::Out)
            .expect("Should have returned outgoing vertex info")
            .get()
            .unwrap();
        let outgoing_vertex_id = outgoing_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(outgoing_vertex_id, &vertex_b.id().to_gvalue());
    }

    #[test]
    fn test_merge_v_into_merge_e() {
        //Based on the reference doc's combo example
        let client = graph();
        let expected_vertex_label = "test_merge_v_into_merge_e_vertex";
        let expected_edge_label = "test_merge_v_into_merge_e_edge";
        drop_vertices(&client, &expected_vertex_label).expect("Failed to drop vertiecs");
        let g = traversal().with_remote(client);

        let expected_toby_id = 100_001i64;
        let expected_brandy_id = 200_001i64;

        let mut vertex_a_map: HashMap<GKey, GValue> = HashMap::new();
        vertex_a_map.insert(T::Label.into(), expected_vertex_label.into());
        vertex_a_map.insert(T::Id.into(), expected_toby_id.into());
        vertex_a_map.insert("name".into(), "Toby".into());

        let mut vertex_b_map: HashMap<GKey, GValue> = HashMap::new();
        vertex_b_map.insert(T::Label.into(), expected_vertex_label.into());
        vertex_b_map.insert(T::Id.into(), expected_brandy_id.into());
        vertex_b_map.insert("name".into(), "Brandy".into());

        let mut edge_map: HashMap<GKey, GValue> = HashMap::new();
        edge_map.insert(T::Label.into(), expected_edge_label.into());
        edge_map.insert("some_key".into(), "some_value".into());
        edge_map.insert(Direction::From.into(), Merge::OutV.into());
        edge_map.insert(Direction::To.into(), Merge::InV.into());

        let combo_merge_edge_properties = g
            .merge_v(vertex_a_map)
            .as_("Toby")
            .merge_v(vertex_b_map)
            .as_("Brandy")
            .merge_e(edge_map)
            .option((Merge::OutV, __.select("Toby")))
            .option((Merge::InV, __.select("Brandy")))
            .element_map(())
            .next()
            .expect("Should get a response")
            .expect("Should return a edge properties");

        let brandy_vertex: &Map = combo_merge_edge_properties
            .get(Direction::In)
            .expect("Should have returned incoming vertex info")
            .get()
            .unwrap();
        let brandy_vertex_id = brandy_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(*brandy_vertex_id, GValue::Int64(expected_brandy_id));

        let toby_vertex: &Map = combo_merge_edge_properties
            .get(Direction::Out)
            .expect("Should have returned outgoing vertex info")
            .get()
            .unwrap();
        let toby_vertex_id = toby_vertex
            .get("id")
            .expect("Should have returned vertex id");
        assert_eq!(*toby_vertex_id, GValue::Int64(expected_toby_id));

        assert_map_property(&combo_merge_edge_properties, "label", expected_edge_label);
    }
}

#[test]
fn test_simple_vertex_traversal() {
    let g = traversal().with_remote(graph());

    let results = g.v(()).to_list().unwrap();

    assert!(results.len() > 0);
}

#[test]
fn test_inject() {
    let g = traversal().with_remote(graph());
    let expected_value = "foo";
    let response: String = g
        .inject(vec![expected_value.into()])
        .next()
        .expect("Should get response")
        .expect("Should have gotten a Some")
        .try_into()
        .expect("Should be parsable into a String");
    assert_eq!(expected_value, response);
}

#[test]
fn test_simple_vertex_traversal_with_id() {
    let client = graph();

    let vertex = create_vertex(&client, "Traversal");

    let g = traversal().with_remote(client);

    let results = g.v(vertex.id()).to_list().unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertex.id(), results[0].id());
}

#[test]
fn test_simple_vertex_traversal_with_multiple_id() {
    let client = graph();
    drop_vertices(&client, "test_simple_vertex_traversal").unwrap();

    let vertex = create_vertex_with_label(&client, "test_simple_vertex_traversal", "Traversal");
    let vertex2 = create_vertex_with_label(&client, "test_simple_vertex_traversal", "Traversal");

    let g = traversal().with_remote(client);

    let results = g.v(vec![vertex.id(), vertex2.id()]).to_list().unwrap();

    assert_eq!(2, results.len());

    assert_eq!(vertex.id(), results[0].id());
    assert_eq!(vertex2.id(), results[1].id());
}

#[test]
fn test_simple_vertex_traversal_with_label() {
    let client = graph();

    drop_vertices(&client, "test_simple_vertex_traversal_with_label").unwrap();

    let vertex = create_vertex_with_label(
        &client,
        "test_simple_vertex_traversal_with_label",
        "Traversal",
    );

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_simple_vertex_traversal_with_label")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertex.id(), results[0].id());
}

#[test]
fn test_simple_vertex_traversal_with_label_and_has() {
    let client = graph();

    drop_vertices(&client, "test_simple_vertex_traversal_with_label_and_has").unwrap();

    let vertex = create_vertex_with_label(
        &client,
        "test_simple_vertex_traversal_with_label_and_has",
        "Traversal",
    );

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_simple_vertex_traversal_with_label_and_has")
        .has(("name", "Traversal"))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertex.id(), results[0].id());

    // with 3 params

    let results = g
        .v(())
        .has((
            "test_simple_vertex_traversal_with_label_and_has",
            "name",
            "Traversal",
        ))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertex.id(), results[0].id());

    // with 1 param

    let results = g
        .v(())
        .has_label("test_simple_vertex_traversal_with_label_and_has")
        .has("name")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertex.id(), results[0].id());

    // hasNot

    let results = g
        .v(())
        .has_label("test_simple_vertex_traversal_with_label_and_has")
        .has_not("surname")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertex.id(), results[0].id());
}

#[test]
fn test_simple_edge_traversal() {
    let g = traversal().with_remote(graph());

    let results = g.e(()).to_list().unwrap();

    assert!(results.len() > 0);
}

#[test]
fn test_simple_edge_traversal_id() {
    let client = graph();

    let v = create_vertex(&client, "Traversal");
    let v1 = create_vertex(&client, "Traversal");

    let e = create_edge(&client, &v, &v1, "TraversalEdge");

    let g = traversal().with_remote(client);

    let results = g.e(e.id()).to_list().unwrap();

    assert_eq!(1, results.len());

    assert_eq!(e.id(), results[0].id());
}

#[test]
fn test_simple_edge_traversal_with_label() {
    let client = graph();

    drop_edges(&client, "test_simple_edge_traversal_with_label").unwrap();

    let v = create_vertex(&client, "Traversal");
    let v1 = create_vertex(&client, "Traversal");

    let e = create_edge(&client, &v, &v1, "test_simple_edge_traversal_with_label");

    let g = traversal().with_remote(client);

    let results = g
        .e(())
        .has_label("test_simple_edge_traversal_with_label")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(e.id(), results[0].id());
}

#[test]
fn test_traversal() {
    let client = graph();

    drop_edges(&client, "test_vertex_out_traversal").unwrap();

    let v = create_vertex(&client, "Traversal");
    let v1 = create_vertex(&client, "Traversal");
    let v2 = create_vertex(&client, "Traversal");

    let _e = create_edge(&client, &v, &v1, "test_vertex_out_traversal");
    let _e2 = create_edge(&client, &v2, &v, "test_vertex_out_traversal");

    let g = traversal().with_remote(client);

    // OUT
    let results = g
        .v(v.id())
        .out("test_vertex_out_traversal")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(v1.id(), results[0].id());

    let results = g.v(v.id()).out("fake").to_list().unwrap();

    assert_eq!(0, results.len());

    // OUT_E

    let results = g
        .v(v.id())
        .out_e("test_vertex_out_traversal")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!("test_vertex_out_traversal", results[0].label());

    assert_eq!(v.id(), results[0].out_v().id());
    assert_eq!(v1.id(), results[0].in_v().id());

    // OUT_E -> IN_V
    let results = g
        .v(v.id())
        .out_e("test_vertex_out_traversal")
        .in_v()
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(v1.id(), results[0].id());

    let results = g.v(v.id()).out("fake").to_list().unwrap();

    assert_eq!(0, results.len());

    // IN
    let results = g
        .v(v1.id())
        .in_("test_vertex_out_traversal")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(v.id(), results[0].id());

    let results = g.v(v1.id()).in_("fake").to_list().unwrap();

    assert_eq!(0, results.len());

    // IN_E

    let results = g
        .v(v1.id())
        .in_e("test_vertex_out_traversal")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!("test_vertex_out_traversal", results[0].label());

    assert_eq!(v.id(), results[0].out_v().id());
    assert_eq!(v1.id(), results[0].in_v().id());

    // IN_E -> OUT_V
    let results = g
        .v(v1.id())
        .in_e("test_vertex_out_traversal")
        .out_v()
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(v.id(), results[0].id());

    let results = g.v(v1.id()).in_("fake").to_list().unwrap();

    assert_eq!(0, results.len());

    // BOTH

    let results = g
        .v(v.id())
        .both("test_vertex_out_traversal")
        .to_list()
        .unwrap();

    assert_eq!(2, results.len());

    assert_eq!(v1.id(), results[0].id());
    assert_eq!(v2.id(), results[1].id());

    let results = g.v(v1.id()).in_("fake").to_list().unwrap();

    assert_eq!(0, results.len());

    // BOTH_E -> OTHER_V

    let results = g
        .v(v.id())
        .both_e("test_vertex_out_traversal")
        .other_v()
        .to_list()
        .unwrap();

    assert_eq!(2, results.len());

    assert_eq!(v1.id(), results[0].id());
    assert_eq!(v2.id(), results[1].id());

    let results = g.v(v1.id()).in_("fake").to_list().unwrap();

    assert_eq!(0, results.len());
}

#[test]
fn test_add_v() {
    let g = traversal().with_remote(graph());

    let results = g.add_v("person").to_list().unwrap();

    assert!(results.len() > 0);

    assert_eq!("person", results[0].label());

    let results = g.add_v("person").add_v(()).to_list().unwrap();

    assert!(results.len() > 0);

    //default label
    assert_eq!("vertex", results[0].label());
}

#[test]
fn test_add_v_with_properties() {
    let client = graph();
    let g = traversal().with_remote(client.clone());

    let results = g
        .add_v("person")
        .property("name", "marko")
        .property("age", 29)
        .to_list()
        .unwrap();

    assert!(results.len() > 0);

    assert_eq!("person", results[0].label());

    let results = client
        .execute("g.V(_id).propertyMap()", &[("_id", results[0].id())])
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties = &results[0];

    assert_eq!(
        &29,
        properties["age"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<i32>()
            .unwrap()
    );

    assert_eq!(
        &"marko",
        properties["name"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );
}

#[test]
fn test_add_v_with_property_many() {
    let client = graph();

    drop_vertices(&client, "test_add_v_with_property_many").unwrap();

    let g = traversal().with_remote(client.clone());

    let results = g
        .add_v("test_add_v_with_property_many")
        .property_many(vec![
            (String::from("name"), "marko"),
            (String::from("age"), "29"),
        ])
        .to_list()
        .unwrap();

    assert!(results.len() > 0);

    assert_eq!("test_add_v_with_property_many", results[0].label());

    let results = client
        .execute("g.V(_id).propertyMap()", &[("_id", results[0].id())])
        .expect("it should execute addV")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .expect("It should be ok");

    let properties = &results[0];

    assert_eq!(
        &"29".to_string(),
        properties["age"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );

    assert_eq!(
        &"marko",
        properties["name"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );
}

#[test]
fn test_has_many() {
    let client = graph();

    drop_vertices(&client, "test_has_many").unwrap();

    let g = traversal().with_remote(client.clone());

    let results = g
        .add_v("test_has_many")
        .property_many(vec![
            (String::from("name"), "josh"),
            (String::from("age"), "21"),
        ])
        .to_list()
        .unwrap();

    assert!(results.len() > 0);

    assert_eq!("test_has_many", results[0].label());

    let results = g
        .v(())
        .has_many(vec![
            (String::from("name"), "josh"),
            (String::from("age"), "21"),
        ])
        .to_list()
        .unwrap();

    assert_eq!(results.len(), 1);
}

#[test]
fn test_add_e() {
    let client = graph();
    let g = traversal().with_remote(client.clone());

    let v = g
        .add_v("person")
        .property("name", "marko")
        .property("age", 29)
        .to_list()
        .unwrap();

    let v1 = g
        .add_v("person")
        .property("name", "jon")
        .property("age", 29)
        .to_list()
        .unwrap();

    let edges = g.add_e("knows").from(&v[0]).to(&v1[0]).to_list().unwrap();

    assert!(edges.len() > 0);

    assert_eq!("knows", edges[0].label());

    let edges = g
        .v(v[0].id())
        .as_("a")
        .out("knows")
        .add_e("livesNear")
        .from("a")
        .property("year", 2009)
        .to_list()
        .unwrap();

    assert!(edges.len() > 0);

    assert_eq!("livesNear", edges[0].label());

    let edges = g
        .v(())
        .as_("a")
        .out("created")
        .add_e("createdBy")
        .to("a")
        .property("acl", "public")
        .to_list()
        .unwrap();

    assert_eq!("createdBy", edges[0].label());

    let edges = g
        .add_e("knows")
        .from(__.v(()).has(("name", "marko")))
        .to(__.v(()).has(("name", "jon")))
        .to_list()
        .unwrap();

    assert!(edges.len() > 0);

    assert_eq!("knows", edges[0].label());
}

#[test]
fn test_label_step() {
    let client = graph();

    let vertex = create_vertex(&client, "Traversal");

    let g = traversal().with_remote(client);

    let results = g.v(vertex.id()).label().to_list().unwrap();

    assert_eq!(1, results.len());

    assert_eq!("person", results[0]);
}

#[test]
fn test_properties_step() {
    let client = graph();

    let vertex = create_vertex(&client, "Traversal");

    let g = traversal().with_remote(client);

    let results = g.v(vertex.id()).properties(()).to_list().unwrap();

    assert_eq!(1, results.len());

    assert_eq!("Traversal", results[0].get::<String>().unwrap());

    let results = g.v(vertex.id()).properties("name").to_list().unwrap();

    assert_eq!(1, results.len());

    assert_eq!("Traversal", results[0].get::<String>().unwrap());

    let results = g.v(vertex.id()).properties("fake").to_list().unwrap();

    assert_eq!(0, results.len());
}

#[test]
fn test_property_map() {
    let client = graph();

    let vertex = create_vertex(&client, "Traversal");

    let g = traversal().with_remote(client);

    let results = g.v(vertex.id()).property_map(()).to_list().unwrap();

    assert_eq!(1, results.len());

    let properties = &results[0];

    assert_eq!(
        "Traversal",
        properties["name"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );

    let results = g.v(vertex.id()).property_map("name").to_list().unwrap();

    assert_eq!(1, results.len());

    let properties = &results[0];

    assert_eq!(
        "Traversal",
        properties["name"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );

    let results = g.v(vertex.id()).property_map("fake").to_list().unwrap();

    assert_eq!(1, results.len());

    let properties = &results[0];

    assert_eq!(0, properties.len());
}

#[test]
fn test_values() {
    let client = graph();

    let vertex = create_vertex(&client, "Traversal");

    let g = traversal().with_remote(client);

    let results = g.v(vertex.id()).values(()).to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!("Traversal", value.get::<String>().unwrap());

    let results = g.v(vertex.id()).values("name").to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!("Traversal", value.get::<String>().unwrap());

    let results = g.v(vertex.id()).values("fake").to_list().unwrap();

    assert_eq!(0, results.len());
}

#[test]
fn test_value_map() {
    let client = graph();

    let g = traversal().with_remote(client);

    let vertices = g
        .add_v("test_value_map")
        .property("name", "test")
        .to_list()
        .unwrap();

    let vertex = &vertices[0];

    let results = g.v(vertex.id()).value_map(()).to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(
        "test",
        value["name"].get::<List>().unwrap()[0]
            .get::<String>()
            .unwrap()
    );

    let results = g.v(vertex.id()).value_map("name").to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(
        "test",
        value["name"].get::<List>().unwrap()[0]
            .get::<String>()
            .unwrap()
    );

    let results = g.v(vertex.id()).value_map("fake").to_list().unwrap();

    assert_eq!(0, results[0].len());

    let results = g.v(vertex.id()).value_map(true).to_list().unwrap();

    assert_eq!(true, results[0].get("id").is_some());
    assert_eq!(true, results[0].get("label").is_some());
    assert_eq!(true, results[0].get("name").is_some());
}

#[test]
fn test_unwrap_map() {
    let client = graph();

    let g = traversal().with_remote(client);

    let vertices = g
        .add_v("test_value_map")
        .property("name", "test")
        .to_list()
        .unwrap();

    let vertex = &vertices[0];

    let results = g.v(vertex.id()).value_map(true).next().unwrap().unwrap();
    let v_id = vertex.id().get::<i64>().unwrap();

    let id = utils::unwrap_map::<i64>(&results, "id", 0);
    let property = utils::unwrap_map::<String>(&results, "name", 0);
    let label = utils::unwrap_map::<String>(&results, "label", 0);

    assert_eq!(id.is_ok(), true);
    assert_eq!(property.is_ok(), true);
    assert_eq!(label.is_ok(), true);

    assert_eq!(id.unwrap(), v_id);
    assert_eq!(property.unwrap(), "test");
    assert_eq!(label.unwrap(), "test_value_map");
}

#[test]
fn test_element_map() {
    let client = graph();

    let g = traversal().with_remote(client);

    let vertices = g
        .add_v("test_element_map")
        .property("name", "test")
        .to_list()
        .unwrap();

    let vertex = &vertices[0];

    let results = g.v(vertex.id()).element_map(()).to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!("test", value["name"].get::<String>().unwrap());

    let results = g.v(vertex.id()).element_map("name").to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!("test", value["name"].get::<String>().unwrap());

    let results = g.v(vertex.id()).element_map("fake").to_list().unwrap();

    assert_eq!(2, results[0].len());
    assert_eq!(true, results[0].get("id").is_some());
    assert_eq!(true, results[0].get("label").is_some());

    let results = g.v(vertex.id()).element_map(()).to_list().unwrap();

    assert_eq!(true, results[0].get("id").is_some());
    assert_eq!(true, results[0].get("label").is_some());
    assert_eq!(true, results[0].get("name").is_some());
}

#[test]
fn test_count() {
    let client = graph();

    let vertex = create_vertex_with_label(&client, "test_count", "Count");

    let g = traversal().with_remote(client);

    let results = g.v(vertex.id()).count().to_list().unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(&1, value);
}

#[test]
fn test_group_count_step() {
    let client = graph();

    drop_vertices(&client, "test_group_count").unwrap();

    let vertex = create_vertex_with_label(&client, "test_group_count", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_group_count")
        .group_count()
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(&1, value[&vertex].get::<i64>().unwrap());

    let results = g
        .v(())
        .has_label("test_group_count")
        .group_count()
        .by("name")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(&1, value["Count"].get::<i64>().unwrap());

    let results = g
        .v(())
        .has_label("test_group_count")
        .group_count()
        .by(T::Label)
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(&1, value["test_group_count"].get::<i64>().unwrap());
}

#[test]
fn test_group_by_step() {
    let client = graph();

    drop_vertices(&client, "test_group_by_step").unwrap();

    create_vertex_with_label(&client, "test_group_by_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_group_by_step")
        .group()
        .by("name")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(1, value["Count"].get::<List>().unwrap().len());

    let results = g
        .v(())
        .has_label("test_group_by_step")
        .group()
        .by(T::Label)
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(1, value["test_group_by_step"].get::<List>().unwrap().len());

    //

    let results = g
        .v(())
        .has_label("test_group_by_step")
        .group()
        .by(T::Label)
        .by(__.count())
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(&1, value["test_group_by_step"].get::<i64>().unwrap());
}

#[test]
fn test_select_step() {
    let client = graph();

    drop_vertices(&client, "test_select_step").unwrap();

    create_vertex_with_label(&client, "test_select_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_select_step")
        .group_count()
        .by("name")
        .select("Count")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(&1, value.get::<i64>().unwrap());
}

#[test]
fn test_fold_step() {
    let client = graph();

    drop_vertices(&client, "test_fold_step").unwrap();

    create_vertex_with_label(&client, "test_fold_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_fold_step")
        .values("name")
        .fold()
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!("Count", value[0].get::<String>().unwrap());
}

#[test]
fn test_unfold_step() {
    let client = graph();

    drop_vertices(&client, "test_unfold_step").unwrap();

    let vertex = create_vertex_with_label(&client, "test_unfold_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(vertex.id())
        .property_map(())
        .unfold()
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(
        "Count",
        value["name"].get::<List>().unwrap()[0]
            .get::<VertexProperty>()
            .unwrap()
            .get::<String>()
            .unwrap()
    );
}

#[test]
fn test_path_step() {
    let client = graph();

    drop_vertices(&client, "test_path_step").unwrap();

    let v = create_vertex_with_label(&client, "test_path_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_path_step")
        .path()
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let value = &results[0];

    assert_eq!(v.id(), value.objects()[0].get::<Vertex>().unwrap().id());
}

#[test]
fn test_limit_step() {
    let client = graph();

    drop_vertices(&client, "test_limit_step").unwrap();

    create_vertex_with_label(&client, "test_limit_step", "Count");
    create_vertex_with_label(&client, "test_limit_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_limit_step")
        .limit(1)
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());
}

#[test]
fn test_dedup_step() {
    let client = graph();

    drop_vertices(&client, "test_limit_step").unwrap();

    create_vertex_with_label(&client, "test_limit_step", "Count");
    create_vertex_with_label(&client, "test_limit_step", "Count");

    let g = traversal().with_remote(client);

    let results = g
        .v(())
        .has_label("test_limit_step")
        .dedup(())
        .by(T::Label)
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());
}

#[test]
fn test_numerical_steps() {
    let client = graph();

    drop_vertices(&client, "test_numerical_steps").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("test_numerical_steps")
        .property("age", 26)
        .to_list()
        .unwrap();
    g.add_v("test_numerical_steps")
        .property("age", 20)
        .to_list()
        .unwrap();

    // sum
    let results = g
        .v(())
        .has_label("test_numerical_steps")
        .values("age")
        .sum(())
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(&46, results[0].get::<i64>().unwrap());

    // max
    let results = g
        .v(())
        .has_label("test_numerical_steps")
        .values("age")
        .max(())
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(&26, results[0].get::<i32>().unwrap());

    // mean

    let results = g
        .v(())
        .has_label("test_numerical_steps")
        .values("age")
        .mean(())
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(&23.0, results[0].get::<f64>().unwrap());

    // min

    let results = g
        .v(())
        .has_label("test_numerical_steps")
        .values("age")
        .min(())
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(&20, results[0].get::<i32>().unwrap());
}

#[test]
fn test_has_with_p_steps() {
    let client = graph();

    drop_vertices(&client, "test_has_with_p_steps").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("test_has_with_p_steps")
        .property("age", 26)
        .to_list()
        .unwrap();
    let vertices = g
        .add_v("test_has_with_p_steps")
        .property("age", 20)
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has(("test_has_with_p_steps", "age", P::within(vec![19, 20])))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertices[0].id(), results[0].id());

    let results = g
        .v(())
        .has_label("test_has_with_p_steps")
        .values("age")
        .is(20)
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(&20, results[0].get::<i32>().unwrap());

    let results = g
        .v(())
        .has_label("test_has_with_p_steps")
        .values("age")
        .is(P::within(vec![19, 20]))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(&20, results[0].get::<i32>().unwrap());
}

#[test]
fn test_has_with_text_p_step() {
    let client = graph();

    drop_vertices(&client, "test_has_with_text_p_step").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("test_has_with_text_p_step")
        .property("name", "Jon")
        .to_list()
        .unwrap();

    let vertices = g
        .add_v("test_has_with_text_p_step")
        .property("name", "Alice")
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has(("test_has_with_text_p_step", "name", TextP::containing("A")))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(vertices[0].id(), results[0].id());

    let results = g
        .v(())
        .has_label("test_has_with_text_p_step")
        .values("name")
        .is("Alice")
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!("Alice", results[0].get::<String>().unwrap());

    let results = g
        .v(())
        .has_label("test_has_with_text_p_step")
        .values("name")
        .is(TextP::containing("Al"))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!("Alice", results[0].get::<String>().unwrap());

    g.add_v("test_has_with_text_p_step")
        .property("name", "Alice2")
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has(("test_has_with_text_p_step", "name", TextP::containing("A")))
        .to_list()
        .unwrap();

    assert_eq!(2, results.len());
}

#[test]
fn where_step_test() {
    let client = graph();

    drop_vertices(&client, "where_step_test").unwrap();

    let g = traversal().with_remote(client);

    let v = g
        .add_v("where_step_test")
        .property("age", 26)
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has_label("where_step_test")
        .where_(__.values("age").is(26))
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!(v[0].id(), results[0].id());
}

#[test]
fn not_step_test() {
    let client = graph();

    drop_vertices(&client, "not_step_test").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("not_step_test")
        .property("age", 26)
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has_label("not_step_test")
        .not(__.values("age").is(26))
        .to_list()
        .unwrap();

    assert_eq!(0, results.len());
}

#[test]
fn order_step_test() {
    let client = graph();

    drop_vertices(&client, "order_step_test").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("order_step_test")
        .property("name", "b")
        .to_list()
        .unwrap();

    g.add_v("order_step_test")
        .property("name", "a")
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has_label("order_step_test")
        .values("name")
        .order(())
        .to_list()
        .unwrap();

    assert_eq!(2, results.len());

    assert_eq!("a", results[0].get::<String>().unwrap());

    let results = g
        .v(())
        .has_label("order_step_test")
        .values("name")
        .order(())
        .by(Order::Desc)
        .to_list()
        .unwrap();

    assert_eq!(2, results.len());

    assert_eq!("b", results[0].get::<String>().unwrap());
}

#[test]
fn match_step_test() {
    let client = graph();

    drop_vertices(&client, "match_step_test").unwrap();

    drop_edges(&client, "match_step_test_edge").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("match_step_test")
        .property("name", "a")
        .to_list()
        .unwrap();

    let v2 = g
        .add_v("match_step_test")
        .property("name", "b")
        .to_list()
        .unwrap();

    let v3 = g
        .add_v("match_step_test")
        .property("name", "c")
        .to_list()
        .unwrap();

    g.add_e("match_step_test_edge")
        .from(&v1[0])
        .to(&v2[0])
        .to_list()
        .unwrap();

    g.add_e("match_step_test_edge")
        .from(&v2[0])
        .to(&v3[0])
        .to_list()
        .unwrap();

    let results = g
        .v(())
        .has_label("match_step_test")
        .match_(vec![
            __.as_("a")
                .has(("name", "a"))
                .out("match_step_test_edge")
                .as_("b"),
            __.as_("b").out("match_step_test_edge").as_("c"),
        ])
        .select(vec!["a", "c"])
        .to_list()
        .unwrap();

    assert_eq!(1, results.len());

    let first = &results[0].get::<Map>().unwrap();

    assert_eq!(&v1[0], first["a"].get::<Vertex>().unwrap());
    assert_eq!(&v3[0], first["c"].get::<Vertex>().unwrap());
}

#[test]
fn drop_step_test() {
    let client = graph();

    drop_vertices(&client, "drop_step_test").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("drop_step_test")
        .property("name", "a")
        .next()
        .unwrap();

    g.add_v("drop_step_test")
        .property("name", "b")
        .next()
        .unwrap();

    let results = g.v(()).has_label("drop_step_test").count().next().unwrap();

    assert_eq!(Some(2), results);

    g.v(())
        .has_label("drop_step_test")
        .drop()
        .to_list()
        .unwrap();

    let results = g.v(()).has_label("drop_step_test").has_next().unwrap();

    assert_eq!(false, results);
}

#[test]
fn or_step_test() {
    let client = graph();

    drop_vertices(&client, "or_step_test").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("or_step_test")
        .property("foo", "bar")
        .property("bar", "foo")
        .next()
        .unwrap();

    g.add_v("or_step_test")
        .property("foo", "nobar")
        .property("bar", "nofoo")
        .next()
        .unwrap();

    let result = g
        .v(())
        .has_label("or_step_test")
        .has(("foo", "bar"))
        .or(())
        .has(("bar", "foo"))
        .to_list()
        .unwrap();
    assert_eq!(result.len(), 1);

    let result = g
        .v(())
        .has_label("or_step_test")
        .has(("foo", "bar"))
        .or(())
        .has(("bar", "nofoo"))
        .to_list()
        .unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn iter_terminator_test() {
    let client = graph();

    drop_vertices(&client, "iter_terminator_test").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("iter_terminator_test")
        .property("name", "a")
        .next()
        .unwrap();

    g.add_v("iter_terminator_test")
        .property("name", "b")
        .next()
        .unwrap();

    let results: Vec<Vertex> = g
        .v(())
        .has_label("iter_terminator_test")
        .iter()
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    assert_eq!(2, results.len())
}

#[test]
fn test_select_pop() {
    let client = graph();

    drop_vertices(&client, "test_select_pop").unwrap();
    drop_vertices(&client, "test_select_pop_child").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("test_select_pop")
        .property("name", "a")
        .to_list()
        .unwrap();

    let v2 = g
        .add_v("test_select_pop")
        .property("name", "b")
        .to_list()
        .unwrap();

    let e1 = g
        .add_v("test_select_pop_child")
        .property("name", "a")
        .to_list()
        .unwrap();

    let e2 = g
        .add_v("test_select_pop_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    g.add_e("child").from(&v1[0]).to(&e1[0]).to_list().unwrap();

    g.add_e("child").from(&v2[0]).to(&e2[0]).to_list().unwrap();

    let results = g
        .v(())
        .has_label("test_select_pop")
        .has(("name", "a"))
        .out("child")
        .as_("v")
        .v(())
        .has_label("test_select_pop")
        .has(("name", "b"))
        .out("child")
        .as_("v")
        .select((Pop::All, "v"))
        .unfold()
        .to_list()
        .unwrap();
    assert_eq!(results.len(), 2);

    let results = g
        .v(())
        .has_label("test_select_pop")
        .has(("name", "a"))
        .out("child")
        .as_("v")
        .v(())
        .has_label("test_select_pop")
        .has(("name", "b"))
        .out("child")
        .as_("v")
        .select((Pop::Last, "v"))
        .unfold()
        .to_list()
        .unwrap();
    assert_eq!(results.len(), 1);

    let results = g
        .v(())
        .has_label("test_select_pop")
        .has(("name", "a"))
        .out("child")
        .as_("v")
        .v(())
        .has_label("test_select_pop")
        .has(("name", "b"))
        .out("child")
        .as_("v")
        .select((Pop::First, "v"))
        .unfold()
        .to_list()
        .unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_repeat_until_loops_loops() {
    let client = graph();

    drop_vertices(&client, "test_repeat_until_loops").unwrap();
    drop_vertices(&client, "test_repeat_until_loops_child").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("test_repeat_until_loops")
        .property("name", "a")
        .to_list()
        .unwrap();

    let e1 = g
        .add_v("test_repeat_until_loops_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    let e2 = g
        .add_v("test_repeat_until_loops_child")
        .property("name", "c")
        .to_list()
        .unwrap();

    g.add_e("child").from(&v1[0]).to(&e1[0]).to_list().unwrap();
    g.add_e("child").from(&e1[0]).to(&e2[0]).to_list().unwrap();

    let results = g
        .v(v1[0].id())
        .repeat(__.out("child"))
        .until(__.loops(()).is(2))
        .to_list()
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], e2[0]);
}

#[test]
fn test_simple_path() {
    let client = graph();

    drop_vertices(&client, "test_simple_path").unwrap();
    drop_vertices(&client, "test_simple_path_child").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("test_simple_path")
        .property("name", "a")
        .to_list()
        .unwrap();

    let e1 = g
        .add_v("test_simple_path_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    let e2 = g
        .add_v("test_simple_path_child")
        .property("name", "c")
        .to_list()
        .unwrap();

    g.add_e("child").from(&v1[0]).to(&e1[0]).to_list().unwrap();
    g.add_e("child").from(&e1[0]).to(&e2[0]).to_list().unwrap();
    g.add_e("child").from(&e2[0]).to(&v1[0]).to_list().unwrap();

    let results = g
        .v(v1[0].id())
        .repeat(__.out("child").simple_path())
        .until(__.loops(()).is(2))
        .to_list()
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], e2[0]);
}

#[test]
fn test_sample() {
    let client = graph();

    drop_vertices(&client, "test_sample").unwrap();
    drop_vertices(&client, "test_sample_child").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("test_sample")
        .property("name", "a")
        .to_list()
        .unwrap();

    let e1 = g
        .add_v("test_sample_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    let e2 = g
        .add_v("test_sample_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    g.add_e("child").from(&v1[0]).to(&e1[0]).to_list().unwrap();
    g.add_e("child").from(&v1[0]).to(&e2[0]).to_list().unwrap();
    let results = g.v(v1[0].id()).out("child").sample(1).to_list().unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_local() {
    let client = graph();

    drop_vertices(&client, "test_local").unwrap();
    drop_vertices(&client, "test_local_child").unwrap();
    drop_vertices(&client, "test_local_child_child").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("test_local")
        .property("name", "a")
        .to_list()
        .unwrap();

    let e1 = g
        .add_v("test_local_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    let e2 = g
        .add_v("test_local_child")
        .property("name", "b")
        .to_list()
        .unwrap();

    let e3 = g
        .add_v("test_local_child_child")
        .property("name", "c")
        .to_list()
        .unwrap();

    let e4 = g
        .add_v("test_local_child_child")
        .property("name", "d")
        .to_list()
        .unwrap();

    let e5 = g
        .add_v("test_local_child_child")
        .property("name", "e")
        .to_list()
        .unwrap();

    g.add_e("child").from(&v1[0]).to(&e1[0]).to_list().unwrap();
    g.add_e("child").from(&v1[0]).to(&e2[0]).to_list().unwrap();

    g.add_e("child_child")
        .from(&e1[0])
        .to(&e3[0])
        .to_list()
        .unwrap();
    g.add_e("child_child")
        .from(&e1[0])
        .to(&e4[0])
        .to_list()
        .unwrap();

    g.add_e("child_child")
        .from(&e2[0])
        .to(&e5[0])
        .to_list()
        .unwrap();

    let results = g
        .v(v1[0].id())
        .out("child")
        .local(__.out("child_child").sample(1)) //Local used here to only get one vertices from each child
        .to_list()
        .unwrap();

    assert_eq!(results.len(), 2);
}

#[test]
fn test_side_effect() {
    let client = graph();
    let test_vertex_label = "test_side_effect";
    let expected_side_effect_key = "prop_key";
    let expected_side_effect_value = "prop_val";

    drop_vertices(&client, &test_vertex_label).unwrap();

    let g = traversal().with_remote(client);

    let element_map = g
        .add_v(test_vertex_label)
        .side_effect(__.property(
            gremlin_client::structure::Either2::A(expected_side_effect_key),
            expected_side_effect_value,
        ))
        .element_map(())
        .next()
        .expect("Should get response")
        .expect("Should have returned an element map");

    assert_map_property(
        &element_map,
        expected_side_effect_key,
        expected_side_effect_value,
    );
}

#[test]
fn test_anonymous_traversal_properties_drop() {
    let client = graph();
    let test_vertex_label = "test_anonymous_traversal_properties_drop";
    let pre_drop_prop_key = "pre_drop_prop_key";
    let expected_prop_value = "prop_val";

    drop_vertices(&client, &test_vertex_label).unwrap();

    let g = traversal().with_remote(client);

    let element_map = g
        .add_v(test_vertex_label)
        .side_effect(__.property(
            gremlin_client::structure::Either2::A(pre_drop_prop_key),
            expected_prop_value,
        ))
        .element_map(())
        .next()
        .expect("Should get response")
        .expect("Should have returned an element map");

    //Make sure the property was assigned
    assert_map_property(&element_map, pre_drop_prop_key, expected_prop_value);

    let created_vertex_id = element_map.get("id").expect("Should have id property");
    let GValue::Int64(id) = created_vertex_id else {
        panic!("Not expected id type");
    };

    let post_drop_prop_key = "post_drop_prop_key";
    //Operate on the same vertex via id
    let post_drop_map = g
        .v(*id)
        //Drop all properties first
        .side_effect(__.properties(()).drop())
        //Then add a different property
        .side_effect(__.property(
            gremlin_client::structure::Either2::A(pre_drop_prop_key),
            expected_prop_value,
        ))
        .element_map(())
        .next()
        .expect("Should get response")
        .expect("Should have returned an element map");

    assert_map_property(&post_drop_map, pre_drop_prop_key, expected_prop_value);

    //Now make sure the pre drop property key is no longer present
    assert!(
        post_drop_map.get(post_drop_prop_key).is_none(),
        "Pre drop key should have been dropped"
    );
}

#[test]
fn test_by_columns() {
    let client = graph();
    let test_vertex_label = "test_by_columns";
    let expected_side_effect_key_a = "prop_key_a";
    let expected_side_effect_value_a = "prop_val_a";
    let expected_side_effect_key_b = "prop_key_b";
    let expected_side_effect_value_b = "prop_val_b";

    drop_vertices(&client, &test_vertex_label).unwrap();

    let g = traversal().with_remote(client);
    let mut property_map: HashMap<GKey, GValue> = HashMap::new();
    property_map.insert(
        expected_side_effect_key_a.into(),
        expected_side_effect_value_a.into(),
    );
    property_map.insert(
        expected_side_effect_key_b.into(),
        expected_side_effect_value_b.into(),
    );

    let element_map = g
        .inject(vec![property_map.into()])
        .unfold()
        .as_("properties")
        .add_v(test_vertex_label)
        .as_("v")
        .side_effect(
            __.select("properties")
                .unfold()
                .as_("kv_pair")
                .select("v")
                .property(
                    __.select("kv_pair").by(Column::Keys),
                    __.select("kv_pair").by(Column::Values),
                ),
        )
        .element_map(())
        .next()
        .expect("Should get response")
        .expect("Should have returned an element map");

    assert_map_property(
        &element_map,
        expected_side_effect_key_a,
        expected_side_effect_value_a,
    );

    assert_map_property(
        &element_map,
        expected_side_effect_key_b,
        expected_side_effect_value_b,
    );
}

#[test]
fn test_property_cardinality() {
    let client = graph();

    drop_vertices(&client, "test_property_cardinality").unwrap();

    let g = traversal().with_remote(client);

    let v1 = g
        .add_v("test_property_cardinality")
        .property("name", "a")
        .to_list()
        .unwrap();

    assert!(v1.len() > 0);

    g.v(v1[0].id())
        .property_with_cardinality(Cardinality::List, "name", "b")
        .next()
        .unwrap();
    let new_v = g.v(v1[0].id()).property_map(()).next().unwrap().unwrap();
    assert_eq!(2, new_v["name"].get::<List>().unwrap().len());

    g.v(v1[0].id())
        .property_with_cardinality(Cardinality::Single, "name", "b")
        .next()
        .unwrap();
    let new_v = g.v(v1[0].id()).property_map(()).next().unwrap().unwrap();
    assert_eq!(1, new_v["name"].get::<List>().unwrap().len());
}

#[test]
fn test_choose() {
    let client = graph();

    drop_vertices(&client, "test_choose").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("test_choose")
        .property("name", "a")
        .as_("source")
        .choose((
            __.select("source").has("name"),
            __.add_v("test_choose_success"),
            __.add_v("test_choose_failure"),
        ))
        .next()
        .unwrap();

    let success_vertices = g.v(()).has_label("test_choose_success").next().unwrap();
    assert_eq!(success_vertices.is_some(), true);

    let success_vertices = g.v(()).has_label("test_choose_failure").next().unwrap();
    assert_eq!(success_vertices.is_some(), false);

    g.add_v("test_choose")
        .property("name", "b")
        .as_("source")
        .choose((
            __.select("source").has("name"),
            __.add_v("test_choose_success2"),
        ))
        .next()
        .unwrap();

    let success_vertices = g.v(()).has_label("test_choose_failure2").next().unwrap();
    assert_eq!(success_vertices.is_some(), false);

    let success_vertices = g.v(()).has_label("test_choose_success2").next().unwrap();
    assert_eq!(success_vertices.is_some(), true);
}

#[test]
fn test_choose_by_literal_options() {
    let client = graph();
    let g = traversal().with_remote(client);

    let choosen_literal_a = g
        .inject(1)
        .unfold()
        .choose(__.identity())
        .option((GValue::Int64(1), __.constant("option-a")))
        .option((GValue::Int64(2), __.constant("option-b")))
        .next()
        .unwrap();

    assert_eq!(choosen_literal_a, Some("option-a".into()));

    let choosen_literal_b = g
        .inject(2)
        .unfold()
        .choose(__.identity())
        .option((GValue::Int64(1), __.constant("option-a")))
        .option((GValue::Int64(2), __.constant("option-b")))
        .next()
        .unwrap();

    assert_eq!(choosen_literal_b, Some("option-b".into()));
}

#[test]
fn test_coalesce() {
    let client = graph();

    use gremlin_client::GValue;

    drop_vertices(&client, "test_coalesce").unwrap();

    let g = traversal().with_remote(client);

    g.add_v("test_coalesce")
        .property("name", "a")
        .to_list()
        .unwrap();

    g.add_v("test_coalesce")
        .property("nickname", "b")
        .to_list()
        .unwrap();

    let v = g
        .v(())
        .has_label("test_coalesce")
        .coalesce::<GValue, _>([__.values("nickname"), __.values("name")])
        .to_list()
        .unwrap();

    let values = v
        .into_iter()
        .map(|e| e.take::<String>().unwrap())
        .collect::<Vec<_>>();

    assert!(values.contains(&String::from("a")));
    assert!(values.contains(&String::from("b")));
}

#[test]
fn test_coalesce_unfold() {
    let client = graph();

    drop_vertices(&client, "test_coalesce_unfold").unwrap();

    let g = traversal().with_remote(client);

    g.v(())
        .has(("test_coalesce_unfold", "name", "unfold"))
        .fold()
        .coalesce::<Vertex, _>([__.unfold(), __.add_v("test_coalesce_unfold")])
        .property("name", "unfold")
        .next()
        .expect("It should create a vertex with coalesce");

    let v = g
        .v(())
        .has_label("test_coalesce_unfold")
        .value_map(())
        .to_list()
        .unwrap();

    let values = v.into_iter().collect::<Vec<_>>();

    assert_eq!(1, values.len());

    assert_eq!(
        "unfold",
        utils::unwrap_map::<String>(&values[0], "name", 0).unwrap()
    );

    g.v(())
        .has(("test_coalesce_unfold", "name", "unfold"))
        .fold()
        .coalesce::<Vertex, _>([__.unfold(), __.add_v("test_coalesce_unfold")])
        .property("name", "unfold")
        .next()
        .expect("It should create a vertex with coalesce");

    let v = g
        .v(())
        .has_label("test_coalesce_unfold")
        .value_map(())
        .to_list()
        .unwrap();

    let values = v.into_iter().collect::<Vec<_>>();

    assert_eq!(1, values.len());

    assert_eq!(
        "unfold",
        utils::unwrap_map::<String>(&values[0], "name", 0).unwrap()
    );
}

#[test]
fn test_none_step() {
    let client = graph();

    drop_vertices(&client, "test_none_step").unwrap();

    let g = traversal().with_remote(client);

    //The addition of a None step however should not IO a vertex back
    g.add_v("test_none_step")
        .none()
        .iter()
        .expect("Should get a iter back")
        .iterate()
        .expect("Shouldn't error consuming iterator");

    //Make sure the vertex is present in the graph
    let vertex_count = g
        .v(())
        .has_label("test_none_step")
        .count()
        .next()
        .ok()
        .flatten()
        .expect("Should have gotten a response");
    assert_eq!(1, vertex_count);
}

#[test]
#[cfg(feature = "derive")]
fn test_traversal_vertex_mapping() {
    use chrono::{DateTime, TimeZone, Utc};
    use gremlin_client::derive::FromGMap;
    use std::convert::TryFrom;

    let client = graph();

    drop_vertices(&client, "test_vertex_mapping").unwrap();

    let g = traversal().with_remote(client);

    let uuid = uuid::Uuid::new_v4();
    let mark = g
        .add_v("person")
        .property("name", "Mark")
        .property("age", 22)
        .property("time", 22 as i64)
        .property("score", 3.2)
        .property("uuid", uuid.clone())
        .property("datetime", chrono::Utc.timestamp(1551825863, 0))
        .property("date", 1551825863 as i64)
        .value_map(true)
        .by(TraversalBuilder::new(Bytecode::new()).unfold())
        .next();
    assert_eq!(mark.is_ok(), true);

    #[derive(Debug, PartialEq, FromGMap)]
    struct Person {
        name: String,
        age: i32,
        time: i64,
        datetime: DateTime<Utc>,
        uuid: uuid::Uuid,
        optional: Option<String>,
    }
    let person = Person::try_from(mark.unwrap().unwrap());
    assert_eq!(person.is_ok(), true);

    assert_eq!(
        Person {
            name: String::from("Mark"),
            age: 22,
            time: 22,
            datetime: chrono::Utc.timestamp(1551825863, 0),
            uuid: uuid,
            optional: None
        },
        person.unwrap()
    );
}
