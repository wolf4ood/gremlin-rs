use gremlin_client::{GremlinClient, List, Map, VertexProperty};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let results = client
        .execute("g.V(param).properties()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<VertexProperty>())
        .collect::<Result<Vec<VertexProperty>, _>>()?;

    println!("{:?}", results[0].get::<String>()?);

    let results = client
        .execute("g.V(param).propertyMap()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()?;

    println!(
        "{:?}",
        results[0]["name"].get::<List>()?[0]
            .get::<VertexProperty>()?
            .get::<String>()?
    );

    Ok(())
}
