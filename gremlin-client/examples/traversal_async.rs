use async_std::task;
use gremlin_client::{aio::GremlinClient, process::traversal::traversal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    task::block_on(async {
        let client = GremlinClient::connect("localhost").await?;

        let g = traversal().with_async_remote(client);

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
    })
}
