use gremlin_client::{GremlinClient, Map, Property};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let results = client
        .execute("g.V(param).outE().properties()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Property>())
        .collect::<Result<Vec<Property>, _>>()?;

    println!("{:?}", results);

    let results = client
        .execute("g.V(param).outE().propertyMap()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()?;

    println!("{:?}", results);

    Ok(())
}
