use gremlin_client::{process::traversal, GremlinClient};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let result = client.execute(
        "g.V().hasLabel('airport').
               sample(30).
               order().by('code').
               local(__.values('code','city').fold()).
               toList()",
        &[],
    );
    let g = traversal().with_remote(client);

    Ok(())
}
