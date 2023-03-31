use gremlin_client::{aio::GremlinClient, process::traversal::traversal};

#[cfg_attr(feature = "async", tokio::main)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost").await?;

    let g = traversal().with_remote_async(client);

    let vertices = g
        .v(())
        .has_label("person")
        .has(("name", "marko"))
        .to_list()
        .await?;

    println!("{:?}", vertices);

    let friends = g
        .v(())
        .has_label("person")
        .has(("name", "marko"))
        .out("knows")
        .to_list()
        .await?;

    println!("{:?}", friends);

    Ok(())
}
