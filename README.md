# gremlin-rs






[![Build Status](https://dev.azure.com/enricorisa/gremlin-rs/_apis/build/status/wolf4ood.gremlin-rs?branchName=master)](https://dev.azure.com/enricorisa/gremlin-rs/_build/latest?definitionId=1&branchName=master)




## gremlin-client

A Rust client for Apache TinkerPop™.



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
gremlin_client = "0.1.0"
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



