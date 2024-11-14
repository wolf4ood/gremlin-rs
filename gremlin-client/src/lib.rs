//! Experimental Rust client for Apache Tinkerpop Gremlin Server.
//! The driver supports the execution of raw Gremlin queries and GLV traversal
//!
//!
//! You can use gremlin-client this lines in your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! gremlin-client = "*"
//! ```
//!
//! For async support via [async-std](https://async.rs/)
//! ```toml
//! [dependencies]
//! gremlin-client =  { version = "*", features = ["async_std"] }
//! ```
//!
//!
//! Here it is an usage example:
//!
//! **Synchronous**
//!
//! ```rust,no_run
//!     
//! use gremlin_client::{GremlinClient, Vertex};
//!
//! fn main() -> Result<(), Box<std::error::Error>> {
//!    let client = GremlinClient::connect("localhost")?;
//!
//!    let results = client
//!        .execute("g.V(param)", &[("param", &1)])?
//!        .filter_map(Result::ok)
//!        .map(|f| f.take::<Vertex>())
//!        .collect::<Result<Vec<Vertex>, _>>()?;
//!
//!    println!("{:?}", results);
//!
//!    Ok(())
//!}
//!
//!
//! ```
//!
//! **Asynchronous**
//!
//! ```rust,no_run,ignore
//!     
//! use gremlin_client::{aio::GremlinClient, Vertex};
//! use async_std::task;
//! use async_std::prelude::*;
//!
//! fn main() -> Result<(), Box<std::error::Error>> {
//!
//!    task::block_on(async {
//!     let client = GremlinClient::connect("localhost").await?;
//!     let results = client
//!            .execute("g.V(param)", &[("param", &1)]).await?
//!         .filter_map(Result::ok)
//!         .map(|f| f.take::<Vertex>())
//!         .collect::<Result<Vec<Vertex>, _>>().await?;
//!         println!("{:?}", results);
//!         Ok(())
//!    })    
//!
//!}
//!
//!
//! ```
//!
//! Here it is an example with traversal:
//!
//! **Synchronous**
//!
//! ```rust,no_run
//!     
//! use gremlin_client::{GremlinClient, Vertex, process::traversal::traversal};
//!
//! fn main() -> Result<(), Box<std::error::Error>> {
//!    let client = GremlinClient::connect("localhost")?;
//!
//!    let g = traversal().with_remote(client);
//!
//!    let results = g.v(()).has_label("person").has(("name","Jon")).to_list()?;   
//!    
//!    println!("{:?}", results);
//!    Ok(())
//!}
//!
//! ```
//! **Aynchronous**
//!
//! ```rust,no_run,ignore
//!     
//! use gremlin_client::{aio::GremlinClient, Vertex, process::traversal::traversal};
//! use async_std::task;
//! use async_std::prelude::*;
//!
//! fn main() -> Result<(), Box<std::error::Error>> {
//!
//!     task::block_on(async {
//!
//!         let client = GremlinClient::connect("localhost").await?;
//!
//!         let g = traversal().with_remote_async(client);
//!
//!         let results = g.v(()).has_label("person").has(("name","Jon")).to_list().await?;   
//!    
//!         println!("{:?}", results);
//!         Ok(())
//!    })
//!}
//!
#[macro_use]
extern crate lazy_static;

mod client;
mod connection;
mod conversion;
mod error;
mod io;
mod message;
mod pool;

pub use client::GremlinClient;
pub use connection::{
    ConnectionOptions, ConnectionOptionsBuilder, TlsOptions, WebSocketOptions,
    WebSocketOptionsBuilder,
};
pub use conversion::{BorrowFromGValue, FromGValue, ToGValue};
pub use error::GremlinError;
pub use io::IoProtocol;
pub use message::Message;

pub type GremlinResult<T> = Result<T, error::GremlinError>;

pub use structure::{
    Cardinality, Edge, GKey, GResultSet, GValue, IntermediateRepr, List, Map, Metric, Path,
    Property, Token, TraversalExplanation, TraversalMetrics, Vertex, VertexProperty, GID,
};

#[cfg(feature = "async_gremlin")]
pub mod aio;

pub mod process;
pub mod structure;
pub mod utils;

#[cfg(feature = "derive")]
pub mod derive {
    pub use gremlin_derive::FromGMap;
    pub use gremlin_derive::FromGValue;
}
