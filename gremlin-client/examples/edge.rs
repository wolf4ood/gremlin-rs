use gremlin_client::{Edge, GremlinClient};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let results = client
        .execute("g.V(param).outE()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()?;

    println!("{:#?}", results);

    Ok(())
}
