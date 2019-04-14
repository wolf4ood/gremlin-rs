use gremlin_client::{process::traversal, GremlinClient};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let g = traversal().with_remote(client);

    let vertices = g.v(()).has_label("person").has("name", "marko").to_list()?;

    println!("{:?}", vertices);

    Ok(())
}
