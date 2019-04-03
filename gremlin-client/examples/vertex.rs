use gremlin_client::{GremlinClient, Vertex};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let results = client
        .execute("g.V(param)", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()?;

    println!("Vertex count: {}", results.len());

    let vertex = &results[0];

    println!(
        "Vertex with id: [{}] and label: [{}]",
        vertex.id().get::<i64>()?,
        vertex.label()
    );

    Ok(())
}
