use gremlin_client::{
    process::traversal::{traversal, GraphTraversalSource, SyncTerminator, __},
    structure::{List, Vertex, P},
    GremlinClient,
};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let g = traversal().with_remote(client);

    create_graph(&g)?;

    let result = g
        .v(())
        .has_label("complex_vertex")
        .has(("name", "test1"))
        .out("complex_label")
        .out("complex_label")
        .value_map(())
        .next()?
        .expect("no vertices found");

    println!(
        "Found vertex with name {:?}",
        result["name"].get::<List>().unwrap()[0]
    );

    let results = g
        .v(())
        .has_label("complex_vertex")
        .has(("number", P::gt(3)))
        .to_list()?;

    println!(
        "Found {} vertices with number greater than 3",
        results.len()
    );

    let results = g
        .v(())
        .has_label("complex_vertex")
        .has(("number", P::within((3, 6))))
        .to_list()?;

    println!("Found {} vertices with number 3 or 6", results.len());

    let results = g
        .v(())
        .has_label("complex_vertex")
        .where_(__.out("complex_label").count().is(P::gte(1)))
        .to_list()?;

    println!(
        "Found {} vertices with 1 or more connected edges with label complex_label",
        results.len()
    );

    Ok(())
}

fn create_graph(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>> {
    g.v(()).has_label("complex_vertex").drop().next()?;
    g.e(()).has_label("complex_label").drop().next()?;

    let mut current_next: Option<Vertex> = None;
    (0..10).for_each(|e| {
        let next = g
            .add_v("complex_vertex")
            .property("name", format!("test{}", e))
            .property("number", e)
            .next()
            .expect("failed to create vertex");

        current_next.iter().zip(next.iter()).for_each(|(a, b)| {
            g.add_e("complex_label")
                .from(a)
                .to(b)
                .next()
                .expect("failed to create edge");
        });

        current_next = next;
    });

    Ok(())
}
