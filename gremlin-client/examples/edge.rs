use gremlin_client::{Edge, GremlinClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    // Find outgoing edges for V[1]
    let results = client
        .execute("g.V(param).outE()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()?;

    println!("Edges count {}", results.len());

    let first = &results[0];

    println!(
        "Edge with id: [{}] and label: [{}] from: [{}] to: [{}]",
        first.id().get::<i32>()?,
        first.label(),
        first.in_v().id().get::<i64>()?,
        first.out_v().id().get::<i64>()?
    );

    Ok(())
}
