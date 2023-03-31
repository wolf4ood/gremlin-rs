use gremlin_client::{aio::GremlinClient, Vertex};

#[cfg(feature = "async")]
use tokio_stream::StreamExt;

#[cfg_attr(feature = "async", tokio::main)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost").await?;

    let results = client
        .execute("g.V(param)", &[("param", &1)])
        .await?
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .await?;

    println!("Vertex count: {}", results.len());

    let vertex = &results[0];

    println!(
        "Vertex with id: [{}] and label: [{}]",
        vertex.id().get::<i64>()?,
        vertex.label()
    );

    Ok(())
}
