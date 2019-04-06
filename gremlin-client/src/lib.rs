//! Experimental Rust client for Apache Tinkerpop Gremlin Server.
//! The driver supports only the execution of raw Gremlin queries
//!
//!
//! You can use gremlin-client this lines in your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! gremlin-client = "*"
//! ```
//!
//! Here it is an usage example:
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

pub use client::GremlinClient;
pub use connection::ConnectionOptions;
pub use error::GremlinError;

pub use conversion::ToGValue;

pub type GremlinResult<T> = Result<T, error::GremlinError>;

pub use structure::{
    Edge, GKey, GResultSet, GValue, IntermediateRepr, List, Map, Metric, Path, Property, Token,
    TraversalExplanation, TraversalMetrics, Vertex, VertexProperty, GID,
};
pub mod process;

mod client;
mod connection;
mod error;
mod io;
mod message;
mod pool;

mod structure;

mod conversion;
