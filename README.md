# gremlin-rs




[![Build Status](https://travis-ci.org/wolf4ood/gremlin-rs.svg?branch=master)](https://travis-ci.org/wolf4ood/gremlin-rs)
[![Codecov Status](https://codecov.io/gh/wolf4ood/gremlin-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/wolf4ood/gremlin-rs)
[![Crates.io](https://img.shields.io/crates/v/gremlin-client.svg)](https://crates.io/crates/gremlin-client)
[![Crates.io Download](https://img.shields.io/crates/d/gremlin-client.svg)](https://crates.io/crates/gremlin-client)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Docs](https://docs.rs/gremlin-client/badge.svg)](https://docs.rs/gremlin-client)
[![dependency status](https://deps.rs/repo/github/wolf4ood/gremlin-rs/status.svg)](https://deps.rs/repo/github/wolf4ood/gremlin-rs)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/e29ea9b1c90a47f797d259de04eb595b)](https://www.codacy.com/app/wolf4ood/gremlin-rs?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=wolf4ood/gremlin-rs&amp;utm_campaign=Badge_Grade)



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



