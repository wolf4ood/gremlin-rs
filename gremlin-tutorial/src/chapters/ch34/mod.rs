mod ch342;

use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

pub fn all(
) -> Vec<Box<Fn(&GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(ch342::chapter_342)]
}
