# gremlin-rs





## gremlin-client

A Rust client for Apache TinkerPop™.



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
gremlin_client = "0.3.0"
```


with async support via [async-std](https://async.rs/)

```toml
[dependencies]
gremlin_client = { version = "0.3.0", features = ["async_std"] }
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

```rust
     
use gremlin_client::{aio::GremlinClient, Vertex};
use async_std::task;
use async_std::prelude::*;

fn main() -> Result<(), Box<std::error::Error>> {

    task::block_on(async {
        let client = GremlinClient::connect("localhost").await?;
        let results = client.execute("g.V(param)", &[("param", &1)]).await?
            .filter_map(Result::ok)
            .map(|f| f.take::<Vertex>())
            .collect::<Result<Vec<Vertex>, _>>().await?;
        println!("{:?}", results);
        Ok(())
    })    
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

```rust
use gremlin_client::{aio::GremlinClient, Vertex, process::traversal::traversal};
use async_std::task;
use async_std::prelude::*;

fn main() -> Result<(), Box<std::error::Error>> {

    task::block_on(async {

        let client = GremlinClient::connect("localhost").await?;

        let g = traversal().with_async_remote(client);

        let results = g.v(()).has_label("person").has(("name","Jon")).to_list().await?;   
    
        println!("{:?}", results);
        Ok(())
    })
}
```


