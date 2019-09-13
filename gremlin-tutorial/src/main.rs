use gremlin_client::{process::traversal::traversal, GremlinClient};

mod chapters;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let g = traversal().with_remote(client);

    chapters::all().iter().for_each(|f| {
        f(&g).unwrap();
    });

    Ok(())
}
