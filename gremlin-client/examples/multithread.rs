use gremlin_client::{GremlinClient, Vertex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let c = client.clone();

    let result = thread::spawn(move || {
        c.execute("g.V(param)", &[("param", &1)])?
            .filter_map(Result::ok)
            .map(|f| f.take::<Vertex>())
            .collect::<Result<Vec<Vertex>, _>>()
    });

    println!("{:?}", result.join());

    Ok(())
}
