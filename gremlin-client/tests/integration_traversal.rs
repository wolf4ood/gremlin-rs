use gremlin_client::process::traversal;
use gremlin_client::structure::{List, Map, VertexProperty};

mod common;

use common::{
    create_edge, create_vertex, create_vertex_with_label, drop_edges, drop_vertices, graph,
};

#[test]
fn test_simple_vertex_traversal() {
    let g = traversal().with_remote(graph());

    let results = g.v(()).to_list().unwrap();

    assert!(results.len() > 0);
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
        .has("name", "Traversal")
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

    let _e = create_edge(&client, &v, &v1, "test_vertex_out_traversal");

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
        .property("name", "marko")
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
}
