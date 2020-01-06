use gremlin_client::{aio::GremlinClient, process::traversal::traversal};
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    task::block_on(async {

        let client = GremlinClient::connect("localhost").await?;

        let g = traversal().with_remote_async(client);

        let results = g.v(()).has_label("person").has(("name","Jon")).to_list().await?;   
    
        println!("{:?}", results);
        Ok(())
    })
}