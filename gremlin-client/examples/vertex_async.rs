use gremlin_client::{aio::GremlinClient, Vertex};

#[cfg(feature = "async-std-runtime")]
use async_std::prelude::*;

#[cfg(feature = "tokio-runtime")]
use tokio::stream::StreamExt;

#[cfg_attr(feature = "async-std-runtime", async_std::main)]
#[cfg_attr(feature = "tokio-runtime", tokio::main)]
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
