use gremlin_client::{ConnectionOptions, GremlinClient, Vertex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect(
        ConnectionOptions::builder()
            .host("localhost")
            .port(8182)
            .pool_size(1)
            .credentials("stephen", "password")
            .build(),
    )?;

    let results = client
        .execute("g.V(param)", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()?;

    println!("{:?}", results);

    Ok(())
}
