# gremlin-rs




[![Build Status](https://travis-ci.org/wolf4ood/gremlin-rs.svg?branch=master)](https://travis-ci.org/wolf4ood/gremlin-rs)
[![Codecov Status](https://codecov.io/gh/wolf4ood/gremlin-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/wolf4ood/gremlin-rs)
[![Crates.io](https://img.shields.io/crates/v/gremlin-client.svg)](https://crates.io/crates/gremlin-client)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Docs](https://docs.rs/gremlin-client/badge.svg)](https://docs.rs/gremlin-client)



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



