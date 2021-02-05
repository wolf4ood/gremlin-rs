

<h1 align="center">Gremlin-rs</h1>
<div align="center">
  <strong>
    Rust driver and tools for Apache TinkerPop™.
  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/wolf4ood/gremlin-rs/actions?query=workflow%3ATests">
    <img src="https://github.com/wolf4ood/gremlin-rs/workflows/Tests/badge.svg"
    alt="Tests status" />
  </a>
  
  <a href="https://coveralls.io/github/wolf4ood/gremlin-rs?branch=master">
    <img src="https://coveralls.io/repos/github/wolf4ood/gremlin-rs/badge.svg?branch=master"
    alt="Coverage status" />
  </a>
  <a href="https://crates.io/crates/gremlin-client">
    <img src="https://img.shields.io/crates/d/gremlin-client.svg?style=flat-square"
      alt="Download" />
  </a>
  <a href="https://docs.rs/gremlin-rs">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

   <a href="https://opensource.org/licenses/Apache-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg"
      alt="license" />
  </a>

   <a href="https://deps.rs/repo/github/wolf4ood/gremlin-rs">
    <img src="https://deps.rs/repo/github/wolf4ood/gremlin-rs/status.svg"
      alt="license" />
  </a>


  
</div>



## gremlin-client

A Rust client for Apache TinkerPop™.



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
gremlin_client = "0.4.0"
```


with [async-std](https://async.rs/) support 

```toml
[dependencies]
gremlin_client = { version = "0.4.0", features = ["async-std-runtime"] }
```

with [tokio](https://tokio.rs/) support 

```toml
[dependencies]
gremlin_client = { version = "0.4.0", features = ["tokio-runtime"] }
```

### Examples


#### Basic usage


Execute a simple Gremlin query with an id and collect the results

**Synchronous**

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


**Asynchronous**

With [async-std](https://async.rs/)

activate the feature `async-std-runtime`

`gremlin-client = { version = "*", features = ["async-std-runtime"] }`

```rust
     
use gremlin_client::{aio::GremlinClient, Vertex};
use async_std::prelude::*;

#[async_std::main]
async fn main() -> Result<(), Box<std::error::Error>> {

    let client = GremlinClient::connect("localhost").await?;
    let results = client.execute("g.V(param)", &[("param", &1)]).await?
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>().await?;
    println!("{:?}", results);
    Ok(())
    
}
```

With [tokio](https://tokio.rs/)

activate the feature `tokio-runtime`

`gremlin-client = { version = "*", features = ["tokio-runtime"] }`

```rust
     
use gremlin_client::{aio::GremlinClient, Vertex};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<std::error::Error>> {

    let client = GremlinClient::connect("localhost").await?;
    let results = client.execute("g.V(param)", &[("param", &1)]).await?
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>().await?;
    println!("{:?}", results);
    Ok(())
    
}
```

#### Traversal example Rust GLV

Create a remote traversal with the provided `GremlinClient` and build a traversal
using Rust language.

**Synchronous**

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


**Aynchronous**

With [async-std](https://async.rs/)

```rust
use gremlin_client::{aio::GremlinClient, Vertex, process::traversal::traversal};
use async_std::prelude::*;

#[async_std::main]
async fn main() -> Result<(), Box<std::error::Error>> {

    
    let client = GremlinClient::connect("localhost").await?;

    let g = traversal().with_remote_async(client);

    let results = g.v(()).has_label("person").has(("name","Jon")).to_list().await?;   

    println!("{:?}", results);
    Ok(())
    
}
```

With [tokio](https://tokio.rs/)

```rust
use gremlin_client::{aio::GremlinClient, Vertex, process::traversal::traversal};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<std::error::Error>> {

    let client = GremlinClient::connect("localhost").await?;

    let g = traversal().with_remote_async(client);

    let results = g.v(()).has_label("person").has(("name","Jon")).to_list().await?;   

    println!("{:?}", results);
    Ok(())
}
```


### Additional Features

#### `derive` feature

By including the `derive` feature in your Cargo.toml

```
[dependencies]
gremlin_client = { version = "*", features = ["derive"] }
```

two derive macros are available 

- FromGMap
- FromGValue

which you can use to derive the mapping from GMap and GValue (only Map currently) into structs.


with `GValue`

```rust
use gremlin_client::derive::{FromGMap, FromGValue};
use gremlin_client::process::traversal::traversal;
use gremlin_client::GremlinClient;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    #[derive(Debug, PartialEq, FromGValue, FromGMap)]
    struct Person {
        name: String,
    }

    let results = client
        .execute("g.V(param).valueMap()", &[("param", &1)])?
        .filter_map(Result::ok)
        .map(|f| Person::try_from(f))
        .collect::<Result<Vec<Person>, _>>()?;

    println!("Person {:?}", results[0);
    Ok(())
}

```

with `GMap`

```rust
use gremlin_client::derive::FromGMap;
use gremlin_client::process::traversal::traversal;
use gremlin_client::GremlinClient;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    #[derive(Debug, PartialEq, FromGMap)]
    struct Person {
        name: String,
    }

    let g = traversal().with_remote(client);

    let results = g
        .v(1)
        .value_map(())
        .iter()?
        .filter_map(Result::ok)
        .map(Person::try_from)
        .collect::<Result<Vec<Person>, _>>()?;

    println!("Person {:?}", results[0);

    Ok(())
}
```


### Development


#### Compiling

```
git clone https://github.com/wolf4ood/gremlin-rs.git
cd gremlin-rs
cargo build
```


#### Running Tests

Some tests run against a running instance of Gremlin Server with a sample in-memory graph installed.

You can use docker-compose to start an instance for testing. Use the env variable `GREMLIN_SERVER`
in order to specify the version of the Gremlin Server

```
cd docker-compose
export GREMLIN_SERVER=3.4.0
docker-compose up -d
cd ..
cargo test --all-features
```




## gremlin-cli 


A minimal cli for exploring graphs data in Gremlin Server.



### Install


```
cargo install gremlin-cli
```

or latest release [here](https://github.com/wolf4ood/gremlin-rs/releases)
