use gremlin_client::{GremlinClient, Path};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let results = client
        .execute("g.V(param).outE().inV().path()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Path>())
        .collect::<Result<Vec<Path>, _>>()?;

    println!("{:#?}", results);

    Ok(())
}
