# gremlin-rs





## gremlin-client

A Rust client for Apache TinkerPop™.



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
gremlin_client = "0.2.0"
```



### Examples


#### Basic usage


Execute a simple Gremlin query with an id and collect the results

```rust
use gremlin_client::{GremlinClient, Vertex};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let results = client
        .execute("g.V(param)", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()?;

    println!("{:?}", results);

    Ok(())
}
```


#### Traversal example Rust GLV

Create a remote traversal with the provided `GremlinClient` and build a traversal
using Rust language.

```rust
 use gremlin_client::{GremlinClient, Vertex, process::traversal::traversal};

 fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let g = traversal().with_remote(client);

    let results = g.v(()).has_label("person").has(("name","Jon")).to_list()?;   
    
    println!("{:?}", results);
    Ok(())
}
```



