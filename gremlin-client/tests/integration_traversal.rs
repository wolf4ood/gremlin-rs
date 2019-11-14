use gremlin_client::process::traversal::{traversal, Order, __};
use gremlin_client::structure::{List, Map, Pop, TextP, Vertex, VertexProperty, P, T};
use gremlin_client::utils;

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
fn test_uwnrap_map() {
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
