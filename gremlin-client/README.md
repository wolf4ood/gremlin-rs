# gremlin-rs





## gremlin-client

A Rust client for Apache TinkerPop™.



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
gremlin_client = "0.4.0"
```


with async support via [async-std](https://async.rs/)

```toml
[dependencies]
gremlin_client = { version = "0.4.0", features = ["async_std"] }
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
use tokio::stream::StreamExt;

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
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<std::error::Error>> {

    let client = GremlinClient::connect("localhost").await?;

    let g = traversal().with_remote_async(client);

    let results = g.v(()).has_label("person").has(("name","Jon")).to_list().await?;   

    println!("{:?}", results);
    Ok(())
}
```


